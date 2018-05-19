#![allow(unknown_lints)]
#![allow(unreadable_literal)]
extern crate tiny_keccak;

use super::common::{as_u8_array};

const PLEN: usize = 25;
const TLEN: usize = 144;

fn xorin(dst: &mut [u8], src: &[u8]) {
    for (d, i) in dst.iter_mut().zip(src) {
        *d ^= *i;
    }
}

#[allow(cast_ptr_alignment)]
fn as_u64_array(t: &mut [u8; TLEN]) -> &mut [u64; TLEN / 8] {
    unsafe { &mut *(t as *mut [u8; TLEN] as *mut [u64; TLEN / 8]) }
}

#[inline(always)]
pub fn keccakf(a: &mut [u64; PLEN]) {
    tiny_keccak::keccakf(a)
}

pub fn keccak(input: &[u8]) -> [u8; 200] {

    let mut a: [u64; PLEN] = [0; PLEN];
    let init_rate = 136; //200 - 512/4;
    let mut rate = init_rate;
    let inlen = input.len();
    let mut tmp: [u8; TLEN] = [0; TLEN];
    tmp[..inlen].copy_from_slice(input);

    //first foldp
    let mut ip = 0;
    let mut l = inlen;
    while l >= rate {
        xorin(&mut as_u8_array(&mut a)[0..][..rate], &input[ip..]);
        tiny_keccak::keccakf(&mut a);
        ip += rate;
        l -= rate;
        rate = init_rate;
    }

    //pad
    tmp[inlen] = 1;
    tmp[rate - 1] |= 0x80;

    let t64 = as_u64_array(&mut tmp);
    for i in 0..(rate/8) {
        a[i] ^= t64[i];
    }

    tiny_keccak::keccakf(&mut a);

    let t8 = as_u8_array(&mut a);
    *t8
}
