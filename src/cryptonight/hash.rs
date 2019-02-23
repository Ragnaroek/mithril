extern crate groestl;
extern crate blake;
extern crate jhffi;
extern crate skeinffi;
extern crate byteorder;

use super::keccak;
use super::aes::{AES};
use super::sse;
use super::common::{as_u64_array, as_u8_array};
use u64x2::u64x2;
use std::boxed::Box;
use self::groestl::{Digest, Groestl256};
use super::super::byte_string;
use self::byteorder::{ByteOrder, LittleEndian};

pub const MEM_SIZE : usize = 2_097_152 / 16;
const ITERATIONS : u32 = 524_288;

const ADDR_MASK : u64 = 0x1F_FFF0;
const SQRT_CONST : u64 = 1023 << 52;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum HashVersion {
    Version6,
    Version7,
    Version8,
}

/// This is mainly for testing, allocates a new scratchpad on every hash
pub fn hash_alloc_scratchpad(input: &[u8], aes: &AES, version: HashVersion) -> String {
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];
    hash(&mut scratchpad, input, aes, version)
}

pub fn hash(mut scratchpad : &mut [u64x2; MEM_SIZE], input: &[u8], aes: &AES, version: HashVersion) -> String {
    //scratchpad init
    let mut state = keccak::keccak(input);
    init_scratchpad(&mut scratchpad, &mut state, aes);

    let al = u64x2::read(&state[0..16]);
    let ar = u64x2::read(&state[32..48]);
    let mut a = al ^ ar;

    let bl = u64x2::read(&state[16..32]);
    let br = u64x2::read(&state[48..64]);
    let mut b = bl ^ br;

    let mut ax0;
    let mut bx0;
    let mut bx1;
    let mut division_res:u64;
    let mut sqrt_res:u64;

    if version == HashVersion::Version8 {
        let cl = u64x2::read(&state[64..80]);
        let cr = u64x2::read(&state[80..96]);
        let dl = u64x2::read(&state[96..112]);
        ax0 = u64x2(a.0, al.1 ^ ar.1);
        bx0 = u64x2(bl.0 ^ br.0, bl.1 ^ br.1);
        bx1 = u64x2(cl.0 ^ cr.0, cl.1 ^ cr.1);
        division_res = dl.0;
        sqrt_res = dl.1;
    } else {
        ax0 = u64x2(0, 0);
        bx0 = u64x2(0, 0);
        bx1 = u64x2(0, 0);
        division_res = 0;
        sqrt_res = 0;
    }

    let monero_const = if version == HashVersion::Version6 {
        0
    } else {
        monero_const(input, &state)
    };

    let mut i = 0;
    while i < ITERATIONS {
        let mut ix = scratchpad_addr(a.0);
        let aes_result = aes.aes_round(scratchpad[ix], a);

        if version == HashVersion::Version8 {
            shuffle_0(a.0, &mut scratchpad, ax0, bx0, bx1);
        }
        if version == HashVersion::Version6 || version == HashVersion::Version8 {
            scratchpad[ix] = b ^ aes_result;
        } else {
            scratchpad[ix] = cryptonight_monero_tweak(&(b ^ aes_result));
        }

        ix = scratchpad_addr(aes_result.0);
        let mut mem = scratchpad[ix];
        if version == HashVersion::Version8 {
            let (sqrt_res_n, division_res_n) = division(ix, &mut scratchpad, &aes_result, sqrt_res, division_res);
            sqrt_res = sqrt_res_n;
            division_res = division_res_n;
            mem = scratchpad[ix];

            let r = (aes_result.0 as u128) * (mem.0 as u128);
            let lo = r as u64;
            let hi = (r >> 64) as u64;
            let (lo_p, hi_p) = shuffle_1(aes_result.0, &mut scratchpad, ax0, bx0, bx1, lo, hi);
            let (a0_p, _) = a.1.overflowing_add(lo_p);
            let (a1_p, _) = a.0.overflowing_add(hi_p);
            a = u64x2(a0_p, a1_p);
            bx1 = bx0;
            bx0 = aes_result;
        }

        scratchpad[ix].0 = a.1;
        if version == HashVersion::Version7 {
            scratchpad[ix].1 = a.0 ^ monero_const;
        } else {
            scratchpad[ix].1 = a.0;
        }
        a = u64x2(a.1 ^ mem.0, a.0 ^ mem.1);
        ax0 = a;
        b = aes_result;

        i += 1;
    }

    let final_result = finalise_scratchpad(scratchpad, &mut state, aes);

    let mut k = 0;
    while k < 8 {
        let block = final_result[k];
        let offset = 64+(k<<4);
        block.write(&mut state[offset..offset+16]);
        k += 1;
    }

    let state_64 = as_u64_array(&mut state);
    keccak::keccakf(state_64);

    final_hash(as_u8_array(state_64))
}

