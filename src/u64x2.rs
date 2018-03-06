//taken from https://github.com/RustCrypto/block-ciphers and modified for Cryptonight
use std::ptr::copy_nonoverlapping;
use std::mem;
use std::ops::BitXor;
use super::cryptonight::sse;

#[allow(non_camel_case_types)]
#[repr(simd)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct u64x2(pub u64, pub u64);

impl u64x2 {
    /// Reads u64x2 from array pointer (potentially unaligned)
    #[inline(always)]
    pub fn read(src: &[u8]) -> Self {
        unsafe {
            let mut tmp: Self = mem::uninitialized();
            copy_nonoverlapping(
                src.as_ptr(),
                &mut tmp as *mut Self as *mut u8,
                16,
            );
            tmp
        }
    }

    /// Write u64x2 content into array pointer (potentially unaligned)
    #[inline(always)]
    pub fn write(self, dst: &mut [u8]) {
        unsafe {
            copy_nonoverlapping(
                &self as *const Self as *const u8,
                dst.as_mut_ptr(),
                16,
            );
        }
    }

    /// Read [u64x2; 8] from array pointer (potentially unaligned)
    #[inline(always)]
    pub fn read8(src: &[u8; 16*8]) -> [Self; 8] {
        unsafe {
            let mut tmp: [Self; 8] = mem::uninitialized();
            copy_nonoverlapping(
                src.as_ptr(),
                &mut tmp as *mut [Self; 8] as *mut u8,
                16*8,
            );
            tmp
        }
    }

    /// Write [u64x2; 8] content into array pointer (potentially unaligned)
    #[inline(always)]
    pub fn write8(src: [u64x2; 8], dst: &mut [u8; 16*8]) {
        unsafe {
            copy_nonoverlapping(
                &src as *const [Self; 8] as *const u8,
                dst.as_mut_ptr(),
                16*8,
            );
        }
    }

    pub fn to_u128(self) -> u128 {
        let mut r = u128::from(self.1);
        r <<= 64;
        r |= u128::from(self.0);
        r
    }
}

impl BitXor for u64x2 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        sse::_mm_xor_si128(self, rhs)
    }
}
