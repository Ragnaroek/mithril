extern crate argon2;

use self::argon2::block::{Block};

const RANDOMX_ARGON_LANES : u32 = 1;
const RANDOMX_ARGON_MEMORY : u32 = 262144;
const RANDOMX_ARGON_SALT : &[u8;8] = b"RandomX\x03";
const RANDOMX_ARGON_ITERATIONS : u32 = 3;

const ARGON2_SYNC_POINTS : u32 = 4;

//256MiB, always used
pub struct SeedMemory {
    pub blocks : Box<[Block]>,
}

impl SeedMemory {

    pub fn no_memory() -> SeedMemory {
        SeedMemory{blocks: Box::new([])}
    }

    /// Creates a new initialised seed memory.
    pub fn new_initialised(key: &[u8]) -> SeedMemory {
        let mut mem = argon2::memory::Memory::new(RANDOMX_ARGON_LANES, RANDOMX_ARGON_MEMORY);
        let context = &create_argon_context(key);
        argon2::core::initialize(context, &mut mem);
        argon2::core::fill_memory_blocks(context, &mut mem);
        SeedMemory{blocks: mem.blocks}
    }
}

fn create_argon_context(key: &[u8]) -> argon2::context::Context {
    let segment_length = RANDOMX_ARGON_MEMORY / (RANDOMX_ARGON_LANES * ARGON2_SYNC_POINTS);
    let config = argon2::config::Config {
        ad: &[],
        hash_length: 0,
        lanes: RANDOMX_ARGON_LANES,
        mem_cost: RANDOMX_ARGON_MEMORY,
        secret: &[],
        thread_mode: argon2::ThreadMode::from_threads(1),
        time_cost: RANDOMX_ARGON_ITERATIONS,
        variant: argon2::Variant::Argon2d,
        version: argon2::Version::Version13,
    };
    argon2::context::Context{
        config,
        memory_blocks: RANDOMX_ARGON_MEMORY,
        pwd: key,
        salt: RANDOMX_ARGON_SALT,
        lane_length: segment_length * ARGON2_SYNC_POINTS,
        segment_length,
    }
}

//2GiB, only filled in mining mode. Caches DataSetItems.
pub struct DatasetMemory {

}

impl DatasetMemory {
    pub fn new_initialised() -> DatasetMemory {
        DatasetMemory{}
    }
}

pub struct VmMemory {
    pub seed_memory: SeedMemory,
    pub dataset_memory: Option<DatasetMemory>,
}

impl VmMemory {

    //only useful for testing
    pub fn no_memory() -> VmMemory {
        VmMemory{seed_memory: SeedMemory::no_memory(), dataset_memory: None}
    }

    pub fn light(key: &[u8]) -> VmMemory {
        VmMemory{seed_memory: SeedMemory::new_initialised(key), dataset_memory: None}
    }
    pub fn full(key: &[u8]) -> VmMemory {
        VmMemory{seed_memory: SeedMemory::new_initialised(key), dataset_memory: Some(DatasetMemory::new_initialised())}
    }
}