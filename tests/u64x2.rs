extern crate mithril;

use mithril::u64x2::u64x2;
use mithril::byte_string;

#[test]
fn test_xor() {
    let a = u64x2::read(&byte_string::string_to_u8_array("00000000000000000000000000000000"));
    let b = u64x2::read(&byte_string::string_to_u8_array("11111111111111111111111111111111"));

    let c = a ^ b;
    assert_eq!(byte_string::u64x2_to_string(c), "11111111111111111111111111111111");

    let a2 = u64x2::read(&byte_string::string_to_u8_array("01000010000000001000000010000001"));
    let b2 = u64x2::read(&byte_string::string_to_u8_array("11111111111111111111111111111111"));

    let c2 = a2 ^ b2;
    assert_eq!(byte_string::u64x2_to_string(c2), "10111101111111011111111101111110");
}

#[test]
fn test_plus() {
    let a0 = u64x2::read(&byte_string::string_to_u8_array("00000000000000000000000000000000"));
    let b0 = u64x2::read(&byte_string::string_to_u8_array("11111111111111111111111111111111"));
    let c0 = a0 + b0;
    assert_eq!(c0, byte_string::hex2_u64x2_be("11111111111111111111111111111111"));

    let a1 = byte_string::hex2_u64x2_be("00000000000000000008B7A7587ABC82");
    let b1 = byte_string::hex2_u64x2_be("0000000000000000000000B47856327A");
    let c1 = a1 + b1;
    assert_eq!(c1, byte_string::hex2_u64x2_be("00000000000000000008B85BD0D0EEFC"));
}

#[test]
fn test_write_offset() {
    let mut dst : [u8;100] = [0;100];
    u64x2(0xFFEEDDCCBBAA1122, 0x33445566778899FF).writeOffset(&mut dst, 66);

    assert_eq!(dst[66],0x22);
    assert_eq!(dst[67],0x11);
    assert_eq!(dst[68],0xAA);
    assert_eq!(dst[69],0xBB);
    assert_eq!(dst[70],0xCC);
    assert_eq!(dst[71],0xDD);
    assert_eq!(dst[72],0xEE);
    assert_eq!(dst[73],0xFF);
    assert_eq!(dst[74],0xFF);
    assert_eq!(dst[75],0x99);
    assert_eq!(dst[76],0x88);
    assert_eq!(dst[77],0x77);
    assert_eq!(dst[78],0x66);
    assert_eq!(dst[79],0x55);
    assert_eq!(dst[80],0x44);
    assert_eq!(dst[81],0x33);
}
