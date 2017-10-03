extern crate mithril;

use mithril::byte_string::{hex2_u32_le, hex2_u64_le};

#[test]
fn test_hex2_u32_le() {
    assert_eq!(hex2_u32_le("00000000"), 0);
    assert_eq!(hex2_u32_le("169f0200"), 171798);
}

fn test_hex2_u64_le() {
    assert_eq!(hex2_u64_le("0000000000000000"), 0);
    assert_eq!(hex2_u64_le("0000b4df231dd4de"), 198870459864286);
}
