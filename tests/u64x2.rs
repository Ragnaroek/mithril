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
