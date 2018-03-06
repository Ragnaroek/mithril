#![allow(unknown_lints)]
#![allow(inline_always)]

use u64x2::u64x2;

//taken from https://github.com/RustCrypto/block-ciphers and modified for Cryptonight

macro_rules! gen_key {
    ($round:expr, $ib:expr, $key:ident, $input0:ident, $input1:ident) => {
        asm!(concat!("
            aeskeygenassist xmm2, xmm1, ", $round,
            "
            pshufd xmm2, xmm2, ", $ib,
            "
            movdqa xmm4, xmm3
            pslldq xmm4, 0x4
            pxor xmm3, xmm4

            pslldq xmm4, 0x4
            pxor xmm3, xmm4

            pslldq xmm4, 0x4
            pxor xmm3, xmm4

            pxor xmm3, xmm2"
            )
            : "={xmm3}"($key)
            : "{xmm1}"($input1),"{xmm3}"($input0)
            : "xmm4", "xmm2"
            : "intel", "alignstack", "volatile"
        );
    }
}

macro_rules! aes_enc {
    ($data:ident, $key:ident, $result:ident) => {
        asm!("aesenc xmm1, xmm2"
            : "={xmm1}"($result)
            : "{xmm1}"($data),"{xmm2}"($key)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
fn gen_key_0x01(input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
    let r0;
    let r1;
    unsafe {
        gen_key!("0x01", "0xFF", r0, input0, input1);
        gen_key!("0x00", "0xAA", r1, input1, r0);
    }
    (r0, r1)
}

#[inline(always)]
fn gen_key_0x02(input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
    let r0;
    let r1;
    unsafe {
        gen_key!("0x02", "0xFF", r0, input0, input1);
        gen_key!("0x00", "0xAA", r1, input1, r0);
    }
    (r0, r1)
}

#[inline(always)]
fn gen_key_0x04(input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
    let r0;
    let r1;
    unsafe {
        gen_key!("0x04", "0xFF", r0, input0, input1);
        gen_key!("0x00", "0xAA", r1, input1, r0);
    }
    (r0, r1)
}

#[inline(always)]
fn gen_key_0x08(input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
    let r0;
    let r1;
    unsafe {
        gen_key!("0x08", "0xFF", r0, input0, input1);
        gen_key!("0x00", "0xAA", r1, input1, r0);
    }
    (r0, r1)
}

pub fn gen_round_keys(input0: u64x2, input1: u64x2) -> [u64x2;10] {
    let mut r : [u64x2;10] = [u64x2(0,0);10];

    r[0] = input0;
    r[1] = input1;

    let (input0, input1) = gen_key_0x01(input0, input1);
    r[2] = input0;
    r[3] = input1;

    let (input0, input1) = gen_key_0x02(input0, input1);
    r[4] = input0;
    r[5] = input1;

    let (input0, input1) = gen_key_0x04(input0, input1);
    r[6] = input0;
    r[7] = input1;

    let (input0, input1) = gen_key_0x08(input0, input1);
    r[8] = input0;
    r[9] = input1;

    r
}

pub fn aes_round(block: u64x2, key: u64x2) -> u64x2 {
    let r;
    unsafe {
        aes_enc!(block, key, r);
    }
    r
}
