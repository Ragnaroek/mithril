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
    let input1 = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c60d2f907");
    let input2 = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
    let input3 = byte_string::string_to_u8_array("66666666d3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
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
    let input1 = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c60d2f907");
    let input2 = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
    let input3 = byte_string::string_to_u8_array("66666666d3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    let aes = aes::new(AESSupport::SW);
    b.iter(|| {
        hash::hash(&mut scratchpad, &input1, &aes);
        hash::hash(&mut scratchpad, &input2, &aes);
        hash::hash(&mut scratchpad, &input3, &aes);
    });
}
