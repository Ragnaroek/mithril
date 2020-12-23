extern crate mithril;

use mithril::randomx::m128::{m128d};
use mithril::randomx::program::{Instr, Opcode, Store, e_reg, f_reg, a_reg, r_reg, Mode, REG_NEEDS_DISPLACEMENT_IX, REG_NEEDS_DISPLACEMENT};
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
const ROUND_TO_NEAREST : u32 = 0;
const ROUND_DOWN : u32 = 1;
const ROUND_UP : u32 = 2;
const ROUND_TO_ZERO : u32 = 3;

#[test]
fn test_exec_iadd_rs() {
    let instr = Instr{op: Opcode::IADD_RS, dst: r_reg(0), src: r_reg(1), imm: None,  unsigned_imm: false, mode: Mode::Shft(3), target: None, effect: Vm::exec_iadd_rs};
    
    let mut vm = new_vm();
    vm.reg.r[0] = 0x8000000000000000;
    vm.reg.r[1] = 0x1000000000000000;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x0);
}

#[test]
fn test_exec_iadd_rs_with_immediate() {
    let instr = Instr{op: Opcode::IADD_RS, dst: REG_NEEDS_DISPLACEMENT, src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::Shft(2), target: None, effect: Vm::exec_iadd_rs};
    let mut vm = new_vm();
    vm.reg.r[REG_NEEDS_DISPLACEMENT_IX] = 0x8000000000000000;
    vm.reg.r[1] = 0x2000000000000000;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[REG_NEEDS_DISPLACEMENT_IX], IMM64);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_isub_r() {
    let instr = Instr{op: Opcode::ISUB_R, dst: r_reg(0), src: r_reg(1), imm: None, unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_isub_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 1;
    vm.reg.r[1] = 0xFFFFFFFF;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0xFFFFFFFF00000002);
}

#[test]
fn test_exec_isub_r_with_immediate() {
    let instr = Instr{op: Opcode::ISUB_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_isub_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0;
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], (!IMM64 + 1));
}

#[test]
fn test_exec_imul_r() {
    let instr = Instr{op: Opcode::IMUL_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_imul_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x28723424A9108E51);
}

#[test]
fn test_exec_imul_r_with_immediate() {
    let instr = Instr{op: Opcode::IMUL_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_imul_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 1;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], IMM64);
}

#[test]
fn test_exec_imulh_r() {
    let instr = Instr{op: Opcode::IMULH_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_imulh_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0xB4676D31D2B34883);
}

#[test]
fn test_exec_ismulh_r() {
    let instr = Instr{op: Opcode::ISMULH_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_ismulh_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 0xBC550E96BA88A72B;
    vm.reg.r[1] = 0xF5391FA9F18D6273;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x02D93EF1269D3EE5);
}

#[test]
fn test_exec_ineg_r() {
    let instr = Instr{op: Opcode::INEG_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_ineg_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 0xFFFFFFFFFFFFFFFF;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 1); 
}

#[test]
fn test_exec_ixor_r() {
    let instr = Instr{op: Opcode::IXOR_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_ixor_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 0x8888888888888888;
    vm.reg.r[1] = 0xAAAAAAAAAAAAAAAA;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0x2222222222222222); 
}

#[test]
fn test_exec_ixor_r_with_immediate() {
    let instr = Instr{op: Opcode::IXOR_R, dst: r_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_ixor_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 0xFFFFFFFFFFFFFFFF;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], !IMM64); 
}

#[test]
fn test_exec_iror_r() {
    let instr = Instr{op: Opcode::IROR_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_iror_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 953360005391419562;
    vm.reg.r[1] = 4569451684712230561;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0xD835C455069D81EF); 
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_iror_r_with_immediate() {
    let instr = Instr{op: Opcode::IROR_R, dst: r_reg(0), src: Store::NONE, imm: Some(4569451684712230561 as i32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_iror_r};
    let mut vm = new_vm(); 
    vm.reg.r[0] = 953360005391419562;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 0xD835C455069D81EF); 
}

#[test]
fn test_exec_irol_r() {
    let instr = Instr{op: Opcode::IROL_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_irol_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 953360005391419562;
    vm.reg.r[1] = 4569451684712230561;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 6978065200552740799); 
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_irol_r_with_immediate() {
    let instr = Instr{op: Opcode::IROL_R, dst: r_reg(0), src: Store::NONE, imm: Some(4569451684712230561 as i32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_irol_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 953360005391419562;

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 6978065200552740799); 
}

#[test]
fn test_exec_iswap_r() {
    let instr = Instr{op: Opcode::ISWAP_R, dst: r_reg(0), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_iswap_r};
    let mut vm = new_vm();
    vm.reg.r[0] = 953360005391419562;
    vm.reg.r[1] = 4569451684712230561; 

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.r[0], 4569451684712230561);
    assert_eq!(vm.reg.r[1], 953360005391419562); 
}

#[test]
fn test_exec_fswap_r() {
    let instr = Instr{op: Opcode::FSWAP_R, dst: f_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fswap_r};
    let mut vm = new_vm();
    vm.reg.f[0] = m128d::from_u64(953360005391419562, 4569451684712230561); 

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.f[0], m128d::from_u64(4569451684712230561, 953360005391419562));
}

#[test]
fn test_exec_fadd_r_round_to_nearest() {
    let instr = Instr{op: Opcode::FADD_R, dst: f_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fadd_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_TO_NEAREST);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1); 

    instr.execute(&mut vm);

    assert_eq!(vm.reg.f[0], m128d::from_u64(0x402dcc3b63eaa6fe, 0xc1ce30a748e032b9))
}

