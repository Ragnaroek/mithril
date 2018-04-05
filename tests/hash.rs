#![feature(box_syntax)]

extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;
use mithril::cryptonight::hash::{MEM_SIZE, ebyte_mul};
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
fn test_hash_hardware() {

    let aes = aes::new(AESSupport::HW);

    //0=Blake
    let mut input = byte_string::string_to_u8_array("546869732069732061207465737431");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "236ebffb019ea9b19c9ff160775bc7a6ed090fce7103a32ee582d24a81db6960");

    //1=groestl
    input = byte_string::string_to_u8_array("5468697320697320612074657374");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605");

    //2=JH (xmr-stak input "This is a test66")
    input = byte_string::string_to_u8_array("54686973206973206120746573743636");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "21fb4137747541810d5f8ce821b6dacf68eb3051778e68b5bcd990c21fa08fd6");

    //3=Skein (xmr-stak input "This is a test6")
    input = byte_string::string_to_u8_array("546869732069732061207465737436");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "bfabcc134608782e8f7322972dba801267f841535372741b554356045910f614");

    input = byte_string::string_to_u8_array("0606cbe692d005ecfebc7d2249d2b43535c237c02359e888b8b05d2e980c1405779241ac3ab48500000004e62a06e71559c98a37e7b6743465f4f72e42784c5719411c935dc002e347826b05");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "5de3f18eff8271adbd3b9848b49d71230d696c7ba6c735554af8e15330ab881b");

    input = byte_string::string_to_u8_array("060687f092d005c5f46c239d1bd5a0667ee32d0687aa566644f81a491a31378fb0f21d8ed5a7a38000000a75c2eacb144fd31b0050c9abb6a52e1e6b9d1692ce6c2f8d2a5e0f01d69d908e15");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "55b4a3163cb2b0720b3b83bee0067dd088891d7fa116fe4c7250f004011c2d99");

    input = byte_string::string_to_u8_array("0606898093d005b6a7bbdd52bf852324ad3c1db10b09501043b3c6f9c436538c848827e65e13e300000008336118421c17ce50b0ea1fa51e4d2255c0b56d5eebc00b4dd4a4ed600010685402");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "c8dcfae5547a922eba99a65692636b2fd17745b62fddb588fe684ee2f80bd8fd");

    input = byte_string::string_to_u8_array("0606ebba9cd005f688598a3ad7ae62d6e150005ded336138b26417772375b1bd5d3c0bc480eeb000000005f3c91e30aab34cbacb1bbb3eecb8b4dfd5e799aa4407b8a0ea4ee397707bc51017");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "038228f36441187229333dd71d3f1b672335ec526d2101cc5fc700692c6aa9cb");
}

#[test]
fn test_hash_hardware_v7() {
    let aes = aes::new(AESSupport::HW);

    let input = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739430000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version7), "a01e369927b90e11d2159a85cedc0fed3f4cc9406b677fbe9c2afb810b20f231");

    let input = byte_string::string_to_u8_array("0707cff699d605f7eb4dbdcad3a38b462b52e9b8ecdf06fb4c95bc5b058a177f84d327f27db739420000000363862429fb90c0fc35fcb9f760c484c8532ee5f2a7cbea4e769d44cd12a7f201");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version7), "ecdf8f5c8c0b399709a764257495382fb0230cf6fa55ee02e0667cf3d5be10d6");
}

#[test]
fn test_hash_from_cryptonote_white_paper_hardware() {
    let aes = aes::new(AESSupport::HW);

    let input = byte_string::string_to_u8_array("");
    assert_eq!(hash::hash_alloc_scratchpad(&input, &aes, hash::HashVersion::Version6), "eb14e8a833fac6fe9a43b57b336789c46ffe93f2868452240720607b14387e11");

    let input2 = b"This is a test";
    assert_eq!(hash::hash_alloc_scratchpad(&input2[0..], &aes, hash::HashVersion::Version6), "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605");
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
fn test_monero_const() {
    let mut state : [u8;200] = [0;200];
    state[24*8] = 0xe2;
    state[24*8+1] = 0x7e;
    state[24*8+2] = 0xf5;
    state[24*8+3] = 0xb8;
    state[24*8+4] = 0xa0;
    state[24*8+5] = 0xf1;
    state[24*8+6] = 0xf0;
    state[24*8+7] = 0x02;
    let input = byte_string::string_to_u8_array("07079faff7d505c8e7a33205e990824c2100443918f5c2c4bba61628118c73973014b285ace21f2510000002db4224571634babd4f631443fe593e71683669822f30cb7a6257f28248b09b01");
    assert_eq!(hash::monero_const(&input, &state), 0x2f0e185a717d267);
}

#[test]
fn test_monero_const2() {
    let mut state : [u8;200] = [0;200];
    state[24*8] = 0x70;
    state[24*8+1] = 0x0b;
    state[24*8+2] = 0x91;
    state[24*8+3] = 0x25;
    state[24*8+4] = 0x90;
    state[24*8+5] = 0x77;
    state[24*8+6] = 0xd3;
    state[24*8+7] = 0x15;
    let input = byte_string::string_to_u8_array("07079db3f7d50511c7730e22e560c374bded7bffab18e541cc8cabf869d5b2055db9d363ddff501a100000fbc1ab1b9203952e930532f4ea9e9a04b110fd16dc164ba19355fcac32ec723d01");
    assert_eq!(hash::monero_const(&input, &state), 0x15d3678a756ed613);
}

#[test]
fn test_cryptonight_monero_tweak() {
    let tmp = u64x2(0x71ce99a5f3c41980, 0xa86f2347633b3786);
    let result = hash::cryptonight_monero_tweak(&tmp);
    assert_eq!(0x71ce99a5f3c41980, result.0, "got {:x}", result.0);
    assert_eq!(0xa86f2347733b3786, result.1, "got {:x}", result.1);
}
