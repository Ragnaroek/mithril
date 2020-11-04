#[cfg(
    all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2"
    )
)]

use std::fmt;
use std::arch::x86_64::{_mm_set_epi32, __m128i, __m128d, _mm_extract_epi64, _mm_aesdec_si128, 
    _mm_aesenc_si128, _mm_cmpeq_epi32, _mm_movemask_epi8, _mm_cvtepi32_pd, _mm_storeh_pd, 
    _mm_store_sd, _mm_set_pd, _mm_cmpeq_pd, _mm_movemask_pd, _mm_add_pd, _mm_set_epi64x};

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128(pub __m128i);//TODO rename to m128i

//TODO move to m128 for scope
pub fn zero_m128() -> m128 {
    from_i32(0, 0, 0, 0)
}

//TODO move to m128 for scope
pub fn from_i32(i3: i32, i2: i32, i1: i32, i0: i32) -> m128 {
    unsafe {
        return m128(_mm_set_epi32(i3, i2, i1, i0));
    }
}

//TODO move to m128 for scope
pub fn from_u64(u1: u64, u0: u64) -> m128 {
    unsafe {
        return m128(_mm_set_epi64x(u1 as i64, u0 as i64));
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
    
    pub fn to_m128d(&self) -> m128d {
        unsafe {
            m128d(_mm_cvtepi32_pd(self.0))
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

fn format_m128i(m: &m128, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (low, high) = m.to_i64();
    f.write_str("(")?;
    fmt::LowerHex::fmt(&high, f)?;
    f.write_str(",")?;
    fmt::LowerHex::fmt(&low, f)?;
    f.write_str(")")
}

impl fmt::LowerHex for m128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128i(self, f)
    }
}

impl fmt::Debug for m128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128i(self, f)
    }
}

//==== m128d

#[allow(nonstandard_style)]
#[derive(Copy, Clone)]
pub struct m128d(pub __m128d);

//TODO move to m128d for scope
pub fn from_f64(h: f64, l: f64) -> m128d {
    unsafe{
        return m128d(_mm_set_pd(h, l));
    }
}

//TODO move to m128d for scope
pub fn zero_m128d() -> m128d {
    return from_f64(0.0, 0.0);
}

impl m128d {
    
    pub fn from_u64(h: u64, l: u64) -> m128d {
        return from_f64(f64::from_bits(h), f64::from_bits(l));
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

fn format_m128d(m: &m128d, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (low, high) = m.to_f64();
    f.write_fmt(format_args!("({},{})", low, high))
}

impl fmt::LowerHex for m128d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (low, high) = self.to_f64();
        f.write_fmt(format_args!("({:x},{:x})", low.to_bits(), high.to_bits()))
    }
}

impl fmt::Debug for m128d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_m128d(self, f)
    }
}
