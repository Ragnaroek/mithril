#![feature(test)]
#![feature(box_syntax)]

extern crate test;
extern crate mithril;

use test::{Bencher};
use mithril::byte_string;
use mithril::u64x2::{u64x2};
use mithril::cryptonight::hash;
use mithril::cryptonight::hash::{MEM_SIZE};
use mithril::cryptonight::aes;
use mithril::cryptonight::aes::{AESSupport};

#[bench]
fn bench_hash_with_hardware(b: &mut Bencher) {
    let input1 = byte_string::string_to_u8_array("060687f092d005c5f46c239d1bd5a0667ee32d0687aa566644f81a491a31378fb0f21d8ed5a7a38000000a75c2eacb144fd31b0050c9abb6a52e1e6b9d1692ce6c2f8d2a5e0f01d69d908e15");
    let input2 = byte_string::string_to_u8_array("0606898093d005b6a7bbdd52bf852324ad3c1db10b09501043b3c6f9c436538c848827e65e13e300000008336118421c17ce50b0ea1fa51e4d2255c0b56d5eebc00b4dd4a4ed600010685402");
    let input3 = byte_string::string_to_u8_array("0606ebba9cd005f688598a3ad7ae62d6e150005ded336138b26417772375b1bd5d3c0bc480eeb000000005f3c91e30aab34cbacb1bbb3eecb8b4dfd5e799aa4407b8a0ea4ee397707bc51017");
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    let aes = aes::new(AESSupport::HW);
    b.iter(|| {
        hash::hash(&mut scratchpad, &input1, &aes);
        hash::hash(&mut scratchpad, &input2, &aes);
        hash::hash(&mut scratchpad, &input3, &aes);
    });
}

#[bench]
fn bench_hash_with_software(b: &mut Bencher) {
    let input1 = byte_string::string_to_u8_array("060687f092d005c5f46c239d1bd5a0667ee32d0687aa566644f81a491a31378fb0f21d8ed5a7a38000000a75c2eacb144fd31b0050c9abb6a52e1e6b9d1692ce6c2f8d2a5e0f01d69d908e15");
    let input2 = byte_string::string_to_u8_array("0606898093d005b6a7bbdd52bf852324ad3c1db10b09501043b3c6f9c436538c848827e65e13e300000008336118421c17ce50b0ea1fa51e4d2255c0b56d5eebc00b4dd4a4ed600010685402");
    let input3 = byte_string::string_to_u8_array("0606ebba9cd005f688598a3ad7ae62d6e150005ded336138b26417772375b1bd5d3c0bc480eeb000000005f3c91e30aab34cbacb1bbb3eecb8b4dfd5e799aa4407b8a0ea4ee397707bc51017");
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    let aes = aes::new(AESSupport::SW);
    b.iter(|| {
        hash::hash(&mut scratchpad, &input1, &aes);
        hash::hash(&mut scratchpad, &input2, &aes);
        hash::hash(&mut scratchpad, &input3, &aes);
    });
}
