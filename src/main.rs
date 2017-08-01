extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::keccak;

fn main() {
    let input = byte_string::string_to_u8_array("0505fbf6ffcb050b68956935c6c2902af098f48b969d6e3577647e80c556d90ab2415c2996bb1625004000676466b9986865ae42affe0bf4b86a43129156457c76bd1968d087cc8a1bd46606".to_string());
    let a = keccak::keccak(&input);
    println!("len: {:?}", a.len());
}

//source: http://blog.nella.org/my-first-ever-rust-program/
