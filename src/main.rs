extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;

fn main() {
    //let input = byte_string::string_to_u8_array("5468697320697320612074657374");
    let input = byte_string::string_to_u8_array("54686973206973206120746573743636");
    let result = hash::hash(&input);
    println!("result={}", result);
}
