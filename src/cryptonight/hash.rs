extern crate groestl;
extern crate blake;
extern crate jhffi;
extern crate skeinffi;

use super::keccak;
use super::aes::{AES};
use u64x2::u64x2;
use std::boxed::Box;

use self::groestl::{Digest, Groestl256};
use super::super::byte_string;

pub const MEM_SIZE : usize = 2097152 / 16;
const ITERATIONS : u32 = 524288;

/// This is mainly for testing, allocates a new scratchpad on every hash
pub fn hash_alloc_scratchpad(input: &[u8], aes: &AES) -> String {
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];
    return hash(&mut scratchpad, input, aes);
}

pub fn hash(mut scratchpad : &mut Box<[u64x2; MEM_SIZE]>, input: &[u8], aes: &AES) -> String {
    //scratchpad init
    let mut state = keccak::keccak(input);
    init_scratchpad(&mut scratchpad, &mut state, &aes);

    let mut a = u64x2::read(&state[0..16]) ^ u64x2::read(&state[32..48]);
    let mut b = u64x2::read(&state[16..32]) ^ u64x2::read(&state[48..64]);

    let mut i = 0;
    while i < ITERATIONS {
        let mut ix = scratchpad_addr(&a);
        let aes_result = aes.aes_round(scratchpad[ix], a);
        scratchpad[ix] = b ^ aes_result;

        ix = scratchpad_addr(&aes_result);
        let mem = scratchpad[ix];
        let add_r = ebyte_add(&a, &ebyte_mul(&aes_result, &mem));
        scratchpad[ix] = add_r;

        a = add_r ^ mem;
        b = aes_result;

        i += 1;
    }

    let final_result = finalise_scratchpad(scratchpad, &mut state, &aes);

    let mut k = 0;
    while k < 8 {
        let block = final_result[k];
        let offset = 64+(k<<4);
        block.write(&mut state[offset..offset+16]);
        k += 1;
    }

    let state_64 = transmute_u64(&mut state);
    keccak::keccakf(state_64);

    return final_hash(transmute_u8(state_64));
}

fn final_hash(keccak_state: &[u8; 200]) -> String {
    let hash_result = match keccak_state[0] & 3 {
        0 => {
              let mut result = [0; 32];
              blake::hash(256, keccak_state, &mut result).unwrap();
              byte_string::u8_array_to_string(&result)
        },
        1 => {
              let mut hasher = Groestl256::default();
              hasher.input(keccak_state);
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
    };
    return hash_result;
}

fn transmute_u64(t: &mut [u8; 200]) -> &mut [u64; 25] {
    unsafe { ::std::mem::transmute(t) }
}

fn transmute_u8(t: &mut [u64; 25]) -> &mut [u8; 200] {
    unsafe { ::std::mem::transmute(t) }
}

pub fn ebyte_mul(a: &u64x2, b: &u64x2) -> u64x2 {
    let r0 = u128::from(a.0);
    let r1 = u128::from(b.0);
    let r = r0 * r1;
    return u64x2((r >> 64) as u64, r as u64);
}

pub fn ebyte_add(a: &u64x2, b: &u64x2) -> u64x2 {
    return u64x2(a.0.wrapping_add(b.0), a.1.wrapping_add(b.1));
}

pub fn scratchpad_addr(u: &u64x2) -> usize {
    return ((u.0 & 0x1FFFF0) >> 4) as usize;
}

pub fn finalise_scratchpad(scratchpad: &mut Box<[u64x2; MEM_SIZE]>, keccak_state: &mut [u8; 200], aes: &AES) -> [u64x2; 8] {
    let t_state = transmute_u64(keccak_state);
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
    return state;
}

pub fn init_scratchpad(scratchpad : &mut Box<[u64x2; MEM_SIZE]>, state: &mut [u8; 200], aes: &AES) {
    let t_state = transmute_u64(state);
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
