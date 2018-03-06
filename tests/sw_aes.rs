#![allow(unknown_lints)]
#![allow(unreadable_literal)]
#![allow(inline_always)]

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
    let u_in = byte_string::hex2_u64x2_be("e6cba1621beee65388b7bfb9053a7182");
    assert_eq!(byte_string::hex2_u64x2_be("aa8e1f368e1f32aa56c4a90cc4a90856"), sw_aes::aes_keygenassist(u_in, 4));

    let u_in = byte_string::hex2_u64x2_be("f9d1a083fb0c294dbfa29b1fa7d7e01b");
    assert_eq!(byte_string::hex2_u64x2_be("ec993ee0993ee0ecc0083a14083a14c0"), sw_aes::aes_keygenassist(u_in, 0));

    let u_in = byte_string::hex2_u64x2_be("e99669e60f5dc88414b32ed79c04916e");
    assert_eq!(byte_string::hex2_u64x2_be("8e1e90f11e90f98e0efa6d39fa6d310e"), sw_aes::aes_keygenassist(u_in, 8));
}

#[test]
fn test_sl_xor() {
    let u_in = byte_string::hex2_u64x2_be("31de2573e06d02798f2f61c0a99f5f49");
    assert_eq!(byte_string::hex2_u64x2_be("f7031983c6dd3cf026b03e89a99f5f49"), sw_aes::sl_xor(u_in));

    let u_in = byte_string::hex2_u64x2_be("fed02602cf0e03712f630108a04c60c8");
    assert_eq!(byte_string::hex2_u64x2_be("bef144b3402162b18f2f61c0a04c60c8"), sw_aes::sl_xor(u_in));
}

#[test]
fn test_aes_keygenassist_sub_0x01() {

    let u_in0 = byte_string::hex2_u64x2_be("2380b05aea3f02a15e85e227dba2f942");
    let u_in1 = byte_string::hex2_u64x2_be("f263052a5f97ad26494e3849429f9844");
    let (out0, out1) = sw_aes::aes_keygenassist_sub(u_in0, u_in1, 0x01);
    assert_eq!("a91152f48a91e2ae60aee00f3e2b0228", byte_string::u64x2_to_string(out0));
    assert_eq!("75a708be87c40d94d853a0b2911d98fb", byte_string::u64x2_to_string(out1));

    assert_eq!(byte_string::hex2_u64x2_be("a91152f48a91e2ae60aee00f3e2b0228"), out0);
    assert_eq!(byte_string::hex2_u64x2_be("75a708be87c40d94d853a0b2911d98fb"), out1);

    let u_in0 = byte_string::hex2_u64x2_be("92ea2321c65589e53bebc98a5261712d");
    let u_in1 = byte_string::hex2_u64x2_be("6cad579bcf337a05078e4cfd10e0ef29");
    let (out0, out1) = sw_aes::aes_keygenassist_sub(u_in0, u_in1, 0x01);
    assert_eq!(byte_string::hex2_u64x2_be("29658739bb8fa4187dda2dfd4631e477"), out0);
    assert_eq!(byte_string::hex2_u64x2_be("11bd99587d10cec3b223b4c6b5adf83b"), out1);
}

#[test]
fn test_aes_keygenassist_sub_0x08() {

    let u_in0 = byte_string::hex2_u64x2_be("270ae071408cc2a4c2683ddbc8a857a0");
    let u_in1 = byte_string::hex2_u64x2_be("9b54322927ef223dc692c35e73b244c2");
    let (out0, out1) = sw_aes::aes_keygenassist_sub(u_in0, u_in1, 0x08);
    assert_eq!(byte_string::hex2_u64x2_be("c8526885ef5888f4afd44a506dbc778b"), out0);
    assert_eq!(byte_string::hex2_u64x2_be("e19bd21f7acfe0365d20c20b9bb20155"), out1);

    let u_in0 = byte_string::hex2_u64x2_be("61398f2024a3b0b907f76bd29c07d5b6");
    let u_in1 = byte_string::hex2_u64x2_be("7524ba9924ef09ed2bf4163501bf99cc");
    let (out0, out1) = sw_aes::aes_keygenassist_sub(u_in0, u_in1, 0x08);
    assert_eq!(byte_string::hex2_u64x2_be("30f7b70151ce3821756d8898729ae34a"), out0);
    assert_eq!(byte_string::hex2_u64x2_be("7fe895f10acc2f682e23268505d730b0"), out1);
}

#[test] //used this to debug some wrongness
fn test_aes_keygenassist_sub_debug() {

    let u_in0 = byte_string::hex2_u64x2_be("63a433749cc480a4e84541acc62fdabf");
    let u_in1 = byte_string::hex2_u64x2_be("2d95fc16cc409d24141bbc7d4cd3d577");
    let (out0, out1) = sw_aes::aes_keygenassist_sub(u_in0, u_in1, 0x08);
    assert_eq!(byte_string::hex2_u64x2_be("96d2027bf576310f69b2b1ab81f7f007"), out0);
    assert_eq!(byte_string::hex2_u64x2_be("29a87f19043d830fc87d1e2bdc66a256"), out1);
}
