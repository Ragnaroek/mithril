extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;

fn main() {
    let input = byte_string::string_to_u8_array("0505bb95bccd050cbe1128cdc929be74a00b706fa005e4436a5008c231fa88da5e203c9922b19d350000005a47548976a23998bacb1665c4db02f250cbba4b6e6eb950382ca85c515ddb8012");
    hash::hash(&input);
}
