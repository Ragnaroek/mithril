extern crate blake2b_simd;

use self::blake2b_simd::{blake2b, Hash};
use super::program::{Instr, Store, Mode, MAX_REG, MAX_FLOAT_REG};
use super::m128::{m128, zero_m128};

pub struct Register {
    pub r: [u64; MAX_REG as usize],
    pub f: [m128; MAX_FLOAT_REG as usize],
    pub e: [m128; MAX_FLOAT_REG as usize],
    pub a: [m128; MAX_FLOAT_REG as usize],
}

pub fn new_register() -> Register {
    Register {
        r: [0; MAX_REG as usize],
        f: [zero_m128(); MAX_FLOAT_REG as usize],
        e: [zero_m128(); MAX_FLOAT_REG as usize],
        a: [zero_m128(); MAX_FLOAT_REG as usize],
    }
}

pub struct Vm {
    pub reg: Register,
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
}

fn shift_mode(instr: &Instr) -> u8 {
    match instr.mode {
        Mode::Shft(x) => x,
        _ => panic!("illegal mode {}", instr.mode),
    }
}

pub fn new_vm() -> Vm {
    Vm{reg: new_register()}
}