pub fn shuffle_0(ix: u64, scratchpad : &mut [u64x2; MEM_SIZE], ax0: u64x2, bx0: u64x2, bx1: u64x2) {
    let addr = (ix & ADDR_MASK) as usize;
    let a1 = (addr ^ 0x10) >> 4;
    let a2 = (addr ^ 0x20) >> 4;
    let a3 = (addr ^ 0x30) >> 4;
    let v1 = scratchpad[a1];
    let v2 = scratchpad[a2];
    let v3 = scratchpad[a3];

    scratchpad[a1] = v3 + bx1;
    scratchpad[a2] = v1 + bx0;
    scratchpad[a3] = v2 + ax0;
}

pub fn shuffle_1(ix: u64, scratchpad : &mut [u64x2; MEM_SIZE], ax0: u64x2, bx0: u64x2, bx1: u64x2, lo: u64, hi: u64) -> (u64, u64){
    let addr = (ix & ADDR_MASK) as usize;
    let a1 = (addr ^ 0x10) >> 4;
    let a2 = (addr ^ 0x20) >> 4;
    let a3 = (addr ^ 0x30) >> 4;
    let v1 = scratchpad[a1] ^ u64x2(hi, lo);
    let v2 = scratchpad[a2];
    let v3 = scratchpad[a3];

    scratchpad[a1] = v3 + bx1;
    scratchpad[a2] = v1 + bx0;
    scratchpad[a3] = v2 + ax0;

    return (lo ^ v2.1, hi ^ v2.0);
}

pub fn division(ix: usize, scratchpad : &mut [u64x2; MEM_SIZE], aes_result: &u64x2, sqrt_res: u64, div_res: u64) -> (u64, u64) {
    let mem = scratchpad[ix];
    let cl_p = mem.0 ^ (div_res ^ (sqrt_res << 32));
    scratchpad[ix].0 = cl_p;
    let (p, _) = aes_result.0.overflowing_add(sqrt_res << 1);
    let d = (p | 0x80000001) & 0xFFFFFFFF;

    let ae = aes_result.1;
    let (k,_) = (ae % d).overflowing_shl(32);
    let ae_d = (ae / d) as u32;
    let result = ae_d as u64 + k;
    let (s, _) = aes_result.0.overflowing_add(result);
    return (sqrt(s), result);
}

pub fn sqrt(v: u64) -> u64 {
    let x0 = (v >> 12) | SQRT_CONST;
    let zero = u64x2(0,0);
    let x1 = u64x2(x0, 0);
    let x = sse::_mm_sqrt_sd(zero, x1);
    let s = x.0 >> 20;
    let r = x.0 >> 19;
    let (x2, _) = (s - (1022 << 32)).overflowing_mul(r - s - (1022 << 32) + 1);
    if x2 < v {
        r + 1
    } else {
        r
    }
}

pub fn cryptonight_monero_tweak(tmp: &u64x2) -> u64x2 {
    let mut vh = tmp.1;
    let x = (vh >> 24) as u8;
	let index = (((x >> 3) & 6) | (x & 1)) << 1;
	vh ^= ((0x7531 >> index) & 0x3) << 28;
    u64x2(tmp.0, vh)
}

