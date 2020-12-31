#[cfg(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2"
    )
)]

use std::fmt;
use std::convert::TryInto;
use std::arch::x86_64::{_mm_set_epi32, __m128i, __m128d, _mm_extract_epi64, _mm_aesdec_si128, 
    _mm_aesenc_si128, _mm_cmpeq_epi32, _mm_movemask_epi8, _mm_cvtepi32_pd, _mm_storeh_pd, 
    _mm_store_sd, _mm_set_pd, _mm_cmpeq_pd, _mm_movemask_pd, _mm_add_pd, _mm_set_epi64x, _mm_shuffle_pd,
    _mm_xor_pd, _mm_mul_pd, _mm_sqrt_pd, _mm_sub_pd, _mm_div_pd, _mm_and_pd, _mm_or_pd};

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128i(pub __m128i);

impl m128i {
    
    pub fn zero() -> m128i {
        m128i::from_i32(0, 0, 0, 0)
    }
   
    pub fn from_u8(bytes: &[u8]) -> m128i {
        debug_assert_eq!(bytes.len(), 16);

        let u0 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let u1 = u64::from_le_bytes(bytes[8..16].try_into().unwrap());

        return m128i::from_u64(u1, u0);
    }
    
    pub fn from_i32(i3: i32, i2: i32, i1: i32, i0: i32) -> m128i {
        unsafe {
            return m128i(_mm_set_epi32(i3, i2, i1, i0));
        }
    }
    
    pub fn from_u64(u1: u64, u0: u64) -> m128i {
        unsafe {
            return m128i(_mm_set_epi64x(u1 as i64, u0 as i64));
        }
    }
    
    pub fn aesdec(&self, key: m128i) -> m128i {
        unsafe {
            m128i(_mm_aesdec_si128(self.0, key.0))
        }
    }
    
    pub fn aesenc(&self, key: m128i) -> m128i {
        unsafe {
            m128i(_mm_aesenc_si128(self.0, key.0))
        }
    }
    
    pub fn to_i64(&self) -> (i64, i64) {
        unsafe {
            let p1 = _mm_extract_epi64(self.0, 1);
            let p2 = _mm_extract_epi64(self.0, 0);
            (p1, p2)
        }
    }

    pub fn to_m128d(&self) -> m128d {
        unsafe {
            m128d(_mm_cvtepi32_pd(self.0))
        }
    }
}

impl PartialEq for m128i {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let test = _mm_cmpeq_epi32(self.0, other.0); 
            return _mm_movemask_epi8(test) == 0xffff;
        }
    }
}

impl Eq for m128i {}

fn format_m128i(m: &m128i, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (low, high) = m.to_i64();
    f.write_fmt(format_args!("({:x},{:x})", high, low))
}

impl fmt::LowerHex for m128i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128i(self, f)
    }
}

impl fmt::Debug for m128i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128i(self, f)
    }
}

//==== m128d

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128d(pub __m128d);

impl m128d {
    
    pub fn zero() -> m128d {
        return m128d::from_f64(0.0, 0.0);
    }
    
    pub fn from_u64(h: u64, l: u64) -> m128d {
        return m128d::from_f64(f64::from_bits(h), f64::from_bits(l));
    }
    
    pub fn from_f64(h: f64, l: f64) -> m128d {
        unsafe{
            return m128d(_mm_set_pd(h, l));
        }
    }
    
    pub fn to_f64(&self) -> (f64, f64) {
        let mut f1 : f64 = 0.0;
        let mut f2 : f64 = 0.0;
        let f1_ptr : *mut f64 = &mut f1;
        let f2_ptr : *mut f64 = &mut f2;
        unsafe {
            _mm_storeh_pd(f1_ptr, self.0);
            _mm_store_sd(f2_ptr, self.0);
        }
        return (f1, f2);
    }

    pub fn to_u64(&self) -> (u64, u64) {
        let (f1, f0) = self.to_f64();
        (f1.to_bits(), f0.to_bits())
    }

    //_mm_shuffle_pd(a, b, 1)
    pub fn shuffle_1(&self, other: &m128d) -> m128d {
        unsafe {
            return m128d(_mm_shuffle_pd(self.0, other.0, 1));
        }
    }

    //_mm_sqrt_pd
    pub fn sqrt(&self) -> m128d {
        unsafe {
            return m128d(_mm_sqrt_pd(self.0));
        }
    }
}

impl PartialEq for m128d {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let test = _mm_cmpeq_pd(self.0, other.0);
            let mask = _mm_movemask_pd(test);
            return mask == 0b11;
        }
    }
}
impl Eq for m128d {}

impl std::ops::Add for m128d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        unsafe {
            return m128d(_mm_add_pd(self.0, other.0));
        }
    }
}

impl std::ops::Sub for m128d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        unsafe {
            return m128d(_mm_sub_pd(self.0, other.0));
        }
    }
}

fn format_m128d(m: &m128d, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (low, high) = m.to_f64();
    f.write_fmt(format_args!("({},{})", low, high))
}

impl fmt::LowerHex for m128d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (low, high) = self.to_f64();
        f.write_fmt(format_args!("({:x},{:x})", high.to_bits(), low.to_bits()))
    }
}

impl fmt::Debug for m128d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128d(self, f)
    }
}

impl std::ops::BitXor for m128d {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe {
            return m128d(_mm_xor_pd(self.0, rhs.0));
        }
    }
}

impl std::ops::BitAnd for m128d {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe {
            return m128d(_mm_and_pd(self.0, rhs.0));
        }
    } 
}

impl std::ops::BitOr for m128d {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe {
            return m128d(_mm_or_pd(self.0, rhs.0));
        }
    } 
}

impl std::ops::Mul for m128d {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe {
            return m128d(_mm_mul_pd(self.0, rhs.0));
        }
    }
}

impl std::ops::Div for m128d {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unsafe {
            return m128d(_mm_div_pd(self.0, rhs.0));
        }
    } 
}
