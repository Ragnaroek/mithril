use super::m128::{m128};
use super::vm::{Vm};
use strum::Display;
use std::fmt;

pub const MAX_FLOAT_REG : i64 = 4;
pub const MAX_REG : i64 = 8;
pub const REG_NEEDS_DISPLACEMENT_IX : usize = 5;
pub const REG_NEEDS_DISPLACEMENT: Store = Store::R(REG_NEEDS_DISPLACEMENT_IX);
const STORE_L3_CONDITION : u8 = 14;
//TODO Move Scratchpad_LX... to vm.rs
pub const SCRATCHPAD_L1_MASK : u64 = 0x3ff8;
pub const SCRATCHPAD_L2_MASK : u64 = 0x3fff8;
pub const SCRATCHPAD_L3_MASK : u64 = 0x1ffff8;
pub const NO_MASK : u64 = 0xffffffffffffffff;

#[allow(nonstandard_style)]
#[derive(Display)]
pub enum Opcode {
    NOP = 0,
    IADD_RS = 0x10,
    IADD_M = 0x17,
    ISUB_R = 0x27,
    ISUB_M = 0x2e,
    IMUL_R = 0x3e,
    IMUL_M = 0x42,
    IMULH_R = 0x46,
    IMULH_M = 0x47,
    ISMULH_R = 0x4b,
    ISMULH_M = 0x4c,
    IMUL_RCP = 0x54,
    INEG_R = 0x56,
    IXOR_R = 0x65,
    IXOR_M = 0x6a,
    IROR_R = 0x72,
    IROL_R = 0x74,
    ISWAP_R = 0x78,
    FSWAP_R = 0x7c,
    FADD_R = 0x8c,
    FADD_M = 0x91,
    FSUB_R = 0xa1,
    FSUB_M = 0xa6,
    FSCAL_R = 0xac,
    FMUL_R = 0xcc,
    FDIV_M = 0xd0,
    FSQRT_R = 0xd6,
    CBRANCH = 0xef,
    CFROUND = 0xf0,
    ISTORE = 0x100,
}

#[derive(Display, PartialEq)]
pub enum Store {
    NONE,
    //registers
    R(usize),
    F(usize),
    E(usize),
    A(usize),
    #[strum(serialize = "i")]
    Imm, //non-register based Lx access
    //Lx memory
    L1(Box<Store>),
    L2(Box<Store>),
    L3(Box<Store>),
}

#[derive(PartialEq)]
pub enum Mode {
    None,
    Cond(u8),
    Shft(u8),
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::None => write!(f, "NONE"),
            Mode::Cond(x) => write!(f, "COND {}", x),
            Mode::Shft(x) => write!(f, "SHFT {}", x),
        }
    }
}

pub struct Instr {
    pub op: Opcode,
    pub src: Store,
    pub dst: Store,
    pub imm: Option<i32>,
    pub unsigned_imm: bool,
    pub mode: Mode,
    pub mem_mask: u64, //TODO REMOVE, not used anymore
    pub effect: fn(&mut Vm, &Instr)
}

fn new_instr(op: Opcode, dst: Store, src: Store, imm: i32, mode: Mode) -> Instr {
    if src == dst {
        return Instr{op, dst, src: Store::NONE, imm: Some(imm), unsigned_imm: false, mem_mask: NO_MASK, mode, effect: nop};
    }
    Instr{op, dst, src, imm: None, unsigned_imm: false, mode, mem_mask: NO_MASK, effect: nop}
}

fn new_imm_instr(op: Opcode, dst: Store, imm: i32, mode: Mode) -> Instr {
    Instr{op, dst, src: Store::NONE, imm: Some(imm), unsigned_imm: false, mem_mask: NO_MASK, mode, effect: nop}
}
 
pub fn new_lcache_instr(op: Opcode, dst_reg: Store, src: i64, imm: i32, modi: u8, effect: fn(&mut Vm, &Instr)) -> Instr {
    let src_reg = r_reg(src);
    if src_reg == dst_reg {
        return Instr{op, dst: dst_reg, src: Store::L3(Box::new(Store::Imm)), imm: Some(imm & (SCRATCHPAD_L3_MASK as i32)), mem_mask: SCRATCHPAD_L3_MASK, unsigned_imm: false, mode: Mode::None, effect};
    }
    let (lx, msk) = l12_cache(src, modi);
    return Instr{op, dst: dst_reg, src: lx, mem_mask: msk, imm: Some(imm), unsigned_imm: false, mode: Mode::None, effect}

}

