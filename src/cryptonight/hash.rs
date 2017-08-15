
use super::keccak;
use super::aes;
use u64x2::u64x2;
use std::boxed::Box;

pub const MEM_SIZE : usize = 2097152 / 16;
const ITERATIONS : u32 = 524288;

pub fn hash(input: &[u8]) {

    //scratchpad init
    let state = keccak::keccak(input);
    let scratchpad = init_scratchpad(&state);

    let a = u64x2::read(&state[0..16]) ^ u64x2::read(&state[32..48]);
    let b = u64x2::read(&state[16..32]) ^ u64x2::read(&state[48..64]);

    println!("a={:?}", super::super::byte_string::u64x2_to_string(a));
    println!("b={:?}", super::super::byte_string::u64x2_to_string(b));

    for i in 0..ITERATIONS {

    }
}

pub fn init_scratchpad(state: &[u8; 200]) -> Box<[u64x2; MEM_SIZE]>{
    let keys = aes_round_keys(&state);

    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];
    for i in 0..8 {
        let offset = i*16;
        let mut block = u64x2::read(&state[64+offset..64+offset+16]);
        for k in 0..10 {
            block = aes::aes_round(block, keys[k]);
        }
        scratchpad[i] = block;
    }

    for k in (0..(MEM_SIZE-8)).step_by(8) {
        for i in k..(k+8) {
            let mut block = scratchpad[i];
            for j in 0..10 {
                block = aes::aes_round(block, keys[j]);
            }
            scratchpad[i+8] = block
        }
    }

    return scratchpad;
}


pub fn aes_round_keys(state: &[u8; 200]) -> [u64x2;10] {
    let mut r : [u64x2;10] = [u64x2(0,0);10];

    let input0 = u64x2::read(&state[0..16]);
    let input1 = u64x2::read(&state[16..32]);
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
