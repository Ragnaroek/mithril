#![allow(unknown_lints)]
#![allow(clippy::unreadable_literal)]

extern crate mithril;

use mithril::worker::worker_pool;

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
fn test_with_nonce() {
    let blob = "0606cbe692d005ecfebc7d2249d2b43535c237c02359e888b8b05d2e980c1405779241ac3ab48500000000e62a06e71559c98a37e7b6743465f4f72e42784c5719411c935dc002e347826b05";
    let nonce = "12345678";
    assert_eq!(worker_pool::with_nonce(blob, nonce),
        "0606cbe692d005ecfebc7d2249d2b43535c237c02359e888b8b05d2e980c1405779241ac3ab48512345678e62a06e71559c98a37e7b6743465f4f72e42784c5719411c935dc002e347826b05");
}

#[test]
fn test_hash_target_value() {
    assert_eq!(worker_pool::hash_target_value("c5c49db95a9da3f0802a34c6f97c364e7455fca7e41f72254fd4624dd2f91578"), 0x7815f9d24d62d44f);
}

#[test]
fn test_job_target_value() {
    assert_eq!(worker_pool::job_target_value("8b4f0100"), 368934881474191);
}
