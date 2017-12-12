extern crate mithril;

use mithril::cryptonight::sw_aes;
use mithril::byte_string;

#[test]
fn test_sub_word() {
    assert_eq!(sw_aes::sub_word(0xb1584737), 0xc86aa09a);
    assert_eq!(sw_aes::sub_word(0x49311db7), 0x3bc7a4a9);
}

#[test]
fn test_rotr() {
    assert_eq!(sw_aes::rotr(0xfcba91c5, 8), 0xc5fcba91);
    assert_eq!(sw_aes::rotr(0x97dea74b, 8), 0x4b97dea7);
}

#[test]
fn test_aeskeygenassist() {

    let mut u_in = byte_string::hex2_u64x2_be("e6cba1621beee65388b7bfb9053a7182");
    assert_eq!(byte_string::hex2_u64x2_be("aa8e1f368e1f32aa56c4a90cc4a90856"), sw_aes::aes_keygenassist(u_in, 4));

    u_in = byte_string::hex2_u64x2_be("f9d1a083fb0c294dbfa29b1fa7d7e01b");
    assert_eq!(byte_string::hex2_u64x2_be("ec993ee0993ee0ecc0083a14083a14c0"), sw_aes::aes_keygenassist(u_in, 0));

    u_in = byte_string::hex2_u64x2_be("e99669e60f5dc88414b32ed79c04916e");
    assert_eq!(byte_string::hex2_u64x2_be("8e1e90f11e90f98e0efa6d39fa6d310e"), sw_aes::aes_keygenassist(u_in, 8));
}
