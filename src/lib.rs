#![crate_name = "mithril"]
#![crate_type = "lib"]

#![feature(i128_type)]
#![feature(asm)]
#![feature(repr_simd)]
#![feature(box_syntax)]
#![feature(iterator_step_by)]

#[macro_use]
extern crate serde_derive;

pub mod byte_string;
pub mod cryptonight;
pub mod stratum;
pub mod worker;
pub mod u64x2;