impl Instr {
    pub fn execute(&self, vm: &mut Vm) {
        (self.effect)(vm, self);
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.op)?;
        match &self.dst {
            Store::NONE => {/* do nothing */},
            Store::L1(reg) => write_l_access(f, self, reg, "L1")?,
            Store::L2(reg) => write_l_access(f, self, reg, "L2")?,
            Store::L3(reg) => write_l_access(f, self, reg, "L3")?,
            Store::R(i) => write!(f, "r{}", i)?,
            Store::F(i) => write!(f, "f{}", i)?,
            Store::E(i) => write!(f, "e{}", i)?,
            Store::A(i) => write!(f, "a{}", i)?,
            _ => write!(f, "{}", self.dst)?,
        }
        if self.dst != Store::NONE && self.src != Store::NONE {
            write!(f, ", ")?;
        }
        match &self.src {
            Store::NONE => {/* do nothing */},
            Store::L1(reg) => write_l_access(f, self, reg, "L1")?,
            Store::L2(reg) => write_l_access(f, self, reg, "L2")?,
            Store::L3(reg) => write_l_access(f, self, reg, "L3")?,
            Store::R(i) => write!(f, "r{}", i)?,
            Store::F(i) => write!(f, "f{}", i)?,
            Store::E(i) => write!(f, "e{}", i)?,
            Store::A(i) => write!(f, "a{}", i)?,
            _ => {
                write!(f, ", {}", self.src)?
            },
        }
        if self.imm.is_some() && !(is_l_cache(&self.dst) || is_l_cache(&self.src)) {
            if self.unsigned_imm {
                write!(f, ", {}", self.imm.unwrap() as u32)?
            } else {
                write!(f, ", {}", self.imm.unwrap())?
            }
        }
        if self.mode != Mode::None {
            write!(f, ", {}", self.mode)?;
        }
        Ok(())
    }
}

fn write_l_access(f: &mut fmt::Formatter<'_>, instr: &Instr, reg: &Store, lstore: &str) -> fmt::Result {
    if reg == &Store::Imm {
        write!(f, "{}[{}]", lstore, instr.imm.unwrap())
    } else {
        write!(f, "{}[", lstore)?;
        match reg {
            Store::R(i) => write!(f, "r{}", i)?,
            Store::F(i) => write!(f, "f{}", i)?,
            Store::E(i) => write!(f, "e{}", i)?,
            Store::A(i) => write!(f, "a{}", i)?,
            _ => write!(f, "{}", reg)?,
        }
        write!(f, "{:+}]", instr.imm.unwrap())
    }
}

pub struct Program {
    program: Vec<Instr>
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for instr in &self.program {
            write!(f, "{}\n", instr)?;
        }
        Ok(())
    }
}

pub fn from_bytes(bytes: Vec<m128>) -> Program {
    
    let mut program = Vec::with_capacity((bytes.len() - 8) * 2);
    
    //first 8 m128 are generated for entropy. We skip them.
    for i in 8..bytes.len() {
        let (op2, op1) = bytes[i].to_i64();
        let instr1 = decode_instruction(op1);
        let instr2 = decode_instruction(op2);
        program.push(instr1);
        program.push(instr2);
    }
    
    Program{program}
}

