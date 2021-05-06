extern crate lazy_static;
extern crate mithril;

use lazy_static::lazy_static;
use mithril::randomx::memory::{init_dataset_item, SeedMemory};

lazy_static! {
    static ref TEST_SEED_MEM: SeedMemory = SeedMemory::new_initialised(b"test key 000");
}

#[test]
fn test_seed_memory_new_initialised() {
    assert_eq!(TEST_SEED_MEM.blocks[0][0], 0x191e0e1d23c02186);
    assert_eq!(TEST_SEED_MEM.blocks[12253][29], 0xf1b62fe6210bf8b1);
    assert_eq!(TEST_SEED_MEM.blocks[262143][127], 0x1f47f056d05cd99b);
}

#[test]
fn test_init_dataset_item() {
    let item = init_dataset_item(&TEST_SEED_MEM, 0);
    assert_eq!(item[0], 0x680588a85ae222db);

    let item = init_dataset_item(&TEST_SEED_MEM, 10000000);
    assert_eq!(item[0], 0x7943a1f6186ffb72);

    let item = init_dataset_item(&TEST_SEED_MEM, 20000000);
    assert_eq!(item[0], 0x9035244d718095e1);

    let item = init_dataset_item(&TEST_SEED_MEM, 30000000);
    assert_eq!(item[0], 0x145a5091f7853099);
}
