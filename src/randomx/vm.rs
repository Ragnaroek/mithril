extern crate blake2b_simd;

use self::blake2b_simd::{blake2b, Hash, Params};
use super::common::{mulh, randomx_reciprocal, smulh, u64_from_i32_imm};
use super::hash::{fill_aes_1rx4_u64, gen_program_aes_4rx4, hash_aes_1rx4};
use super::m128::{m128d, m128i};
use super::memory::{VmMemory, CACHE_LINE_SIZE};
use super::program::{Instr, Mode, Program, Store, MAX_FLOAT_REG, MAX_REG};
use std::arch::x86_64::{_mm_getcsr, _mm_setcsr};
use std::convert::TryInto;
use std::sync::Arc;

pub const SCRATCHPAD_L1_MASK: u64 = 0x3ff8;
pub const SCRATCHPAD_L2_MASK: u64 = 0x3fff8;
pub const SCRATCHPAD_L3_MASK: u64 = 0x1ffff8;
const SCRATCHPAD_L3_MASK_U32: u32 = 0x1fffc0;

const SCRATCHPAD_SIZE: usize = 262144;
const MXCSR_DEFAULT: u32 = 0x9FC0;
const CONDITION_OFFSET: u64 = 8;
const CONDITION_MASK: u64 = (1 << CONDITION_OFFSET) - 1;

const RANDOMX_PROGRAM_COUNT: usize = 8;
const RANDOMX_PROGRAM_SIZE: i32 = 256;
const RANDOMX_PROGRAM_ITERATIONS: usize = 2048;
const RANDOMX_DATASET_BASE_SIZE: usize = 2147483648;
const RANDOMX_DATASET_ITEM_SIZE: usize = 64;
const RANDOMX_DATASET_EXTRA_SIZE: usize = 33554368;
const RANDOMX_HASH_SIZE: usize = 32;

const DATASET_EXTRA_ITEMS: usize = RANDOMX_DATASET_EXTRA_SIZE / RANDOMX_DATASET_ITEM_SIZE;

const MANTISSA_SIZE: u64 = 52;
const MANTISSA_MASK: u64 = (1 << MANTISSA_SIZE) - 1;
const EXPONENT_SIZE: u64 = 11;
const EXPONENT_BIAS: u64 = 1023;
const EXPONENT_MASK: u64 = (1 << EXPONENT_SIZE) - 1;
const EXPONENT_BITS: u64 = 0x300;
const DYNAMIC_EXPONENT_BITS: u64 = 4;
const STATIC_EXPONENT_BITS: u64 = 4;
const DYNAMIC_MANTISSA_MASK: u64 = (1 << (MANTISSA_SIZE + DYNAMIC_EXPONENT_BITS)) - 1;

const CACHE_LINE_ALIGN_MASK: u64 =
    ((RANDOMX_DATASET_BASE_SIZE - 1) & !(RANDOMX_DATASET_ITEM_SIZE - 1)) as u64;

pub struct MemoryRegister {
    pub mx: usize,
    pub ma: usize,
}

pub struct Register {
    pub r: [u64; MAX_REG as usize],
    pub f: [m128d; MAX_FLOAT_REG as usize],
    pub e: [m128d; MAX_FLOAT_REG as usize],
    pub a: [m128d; MAX_FLOAT_REG as usize],
}

pub fn new_register() -> Register {
    Register {
        r: [0; MAX_REG as usize],
        f: [m128d::zero(); MAX_FLOAT_REG as usize],
        e: [m128d::zero(); MAX_FLOAT_REG as usize],
        a: [m128d::zero(); MAX_FLOAT_REG as usize],
    }
}

