
use super::hw_aes;
use super::sw_aes;

use u64x2::u64x2;

#[derive(Clone)]
pub enum AESSupport{
    HW,
    SW
}

pub struct AES {
    gen_key_0x01_f: fn(u64x2, u64x2) -> (u64x2, u64x2),
    gen_key_0x02_f: fn(u64x2, u64x2) -> (u64x2, u64x2),
    gen_key_0x04_f: fn(u64x2, u64x2) -> (u64x2, u64x2),
    gen_key_0x08_f: fn(u64x2, u64x2) -> (u64x2, u64x2),
    aes_round_f:    fn(u64x2, u64x2) -> u64x2,
}

pub fn new(aes: AESSupport) -> AES {
    let gen_key_0x01_f = match aes {
        AESSupport::SW => sw_aes::gen_key_0x01,
        AESSupport::HW => hw_aes::gen_key_0x01,
    };
    let gen_key_0x02_f = match aes {
        AESSupport::SW => sw_aes::gen_key_0x02,
        AESSupport::HW => hw_aes::gen_key_0x02,
    };
    let gen_key_0x04_f = match aes {
        AESSupport::SW => sw_aes::gen_key_0x04,
        AESSupport::HW => hw_aes::gen_key_0x04,
    };
    let gen_key_0x08_f = match aes {
        AESSupport::SW => sw_aes::gen_key_0x08,
        AESSupport::HW => hw_aes::gen_key_0x08,
    };
    let aes_round_f = match aes {
        AESSupport::SW => sw_aes::aes_round,
        AESSupport::HW => hw_aes::aes_round,
    };

    return AES{gen_key_0x01_f, gen_key_0x02_f, gen_key_0x04_f, gen_key_0x08_f, aes_round_f}
}

impl AES {
    pub fn gen_key_0x01(&self, input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
        return (self.gen_key_0x01_f)(input0, input1);
    }
    pub fn gen_key_0x02(&self, input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
        return (self.gen_key_0x02_f)(input0, input1);
    }
    pub fn gen_key_0x04(&self, input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
        return (self.gen_key_0x04_f)(input0, input1);
    }
    pub fn gen_key_0x08(&self, input0: u64x2, input1: u64x2) -> (u64x2, u64x2) {
        return (self.gen_key_0x08_f)(input0, input1);
    }
    pub fn aes_round(&self, block: u64x2, key: u64x2) -> u64x2 {
        return (self.aes_round_f)(block, key);
    }
}
