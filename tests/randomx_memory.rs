extern crate mithril;

use mithril::randomx::memory::{SeedMemory};

#[test]
fn test_seed_memory_new_initialised() {
    let seed_mem = SeedMemory::new_initialised(b"test key 000");
    assert_eq!(seed_mem.blocks[0][0], 0x191e0e1d23c02186);
    assert_eq!(seed_mem.blocks[12253][29], 0xf1b62fe6210bf8b1);
    assert_eq!(seed_mem.blocks[262143][127], 0x1f47f056d05cd99b);
}