impl Register {
    pub fn to_bytes(&self) -> [u8; 256] {
        let mut bytes = [0; 256];
        let mut offset = 0;
        for i in 0..MAX_REG {
            Register::copy_into_le(&mut bytes, offset, self.r[i]);
            offset += 1;
        }

        for i in 0..MAX_FLOAT_REG {
            let (h, l) = self.f[i].as_u64();
            Register::copy_into_le(&mut bytes, offset, l);
            offset += 1;
            Register::copy_into_le(&mut bytes, offset, h);
            offset += 1;
        }

        for i in 0..MAX_FLOAT_REG {
            let (h, l) = self.e[i].as_u64();
            Register::copy_into_le(&mut bytes, offset, l);
            offset += 1;
            Register::copy_into_le(&mut bytes, offset, h);
            offset += 1;
        }

        for i in 0..MAX_FLOAT_REG {
            let (h, l) = self.a[i].as_u64();
            Register::copy_into_le(&mut bytes, offset, l);
            offset += 1;
            Register::copy_into_le(&mut bytes, offset, h);
            offset += 1;
        }

        bytes
    }

    fn copy_into_le(bytes: &mut [u8; 256], offset: usize, u: u64) {
        let reg_bytes = u.to_le_bytes();
        for k in 0..8 {
            bytes[offset * 8 + k] = reg_bytes[k];
        }
    }
}

pub struct VmConfig {
    pub e_mask: [u64; 2],
    pub read_reg: [usize; 4],
}

pub struct Vm {
    pub mem_reg: MemoryRegister,
    pub reg: Register,
    pub scratchpad: Vec<u64>,
    pub pc: i32,
    pub config: VmConfig,
    pub mem: Arc<VmMemory>,
    pub dataset_offset: u64,
}

impl Vm {
    pub fn init_vm(&mut self, prog: &Program) {
        self.reg.a[0] = m128d::from_u64(
            small_positive_float_bit(prog.entropy[1]),
            small_positive_float_bit(prog.entropy[0]),
        );
        self.reg.a[1] = m128d::from_u64(
            small_positive_float_bit(prog.entropy[3]),
            small_positive_float_bit(prog.entropy[2]),
        );
        self.reg.a[2] = m128d::from_u64(
            small_positive_float_bit(prog.entropy[5]),
            small_positive_float_bit(prog.entropy[4]),
        );
        self.reg.a[3] = m128d::from_u64(
            small_positive_float_bit(prog.entropy[7]),
            small_positive_float_bit(prog.entropy[6]),
        );

        self.mem_reg.ma = ((prog.entropy[8] & CACHE_LINE_ALIGN_MASK) as u32) as usize;
        self.mem_reg.mx = (prog.entropy[10] as u32) as usize;

        let mut address_reg = prog.entropy[12] as usize;
        self.config.read_reg[0] = address_reg & 1;
        address_reg >>= 1;
        self.config.read_reg[1] = 2 + (address_reg & 1);
        address_reg >>= 1;
        self.config.read_reg[2] = 4 + (address_reg & 1);
        address_reg >>= 1;
        self.config.read_reg[3] = 6 + (address_reg & 1);

        self.dataset_offset =
            (prog.entropy[13] % (DATASET_EXTRA_ITEMS as u64 + 1)) * CACHE_LINE_SIZE;

        self.config.e_mask[0] = float_mask(prog.entropy[14]);
        self.config.e_mask[1] = float_mask(prog.entropy[15]);

        for i in 0..MAX_REG {
            self.reg.r[i] = 0;
        }
    }

    pub fn init_scratchpad(&mut self, seed: &[m128i; 4]) -> [m128i; 4] {
        fill_aes_1rx4_u64(seed, &mut self.scratchpad)
    }

    pub fn calculate_hash(&mut self, input: &[u8]) -> Hash {
        let hash = blake2b(input);
        let seed = hash_to_m128i_array(&hash);

        let mut tmp_hash = self.init_scratchpad(&seed);
        self.reset_rounding_mode();

        for _ in 0..(RANDOMX_PROGRAM_COUNT - 1) {
            self.run(&tmp_hash);
            let blake_result = blake2b(&self.reg.to_bytes());
            tmp_hash = hash_to_m128i_array(&blake_result);
        }

        self.run(&tmp_hash);
        let final_hash = hash_aes_1rx4(&self.scratchpad);
        self.reg.a[0] = final_hash[0].as_m128d();
        self.reg.a[1] = final_hash[1].as_m128d();
        self.reg.a[2] = final_hash[2].as_m128d();
        self.reg.a[3] = final_hash[3].as_m128d();

        let mut params = Params::new();
        params.hash_length(RANDOMX_HASH_SIZE);
        params.hash(&self.reg.to_bytes())
    }