pub fn monero_const(input: &[u8], state: &[u8]) -> u64 {
    let ip1 = LittleEndian::read_u64(&input[35..64]);
    let ip2 = LittleEndian::read_u64(&state[(8*24)..(8*24+8)]);
    ip1 ^ ip2
}

fn final_hash(keccak_state: &[u8; 200]) -> String {
    match keccak_state[0] & 3 {
        0 => {
              let mut result = [0; 32];
              blake::hash(256, keccak_state, &mut result).unwrap();
              byte_string::u8_array_to_string(&result)
        },
        1 => {
              let mut hasher = Groestl256::default();
              let state_ref : &[u8] = keccak_state;
              hasher.input(state_ref);
              format!("{:x}", hasher.result())
        },
        2 => {
              let mut result = [0; 32];
              jhffi::hash(256, keccak_state, &mut result).unwrap();
              byte_string::u8_array_to_string(&result)
        },
        3 => {
              let mut result = [0; 32];
              skeinffi::hash(256, keccak_state, &mut result).unwrap();
              byte_string::u8_array_to_string(&result)
        },
        _ => panic!("hash select error")
    }
}

pub fn ebyte_mul(a: &u64x2, b: &u64x2) -> u64x2 {
    let r0 = u128::from(a.0);
    let r1 = u128::from(b.0);
    let r = r0 * r1;
    u64x2((r >> 64) as u64, r as u64)
}

pub fn ebyte_add(a: &u64x2, b: &u64x2) -> u64x2 {
    u64x2(a.0.wrapping_add(b.0), a.1.wrapping_add(b.1))
}

pub fn scratchpad_addr(u: u64) -> usize {
    ((u & ADDR_MASK) >> 4) as usize
}

pub fn finalise_scratchpad(scratchpad: &mut [u64x2; MEM_SIZE], keccak_state: &mut [u8; 200], aes: &AES) -> [u64x2; 8] {
    let t_state = as_u64_array(keccak_state);
    let input0 = u64x2(t_state[4], t_state[5]);
    let input1 = u64x2(t_state[6], t_state[7]);

    let keys = aes.gen_round_keys(input0, input1);

    let mut state : [u64x2; 8] = [u64x2(0,0); 8];
    let mut i = 0;
    while i < 8 {
        let offset = i*2;
        let mut block = u64x2(t_state[8+offset], t_state[8+offset+1]);
        block = scratchpad[i] ^ block;
        let mut k = 0;
        while k < 10 {
            block = aes.aes_round(block, keys[k]);
            k += 1;
        }
        state[i] = block;
        i += 1;
    }

    let mut k = 8;
    while k < MEM_SIZE {
        let mut i = 0;
        while i < 8 {
            let mut block = scratchpad[k+i];
            block = state[i] ^ block;
            let mut j = 0;
            while j < 10 {
                block = aes.aes_round(block, keys[j]);
                j += 1;
            }
            state[i] = block;
            i += 1;
        }
        k += 8;
    }
    state
}

pub fn init_scratchpad(scratchpad : &mut [u64x2; MEM_SIZE], state: &mut [u8; 200], aes: &AES) {
    let t_state = as_u64_array(state);
    let input0 = u64x2(t_state[0], t_state[1]);
    let input1 = u64x2(t_state[2], t_state[3]);
    let keys = aes.gen_round_keys(input0, input1);

    let mut i = 0;
    while i < 8 {
        let offset = i*2;
        let mut block = u64x2(t_state[8+offset], t_state[8+offset+1]);
        let mut k = 0;
        while k < 10 {
            block = aes.aes_round(block, keys[k]);
            k += 1;
        }
        scratchpad[i] = block;
        i += 1;
    }

    let mut k = 0;
    while k < (MEM_SIZE-8) {
        let mut i = k;
        while i < (k+8) {
            let mut block = scratchpad[i];
            let mut j = 0;
            while j < 10 {
                block = aes.aes_round(block, keys[j]);
                j += 1;
            }
            scratchpad[i+8] = block;
            i += 1;
        }
        k += 8;
    }
}
