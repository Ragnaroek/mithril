#[cfg(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2"
    )
)]

use std::fmt;
use std::arch::x86_64::{_mm_set_epi32, __m128i, _mm_extract_epi64, _mm_aesdec_si128, _mm_aesenc_si128, _mm_cmpeq_epi32, _mm_movemask_epi8};

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128(pub __m128i);

pub fn zero_m128() -> m128 {
    from_i32(0, 0, 0, 0)
}

pub fn from_i32(i3: i32, i2: i32, i1: i32, i0: i32) -> m128 {
    unsafe {
        return m128(_mm_set_epi32(i3, i2, i1, i0));
    }
}

impl m128 {
    pub fn aesdec(&self, key: m128) -> m128 {
        unsafe {
            m128(_mm_aesdec_si128(self.0, key.0))
        }
    }
    
    pub fn aesenc(&self, key: m128) -> m128 {
        unsafe {
            m128(_mm_aesenc_si128(self.0, key.0))
        }
    }
    
    pub fn to_i64(&self) -> (i64, i64) {
        unsafe {
            let p1 = _mm_extract_epi64(self.0, 1);
            let p2 = _mm_extract_epi64(self.0, 0);
            (p1, p2)
        }
    }
}

impl PartialEq for m128 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let test = _mm_cmpeq_epi32(self.0, other.0); 
            return _mm_movemask_epi8(test) == 0xffff;
        }
    }
}
impl Eq for m128 {}

fn format(m: &m128, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let low;
    let high;
    unsafe {
        low = _mm_extract_epi64(m.0, 1);
        high = _mm_extract_epi64(m.0, 0);
    }
    f.write_str("(")?;
    fmt::LowerHex::fmt(&high, f)?;
    f.write_str(",")?;
    fmt::LowerHex::fmt(&low, f)?;
    f.write_str(")")
}

impl fmt::LowerHex for m128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format(self, f)
    }
}

impl fmt::Debug for m128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format(self, f)
    }
}