#[test]
fn test_exec_fadd_r_round_down() {
    let instr = Instr{op: Opcode::FADD_R, dst: f_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fadd_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_DOWN);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1); 

    instr.execute(&mut vm);

    assert_eq!(vm.reg.f[0], m128d::from_u64(0x402dcc3b63eaa6fd, 0xc1ce30a748e032b9))
}

#[test]
fn test_exec_fadd_r_round_up() {
    let instr = Instr{op: Opcode::FADD_R, dst: f_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fadd_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_UP);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1); 

    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.f[0], m128d::from_u64(0x402dcc3b63eaa6fe, 0xc1ce30a748e032b8));
}

#[test]
fn test_exec_fadd_r_round_to_zero() {
    let instr = Instr{op: Opcode::FADD_R, dst: f_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fadd_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_TO_ZERO);

    vm.reg.f[0] = m128d::from_u64(0x3ffd2c97cc4ef015, 0xc1ce30b3c4223576);
    vm.reg.a[1] = m128d::from_u64(0x402a26a86a60c8fb, 0x40b8f684057a59e1); 

    instr.execute(&mut vm);

    assert_eq!(vm.reg.f[0], m128d::from_u64(0x402dcc3b63eaa6fd, 0xc1ce30a748e032b8))
}

#[test]
fn test_exec_fscal_r() {
    let instr = Instr{op: Opcode::FSCAL_R, dst: f_reg(0), src: Store::L1(Box::new(Store::R(1))), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fscal_r};
    let mut vm = new_vm();
    vm.reg.f[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    
    instr.execute(&mut vm);
   
    assert_eq!(vm.reg.f[0], m128d::from_u64(0xc12bc35cef248783, 0xc00dfdabb6173d07));
}

#[test]
fn test_exec_fmul_r_round_to_nearest() {
    let instr = Instr{op: Opcode::FMUL_R, dst: e_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fmul_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_TO_NEAREST);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50); 

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x42d7feeccd89152f, 0x42d30f35ff7a6969)) 
}

#[test]
fn test_exec_fmul_r_round_round_down() {
    let instr = Instr{op: Opcode::FMUL_R, dst: e_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fmul_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_DOWN);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50); 

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x42d7feeccd89152e, 0x42d30f35ff7a6969)) 
}

#[test]
fn test_exec_fmul_r_round_up() {
    let instr = Instr{op: Opcode::FMUL_R, dst: e_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fmul_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_UP);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50); 

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x42d7feeccd89152f, 0x42d30f35ff7a696a)) 
}

#[test]
fn test_exec_fmul_r_round_to_zero() {
    let instr = Instr{op: Opcode::FMUL_R, dst: e_reg(0), src: a_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fmul_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_TO_ZERO);

    vm.reg.e[0] = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    vm.reg.a[1] = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50); 

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x42d7feeccd89152e, 0x42d30f35ff7a6969)) 
}

#[test]
fn test_exec_fsqrt_r_round_to_nearest() {
    let instr = Instr{op: Opcode::FSQRT_R, dst: e_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fsqrt_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_TO_NEAREST);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x40d30e573fa3ba8d, 0x40212a610b301fe8));
}

#[test]
fn test_exec_fsqrt_r_round_up() {
    let instr = Instr{op: Opcode::FSQRT_R, dst: e_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fsqrt_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_UP);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x40d30e573fa3ba8d, 0x40212a610b301fe9));
}

#[test]
fn test_exec_fsqrt_r_round_down() {
    let instr = Instr{op: Opcode::FSQRT_R, dst: e_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fsqrt_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_DOWN);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x40d30e573fa3ba8c, 0x40212a610b301fe8));
}

