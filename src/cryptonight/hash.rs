extern crate groestl;
extern crate blake;
extern crate jhffi;
extern crate skeinffi;
extern crate aesti;

use super::keccak;
use super::aes::{AES};
use u64x2::u64x2;
use std::boxed::Box;

use self::groestl::{Digest, Groestl256};
use super::super::byte_string;

pub const MEM_SIZE : usize = 2097152 / 16;
const ITERATIONS : u32 = 524288;

pub fn hash(input: &[u8], aes: &AES) -> String {
    //scratchpad init
    let mut state = keccak::keccak(input);
    let mut scratchpad = init_scratchpad(&state, &aes);

    let mut a = u64x2::read(&state[0..16]) ^ u64x2::read(&state[32..48]);
    let mut b = u64x2::read(&state[16..32]) ^ u64x2::read(&state[48..64]);

    for _ in 0..ITERATIONS {
        let mut ix = scratchpad_addr(&a);
        let aes_result = aes.aes_round(scratchpad[ix], a);
        scratchpad[ix] = b ^ aes_result;

        ix = scratchpad_addr(&aes_result);
        let mem = scratchpad[ix];
        let add_r = ebyte_add(&a, &ebyte_mul(&aes_result, &mem));
        scratchpad[ix] = add_r;

        a = add_r ^ mem;
        b = aes_result;
    }

    let final_result = finalise_scratchpad(scratchpad, &state, &aes);

    for k in 0..8 {
        let block = final_result[k];
        let offset = 64+(k<<4);
        block.write(&mut state[offset..offset+16]);
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

pub fn finalise_scratchpad(scratchpad: Box<[u64x2; MEM_SIZE]>, keccak_state: &[u8; 200], aes: &AES) -> [u64x2; 8] {
    let keys = aes_round_keys(&keccak_state, 32, aes);

    let mut state : [u64x2; 8] = [u64x2(0,0); 8];
    for i in 0..8 {
        let offset = i*16;
        let mut block = u64x2::read(&keccak_state[64+offset..64+offset+16]);
        block = scratchpad[i] ^ block;
        for k in 0..10 {
            block = aes.aes_round(block, keys[k]);
        }
        state[i] = block;
    }

    for k in (8..MEM_SIZE).step_by(8) {
        for i in 0..8 {
            let mut block = scratchpad[k+i];
            block = state[i] ^ block;
            for j in 0..10 {
                block = aes.aes_round(block, keys[j]);
            }
            state[i] = block;
        }
    }

    return state;
}

pub fn init_scratchpad(state: &[u8; 200], aes: &AES) -> Box<[u64x2; MEM_SIZE]>{
    let keys = aes_round_keys(&state, 0, aes);

    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];
    for i in 0..8 {
        let offset = i*16;
        let mut block = u64x2::read(&state[64+offset..64+offset+16]);
        for k in 0..10 {
            block = aes.aes_round(block, keys[k]);
        }
        scratchpad[i] = block;
    }

    for k in (0..(MEM_SIZE-8)).step_by(8) {
        for i in k..(k+8) {
            let mut block = scratchpad[i];
            for j in 0..10 {
                block = aes.aes_round(block, keys[j]);
            }
            scratchpad[i+8] = block
        }
    }

    return scratchpad;
}

//TODO move this to aes.rs and only expose this function in AES
pub fn aes_round_keys(state: &[u8; 200], offset: usize, aes: &AES) -> [u64x2;10] {
    let mut r : [u64x2;10] = [u64x2(0,0);10];

    let input0 = u64x2::read(&state[offset..(offset+16)]);
    let input1 = u64x2::read(&state[(offset+16)..(offset+32)]);
    r[0] = input0;
    r[1] = input1;

    let (input0, input1) = aes.gen_key_0x01(input0, input1);
    r[2] = input0;
    r[3] = input1;

    let (input0, input1) = aes.gen_key_0x02(input0, input1);
    r[4] = input0;
    r[5] = input1;

    let (input0, input1) = aes.gen_key_0x04(input0, input1);
    r[6] = input0;
    r[7] = input1;

    let (input0, input1) = aes.gen_key_0x08(input0, input1);
    r[8] = input0;
    r[9] = input1;

    return r;
}
