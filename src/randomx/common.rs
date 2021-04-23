const P_2EXP63: u64 = 1 << 63;
const INT32_MAX: u32 = i32::MAX as u32;

pub fn u64_from_i32_imm(imm: i32) -> u64 {
    let x = imm as u32;
    if x > INT32_MAX {
        x as u64 | 0xffffffff00000000
    } else {
        x as u64
    }
}

pub fn u64_from_u32_imm(imm: u32) -> u64 {
    if imm > INT32_MAX {
        imm as u64 | 0xffffffff00000000
    } else {
        imm as u64
    }
}

pub fn mulh(a: u64, b: u64) -> u64 {
    ((a as u128).wrapping_mul(b as u128) >> 64) as u64
}

pub fn smulh(a: u64, b: u64) -> u64 {
    let v_src = (a as i64) as i128; //we have to go through i64 to get the proper complement version in i128 if the u64 is negative in i64
    let v_dst = (b as i64) as i128;
    (v_src.wrapping_mul(v_dst) >> 64) as u64
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
