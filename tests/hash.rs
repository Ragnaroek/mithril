extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;
use mithril::cryptonight::keccak;

#[test]
fn test_aes_round_keys() {
    let input = byte_string::string_to_u8_array("05059fa5a8cc05b2df5d8fa271bb2d0304d4b1842f0f50844b735746db97ee5c196c647c3a5adc0c000000640680be903f504e896daebe42cdbe11e1a938d5c7fb2d64baa6356fe6fbacb704");
    let a = keccak::keccak(&input);
    let keys = hash::aes_round_keys(&a, 0);

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
    assert_eq!(byte_string::u64x2_to_string(scratchpad[0]), "f4e41f8bb21278bf69fef5414eedbd5d");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[1]), "d49d9e57821fa5220426015c6d9f218f");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[2]), "44c7e927a427b335d76fb01c18cb7629");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[3]), "99fcd81389062cc471260947e3ef3858");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[4]), "904dc9e321b05fe70537886ffeff76b4");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[5]), "e581057ea64bfc688f6262478adfda4d");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[6]), "1beb312a04e2418ff2d9f10376ca3142");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[7]), "1f88f1e57bb80c1717e1cdf74f9b5d31");

    assert_eq!(byte_string::u64x2_to_string(scratchpad[8]),  "851f0b8f4f30f744a8b2bcecdb468198");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[9]),  "c3c4d13cd0f1502ced9c63929e3e9588");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[10]), "3e8825dccc7726e25a937432d724b273");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[11]), "f891bc42841bf3dab14bd7b7fdf89a33");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[12]), "6e907a303bbc32fe47cd0cb080969894");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[13]), "f3a068a0fe82415a9e9e45b3eb6a76da");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[14]), "9262feadb97bf76a8dbcc0a32e395968");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[15]), "d722f17a07c4eaa5486fe39cc6d9609b");

    assert_eq!(byte_string::u64x2_to_string(scratchpad[16]), "5ac5dba2328b12c869e7cf919b347e80");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[17]), "3ad72813d8215a1e8b966a7c19258003");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[18]), "802219ba78b4259525b0cd8bd2112336");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[19]), "9e01d10e7cc1be2855b783b3a79884bf");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[20]), "c91bb830de5effea6cf238b15f54d4b5");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[21]), "9372f958c16b5591b1c44bf22b3d4d20");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[22]), "e4f26cf077bee0304fb11d6eaad48e82");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[23]), "953dea5d5b4fa058ab3f06ffc1ea0ec1");
}

#[test]
fn test_init_scratchpad_tail() {
    let input = byte_string::string_to_u8_array("0505a9e6c9cc0529b1608dbf9840e20164ee24efd67979e6a937ce174f9aff423a96a7cc5bdcd504008000ca5d84112bf941d3df2c44132b2df08fb766ebf0cc0ad4ccc4012b0929e4edeb04");
    let a = keccak::keccak(&input);

    let scratchpad = hash::init_scratchpad(&a);
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-16]), "c7a1f8660d2cf76f652e90e067f41e30");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-15]), "29f328053cb5ce9a3144fedcebeb0455");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-14]), "e3592994985e0937fc0b43c1a6ac738c");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-13]), "8d6844339f9196e249add1d2531907a9");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-12]), "a774d67ff9a5836f6f315822984e3e82");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-11]), "e1c5aaeb19b05eed5637d023056b8205");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-10]), "0501dea25f90b0049e92261354ecf772");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-9]),  "94c5166924405464f762963e09b8c55c");

    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-8]), "304fe9475ecec1065413f0a591b4b2ba");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-7]), "a70bd25d9d8011b68a8ff4282ba35eef");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-6]), "39ed29569a1736736f1eb608f73372bd");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-5]), "aaecaabb587e5027f48ac0832a157471");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-4]), "02935ff82a7c59380f69a1a9dfbf66e0");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-3]), "ae0d8323c4dbe1ec68f8ae668d447bcd");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-2]), "e2a0e238f8d5f1dd3dfefa5ac05445b0");
    assert_eq!(byte_string::u64x2_to_string(scratchpad[hash::MEM_SIZE-1]), "76696694f5e369e0c543e82f84559129");
}

#[test]
fn test_hash() {
    //0=Blake
    let mut input = byte_string::string_to_u8_array("546869732069732061207465737431");
    assert_eq!(hash::hash(&input), "236ebffb019ea9b19c9ff160775bc7a6ed090fce7103a32ee582d24a81db6960");

    //1=groestl
    input = byte_string::string_to_u8_array("5468697320697320612074657374");
    assert_eq!(hash::hash(&input), "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605");

    //2=JH (xmr-stak input "This is a test66")
    input = byte_string::string_to_u8_array("54686973206973206120746573743636");
    assert_eq!(hash::hash(&input), "21fb4137747541810d5f8ce821b6dacf68eb3051778e68b5bcd990c21fa08fd6");

    //3=Skein (xmr-stak input "This is a test6")
    //input = byte_string::string_to_u8_array("546869732069732061207465737436");
    //assert_eq!(hash::hash(&input), "bfabcc134608782e8f7322972dba801267f841535372741b554356045910f614");
}