    /// Runs one round
    pub fn run(&mut self, seed: &[m128i; 4]) {
        let prog = Program::from_bytes(gen_program_aes_4rx4(seed, 136));

        self.init_vm(&prog);

        let mut sp_addr_0: u32 = self.mem_reg.mx as u32;
        let mut sp_addr_1: u32 = self.mem_reg.ma as u32;

        for _ in 0..RANDOMX_PROGRAM_ITERATIONS {
            let sp_mix = self.reg.r[self.config.read_reg[0]] ^ self.reg.r[self.config.read_reg[1]];

            sp_addr_0 ^= sp_mix as u32;
            sp_addr_0 &= SCRATCHPAD_L3_MASK_U32;
            sp_addr_0 /= 8;
            sp_addr_1 ^= (sp_mix >> 32) as u32;
            sp_addr_1 &= SCRATCHPAD_L3_MASK_U32;
            sp_addr_1 /= 8;

            for i in 0..MAX_REG {
                self.reg.r[i] ^= self.scratchpad[sp_addr_0 as usize + i];
            }
            for i in 0..MAX_FLOAT_REG {
                self.reg.f[i] =
                    m128i::from_u64(0, self.scratchpad[sp_addr_1 as usize + i]).lower_to_m128d();
            }
            for i in 0..MAX_FLOAT_REG {
                self.reg.e[i] = self.mask_register_exponent_mantissa(
                    m128i::from_u64(0, self.scratchpad[sp_addr_1 as usize + i + MAX_FLOAT_REG])
                        .lower_to_m128d(),
                );
            }

            self.pc = 0;
            while self.pc < RANDOMX_PROGRAM_SIZE {
                let instr = &prog.program[self.pc as usize];
                instr.execute(self);
                self.pc += 1;
            }

            self.mem_reg.mx ^= (self.reg.r[self.config.read_reg[2]]
                ^ self.reg.r[self.config.read_reg[3]]) as usize;
            self.mem_reg.mx &= CACHE_LINE_ALIGN_MASK as usize;
            self.mem.dataset_prefetch(self.mem_reg.mx as u64);
            self.mem.dataset_read(
                self.dataset_offset + self.mem_reg.ma as u64,
                &mut self.reg.r,
            );

            std::mem::swap(&mut self.mem_reg.mx, &mut self.mem_reg.ma);

            for i in 0..MAX_REG {
                self.scratchpad[sp_addr_1 as usize + i] = self.reg.r[i];
            }
            for i in 0..MAX_FLOAT_REG {
                self.reg.f[i] = self.reg.f[i] ^ self.reg.e[i];
            }

            for i in 0..MAX_FLOAT_REG {
                let (u1, u0) = self.reg.f[i].as_u64();
                let ix = sp_addr_0 as usize + 2 * i;
                self.scratchpad[ix] = u0;
                self.scratchpad[ix + 1] = u1;
            }
            sp_addr_0 = 0;
            sp_addr_1 = 0;
        }
    }

    pub fn reset_rounding_mode(&mut self) {
        unsafe {
            _mm_setcsr(MXCSR_DEFAULT);
        }
    }

    pub fn set_rounding_mode(&mut self, mode: u32) {
        unsafe { _mm_setcsr(MXCSR_DEFAULT | (mode << 13)) }
    }

    pub fn get_rounding_mode(&self) -> u32 {
        unsafe { (_mm_getcsr() >> 13) & 3 }
    }

    //f...

    pub fn exec_fswap_r(&mut self, instr: &Instr) {
        let v_dst = self.read_float_reg(&instr.dst);
        self.write_float_reg(&instr.dst, v_dst.shuffle_1(&v_dst));
    }

    pub fn exec_fadd_r(&mut self, instr: &Instr) {
        let v_src = self.read_a(&instr.src);
        let v_dst = self.read_f(&instr.dst);
        self.write_f(&instr.dst, v_src + v_dst);
    }

