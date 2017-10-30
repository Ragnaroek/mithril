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
