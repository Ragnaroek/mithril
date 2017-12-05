
extern crate aesti;

use u64x2::u64x2;

pub fn gen_round_keys(input0: u64x2, input1: u64x2) -> [u64x2;10] {

/*
    let mut k : [u8;16] = [0;16];
    input0.write(&mut k);
    let aes = aesti::Aes::with_key(&mut k).unwrap();
    println!("key_enc={:?}", aes.key_enc[0]);
*/
    let r : [u64x2;10] = [u64x2(0,0);10];
    return r;
}

pub fn aes_round(block: u64x2, key: u64x2) -> u64x2 {
    return u64x2(0, 0);
}
