
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

macro_rules! mm_shuffle_epi32 {
    ($key:expr, $ib:expr, $result:ident) => {
        asm!(concat!("ps xmm1, xmm2, ", $ib)
            : "={xmm1}"($result)
            : "{xmm2}"($key)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

pub fn _mm_cvtsi128_si32(v: u64x2) -> u32 {
    //TODO implement this native in Rust???
    return 0;
}
