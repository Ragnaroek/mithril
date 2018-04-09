#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate mithril;

use mithril::worker::worker_pool;
use mithril::cryptonight::hash;

#[test]
fn test_num_bits() {
    assert_eq!(worker_pool::num_bits(0), 0);
    assert_eq!(worker_pool::num_bits(1), 1);
    assert_eq!(worker_pool::num_bits(2), 1);
    assert_eq!(worker_pool::num_bits(3), 2);
    assert_eq!(worker_pool::num_bits(7), 3);
    assert_eq!(worker_pool::num_bits(8), 3);
    assert_eq!(worker_pool::num_bits(9), 4);
    assert_eq!(worker_pool::num_bits(16), 4);
}

#[test]
fn test_target_u64() {
    assert_eq!(worker_pool::target_u64(171798), 737869762948382);
}

#[test]
fn test_with_nonce() {
    let blob = "0606cbe692d005ecfebc7d2249d2b43535c237c02359e888b8b05d2e980c1405779241ac3ab48500000000e62a06e71559c98a37e7b6743465f4f72e42784c5719411c935dc002e347826b05";
    let nonce = "12345678";
    assert_eq!("0606cbe692d005ecfebc7d2249d2b43535c237c02359e888b8b05d2e980c1405779241ac3ab48512345678e62a06e71559c98a37e7b6743465f4f72e42784c5719411c935dc002e347826b05",
               worker_pool::with_nonce(blob, nonce));
}

#[test]
fn test_hash_version_pre_7() {
    let blob = "0606cbe692d005ecfeb";
    assert_eq!(hash::HashVersion::Version6, worker_pool::hash_version(blob));
}

#[test]
fn test_hash_version_7() {
    let blob = "07079db3f7d50511";
    assert_eq!(hash::HashVersion::Version7, worker_pool::hash_version(blob));
}
