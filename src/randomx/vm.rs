extern crate blake2b_simd;

use self::blake2b_simd::{blake2b, Hash};
use super::program::{Program, Instr, Store, Mode, MAX_REG, MAX_FLOAT_REG};
use super::m128::{m128d, m128i};
use super::hash::{fill_aes_1rx4_u64, gen_program_aes_4rx4};
use std::convert::TryInto;
use std::arch::x86_64::{_mm_setcsr, _mm_getcsr};

pub const SCRATCHPAD_L1_MASK : u64 = 0x3ff8;
pub const SCRATCHPAD_L2_MASK : u64 = 0x3fff8;
pub const SCRATCHPAD_L3_MASK : u64 = 0x1ffff8;

const SCRATCHPAD_SIZE : usize = 262144;
const MXCSR_DEFAULT : u32 = 0x9FC0;
const CONDITION_OFFSET : u64 = 8;
const CONDITION_MASK : u64 = (1 << CONDITION_OFFSET) - 1;
const RANDOMX_PROGRAM_ITERATIONS : usize = 2048;

const P_2EXP63 : u64 = 1 << 63;
const MANTISSA_SIZE : u64 = 52;
const MANTISSA_MASK : u64 = (1 << MANTISSA_SIZE) - 1;
const EXPONENT_SIZE : u64 = 11;
const EXPONENT_BIAS : u64 = 1023;
const EXPONENT_MASK : u64 = (1 << EXPONENT_SIZE) - 1;
const EXPONENT_BITS : u64 = 0x300;
const DYNAMIC_EXPONENT_BITS : u64 = 4;
const STATIC_EXPONENT_BITS : u64 = 4;
const DYNAMIC_MANTISSA_MASK : u64 = (1 << (MANTISSA_SIZE + DYNAMIC_EXPONENT_BITS)) - 1;

const RANDOMX_PROGRAM_COUNT : usize = 8;

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

pub struct VmConfig {
    pub e_mask: [u64; 2], 
}

pub struct Vm {
    pub reg: Register,
    pub scratchpad: Vec<u64>,
    pub pc : usize,
    pub config: VmConfig,
}

impl Vm {

    pub fn init_vm(&mut self, prog: &Program) {
        self.reg.a[0] = m128d::from_u64(small_positive_float_bit(prog.entropy[1]), small_positive_float_bit(prog.entropy[0]));
        self.reg.a[1] = m128d::from_u64(small_positive_float_bit(prog.entropy[3]), small_positive_float_bit(prog.entropy[2]));
        self.reg.a[2] = m128d::from_u64(small_positive_float_bit(prog.entropy[5]), small_positive_float_bit(prog.entropy[4]));
        self.reg.a[3] = m128d::from_u64(small_positive_float_bit(prog.entropy[7]), small_positive_float_bit(prog.entropy[6]));

        self.config.e_mask[0] = float_mask(prog.entropy[14]);
        self.config.e_mask[1] = float_mask(prog.entropy[15]);
    }

    pub fn init_scratchpad(&mut self, seed: &[m128i;4]) -> [m128i;4] {
        fill_aes_1rx4_u64(seed, &mut self.scratchpad)
    }

    pub fn calculate_hash(&mut self, input: &str) -> Hash {
        let hash = blake2b(input.as_bytes());
        let seed = hash_to_m128i_array(&hash);
        let seed = self.init_scratchpad(&seed);
        self.reset_rounding_mode();

        for _ in 0..RANDOMX_PROGRAM_COUNT {
            self.run(&seed);
            //TODO generate hash from register state!
        }
        /* TODO
		for (int chain = 0; chain < RANDOMX_PROGRAM_COUNT - 1; ++chain) {
			machine->run(&tempHash);
			blakeResult = blake2b(tempHash, sizeof(tempHash), machine->getRegisterFile(), sizeof(randomx::RegisterFile), nullptr, 0);
			assert(blakeResult == 0);
		}
		machine->run(&tempHash);
		machine->getFinalResult(output, RANDOMX_HASH_SIZE);
		fesetenv(&fpstate);
        */
        hash
    }
    
