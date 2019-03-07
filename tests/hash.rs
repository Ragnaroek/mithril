#![feature(box_syntax)]

extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;
use mithril::cryptonight::hash::{MEM_SIZE, ebyte_mul, shuffle_0, shuffle_1, division};
use mithril::cryptonight::keccak;
use mithril::cryptonight::aes;
use mithril::cryptonight::aes::{AESSupport};
use mithril::u64x2::{u64x2};
use std::u64;

#[test]
fn test_init_scratchpad() {
    let input = byte_string::string_to_u8_array("0505988ab3cc05c725e9fe211fb23e9ccd442829a684d9a887d097ec33dbfd6085e70068ee779714000000cd484698d1fa1981993198f995e2c4fea6f31b6b3f8fbcf742b32ce2d5951cdd07");
    let aes = aes::new(AESSupport::HW);
    let mut a = keccak::keccak(&input);
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    hash::init_scratchpad(&mut scratchpad, &mut a, &aes);
    assert_eq!(byte_string::u64x2_to_string(scratchpad[0]), "f4e41f8bb21278bf69fef5414eedbd5d");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[1]), "d49d9e57821fa5220426015c6d9f218f");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[2]), "44c7e927a427b335d76fb01c18cb7629");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[3]), "99fcd81389062cc471260947e3ef3858");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[4]), "904dc9e321b05fe70537886ffeff76b4");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[5]), "e581057ea64bfc688f6262478adfda4d");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[6]), "1beb312a04e2418ff2d9f10376ca3142");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[7]), "1f88f1e57bb80c1717e1cdf74f9b5d31");

    assert_eq!(byte_string::u64x2_to_string(scratchpad[8]),  "851f0b8f4f30f744a8b2bcecdb468198");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[9]),  "c3c4d13cd0f1502ced9c63929e3e9588");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[10]), "3e8825dccc7726e25a937432d724b273");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[11]), "f891bc42841bf3dab14bd7b7fdf89a33");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[12]), "6e907a303bbc32fe47cd0cb080969894");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[13]), "f3a068a0fe82415a9e9e45b3eb6a76da");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[14]), "9262feadb97bf76a8dbcc0a32e395968");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[15]), "d722f17a07c4eaa5486fe39cc6d9609b");

    assert_eq!(byte_string::u64x2_to_string(scratchpad[16]), "5ac5dba2328b12c869e7cf919b347e80");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[17]), "3ad72813d8215a1e8b966a7c19258003");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[18]), "802219ba78b4259525b0cd8bd2112336");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[19]), "9e01d10e7cc1be2855b783b3a79884bf");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[20]), "c91bb830de5effea6cf238b15f54d4b5");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[21]), "9372f958c16b5591b1c44bf22b3d4d20");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[22]), "e4f26cf077bee0304fb11d6eaad48e82");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[23]), "953dea5d5b4fa058ab3f06ffc1ea0ec1");
}

#[test]
fn test_init_scratchpad_tail() {
    let input = byte_string::string_to_u8_array("0505a9e6c9cc0529b1608dbf9840e20164ee24efd67979e6a937ce174f9aff423a96a7cc5bdcd504008000ca5d84112bf941d3df2c44132b2df08fb766ebf0cc0ad4ccc4012b0929e4edeb04");
    let aes = aes::new(AESSupport::HW);
    let mut a = keccak::keccak(&input);
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    hash::init_scratchpad(&mut scratchpad, &mut a, &aes);
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-16]), "c7a1f8660d2cf76f652e90e067f41e30");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-15]), "29f328053cb5ce9a3144fedcebeb0455");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-14]), "e3592994985e0937fc0b43c1a6ac738c");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-13]), "8d6844339f9196e249add1d2531907a9");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-12]), "a774d67ff9a5836f6f315822984e3e82");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-11]), "e1c5aaeb19b05eed5637d023056b8205");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-10]), "0501dea25f90b0049e92261354ecf772");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-9]),  "94c5166924405464f762963e09b8c55c");

    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-8]), "304fe9475ecec1065413f0a591b4b2ba");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-7]), "a70bd25d9d8011b68a8ff4282ba35eef");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-6]), "39ed29569a1736736f1eb608f73372bd");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-5]), "aaecaabb587e5027f48ac0832a157471");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-4]), "02935ff82a7c59380f69a1a9dfbf66e0");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-3]), "ae0d8323c4dbe1ec68f8ae668d447bcd");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-2]), "e2a0e238f8d5f1dd3dfefa5ac05445b0");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-1]), "76696694f5e369e0c543e82f84559129");
}

#[test]
fn test_hash_hardware_v8() {
    let aes = aes::new(AESSupport::HW);

    let input = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c60d2f907");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes), "f12b181f2b5a84d8fca047206c605f20b6b3a9b29da3505152caaeee758e39fe");

    let input = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes), "b5bc564bf7f67622f4ebbfd9c2754f994c24afae820f69acac3f633fa19f9131");

    let input = byte_string::string_to_u8_array("66666666d3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes), "f4e15a61d170cac5e21deff989b1db2af88455c1a8539c3fabfee5be077f32f9");
}

