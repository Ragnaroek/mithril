#![allow(unknown_lints)]
#![allow(inline_always)]

use u64x2::u64x2;

macro_rules! mm_shuffle_epi32 {
    ($key:expr, $ib:expr, $result:ident) => {
        llvm_asm!(concat!("pshufd xmm1, xmm2, ", $ib)
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
    r
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0xff(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0xFF, r)
    }
    r
}

#[inline(always)]
pub fn _mm_shuffle_epi32_0xaa(key: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_shuffle_epi32!(key, 0xAA, r)
    }
    r
}

macro_rules! mm_cvtsi128_si32 {
    ($v:expr, $result:ident) => {
        llvm_asm!("movd eax, xmm1"
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
    r
}

macro_rules! _mm_srli_si128 {
    ($v:expr, $ib:expr, $result:ident) => {
        llvm_asm!(concat!("psrldq xmm1, ", $ib)
            : "={xmm1}"($result)
            : "{xmm1}"($v)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_srli_si128_0x08(v: u64x2) -> u64x2 {
    let r;
    unsafe {
        _mm_srli_si128!(v, 0x08, r)
    }
    r
}

macro_rules! mm_slli_si128 {
    ($v:expr, $ib:expr, $result:ident) => {
        llvm_asm!(concat!("pslldq xmm1, ", $ib)
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
    r
}

macro_rules! mm_xor_si128 {
    ($v0:expr, $v1:expr, $result:ident) => {
        llvm_asm!("pxor xmm1, xmm2"
            : "={xmm1}"($result)
            : "{xmm1}"($v0), "{xmm2}"($v1)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_xor_si128(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_xor_si128!(v0, v1, r)
    }
    r
}

macro_rules! mm_mul_su32 {
    ($v0:expr, $v1:expr, $result:ident) => {
        llvm_asm!("PMULUDQ xmm1, xmm2"
            : "={xmm1}"($result)
            : "{xmm1}"($v0), "{xmm2}"($v1)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_mul_su32(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_mul_su32!(v0, v1, r)
    }
    r
}

macro_rules! mm_add_epi64 {
    ($v0:expr, $v1:expr, $result:ident) => {
        llvm_asm!("PADDQ xmm1, xmm2"
            : "={xmm1}"($result)
            : "{xmm1}"($v0), "{xmm2}"($v1)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_add_epi64(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_add_epi64!(v0, v1, r)
    }
    r
}

//_mm_sqrt_sd

macro_rules! mm_sqrt_sd {
    ($v0:expr, $v1:expr, $result:ident) => {
        llvm_asm!("SQRTSD xmm1, xmm2"
            : "={xmm1}"($result)
            : "{xmm1}"($v0), "{xmm2}"($v1)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_sqrt_sd(v0: u64x2, v1: u64x2) -> u64x2 {
    let r;
    unsafe {
        mm_sqrt_sd!(v0, v1, r)
    }
    r
}

//_mm_cvtsi128_si64

macro_rules! mm_cvtsi128_si64 {
    ($v:expr, $result:ident) => {
        llvm_asm!("movq rax, xmm1"
            : "={rax}"($result)
            : "{xmm1}"($v)
            :
            : "intel", "alignstack", "volatile"
        );
    }
}

#[inline(always)]
pub fn _mm_cvtsi128_si64(v: u64x2) -> u64 {
    let r;
    unsafe {
        mm_cvtsi128_si64!(v, r)
    }
    r
}