    /// Runs one round
    pub fn run(&mut self, seed: &[m128i;4]) {
        let prog = Program::from_bytes(gen_program_aes_4rx4(seed, 136));
        self.init_vm(&prog);

        for _ in 0..RANDOMX_PROGRAM_ITERATIONS {
            for instr in &prog.program {
                instr.execute(self);
            }   
        }
    }

    pub fn reset_rounding_mode(&mut self) {
        unsafe {
            _mm_setcsr(MXCSR_DEFAULT);
        }
    }

    pub fn set_rounding_mode(&mut self, mode: u32) {
        unsafe {
            _mm_setcsr(MXCSR_DEFAULT | (mode << 13))
        }
    }

    pub fn get_rounding_mode(&self) -> u32 {
        unsafe {
            (_mm_getcsr() >> 13) & 3
        } 
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
        let v_src = m128i::from_u64(0, v).to_m128d();
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
        let v_src = m128i::from_u64(0, v).to_m128d();
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
        let v_src = self.mask_register_exponent_mantissa(m128i::from_u64(0, v).to_m128d());
        let v_dst = self.read_e(&instr.dst);
        self.write_e(&instr.dst, v_dst / v_src); 
    }

    //i...

    pub fn exec_iadd_m(&mut self, instr: &Instr) {
        let ix = self.scratchpad_src_ix(instr);
        self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_add(self.scratchpad[ix]));
    }

    pub fn exec_isub_m(&mut self, instr: &Instr) {
        let ix = self.scratchpad_src_ix(instr);
        self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_sub(self.scratchpad[ix]));
    }

    pub fn exec_imul_m(&mut self, instr: &Instr) {
        let ix = self.scratchpad_src_ix(instr);
        self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_mul(self.scratchpad[ix]));
    }
   
    pub fn exec_iadd_rs(&mut self, instr: &Instr) {
        let mut v = self.read_r(&instr.src) << shift_mode(instr);
        if let Some(imm) = instr.imm {
            v = v.wrapping_add(u64_imm(imm));
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
        let v_src = self.read_r(&instr.src) as u128;
        let v_dst = self.read_r(&instr.dst) as u128;
        self.write_r(&instr.dst, (v_src.wrapping_mul(v_dst) >> 64) as u64);
    }

    pub fn exec_imulh_m(&mut self, instr: &Instr) {
        let v_dst = self.read_r(&instr.dst) as u128;
        let v_src = self.scratchpad[self.scratchpad_src_ix(instr)] as u128;
        self.write_r(&instr.dst, (v_src.wrapping_mul(v_dst) >> 64) as u64);
    }

    pub fn exec_ismulh_r(&mut self, instr: &Instr) {
        let v_src = (self.read_r(&instr.src) as i64) as i128; //we have to go through i64 to get the proper complement version in i128 if the u64 is negative in i64
        let v_dst = (self.read_r(&instr.dst) as i64) as i128; 
        self.write_r(&instr.dst, (v_src.wrapping_mul(v_dst) >> 64) as u64); 
    }

    pub fn exec_ismulh_m(&mut self, instr: &Instr) {
        let v_src = (self.scratchpad[self.scratchpad_src_ix(instr)] as i64) as i128;
        let v_dst = (self.read_r(&instr.dst) as i64) as i128; 
        self.write_r(&instr.dst, (v_src.wrapping_mul(v_dst) >> 64) as u64);
    }

    pub fn exec_ineg_r(&mut self, instr: &Instr) {
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, !v_dst + 1);
    }

    pub fn exec_ixor_r(&mut self, instr: &Instr) {
        let v_src = self.imm_or_r(instr);
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, v_dst^v_src);
    }

    pub fn exec_ixor_m(&mut self, instr: &Instr) {
        let v_src = self.scratchpad[self.scratchpad_src_ix(instr)];
        let v_dst = self.read_r(&instr.dst);
        self.write_r(&instr.dst, v_dst^v_src);
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
        let mut imm = u64_imm(instr.imm.unwrap()) | 1 << shift;
        imm &= !(1 << (shift - 1));
        let v_dst = self.read_r(&instr.dst).wrapping_add(imm);

        self.write_r(&instr.dst, v_dst);
        if v_dst & (CONDITION_MASK << shift) == 0 {
            self.pc = instr.target.unwrap() as usize;   
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
            Store::F(i) => self.reg.f[*i] = v ,
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
        let imm = u64_imm(instr.imm.unwrap());
        let addr : usize = match &instr.src {
            Store::L1(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L1_MASK,
            Store::L2(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L2_MASK,
            Store::L3(_) => imm & SCRATCHPAD_L3_MASK,
            _ => panic!("illegal read from scratchpad"),
        }.try_into().unwrap();
        addr / 8
    }

    fn scratchpad_dst_ix(&self, instr: &Instr) -> usize {
        let imm = u64_imm(instr.imm.unwrap());
        let addr : usize = match &instr.dst {
            Store::L1(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L1_MASK,
            Store::L2(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L2_MASK,
            Store::L3(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L3_MASK,
            _ => panic!("illegal read from scratchpad"),
        }.try_into().unwrap();
        addr / 8
    }

    fn mask_register_exponent_mantissa(&self, v: m128d) -> m128d {
        let mantissa_mask = m128d::from_u64(DYNAMIC_MANTISSA_MASK, DYNAMIC_MANTISSA_MASK);
        let exponent_mask = m128d::from_u64(self.config.e_mask[1], self.config.e_mask[0]);
        (v & mantissa_mask) | exponent_mask
    }
}

pub fn hash_to_m128i_array(hash: &Hash) -> [m128i;4] {
    let bytes = hash.as_bytes();
    let i1 = m128i::from_u8(&bytes[0..16]);
    let i2 = m128i::from_u8(&bytes[16..32]);
    let i3 = m128i::from_u8(&bytes[32..48]);
    let i4 = m128i::from_u8(&bytes[48..64]);
    [i1, i2, i3, i4]
}

fn u64_imm(imm: i32) -> u64 {
    (imm as u64) | 0xffffffff00000000
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

/*
    Directly taken from: https://github.com/tevador/RandomX    
    Calculates rcp = 2**x / divisor for highest integer x such that rcp < 2**64.
	divisor must not be 0 or a power of 2

	Equivalent x86 assembly (divisor in rcx):

	mov edx, 1
	mov r8, rcx
	xor eax, eax
	bsr rcx, rcx
	shl rdx, cl
	div r8
	ret
*/
pub fn randomx_reciprocal(divisor: u64) -> u64 {
    assert_ne!(divisor, 0);

    let mut quotient = P_2EXP63 / divisor;
    let mut remainder = P_2EXP63 % divisor;
    let mut bsr = 0;

    let mut bit = divisor;

    loop {
        if bit == 0 {
            break;
        }
        bsr += 1;
        bit >>= 1;
    }

    for _ in 0..bsr {        
        if remainder >= divisor.wrapping_sub(remainder) {
            quotient = quotient.wrapping_mul(2).wrapping_add(1);
            remainder = remainder.wrapping_mul(2).wrapping_sub(divisor);
        } else {
            quotient = quotient.wrapping_mul(2);
            remainder = remainder.wrapping_mul(2);
        }
    }
    quotient
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

pub fn new_vm() -> Vm {
    Vm{reg: new_register(), scratchpad: vec![0; SCRATCHPAD_SIZE], pc: 0, config: VmConfig{e_mask: [0, 2]}}
}
