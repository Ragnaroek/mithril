extern crate mithril;

use mithril::byte_string::{hex2_u32_le, hex2_u64_be};

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
fn test() {
    let n : u32 = 8388665;
    assert_eq!("39008000", format!("{:x}", n.to_be()));
}
