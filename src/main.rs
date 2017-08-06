extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;

fn main() {
    let input = byte_string::string_to_u8_array("05059fa5a8cc05b2df5d8fa271bb2d0304d4b1842f0f50844b735746db97ee5c196c647c3a5adc0c000000640680be903f504e896daebe42cdbe11e1a938d5c7fb2d64baa6356fe6fbacb704");

    hash::hash(&input);
}
