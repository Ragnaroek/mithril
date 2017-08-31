extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;

fn main() {
    let input = byte_string::string_to_u8_array("0505ead6b8cd05609fe563801f8d2689108eef9fb731308d2c718f5c4fceb3d9cb4558216b7bac2d004000a30b91824b2f964a613bddfefa99d000116b4978953d354a4a831f59e8fe124903");

    hash::hash(&input);
}
