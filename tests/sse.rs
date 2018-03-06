#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate mithril;

use mithril::cryptonight::sse;
use mithril::byte_string;

#[test]
fn test_mm_shuffle_epi32_0x55() {
    let mut u_in = byte_string::hex2_u64x2_be("ef49b24c5ec09109bc268b0a0e0fca62");
    assert_eq!(byte_string::hex2_u64x2_be("bc268b0abc268b0abc268b0abc268b0a"), sse::_mm_shuffle_epi32_0x55(u_in));

    u_in = byte_string::hex2_u64x2_be("5d5e9843abd1ad2aa1b2434cd12a4e3a");
    assert_eq!(byte_string::hex2_u64x2_be("a1b2434ca1b2434ca1b2434ca1b2434c"), sse::_mm_shuffle_epi32_0x55(u_in));
}

#[test]
fn test_mm_shuffle_epi32_0xff() {
    let mut u_in = byte_string::hex2_u64x2_be("2d95fc16cc409d24141bbc7d4cd3d577");
    assert_eq!(byte_string::hex2_u64x2_be("2d95fc162d95fc162d95fc162d95fc16"), sse::_mm_shuffle_epi32_0xff(u_in));

    u_in = byte_string::hex2_u64x2_be("96d2027bf576310f69b2b1ab81f7f007");
    assert_eq!(byte_string::hex2_u64x2_be("96d2027b96d2027b96d2027b96d2027b"), sse::_mm_shuffle_epi32_0xff(u_in));
}

#[test]
fn test_mm_shuffle_epi32_0xaa() {
    let u_in = byte_string::hex2_u64x2_be("ee6eb8756eb875ee43148cae148cae43");
    assert_eq!(byte_string::hex2_u64x2_be("6eb875ee6eb875ee6eb875ee6eb875ee"), sse::_mm_shuffle_epi32_0xaa(u_in));

    let u_in = byte_string::hex2_u64x2_be("b7ef1273ef1273b7b5c5687fc5687fb5");
    assert_eq!(byte_string::hex2_u64x2_be("ef1273b7ef1273b7ef1273b7ef1273b7"), sse::_mm_shuffle_epi32_0xaa(u_in));
}

#[test]
fn test_mm_cvtsi128_si32() {
    let u_in = byte_string::hex2_u64x2_be("396d009f396d009f396d009f396d009f");
    assert_eq!(0x396d009f, sse::_mm_cvtsi128_si32(u_in));

    let u_in = byte_string::hex2_u64x2_be("f90cc069f90cc069f90cc069f90cc069");
    assert_eq!(0xf90cc069, sse::_mm_cvtsi128_si32(u_in));
}

#[test]
fn test_mm_slli_si128_0x04() {
    let u_in = byte_string::hex2_u64x2_be("2d95fc16cc409d24141bbc7d4cd3d577");
    assert_eq!(byte_string::hex2_u64x2_be("cc409d24141bbc7d4cd3d57700000000"), sse::_mm_slli_si128_0x04(u_in));

    let u_in = byte_string::hex2_u64x2_be("63a433749cc480a4e84541acc62fdabf");
    assert_eq!(byte_string::hex2_u64x2_be("9cc480a4e84541acc62fdabf00000000"), sse::_mm_slli_si128_0x04(u_in));
}

#[test]
fn test_mm_xor_si128() {
    let u_in0 = byte_string::hex2_u64x2_be("cdd61469a07994f7672a138623d3ee57");
    let u_in1 = byte_string::hex2_u64x2_be("a07994f7672a138623d3ee5700000000");
    assert_eq!(byte_string::hex2_u64x2_be("6daf809ec753877144f9fdd123d3ee57"), sse::_mm_xor_si128(u_in0, u_in1));

    let u_in0 = byte_string::hex2_u64x2_be("f6816d7c96db4a8a5f872d620824e4da");
    let u_in1 = byte_string::hex2_u64x2_be("96db4a8a5f872d620824e4da00000000");
    assert_eq!(byte_string::hex2_u64x2_be("605a27f6c95c67e857a3c9b80824e4da"), sse::_mm_xor_si128(u_in0, u_in1));
}

#[test]
fn test_mm_mul_su32() {
    let u_in0 = byte_string::hex2_u64x2_be("00000000000000000000000000000005");
    let u_in1 = byte_string::hex2_u64x2_be("0000000000000000000000000000000A");
    assert_eq!( byte_string::hex2_u64x2_be("00000000000000000000000000000032"), sse::_mm_mul_su32(u_in0, u_in1));
}
