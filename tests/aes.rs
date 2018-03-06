#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate mithril;

use mithril::cryptonight::aes;
use mithril::cryptonight::aes::{AESSupport};
use mithril::cryptonight::keccak;
use mithril::u64x2::u64x2;
use mithril::byte_string;

#[test]
fn test_aes_round_hardware() {

    let aes = aes::new(AESSupport::HW);

    let result = aes.aes_round(u64x2(0xd822831cb9e31fcf,0x8818ec8e51cbeb2e),
                                u64x2(0x0d137d1656ef4daf,0x4653c1691bbe16e8));
    assert_eq!(result, u64x2(0x5add92eeb3e0e154,0x87df3c4255c0f2b3));

    let result2 = aes.aes_round(u64x2(0x5add92eeb3e0e154, 0x87df3c4255c0f2b3),
                                 u64x2(0xdc00341a8dde2c34, 0x52bd7edcacd25e2b));
    assert_eq!(result2, u64x2(0x6931c13936e75008, 0xfcef1daa76547888));
}

#[test]
fn test_aes_round_software() {

    let aes = aes::new(AESSupport::SW);

    let result = aes.aes_round(u64x2(0xd822831cb9e31fcf,0x8818ec8e51cbeb2e),
                                u64x2(0x0d137d1656ef4daf,0x4653c1691bbe16e8));
    assert_eq!(result, u64x2(0x5add92eeb3e0e154,0x87df3c4255c0f2b3));

    let result2 = aes.aes_round(u64x2(0x5add92eeb3e0e154, 0x87df3c4255c0f2b3),
                                 u64x2(0xdc00341a8dde2c34, 0x52bd7edcacd25e2b));
    assert_eq!(result2, u64x2(0x6931c13936e75008, 0xfcef1daa76547888));
}

#[test]
fn test_aes_round_keys_hardware() {
    let input = byte_string::string_to_u8_array("05059fa5a8cc05b2df5d8fa271bb2d0304d4b1842f0f50844b735746db97ee5c196c647c3a5adc0c000000640680be903f504e896daebe42cdbe11e1a938d5c7fb2d64baa6356fe6fbacb704");
    let a = keccak::keccak(&input);
    let aes = aes::new(AESSupport::HW);
    let input0 = u64x2::read(&a[0..16]);
    let input1 = u64x2::read(&a[16..32]);
    let keys = aes.gen_round_keys(input0, input1);

    assert_eq!(byte_string::u64x2_to_string(keys[0]), "4c73438521575791be9c2c292f259ec4");
    assert_eq!(byte_string::u64x2_to_string(keys[1]), "2dba7e0233542a04a9e58cf7213e2f56");
    assert_eq!(byte_string::u64x2_to_string(keys[2]), "8b45520bc736118ee661461f58fd6a36");
    assert_eq!(byte_string::u64x2_to_string(keys[3]), "ab5bf78c86e1898eb5b5a38a1c502f7d");
    assert_eq!(byte_string::u64x2_to_string(keys[4]), "968d56c61dc804cddafe15433c9f535c");
    assert_eq!(byte_string::u64x2_to_string(keys[5]), "14024341bf59b4cd39b83d438c0d9ec9");
    assert_eq!(byte_string::u64x2_to_string(keys[6]), "eede630a785335cc659b3101bf652442");
    assert_eq!(byte_string::u64x2_to_string(keys[7]), "36f3af6122f1ec209da858eda41065ae");
    assert_eq!(byte_string::u64x2_to_string(keys[8]), "a3764ef44da82dfe35fb183250602933");
    assert_eq!(byte_string::u64x2_to_string(keys[9]), "278251bd1171fedc338012fcae284a11");
}

#[test]
fn test_aes_round_keys_software() {
    let input = byte_string::string_to_u8_array("05059fa5a8cc05b2df5d8fa271bb2d0304d4b1842f0f50844b735746db97ee5c196c647c3a5adc0c000000640680be903f504e896daebe42cdbe11e1a938d5c7fb2d64baa6356fe6fbacb704");
    let a = keccak::keccak(&input);
    let aes = aes::new(AESSupport::SW);
    let input0 = u64x2::read(&a[0..16]);
    let input1 = u64x2::read(&a[16..32]);
    let keys = aes.gen_round_keys(input0, input1);

    assert_eq!(byte_string::u64x2_to_string(keys[0]), "4c73438521575791be9c2c292f259ec4");
    assert_eq!(byte_string::u64x2_to_string(keys[1]), "2dba7e0233542a04a9e58cf7213e2f56");
    assert_eq!(byte_string::u64x2_to_string(keys[2]), "8b45520bc736118ee661461f58fd6a36");
    assert_eq!(byte_string::u64x2_to_string(keys[3]), "ab5bf78c86e1898eb5b5a38a1c502f7d");
    assert_eq!(byte_string::u64x2_to_string(keys[4]), "968d56c61dc804cddafe15433c9f535c");
    assert_eq!(byte_string::u64x2_to_string(keys[5]), "14024341bf59b4cd39b83d438c0d9ec9");
    assert_eq!(byte_string::u64x2_to_string(keys[6]), "eede630a785335cc659b3101bf652442");
    assert_eq!(byte_string::u64x2_to_string(keys[7]), "36f3af6122f1ec209da858eda41065ae");
    assert_eq!(byte_string::u64x2_to_string(keys[8]), "a3764ef44da82dfe35fb183250602933");
    assert_eq!(byte_string::u64x2_to_string(keys[9]), "278251bd1171fedc338012fcae284a11");
}
