
use super::keccak;
use super::aes;
use super::aes::u64x2;
use std::boxed::Box;

const MEM_SIZE : usize = 2097152 / 16;

pub fn hash(input: &[u8]) {

    //scratchpad init
    let state = keccak::keccak(input);

    let scratchpad = init_scratchpad(&state);

}

pub fn init_scratchpad(state: &[u8; 200]) -> Box<[u64x2; MEM_SIZE]>{
    let keys = aes_round_keys(&state);

    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];
    for i in 0..8 {
        let offset = i*16;
        let mut block = aes::u64x2::read(&state[64+offset..64+offset+16]);
        for k in 0..10 {
            block = aes::aes_round(block, keys[k]);
        }
        scratchpad[i] = block;
    }

    //TODO init rest of scratchpad => encrypt 8x16 previous blocks
    // ==> directly read u64x2 from scratchpad

    return scratchpad;
}


pub fn aes_round_keys(state: &[u8; 200]) -> [u64x2;10] {
    let mut r : [u64x2;10] = [u64x2(0,0);10];

    let input0 = aes::u64x2::read(&state[0..16]);
    let input1 = aes::u64x2::read(&state[16..32]);
    r[0] = input0;
    r[1] = input1;

    let (input0, input1) = aes::gen_key_0x01(input0, input1);
    r[2] = input0;
    r[3] = input1;

    let (input0, input1) = aes::gen_key_0x02(input0, input1);
    r[4] = input0;
    r[5] = input1;

    let (input0, input1) = aes::gen_key_0x04(input0, input1);
    r[6] = input0;
    r[7] = input1;

    let (input0, input1) = aes::gen_key_0x08(input0, input1);
    r[8] = input0;
    r[9] = input1;

    return r;
}

/*
fn to_u128(input: &[u8]) -> u128 {
    let mut r = 0;
    for i in 0..16 {
        let mut m = u128::from(input[i]);
        m = m << i * 8;
        r |= m;
    }
    return r;
}*/