#[test]
fn test_exec_fsqrt_r_round_to_zero() {
    let instr = Instr{op: Opcode::FSQRT_R, dst: e_reg(0), src: Store::NONE, imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fsqrt_r};
    let mut vm = new_vm();
    vm.set_rounding_mode(ROUND_TO_ZERO);

    vm.reg.e[0] = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    instr.execute(&mut vm);

    assert_eq!(vm.reg.e[0], m128d::from_u64(0x40d30e573fa3ba8c, 0x40212a610b301fe8));
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_fadd_m() {
    let instr = Instr{op: Opcode::FADD_M, dst: f_reg(0), src: Store::L1(Box::new(Store::R(1))), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_fadd_m};
    let mut vm = new_vm();
    vm.scratchpad[0] = 0x1234567890abcdef;
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.f[0] = m128d::zero();
    
    instr.execute(&mut vm);
    
    assert_eq!(vm.reg.f[0], m128d::from_u64(0x41b2345678000000, 0xc1dbd50c84400000));
}

#[test]
fn test_exec_cfround() {
    let instr = Instr{op: Opcode::CFROUND, dst: Store::NONE, src: r_reg(0), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_cfround};
    let mut vm = new_vm();
    vm.reg.r[0] = 0xFFFFFFFFFFFC6800;

    assert_eq!(vm.get_rounding_mode(), ROUND_TO_NEAREST); //new vm starts with default rounding mode

    instr.execute(&mut vm);

    assert_eq!(vm.get_rounding_mode(), ROUND_TO_ZERO);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_cbranch_taken() {
    let instr = Instr{op: Opcode::CBRANCH, dst: r_reg(0), src: Store::NONE, imm: Some(0xFFFFFFFFC0CB9AD2), unsigned_imm: false, mode: Mode::Cond(3), target: Some(100), effect: Vm::exec_cbranch};
    let mut vm = new_vm();
    vm.pc = 200;    
    vm.reg.r[0] = 0xFFFFFFFFFFFC6800;

    instr.execute(&mut vm);

    assert_eq!(vm.pc, 100)
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_cbranch_not_taken() {
    let instr = Instr{op: Opcode::CBRANCH, dst: r_reg(0), src: Store::NONE, imm: Some(0xFFFFFFFFC0CB9AD2), unsigned_imm: false, mode: Mode::Cond(3), target: None, effect: Vm::exec_cbranch};
    let mut vm = new_vm();
    vm.pc = 200;    
    vm.reg.r[0] = 0;

    instr.execute(&mut vm);

    assert_eq!(vm.pc, 200)
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_istore_l1() {
    let instr = Instr{op: Opcode::ISTORE, dst: Store::L1(Box::new(r_reg(0))), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_istore};
    let mut vm = new_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFC6800;
    vm.reg.r[0] = 0xFFFFFFFFC0C802D2;

    instr.execute(&mut vm);

    assert_eq!(vm.scratchpad[0x19A0 / 8], 0xFFFFFFFFFFFC6800);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_istore_l2() {
    let instr = Instr{op: Opcode::ISTORE, dst: Store::L2(Box::new(r_reg(0))), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_istore};
    let mut vm = new_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFC6800;
    vm.reg.r[0] = 0xFFFFFFFFC0C802D2;

    instr.execute(&mut vm);

    assert_eq!(vm.scratchpad[0x399A0 / 8], 0xFFFFFFFFFFFC6800);
}

#[test]
#[allow(overflowing_literals)]
fn test_exec_istore_l3() {
    let instr = Instr{op: Opcode::ISTORE, dst: Store::L3(Box::new(r_reg(0))), src: r_reg(1), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_istore};
    let mut vm = new_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFC6800;
    vm.reg.r[0] = 0xFFFFFFFFC0C802D2;

    instr.execute(&mut vm);

    assert_eq!(vm.scratchpad[0x1399A0 / 8], 0xFFFFFFFFFFFC6800);
}


#[test]
fn test_exec_iadd_m_l1() {
    let instr = Instr{op: Opcode::IADD_M, dst: r_reg(0), src: Store::L1(Box::new(r_reg(1))), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_iadd_m};
    let mut vm = new_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0] = 0x0203;
    
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x869);
}

#[test]
fn test_exec_iadd_m_l2() {
    let instr = Instr{op: Opcode::IADD_M, dst: r_reg(0), src: Store::L2(Box::new(r_reg(1))), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_iadd_m};
    let mut vm = new_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0x38000/8] = 0x0203;
    
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x869);
}

#[test]
fn test_exec_iadd_m_l3() {
    let instr = Instr{op: Opcode::IADD_M, dst: r_reg(0), src: Store::L3(Box::new(r_reg(1))), imm: Some(IMM32), unsigned_imm: false, mode: Mode::None, target: None, effect: Vm::exec_iadd_m};
    let mut vm = new_vm();
    vm.reg.r[1] = 0xFFFFFFFFFFFFE930;
    vm.reg.r[0] = 0x666;
    vm.scratchpad[0xb96d0/8] = 0x0203;
    
    instr.execute(&mut vm);

    assert_eq!(vm.reg.r[0], 0x869);
}

pub fn nop(_state: &mut Vm) {}