
use super::hw_aes;
use super::sw_aes;

use u64x2::u64x2;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AESSupport{
    HW,
    SW
}

pub struct AES {
    gen_aes_round_keys_f: fn(input0: u64x2, input1: u64x2) -> [u64x2;10],
    aes_round_f:          fn(u64x2, u64x2) -> u64x2,
}

pub fn new(aes: AESSupport) -> AES {
    let gen_aes_round_keys_f = match aes {
        AESSupport::SW => sw_aes::gen_round_keys,
        AESSupport::HW => hw_aes::gen_round_keys,
    };
    let aes_round_f = match aes {
        AESSupport::SW => sw_aes::aes_round,
        AESSupport::HW => hw_aes::aes_round,
    };
    AES{gen_aes_round_keys_f, aes_round_f}
}

impl AES {
    pub fn gen_round_keys(&self, input0: u64x2, input1: u64x2) -> [u64x2;10] {
        (self.gen_aes_round_keys_f)(input0, input1)
    }
    pub fn aes_round(&self, block: u64x2, key: u64x2) -> u64x2 {
        (self.aes_round_f)(block, key)
    }
}
