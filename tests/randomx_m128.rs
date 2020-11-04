extern crate mithril;

use mithril::randomx::m128::{from_i32, from_f64, zero_m128d};

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

#[test]
#[allow(overflowing_literals)]
fn test_to_i64() {
    let m = from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let (p1, p2) = m.to_i64();
    
    assert_eq!(p1, 0x31903876bb7a2914);
    assert_eq!(p2, 0xb370f616d6f7e4f3);
}

#[test]
#[allow(overflowing_literals)]
fn test_m128d_eq() {
    let m128_0 = from_f64(6.66, -8936.6584);
    let m128_0_2 = from_f64(6.66, -8936.6584);
    let m128_1 = from_f64(5.55, -8936.6584);
    
    assert_eq!(m128_0, m128_0);
    assert_eq!(m128_0, m128_0_2);
    assert_eq!(m128_0_2, m128_0);
    assert_ne!(m128_0, m128_1);
    assert_ne!(m128_1, m128_0);
}

#[test]
#[allow(overflowing_literals)]
fn test_to_m128d() {
    let md = from_f64(666.666, 5243.87876);
    let (f1, f2) = md.to_f64();
    
    assert_eq!(f1, 666.666);
    assert_eq!(f2, 5243.87876);
}

#[test]
fn test_m128d_add() {
    let m1 = zero_m128d();
    let m2 = from_f64(788.888, 54920.0);
    let m3 = from_f64(63.839, 65638.3748);
    
    let m_add1 = m1 + m2;
    let m_add2 = m2 + m3;
    assert_eq!(m_add1, m2);
    assert_eq!(m_add2, from_f64(788.888+63.839, 54920.0+65638.3748));
}