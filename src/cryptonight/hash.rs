
use super::keccak;
use super::aes;

const MEM_SIZE : usize = 2097152 / 16;

pub fn hash(input: &[u8]) {

    //scratchpad init
    let state = keccak::keccak(input);
    let keys = aes_round_keys(&state);

    let mut scratchpad : [u128; MEM_SIZE] = [0; MEM_SIZE];
    for i in 0..8 {
        let offset = i*16;
        let mut block = to_u128(&state[64+offset..64+offset+16]);
        for k in 0..10 {
            block = aes::aes_round(block, keys[i]);
        }
        scratchpad[i] = block;
    }
}

fn to_u128(input: &[u8]) -> u128 {
    let mut r = 0;
    for i in 0..16 {
        let mut m = u128::from(input[i]);
        m = m << i * 8;
        r |= m;
    }
    return r;
}

pub fn aes_round_keys(state: &[u8; 200]) -> [u128;10] {
    let mut r : [u128;10] = [0;10];

    r[0] = to_u128(&state[0..16]);
    r[1] = to_u128(&state[16..32]);

    let input0 = aes::u64x2::read(&state[0..16]);
    let input1 = aes::u64x2::read(&state[16..32]);

    let (input0, input1) = aes::gen_key_0x01(input0, input1);
    r[2] = input0.to_u128();
    r[3] = input1.to_u128();

    let (input0, input1) = aes::gen_key_0x02(input0, input1);
    r[4] = input0.to_u128();
    r[5] = input1.to_u128();

    let (input0, input1) = aes::gen_key_0x04(input0, input1);
    r[6] = input0.to_u128();
    r[7] = input1.to_u128();

    let (input0, input1) = aes::gen_key_0x08(input0, input1);
    r[8] = input0.to_u128();
    r[9] = input1.to_u128();

    return r;
}
