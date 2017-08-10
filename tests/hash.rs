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

#[test]
fn test_init_scratchpad() {
    let input = byte_string::string_to_u8_array("0505988ab3cc05c725e9fe211fb23e9ccd442829a684d9a887d097ec33dbfd6085e70068ee779714000000cd484698d1fa1981993198f995e2c4fea6f31b6b3f8fbcf742b32ce2d5951cdd07");
    let a = keccak::keccak(&input);

    let scratchpad = hash::init_scratchpad(&a);
    assert_eq!(byte_string::u128_to_string(scratchpad[0]), "f4e41f8bb21278bf69fef5414eedbd5d");
    assert_eq!(byte_string::u128_to_string(scratchpad[1]), "d49d9e57821fa5220426015c6d9f218f");
    assert_eq!(byte_string::u128_to_string(scratchpad[2]), "44c7e927a427b335d76fb01c18cb7629");
    assert_eq!(byte_string::u128_to_string(scratchpad[3]), "99fcd81389062cc471260947e3ef3858");
    assert_eq!(byte_string::u128_to_string(scratchpad[4]), "904dc9e321b05fe70537886ffeff76b4");
    assert_eq!(byte_string::u128_to_string(scratchpad[5]), "e581057ea64bfc688f6262478adfda4d");
    assert_eq!(byte_string::u128_to_string(scratchpad[6]), "1beb312a04e2418ff2d9f10376ca3142");
    assert_eq!(byte_string::u128_to_string(scratchpad[7]), "1f88f1e57bb80c1717e1cdf74f9b5d31");

    assert_eq!(byte_string::u128_to_string(scratchpad[8]),  "851f0b8f4f30f744a8b2bcecdb468198");
    assert_eq!(byte_string::u128_to_string(scratchpad[9]),  "c3c4d13cd0f1502ced9c63929e3e9588");
    assert_eq!(byte_string::u128_to_string(scratchpad[10]), "3e8825dccc7726e25a937432d724b273");
    assert_eq!(byte_string::u128_to_string(scratchpad[11]), "f891bc42841bf3dab14bd7b7fdf89a33");
    assert_eq!(byte_string::u128_to_string(scratchpad[12]), "6e907a303bbc32fe47cd0cb080969894");

    //TODO Rest of data:
/*
    f3a068a0fe82415a9e9e45b3eb6a76da
    9262feadb97bf76a8dbcc0a32e395968
    d722f17a07c4eaa5486fe39cc6d9609b
    5ac5dba2328b12c869e7cf919b347e80
    3ad72813d8215a1e8b966a7c19258003
    802219ba78b4259525b0cd8bd2112336
    9e01d10e7cc1be2855b783b3a79884bf
    c91bb830de5effea6cf238b15f54d4b5
    9372f958c16b5591b1c44bf22b3d4d20
    e4f26cf077bee0304fb11d6eaad48e82
    953dea5d5b4fa058ab3f06ffc1ea0ec1
    */
}
