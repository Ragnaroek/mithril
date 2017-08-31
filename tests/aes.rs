extern crate mithril;

use mithril::cryptonight::aes;
use mithril::u64x2::u64x2;

#[test]
fn test_aes_round() {
    let result = aes::aes_round(u64x2(0xd822831cb9e31fcf,0x8818ec8e51cbeb2e),
                                u64x2(0x0d137d1656ef4daf,0x4653c1691bbe16e8));
    assert_eq!(result, u64x2(0x5add92eeb3e0e154,0x87df3c4255c0f2b3));

    let result2 = aes::aes_round(u64x2(0x5add92eeb3e0e154, 0x87df3c4255c0f2b3),
                                 u64x2(0xdc00341a8dde2c34, 0x52bd7edcacd25e2b));
    assert_eq!(result2, u64x2(0x6931c13936e75008, 0xfcef1daa76547888));
}
