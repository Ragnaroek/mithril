extern crate mithril;

use mithril::byte_string::{hex2_u32_le};

#[test]
fn test_hex2_u32_le() {
    assert_eq!(hex2_u32_le("00000000"), 0);
    assert_eq!(hex2_u32_le("169f0200"), 171798);
}
