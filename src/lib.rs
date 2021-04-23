#![crate_name = "mithril"]
#![crate_type = "lib"]
#![feature(repr_simd)]
#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate strum;

pub mod bandit_tools;
pub mod byte_string;
pub mod metric;
pub mod mithril_config;
pub mod randomx;
pub mod stratum;
pub mod timer;
pub mod worker;
