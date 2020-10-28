extern crate mithril;

use mithril::randomx::program::{Instr, Opcode, r_reg, Mode, REG_NEEDS_DISPLACEMENT_IX, REG_NEEDS_DISPLACEMENT};
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

#[test]
fn test_exec_iadd_rs() {
    let instr = Instr{op: Opcode::IADD_RS, dst: r_reg(0), src: r_reg(1), imm: None, unsigned_imm: false, mode: Mode::Shft(3), effect: Vm::exec_iadd_rs};
    let mut vm = new_vm();
    vm.reg.r[0] = 0x8000000000000000;
    vm.reg.r[1] = 0x1000000000000000;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x0);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_iadd_rs_with_immediate() {
    let instr = Instr{op: Opcode::IADD_RS, dst: REG_NEEDS_DISPLACEMENT, src: r_reg(1), imm: Some(3234567890), unsigned_imm: false, mode: Mode::Shft(2), effect: Vm::exec_iadd_rs};
    let mut vm = new_vm();
    vm.reg.r[REG_NEEDS_DISPLACEMENT_IX] = 0x8000000000000000;
    vm.reg.r[1] = 0x2000000000000000;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[REG_NEEDS_DISPLACEMENT_IX], 3234567890 as u64 | 0xffffffff00000000);
}

pub fn nop(_state: &mut Vm) {}