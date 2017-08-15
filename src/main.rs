extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;

fn main() {
    let input = byte_string::string_to_u8_array("0505f8fcc9cc05b7844da6f865bb3f5db086fe87af9f0ddc21a20a06a9b6c76ce510bb795ed20425004000f8ad73db1c595d3a6e2c06368ee411c184d7be7b15a5e4da928c19a958303d7a01");

    hash::hash(&input);
}
