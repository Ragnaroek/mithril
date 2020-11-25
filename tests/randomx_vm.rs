extern crate mithril;

use mithril::randomx::m128::{m128d};
use mithril::randomx::program::{Instr, Opcode, Store, f_reg, r_reg, Mode, REG_NEEDS_DISPLACEMENT_IX, REG_NEEDS_DISPLACEMENT};
use mithril::randomx::vm::{new_vm, Vm};
//use mithril::byte_string::{u8_array_to_string};

/*
#[test]
fn test_calc_hash() {
    let vm = new_vm();
    let result = vm.calculate_hash("This is a test");
    assert_eq!("639183aae1bf4c9a35884cb46b09cad9175f04efd7684e7262a0ac1c2f0b4e3f", u8_array_to_string(result.as_bytes()));
}
*/

#[allow(overflowing_literals)]
const IMM32 : i32 = 0xc0cb96d2; //3234567890
const IMM64 : u64 = 0xffffffffc0cb96d2;

#[test]
fn test_exec_iadd_rs() {
    let instr = Instr{op: Opcode::IADD_RS, dst: r_reg(0), src: r_reg(1), imm: None,  unsigned_imm: false, mode: Mode::Shft(3), effect: Vm::exec_iadd_rs};
    
    let mut vm = new_vm();
    vm.reg.r[0] = 0x8000000000000000;
    vm.reg.r[1] = 0x1000000000000000;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x0);
}

#[test]
fn test_exec_iadd_rs_with_immediate() {
    let instr = Instr{op: Opcode::IADD_RS, dst: REG_NEEDS_DISPLACEMENT, src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::Shft(2), effect: Vm::exec_iadd_rs};
    let mut vm = new_vm();
    vm.reg.r[REG_NEEDS_DISPLACEMENT_IX] = 0x8000000000000000;
    vm.reg.r[1] = 0x2000000000000000;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[REG_NEEDS_DISPLACEMENT_IX], IMM64);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_isub_r() {
    let instr = Instr{op: Opcode::ISUB_R, dst: r_reg(0), src: r_reg(1), imm: None, unsigned_imm: false, mode: Mode::None, effect: Vm::exec_isub_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 1;
    vm.reg.r[1] = 0xFFFFFFFF;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0xFFFFFFFF00000002);
}

#[test]
fn test_exec_isub_r_with_immediate() {
    let instr = Instr{op: Opcode::ISUB_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_isub_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], (!IMM64 + 1));
}

#[test]
fn test_exec_imul_r() {
    let instr = Instr{op: Opcode::IMUL_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_imul_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x28723424A9108E51);
}

#[test]
fn test_exec_imul_r_with_immediate() {
    let instr = Instr{op: Opcode::IMUL_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_imul_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 1;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], IMM64);
}

#[test]
fn test_exec_imulh_r() {
    let instr = Instr{op: Opcode::IMULH_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_imulh_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0xB4676D31D2B34883);
}

#[test]
fn test_exec_ismulh_r() {
    let instr = Instr{op: Opcode::ISMULH_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_ismulh_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x02D93EF1269D3EE5);
}

#[test]
fn test_exec_ineg_r() {
    let instr = Instr{op: Opcode::INEG_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_ineg_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 0xFFFFFFFFFFFFFFFF;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 1); 
}

#[test]
fn test_exec_ixor_r() {
    let instr = Instr{op: Opcode::IXOR_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_ixor_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 0x8888888888888888;
    vm.reg.r[1] = 0xAAAAAAAAAAAAAAAA;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x2222222222222222); 
}

#[test]
fn test_exec_ixor_r_with_immediate() {
    let instr = Instr{op: Opcode::IXOR_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_ixor_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 0xFFFFFFFFFFFFFFFF;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], !IMM64); 
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_fadd_m() {
    let instr = Instr{op: Opcode::FADD_M, dst: f_reg(0), src: Store::L1(Box::new(Store::R(1))), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, effect: Vm::exec_fadd_m};
    let mut vm = new_vm();
    vm.scratchpad[0] = 0x1234567890abcdef;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.f[0] = m128d::zero();
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.f[0], m128d::from_u64(0x41b2345678000000, 0xc1dbd50c84400000));
}

/*
#[test]
fn test_exec_iadd_m() {
    let instr = new_lcache_instr(Opcode::IADD_M, r_reg(0), 1, 666, 1);
    let mut vm = new_vm();
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x0);
}*/

pub fn nop(_state: &mut Vm) {}