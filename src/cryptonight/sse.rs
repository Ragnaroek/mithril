
use u64x2::u64x2;

macro_rules! mm_shuffle_epi32 {
    ($key:expr, $ib:expr, $result:ident) => {
        asm!(concat!("pshufd xmm1, xmm2, ", $ib)
            : "={xmm1}"($result)
            : "{xmm2}"($key)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0x55(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0x55, r)
    }
    return r;
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0xff(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0xFF, r)
    }
    return r;
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0xaa(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0xAA, r)
    }
    return r;
}

macro_rules! mm_cvtsi128_si32 {
    ($v:expr, $result:ident) => {
        asm!("movd eax, xmm1"
            : "={eax}"($result)
            : "{xmm1}"($v)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_cvtsi128_si32(v: u64x2) -> u32 {
    let r;
    unsafe {
        mm_cvtsi128_si32!(v, r)
    }
    return r;
}

macro_rules! mm_slli_si128 {
    ($v:expr, $ib:expr, $result:ident) => {
        asm!(concat!("pslldq xmm1, ", $ib)
            : "={xmm1}"($result)
            : "{xmm1}"($v)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_slli_si128_0x04(v: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_slli_si128!(v, 0x04, r)
    }
    return r;
}
