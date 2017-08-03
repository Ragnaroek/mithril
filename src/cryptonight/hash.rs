
use super::keccak;
use super::aes;

pub fn hash(input: &[u8]) {
    let a = keccak::keccak(input);


    //TODO extract round key 0..9

}

fn aes_round_keys(state: &[u8; 200]) -> [u128;10] {

    //aes::expand(/*read first 16 bytes as u128*/)

    let r : [u128;10] = [0;10];
    return r;
}
