
use super::keccak;

pub fn hash(input: &[u8]) {
    let a = keccak::keccak(input);

    //TODO extract round key 0..9

}
