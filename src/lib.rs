#![crate_name = "mithril"]
#![crate_type = "lib"]

#![feature(asm)]
#![feature(repr_simd)]
#![feature(box_syntax)]
#![feature(integer_atomics)]
#![feature(mpsc_select)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod byte_string;
pub mod cryptonight;
pub mod stratum;
pub mod worker;
pub mod u64x2;
pub mod metric;
pub mod bandit_tools;
pub mod mithril_config;
pub mod timer;