#[test]
fn test_hash_software_v8() {
    let aes = aes::new(AESSupport::SW);

    let input = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c60d2f907");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes), "f12b181f2b5a84d8fca047206c605f20b6b3a9b29da3505152caaeee758e39fe");

    let input = byte_string::string_to_u8_array("09099aebd3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes), "b5bc564bf7f67622f4ebbfd9c2754f994c24afae820f69acac3f633fa19f9131");

    let input = byte_string::string_to_u8_array("66666666d3e1057aad462f2d998d8b9adcf16e03a5bf1820728240eefe433735904fcf663eeb1d00000000b0203ca955ed446e47ab9e884941bc67c75ecb06e444036aafc7ff442c66d26666");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes), "f4e15a61d170cac5e21deff989b1db2af88455c1a8539c3fabfee5be077f32f9");
}

#[test]
fn test_shuffle_0() {
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    scratchpad[0x1b87c] = u64x2(0x3877694fc39e5d94, 0x799ca5c6420917cb);
    scratchpad[0x1b87f] = u64x2(0xb22757c5fb5bf452, 0xf50fb07fc457b691);
    scratchpad[0x1b87e] = u64x2(0x883ac9cd01a17561, 0x284e85a9bfcef8e9);

    shuffle_0(0x148f30d3747b87d5, &mut scratchpad, u64x2(0x148f30d3747b87d5, 0xe2d9028ffc71e2ef), u64x2(0xd93c328b6174f87e,0xf1f90c1078c99e1), u64x2(0xa4cdd03a27c55885,0x3cad9888b6b3e0a6));

    assert_eq!(scratchpad[0x1b87c], u64x2(0x2d089a072966cde6, 0x64fc1e327682d98f));
    assert_eq!(scratchpad[0x1b87f], u64x2(0x11b39bdb25135612, 0x88bc36874995b1ac));
    assert_eq!(scratchpad[0x1b87e], u64x2(0xc6b688996fd77c27, 0xd7e8b30fc0c99980));
}

#[test]
fn test_shuffle_1() {
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    scratchpad[0x1b87c] = u64x2(0x6c9cc13c764ca34, 0x2774da42a925d54f);
    scratchpad[0x1b87f] = u64x2(0x79ff3840385e7f10, 0xb657be9b5ac5e4c9);
    scratchpad[0x1b87e] = u64x2(0x5a167882ac6614cb, 0xb2182b918baf4c32);

    let (lo, hi) = shuffle_1(0x148f30d3747b87d5, &mut scratchpad,
        u64x2(0x148f30d3747b87d5, 0xe2d9028ffc71e2ef),
        u64x2(0xd93c328b6174f87e,0xf1f90c1078c99e1),
        u64x2(0xa4cdd03a27c55885,0x3cad9888b6b3e0a6),
        0xd11111a5c21c5d, 0xc2cee564d624e13f);

    assert_eq!(scratchpad[0x1b87c], u64x2(0xfee448bcd42b6d50, 0xeec5c41a42632cd8));
    assert_eq!(scratchpad[0x1b87f], u64x2(0x9d435c0272b52389, 0x36c55c14147462f3));
    assert_eq!(scratchpad[0x1b87e], u64x2(0x8e8e6913acda06e5, 0x9930c12b5737c7b8));

    assert_eq!(lo, 0xb686af8aff07f894);
    assert_eq!(hi, 0xbb31dd24ee7a9e2f);
}

#[test]
fn test_division() {

    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    let aes_result = u64x2(0xfd1e39f73fe70437, 0x6a723c2ebf8e89bc);
    let sqrt_res = 0x3fa2f8323bb48333;
    let div_res = 0xf8e216d89e32a083;

    scratchpad[0x7043].0 = 0x65023ca86652288;

    let (sqr, div) = division(0x7043, &mut scratchpad, &aes_result, sqrt_res, div_res);
    assert_eq!(0x7fe4948070f, sqr);
    assert_eq!(0x5168572a94a7873a, div);
    assert_eq!(0xc506b6211857820b, scratchpad[0x7043].0);
}

#[test]
fn test_ebyte_mul() {
    let u1 = u64x2(5, 42);
    let u2 = u64x2(10, 32);
    assert_eq!(ebyte_mul(&u1, &u2), u64x2(0, 50));

    let u1 = u64x2(0, 42);
    let u2 = u64x2(0, 32);
    assert_eq!(ebyte_mul(&u1, &u2), u64x2(0, 0));

    let u1 = u64x2(u64::MAX, 42);
    let u2 = u64x2(u64::MAX, 32);
    assert_eq!(ebyte_mul(&u1, &u2), u64x2(u64::MAX-1, 1));
}

#[test]
fn test_xoru64() {
    assert_eq!(0x7cdcb5631830db27 as u64, 0x995fb21afb79db83 as u64 ^ 0xe5830779e34900a4 as u64);
}
