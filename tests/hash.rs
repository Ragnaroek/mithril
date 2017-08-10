extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;
use mithril::cryptonight::keccak;

#[test]
fn test_aes_round_keys() {
    let input = byte_string::string_to_u8_array("05059fa5a8cc05b2df5d8fa271bb2d0304d4b1842f0f50844b735746db97ee5c196c647c3a5adc0c000000640680be903f504e896daebe42cdbe11e1a938d5c7fb2d64baa6356fe6fbacb704");
    let a = keccak::keccak(&input);
    let keys = hash::aes_round_keys(&a);

    assert_eq!(byte_string::u64x2_to_string(keys[0]), "4c73438521575791be9c2c292f259ec4");
    assert_eq!(byte_string::u64x2_to_string(keys[1]), "2dba7e0233542a04a9e58cf7213e2f56");
    assert_eq!(byte_string::u64x2_to_string(keys[2]), "8b45520bc736118ee661461f58fd6a36");
    assert_eq!(byte_string::u64x2_to_string(keys[3]), "ab5bf78c86e1898eb5b5a38a1c502f7d");
    assert_eq!(byte_string::u64x2_to_string(keys[4]), "968d56c61dc804cddafe15433c9f535c");
    assert_eq!(byte_string::u64x2_to_string(keys[5]), "14024341bf59b4cd39b83d438c0d9ec9");
    assert_eq!(byte_string::u64x2_to_string(keys[6]), "eede630a785335cc659b3101bf652442");
    assert_eq!(byte_string::u64x2_to_string(keys[7]), "36f3af6122f1ec209da858eda41065ae");
    assert_eq!(byte_string::u64x2_to_string(keys[8]), "a3764ef44da82dfe35fb183250602933");
    assert_eq!(byte_string::u64x2_to_string(keys[9]), "278251bd1171fedc338012fcae284a11");
}