#[allow(overflowing_literals)]
fn decode_instruction(bytes: i64) -> Instr {
    let op = bytes & 0xFF;
    let dst = (bytes & 0xFF00) >> 8;
    let src = (bytes & 0xFF0000) >> 16;
    let modi = ((bytes & 0xFF000000) >> 24) as u8;
    let imm = ((bytes & 0xFFFFFFFF00000000) >> 32) as i32;
    
    if op < Opcode::IADD_RS as i64 {
        let dst_reg = r_reg(dst);
        let imm_val;
        if dst_reg == REG_NEEDS_DISPLACEMENT {
            imm_val = Some(imm);
        } else {
            imm_val = None;
        }
        return Instr{op: Opcode::IADD_RS, dst: dst_reg, src: r_reg(src), imm: imm_val, mem_mask: NO_MASK, unsigned_imm: false, mode: mod_shft(modi), effect: Vm::exec_iadd_rs}
    }
    if op < Opcode::IADD_M as i64 {
        return new_lcache_instr(Opcode::IADD_M, r_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::ISUB_R as i64 {
        return new_instr(Opcode::ISUB_R, r_reg(dst), r_reg(src), imm, Mode::None);
    }
    if op < Opcode::ISUB_M as i64 {
        return new_lcache_instr(Opcode::ISUB_M, r_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::IMUL_R as i64 {
        return new_instr(Opcode::IMUL_R, r_reg(dst), r_reg(src), imm, Mode::None);
    }
    if op < Opcode::IMUL_M as i64 {
        return new_lcache_instr(Opcode::IMUL_M, r_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::IMULH_R as i64 {
        return Instr{op: Opcode::IMULH_R, dst: r_reg(dst), src: r_reg(src), imm: None, mem_mask: NO_MASK, unsigned_imm: false, mode: Mode::None, effect: nop}
    }
    if op < Opcode::IMULH_M as i64 {
        return new_lcache_instr(Opcode::IMULH_M, r_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::ISMULH_R as i64 {
        return Instr{op: Opcode::ISMULH_R, dst: r_reg(dst), src: r_reg(src), imm: None, mem_mask: NO_MASK, unsigned_imm: false, mode: Mode::None, effect: nop}
    }
    if op < Opcode::ISMULH_M as i64 {
        return new_lcache_instr(Opcode::ISMULH_M, r_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::IMUL_RCP as i64 {
        let mut instr = new_imm_instr(Opcode::IMUL_RCP, r_reg(dst), imm, Mode::None);
        instr.unsigned_imm = true;
        return instr;
    }
    if op < Opcode::INEG_R as i64 {
        return new_instr(Opcode::INEG_R, r_reg(dst), Store::NONE, imm, Mode::None);
    }
    if op < Opcode::IXOR_R as i64 {
        return new_instr(Opcode::IXOR_R, r_reg(dst), r_reg(src), imm, Mode::None);
    }
    if op < Opcode::IXOR_M as i64 {
        return new_lcache_instr(Opcode::IXOR_M, r_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::IROR_R as i64 {
        return new_instr(Opcode::IROR_R, r_reg(dst), r_reg(src), imm & 63, Mode::None);
    }
    if op < Opcode::IROL_R as i64 {
        return new_instr(Opcode::IROL_R, r_reg(dst), r_reg(src), imm & 63, Mode::None);
    }
    if op < Opcode::ISWAP_R as i64 {
        return Instr{op: Opcode::ISWAP_R, dst: r_reg(dst), src: r_reg(src), imm: None, unsigned_imm: false, mem_mask: NO_MASK, mode: Mode::None, effect: nop}
    }
    if op < Opcode::FSWAP_R as i64 {
        let dst_ix = dst % MAX_REG;
        if dst_ix >= MAX_FLOAT_REG {
            return new_instr(Opcode::FSWAP_R, e_reg_ix(dst_ix % MAX_FLOAT_REG) , Store::NONE, imm, Mode::None);
        } else {
            return new_instr(Opcode::FSWAP_R, f_reg_ix(dst_ix % MAX_FLOAT_REG), Store::NONE, imm, Mode::None);
        }
    }
    if op < Opcode::FADD_R as i64 {
        return new_instr(Opcode::FADD_R, f_reg(dst), a_reg(src), imm, Mode::None);
    }
    if op < Opcode::FADD_M as i64 {
        return new_lcache_instr(Opcode::FADD_M, f_reg(dst), src, imm, modi, Vm::exec_fadd_m);
    }
    if op < Opcode::FSUB_R as i64 {
        return new_instr(Opcode::FSUB_R, f_reg(dst), a_reg(src), imm, Mode::None);
    }
    if op < Opcode::FSUB_M as i64 {
        return new_lcache_instr(Opcode::FSUB_M, f_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::FSCAL_R as i64 {
        return new_instr(Opcode::FSCAL_R, f_reg(dst), Store::NONE, imm, Mode::None);
    }
    if op < Opcode::FMUL_R as i64 {
        return new_instr(Opcode::FMUL_R, e_reg(dst), a_reg(src), imm, Mode::None);
    }
    if op < Opcode::FDIV_M as i64 {
        return new_lcache_instr(Opcode::FDIV_M, e_reg(dst), src, imm, modi, nop);
    }
    if op < Opcode::FSQRT_R as i64 {
        return new_instr(Opcode::FSQRT_R, e_reg(dst), Store::NONE, imm, Mode::None);
    }
    if op < Opcode::CBRANCH as i64 {
        return new_imm_instr(Opcode::CBRANCH, r_reg(dst), imm, mod_cond(modi));
    }
    if op < Opcode::CFROUND as i64 {
        return Instr{op: Opcode::CFROUND , dst: Store::NONE, src: r_reg(src), imm: Some(imm & 63), unsigned_imm: false, mem_mask: NO_MASK, mode: Mode::None, effect: nop}
    }
    if op < Opcode::ISTORE as i64 {
        return Instr{op: Opcode::ISTORE, dst: l_cache(dst, modi), src: r_reg(src), imm: Some(imm), unsigned_imm: false, mem_mask: NO_MASK, mode: Mode::None, effect: nop};
    }
    return new_instr(Opcode::NOP, Store::NONE, Store::NONE, imm, Mode::None);
}

pub fn r_reg(dst: i64) -> Store {
    match dst%MAX_REG {
        0 => Store::R(0),
        1 => Store::R(1),
        2 => Store::R(2),
        3 => Store::R(3),
        4 => Store::R(4),
        5 => Store::R(5),
        6 => Store::R(6),
        7 => Store::R(7),
        _ => Store::R(0),
    }
}

pub fn a_reg(dst: i64) -> Store {
    match dst%MAX_FLOAT_REG {
        0 => Store::A(0),
        1 => Store::A(1),
        2 => Store::A(2),
        3 => Store::A(3),
        _ => Store::A(0),
    }
}

pub fn e_reg(dst: i64) -> Store {
    e_reg_ix(dst%MAX_FLOAT_REG)
}

fn e_reg_ix(ix: i64) -> Store {
    match ix {
        0 => Store::E(0),
        1 => Store::E(1),
        2 => Store::E(2),
        3 => Store::E(3),
        _ => Store::E(0),
    }
}

pub fn f_reg(dst: i64) -> Store {
    f_reg_ix(dst%MAX_FLOAT_REG)
}

fn f_reg_ix(ix: i64) -> Store {
    match ix {
        0 => Store::F(0),
        1 => Store::F(1),
        2 => Store::F(2),
        3 => Store::F(3),
        _ => Store::F(0),
    }
}

fn l_cache(dst: i64, modi: u8) -> Store {
    let reg = r_reg(dst);
    let cond = mod_cond_u8(modi);
    if cond < STORE_L3_CONDITION {
        if mod_mem_u8(modi) == 0 {
            return Store::L2(Box::new(reg));
        }
        return Store::L1(Box::new(reg));
    } 
    return Store::L3(Box::new(reg));
}

fn l12_cache(src: i64, modi: u8) -> (Store, u64) {
    let reg = r_reg(src);
    if mod_mem_u8(modi) == 0 {
        return (Store::L2(Box::new(reg)), SCRATCHPAD_L2_MASK);
    }
    return (Store::L1(Box::new(reg)), SCRATCHPAD_L1_MASK);
}

fn is_l_cache(store: &Store) -> bool {
    match store {
        Store::L1(_) => true,
        Store::L2(_) => true,
        Store::L3(_) => true,
        _ => false,
    }
}

fn mod_mem_u8(modi: u8) -> u8 {
    modi % 4 //bit 0-1
}

fn mod_cond_u8(modi: u8) -> u8 {
    modi >> 4 //bits 4-7
}

fn mod_cond(modi: u8) -> Mode {
    Mode::Cond(mod_cond_u8(modi)) 
}

fn mod_shft(modi: u8) -> Mode {
    Mode::Shft((modi >> 2) % 4)
}

pub fn nop(_state: &mut Vm, _instr: &Instr) {}