    pub fn exec_fadd_m(&mut self, instr: &Instr) {
        let v = self.scratchpad[self.scratchpad_src_ix(instr)];
        let v_src = m128i::from_u64(0, v).lower_to_m128d();
        let v_dst = self.read_f(&instr.dst);
        self.write_f(&instr.dst, v_dst + v_src);
    }

    pub fn exec_fsub_r(&mut self, instr: &Instr) {
        let v_src = self.read_a(&instr.src);
        let v_dst = self.read_f(&instr.dst);
        self.write_f(&instr.dst, v_dst - v_src);
    }

    pub fn exec_fsub_m(&mut self, instr: &Instr) {
        let v = self.scratchpad[self.scratchpad_src_ix(instr)];
        let v_src = m128i::from_u64(0, v).lower_to_m128d();
        let v_dst = self.read_f(&instr.dst);
        self.write_f(&instr.dst, v_dst - v_src);
    }

    pub fn exec_fscal_r(&mut self, instr: &Instr) {
        let v_dst = self.read_f(&instr.dst);
        let mask = m128d::from_u64(0x80F0000000000000, 0x80F0000000000000);
        self.write_f(&instr.dst, v_dst ^ mask);
    }

    pub fn exec_fmul_r(&mut self, instr: &Instr) {
        let v_src = self.read_a(&instr.src);
        let v_dst = self.read_e(&instr.dst);
        self.write_e(&instr.dst, v_src * v_dst);
    }

    pub fn exec_fsqrt_r(&mut self, instr: &Instr) {
        let v_dst = self.read_e(&instr.dst);
        self.write_e(&instr.dst, v_dst.sqrt());
    }

    pub fn exec_fdiv_m(&mut self, instr: &Instr) {
        let v = self.scratchpad[self.scratchpad_src_ix(instr)];
        let v_src = self.mask_register_exponent_mantissa(m128i::from_u64(0, v).lower_to_m128d());
        let v_dst = self.read_e(&instr.dst);
        self.write_e(&instr.dst, v_dst / v_src);
    }

    //i...

    pub fn exec_iadd_m(&mut self, instr: &Instr) {
        let ix = self.scratchpad_src_ix(instr);
        self.write_r(
            &instr.dst,
            self.read_r(&instr.dst).wrapping_add(self.scratchpad[ix]),
        );
    }

    pub fn exec_isub_m(&mut self, instr: &Instr) {
        let ix = self.scratchpad_src_ix(instr);
        self.write_r(
            &instr.dst,
            self.read_r(&instr.dst).wrapping_sub(self.scratchpad[ix]),
        );
    }

