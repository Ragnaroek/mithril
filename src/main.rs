extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;

fn main() {
    let input = byte_string::string_to_u8_array("5468697320697320612074657374");
    hash::hash(&input);
}
