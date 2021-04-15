extern crate mithril;

use mithril::randomx::m128::{m128i, m128d};

#[test]
#[allow(overflowing_literals)]
fn test_m128i_from_u8() {
    let m32 = m128i::from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);

    let bytes : [u8; 16] = [0xf3, 0xe4, 0xf7, 0xd6, 0x16, 0xf6, 0x70, 0xb3, 0x14, 0x29, 0x7a, 0xbb, 0x76, 0x38, 0x90, 0x31];
    let m8 = m128i::from_u8(&bytes);

    assert_eq!(m32, m8);
}

#[test]
#[allow(overflowing_literals)]
fn test_m128i_eq() {
    let m128_0 = m128i::from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let m128_0_2 = m128i::from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let m128_1 = m128i::from_i32(0xb5a8ef6, 0x7749809c8, 0xf349884, 0xa05c9f5ef);
    
    assert_eq!(m128_0, m128_0);
    assert_eq!(m128_0, m128_0_2);
    assert_eq!(m128_0_2, m128_0);
    assert_ne!(m128_0, m128_1);
    assert_ne!(m128_1, m128_0);
}

#[test]
#[allow(overflowing_literals)]
fn test_m128i_to_i64() {
    let m = m128i::from_i32(0x31903876, 0xbb7a2914, 0xb370f616, 0xd6f7e4f3);
    let (p1, p2) = m.as_i64();
    
    assert_eq!(p1, 0x31903876bb7a2914);
    assert_eq!(p2, 0xb370f616d6f7e4f3);
}

#[test]
#[allow(overflowing_literals)]
fn test_m128d_eq() {
    let m128_0 = m128d::from_f64(6.66, -8936.6584);
    let m128_0_2 = m128d::from_f64(6.66, -8936.6584);
    let m128_1 = m128d::from_f64(5.55, -8936.6584);
    
    assert_eq!(m128_0, m128_0);
    assert_eq!(m128_0, m128_0_2);
    assert_eq!(m128_0_2, m128_0);
    assert_ne!(m128_0, m128_1);
    assert_ne!(m128_1, m128_0);
}

#[test]
#[allow(overflowing_literals)]
fn test_to_m128d() {
    let md = m128d::from_f64(666.666, 5243.87876);
    let (f1, f2) = md.as_f64();
    
    assert_eq!(f1, 666.666);
    assert_eq!(f2, 5243.87876);
}

#[test]
fn test_m128d_add() {
    let m1 = m128d::zero();
    let m2 = m128d::from_f64(788.888, 54920.0);
    let m3 = m128d::from_f64(63.839, 65638.3748);
    
    let m_add1 = m1 + m2;
    let m_add2 = m2 + m3;
    assert_eq!(m_add1, m2);
    assert_eq!(m_add2, m128d::from_f64(788.888+63.839, 54920.0+65638.3748));
}

#[test]
fn test_m128d_sub() {
    let m1 = m128d::zero();
    let m2 = m128d::from_f64(788.888, 54920.0);
    let m3 = m128d::from_f64(63.839, 65638.3748);
    
    let m_add1 = m1 - m2;
    let m_add2 = m2 - m3;
    assert_eq!(m_add1, m128d::from_f64(-788.888, -54920.0));
    assert_eq!(m_add2, m128d::from_f64(788.888-63.839, 54920.0-65638.3748));
}

#[test]
fn test_m128d_div() {
    let m1 = m128d::from_f64(788.888, 54920.0);
    let m2 = m128d::from_f64(63.839, 65638.3748);
    
    let m_dived = m1 / m2;
    assert_eq!(m_dived, m128d::from_f64(788.888/63.839, 54920.0/65638.3748));
}

#[test]
fn test_m128d_and() {
    let m1 = m128d::from_u64(0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF);
    let m2 = m128d::from_u64(0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA);
    
    let m_anded = m1 & m2;
    assert_eq!(m_anded, m128d::from_u64(0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA));
}

#[test]
fn test_m128d_or() {
    let m1 = m128d::from_u64(0x0, 0x0);
    let m2 = m128d::from_u64(0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA);
    
    let m_anded = m1 | m2;
    assert_eq!(m_anded, m128d::from_u64(0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA));
}

#[test]
fn test_m128d_shuffle_1() {
    let m1 = m128d::from_f64(788.888, 54920.0);
    let m2 = m128d::from_f64(63.839, 65638.3748);

    let m_shuffled = m1.shuffle_1(&m2);
    assert_eq!(m_shuffled, m128d::from_f64(65638.3748, 788.888)); 
}

#[test]
fn test_m128d_xor() {
    let m1 = m128d::from_u64(0x0555555555555555, 0x0555555555555555);
    let m2 = m128d::from_u64(0x0AAAAAAAAAAAAAAA, 0x0AAAAAAAAAAAAAAA);
    
    let m_xored = m1 ^ m2;
    
    assert_eq!(m_xored, m128d::from_u64(0x0FFFFFFFFFFFFFFF, 0x0FFFFFFFFFFFFFFF));

    let nulled = m1 ^ m1;
    assert_eq!(nulled, m128d::from_u64(0x0, 0x0));    
}

#[test]
fn test_m128d_mul() {
    let m1 = m128d::from_u64(0x41dbc35cef248783, 0x40fdfdabb6173d07);
    let m2 = m128d::from_u64(0x40eba861aa31c7c0, 0x41c4561212ae2d50); 

    let m_muled = m1 * m2;

    assert_eq!(m_muled, m128d::from_u64(0x42d7feeccd89152f, 0x42d30f35ff7a6969));
}

#[test]
fn test_m128_sqrt() {
    let m = m128d::from_u64(0x41b6b21c11affea7, 0x40526a7e778d9824);

    let m_sqrted = m.sqrt();

    assert_eq!(m_sqrted, m128d::from_u64(0x40d30e573fa3ba8d, 0x40212a610b301fe8));
}