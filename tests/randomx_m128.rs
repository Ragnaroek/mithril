extern crate mithril;

use mithril::randomx::m128::{from_i32};

#[test]
#[allow(overflowing_literals)]
fn test_m128_eq() {
    
    let m128_0 = from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let m128_0_2 = from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let m128_1 = from_i32(0xb5a8ef6, 0x7749809c8, 0xf349884, 0xa05c9f5ef);
    
    assert_eq!(m128_0, m128_0);
    assert_eq!(m128_0, m128_0_2);
    assert_eq!(m128_0_2, m128_0);
    assert_ne!(m128_0, m128_1);
    assert_ne!(m128_1, m128_0);
}