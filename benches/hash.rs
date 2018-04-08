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
    let input1 = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739430000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let input2 = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739420000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let input3 = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739410000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    let aes = aes::new(AESSupport::HW);
    b.iter(|| {
        hash::hash(&mut scratchpad, &input1, &aes, hash::HashVersion::Version7);
        hash::hash(&mut scratchpad, &input2, &aes, hash::HashVersion::Version7);
        hash::hash(&mut scratchpad, &input3, &aes, hash::HashVersion::Version7);
    });
}

#[bench]
fn bench_hash_with_software(b: &mut Bencher) {
    let input1 = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739430000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let input2 = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739420000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let input3 = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739410000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    let aes = aes::new(AESSupport::SW);
    b.iter(|| {
        hash::hash(&mut scratchpad, &input1, &aes, hash::HashVersion::Version7);
        hash::hash(&mut scratchpad, &input2, &aes, hash::HashVersion::Version7);
        hash::hash(&mut scratchpad, &input3, &aes, hash::HashVersion::Version7);
    });
}