    pub fn exec_imul_m(&mut self, instr: &Instr) {
        let ix = self.scratchpad_src_ix(instr);
        self.write_r(
            &instr.dst,
            self.read_r(&instr.dst).wrapping_mul(self.scratchpad[ix]),
        );
    }
    pub fn exec_iadd_rs(&mut self, instr: &Instr) {
        let mut v = self.read_r(&instr.src) << shift_mode(instr);
        if let Some(imm) = instr.imm {
            v = v.wrapping_add(u64_from_i32_imm(imm));
        }
        self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_add(v));
    }
    pub fn exec_isub_r(&mut self, instr: &Instr) {
        let v = self.imm_or_r(instr);
        self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_sub(v));
    }

    pub fn exec_imul_r(&mut self, instr: &Instr) {
        let v = self.imm_or_r(instr);
        self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_mul(v));
    }

    pub fn exec_imul_rcp(&mut self, instr: &Instr) {
        if !is_zero_or_power_of_2(instr.imm.unwrap() as u64) {
            let v = randomx_reciprocal((instr.imm.unwrap() as u64) & 0x00000000FFFFFFFF);
            self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_mul(v));
        } //else: nop
    }

    pub fn exec_imulh_r(&mut self, instr: &Instr) {
        let v_src = self.read_r(&instr.src);
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, mulh(v_src, v_dst));
    }

    pub fn exec_imulh_m(&mut self, instr: &Instr) {
        let v_dst = self.read_r(&instr.dst);
        let v_src = self.scratchpad[self.scratchpad_src_ix(instr)];
        self.write_r(&instr.dst, mulh(v_src, v_dst));
    }

    pub fn exec_ismulh_r(&mut self, instr: &Instr) {
        let v_src = self.read_r(&instr.src);
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, smulh(v_src, v_dst));
    }

    pub fn exec_ismulh_m(&mut self, instr: &Instr) {
        let v_src = self.scratchpad[self.scratchpad_src_ix(instr)];
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, smulh(v_src, v_dst));
    }

    pub fn exec_ineg_r(&mut self, instr: &Instr) {
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, (!v_dst).wrapping_add(1));
    }

    pub fn exec_ixor_r(&mut self, instr: &Instr) {
        let v_src = self.imm_or_r(instr);
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, v_dst ^ v_src);
    }

    pub fn exec_ixor_m(&mut self, instr: &Instr) {
        let v_src = self.scratchpad[self.scratchpad_src_ix(instr)];
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, v_dst ^ v_src);
    }

    pub fn exec_iror_r(&mut self, instr: &Instr) {
        let v_src = (self.imm_or_r(instr) & 0xFFFFFF) as u32;
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, v_dst.rotate_right(v_src));
    }

    pub fn exec_irol_r(&mut self, instr: &Instr) {
        let v_src = (self.imm_or_r(instr) & 0xFFFFFF) as u32;
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, v_dst.rotate_left(v_src));
    }

    pub fn exec_iswap_r(&mut self, instr: &Instr) {
        let v_src = self.read_r(&instr.src);
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, v_src);
        self.write_r(&instr.src, v_dst);
    }

    pub fn exec_istore(&mut self, instr: &Instr) {
        let ix = self.scratchpad_dst_ix(instr);
        self.scratchpad[ix] = self.read_r(&instr.src);
    }

    //c..

    pub fn exec_cfround(&mut self, instr: &Instr) {
        let v_src = self.read_r(&instr.src);
        let mode = (v_src.rotate_right(instr.imm.unwrap() as u32) % 4) as u32;
        self.set_rounding_mode(mode);
    }

    pub fn exec_cbranch(&mut self, instr: &Instr) {
        let shift = cond_mode(instr) as u64 + CONDITION_OFFSET;
        let mut imm = u64_from_i32_imm(instr.imm.unwrap()) | 1 << shift;
        if CONDITION_OFFSET > 0 || shift > 0 {
            imm &= !(1 << (shift - 1));
        }
        let v_dst = self.read_r(&instr.dst).wrapping_add(imm);
        self.write_r(&instr.dst, v_dst);
        if v_dst & (CONDITION_MASK << shift) == 0 {
            self.pc = instr.target.unwrap();
        }
    }

    //helper

    fn imm_or_r(&self, instr: &Instr) -> u64 {
        if instr.src == Store::NONE {
            return instr.imm.unwrap() as u64;
        }
        self.read_r(&instr.src)
    }

    fn read_float_reg(&self, store: &Store) -> m128d {
        match store {
            Store::A(i) => self.reg.a[*i],
            Store::E(i) => self.reg.e[*i],
            Store::F(i) => self.reg.f[*i],
            _ => panic!("illegal read from float register"),
        }
    }

    fn write_float_reg(&mut self, store: &Store, v: m128d) {
        match store {
            Store::A(i) => self.reg.a[*i] = v,
            Store::E(i) => self.reg.e[*i] = v,
            Store::F(i) => self.reg.f[*i] = v,
            _ => panic!("illegal write to float register"),
        }
    }

    fn read_r(&self, store: &Store) -> u64 {
        match store {
            Store::R(i) => self.reg.r[*i],
            _ => panic!("illegal read from register r"),
        }
    }
    fn write_r(&mut self, store: &Store, v: u64) {
        match store {
            Store::R(i) => self.reg.r[*i] = v,
            _ => panic!("illegal store to register r"),
        }
    }
    fn read_f(&self, store: &Store) -> m128d {
        match store {
            Store::F(i) => self.reg.f[*i],
            _ => panic!("illegal read from register f"),
        }
    }
    fn write_f(&mut self, store: &Store, v: m128d) {
        match store {
            Store::F(i) => self.reg.f[*i] = v,
            _ => panic!("illegal store to register f"),
        }
    }

    fn read_a(&self, store: &Store) -> m128d {
        match store {
            Store::A(i) => self.reg.a[*i],
            _ => panic!("illegal read from register a"),
        }
    }

    fn read_e(&self, store: &Store) -> m128d {
        match store {
            Store::E(i) => self.reg.e[*i],
            _ => panic!("illegal read from register e"),
        }
    }

    fn write_e(&mut self, store: &Store, v: m128d) {
        match store {
            Store::E(i) => self.reg.e[*i] = v,
            _ => panic!("illegal store to register e"),
        }
    }
    fn scratchpad_src_ix(&self, instr: &Instr) -> usize {
        let imm = u64_from_i32_imm(instr.imm.unwrap());
        let addr: usize = match &instr.src {
            Store::L1(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L1_MASK,
            Store::L2(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L2_MASK,
            Store::L3(_) => imm & SCRATCHPAD_L3_MASK,
            _ => panic!("illegal read from scratchpad"),
        }
        .try_into()
        .unwrap();
        addr / 8
    }

    fn scratchpad_dst_ix(&self, instr: &Instr) -> usize {
        let imm = u64_from_i32_imm(instr.imm.unwrap());
        let addr: usize = match &instr.dst {
            Store::L1(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L1_MASK,
            Store::L2(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L2_MASK,
            Store::L3(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L3_MASK,
            _ => panic!("illegal read from scratchpad"),
        }
        .try_into()
        .unwrap();
        addr / 8
    }

    fn mask_register_exponent_mantissa(&self, v: m128d) -> m128d {
        let mantissa_mask = m128d::from_u64(DYNAMIC_MANTISSA_MASK, DYNAMIC_MANTISSA_MASK);
        let exponent_mask = m128d::from_u64(self.config.e_mask[1], self.config.e_mask[0]);
        (v & mantissa_mask) | exponent_mask
    }
}

pub fn hash_to_m128i_array(hash: &Hash) -> [m128i; 4] {
    let bytes = hash.as_bytes();
    let i1 = m128i::from_u8(&bytes[0..16]);
    let i2 = m128i::from_u8(&bytes[16..32]);
    let i3 = m128i::from_u8(&bytes[32..48]);
    let i4 = m128i::from_u8(&bytes[48..64]);
    [i1, i2, i3, i4]
}

fn shift_mode(instr: &Instr) -> u8 {
    match instr.mode {
        Mode::Shft(x) => x,
        _ => panic!("illegal shift mode {}", instr.mode),
    }
}

fn cond_mode(instr: &Instr) -> u8 {
    match instr.mode {
        Mode::Cond(x) => x,
        _ => panic!("illegal cond mode {}", instr.mode),
    }
}

pub fn is_zero_or_power_of_2(imm: u64) -> bool {
    imm & imm.wrapping_sub(1) == 0
}

fn small_positive_float_bit(entropy: u64) -> u64 {
    let mut exponent = entropy >> 59; //0..31
    let mantissa = entropy & MANTISSA_MASK;
    exponent += EXPONENT_BIAS;
    exponent &= EXPONENT_MASK;
    exponent <<= MANTISSA_SIZE;
    exponent | mantissa
}

fn float_mask(entropy: u64) -> u64 {
    let mask22bit = (1 << 22) - 1;
    entropy & mask22bit | static_exponent(entropy)
}

fn static_exponent(entropy: u64) -> u64 {
    let mut exponent = EXPONENT_BITS;
    exponent |= (entropy >> (64 - STATIC_EXPONENT_BITS)) << DYNAMIC_EXPONENT_BITS;
    exponent << MANTISSA_SIZE
}

pub fn new_vm(mem: Arc<VmMemory>) -> Vm {
    Vm {
        mem_reg: MemoryRegister { mx: 0, ma: 0 },
        reg: new_register(),
        scratchpad: vec![0; SCRATCHPAD_SIZE],
        pc: 0,
        config: VmConfig {
            e_mask: [0; 2],
            read_reg: [0; 4],
        },
        mem,
        dataset_offset: 0,
    }
}
