extern crate blake2b_simd;

use self::blake2b_simd::{blake2b, Hash};
use super::program::{Instr, Store, Mode, MAX_REG, MAX_FLOAT_REG, SCRATCHPAD_L1_MASK, SCRATCHPAD_L2_MASK, SCRATCHPAD_L3_MASK};
use super::m128::{m128d, zero_m128d, from_u64};
use std::convert::TryInto;

const SCRATCHPAD_SIZE : usize = 262144;

pub struct Register {
    pub r: [u64; MAX_REG as usize],
    pub f: [m128d; MAX_FLOAT_REG as usize],
    pub e: [m128d; MAX_FLOAT_REG as usize],
    pub a: [m128d; MAX_FLOAT_REG as usize],
}

pub fn new_register() -> Register {
    Register {
        r: [0; MAX_REG as usize],
        f: [zero_m128d(); MAX_FLOAT_REG as usize],
        e: [zero_m128d(); MAX_FLOAT_REG as usize],
        a: [zero_m128d(); MAX_FLOAT_REG as usize],
    }
}

pub struct Vm {
    pub reg: Register,
    pub scratchpad: Box<Vec<u64>>,
}

impl Vm {
    pub fn calculate_hash(&self, input: &str) -> Hash {
        //TODO Implement, once all instructions are implemented
        let hash = blake2b(input.as_bytes());
        println!("##hash={:?}", hash);
        /*
		fenv_t fpstate;
		fegetenv(&fpstate);
		alignas(16) uint64_t tempHash[8];
		int blakeResult = blake2b(tempHash, sizeof(tempHash), input, inputSize, nullptr, 0);
		assert(blakeResult == 0);
		machine->initScratchpad(&tempHash);
		machine->resetRoundingMode();
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
    pub fn run() {
        //TODO Implement, once all instructions are implemented
        //generate program here from seed
    }
    
    pub fn exec_iadd_rs(&mut self, instr: &Instr) {
        let mut v = self.read_r(&instr.src) << shift_mode(instr);
        if let Some(imm) = instr.imm {
            v = v.wrapping_add(imm as u64 | 0xffffffff00000000);
        }
        self.write_r(&instr.dst, self.read_r(&instr.dst).wrapping_add(v));
    }
    
    pub fn exec_iadd_m(&mut self, instr: &Instr) {
        //TODO
    }
    
    pub fn exec_fadd_m(&mut self, instr: &Instr) {
        let ix = self.scratchpad_ix(instr);
        let v = self.scratchpad[ix];
        let iv = from_u64(0, v);
        self.write_f(&instr.dst, self.read_f(&instr.dst) + iv.to_m128d());
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
    
    fn scratchpad_ix(&self, instr: &Instr) -> usize {
        let imm = u64_imm(instr.imm.unwrap());
        let addr : usize = match &instr.src {
            Store::L1(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L1_MASK,
            Store::L2(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L2_MASK,
            Store::L3(d) => (self.read_r(d).wrapping_add(imm)) & SCRATCHPAD_L3_MASK,
            _ => panic!("illegal read from scratchpad"),
        }.try_into().unwrap();
        addr / 8
    }
}

fn u64_imm(imm: i32) -> u64 {
    (imm as u64) | 0xffffffff00000000
}

fn shift_mode(instr: &Instr) -> u8 {
    match instr.mode {
        Mode::Shft(x) => x,
        _ => panic!("illegal mode {}", instr.mode),
    }
}

pub fn new_vm() -> Vm {
    Vm{reg: new_register(), scratchpad: Box::new(vec![0; SCRATCHPAD_SIZE])}
}
