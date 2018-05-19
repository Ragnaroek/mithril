#![allow(unknown_lints)]


#[inline(always)]
#[allow(cast_ptr_alignment)]
pub fn as_u64_array(t: &mut [u8; 200]) -> &mut [u64; 25] {
    unsafe { &mut *(t as *mut [u8; 200] as *mut [u64; 25]) }
}

pub fn as_u8_array(t: &mut [u64; 25]) -> &mut [u8; 200] {
    unsafe { &mut *(t as *mut [u64; 25] as *mut [u8; 200]) }
}
