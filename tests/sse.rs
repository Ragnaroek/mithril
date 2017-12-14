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

/*
x-or-input: 2d95fc16cc409d24141bbc7d4cd3d577
x-or-output: cc409d24141bbc7d4cd3d57700000000

x-or-input: 63a433749cc480a4e84541acc62fdabf
x-or-output: 9cc480a4e84541acc62fdabf00000000
*/