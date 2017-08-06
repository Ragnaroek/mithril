
use super::keccak;
use super::aes;

pub fn hash(input: &[u8]) {
    let a = keccak::keccak(input);

    let r = aes_round_keys(&a);

    println!("r[0]={:016x}", r[0]);
    println!("r[1]={:016x}", r[1]);
    println!("r[2]={:016x}", r[2]);
    println!("r[3]={:016x}", r[3]);
    //TODO extract round key 0..9

}

fn to_u128(input: &[u8;16]) -> u128 {
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

    r[0] = to_u128(&copy_128bit(state, 0));
    r[1] = to_u128(&copy_128bit(state, 16));

    let input0 = aes::u64x2::read(&copy_128bit(state, 0));
    let input1 = aes::u64x2::read(&copy_128bit(state, 16));
    let (k0, k1) = aes::gen_key(input0, input1);
    r[2] = k0.to_u128();
    r[3] = k1.to_u128();

    //TODO generate other round keys from input0 and input1
    println!("k0={:016x}{:016x}", k0.1, k0.0);
    println!("k1={:016x}{:016x}", k1.1, k1.0);

    return r;
}

fn copy_128bit(input: &[u8], offset: usize) -> [u8;16] {
    let mut r = [0;16];
    for i in 0..16 {
        r[i] = input[offset+i];
    }
    return r;
}
