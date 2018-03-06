#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate mithril;

use mithril::byte_string::{hex2_u32_le, hex2_u64_be, string_to_u8_array, u8_array_to_string, hex2_u64x2_be, u64x2_to_string};

#[test]
fn test_hex2_u32_le() {
    assert_eq!(hex2_u32_le("00000000"), 0);
    assert_eq!(hex2_u32_le("169f0200"), 171798);
}

#[test]
fn test_hex2_u64_be() {
    assert_eq!(hex2_u64_be("0000000000000000"), 0);
    assert_eq!(hex2_u64_be("0000765ad382642b"), 130132467672107);
    assert_eq!(hex2_u64_be("b4df231dd4de"), 198870459864286);
    assert_eq!(hex2_u64_be("0000b4df231dd4de"), 198870459864286);
}

#[test]
fn test_conv_back_and_forth() {
    let str_in = "06068cf792d0057f8b118fb8ee53bc32f72dcbae3e6ab44fd846995e8e145566eca098b19020f30000000104c599c5199374899d45470ffd1a381cb6d3aa186965298dbf37a37d03cea32a05";
    let a = string_to_u8_array(str_in);
    let str_out = u8_array_to_string(&a);
    assert_eq!(str_in, str_out);
}

#[test]
fn test_hex2_u64x2_be() {
    let u = hex2_u64x2_be("ef49b24c5ec09109bc268b0a0e0fca62");
    assert_eq!("ef49b24c5ec09109bc268b0a0e0fca62", u64x2_to_string(u));
}
