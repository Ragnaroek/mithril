extern crate argon2;

use self::argon2::block::Block;
use super::superscalar::{Blake2Generator, ScProgram};
use std::sync::RwLock;

const RANDOMX_ARGON_LANES: u32 = 1;
const RANDOMX_ARGON_MEMORY: u32 = 262144;
const RANDOMX_ARGON_SALT: &[u8; 8] = b"RandomX\x03";
const RANDOMX_ARGON_ITERATIONS: u32 = 3;
const RANDOMX_CACHE_ACCESSES: usize = 8;

const ARGON2_SYNC_POINTS: u32 = 4;
const ARGON_BLOCK_SIZE: u32 = 1024;

pub const CACHE_LINE_SIZE: u64 = 64;
pub const DATASET_ITEM_COUNT: usize = (2147483648 + 33554368) / 64; //34.078.719

const SUPERSCALAR_MUL_0: u64 = 6364136223846793005;
const SUPERSCALAR_ADD_1: u64 = 9298411001130361340;
const SUPERSCALAR_ADD_2: u64 = 12065312585734608966;
const SUPERSCALAR_ADD_3: u64 = 9306329213124626780;
const SUPERSCALAR_ADD_4: u64 = 5281919268842080866;
const SUPERSCALAR_ADD_5: u64 = 10536153434571861004;
const SUPERSCALAR_ADD_6: u64 = 3398623926847679864;
const SUPERSCALAR_ADD_7: u64 = 9549104520008361294;

//256MiB, always used, named randomx_cache in the reference implementation
pub struct SeedMemory {
    pub blocks: Box<[Block]>,
    pub programs: Vec<ScProgram<'static>>,
}

impl SeedMemory {
    pub fn no_memory() -> SeedMemory {
        SeedMemory {
            blocks: Box::new([]),
            programs: Vec::with_capacity(0),
        }
    }

    /// Creates a new initialised seed memory.
    pub fn new_initialised(key: &[u8]) -> SeedMemory {
        let mut mem = argon2::memory::Memory::new(RANDOMX_ARGON_LANES, RANDOMX_ARGON_MEMORY);
        let context = &create_argon_context(key);
        argon2::core::initialize(context, &mut mem);
        argon2::core::fill_memory_blocks(context, &mut mem);

        let mut programs = Vec::with_capacity(RANDOMX_CACHE_ACCESSES);
        let mut gen = Blake2Generator::new(key, 0);
        for _ in 0..RANDOMX_CACHE_ACCESSES {
            programs.push(ScProgram::generate(&mut gen));
        }

        SeedMemory {
            blocks: mem.blocks,
            programs,
        }
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
    argon2::context::Context {
        config,
        memory_blocks: RANDOMX_ARGON_MEMORY,
        pwd: key,
        salt: RANDOMX_ARGON_SALT,
        lane_length: segment_length * ARGON2_SYNC_POINTS,
        segment_length,
    }
}

fn mix_block_value(seed_mem: &SeedMemory, reg_value: u64, r: usize) -> u64 {
    let mask = (((RANDOMX_ARGON_MEMORY * ARGON_BLOCK_SIZE) as u64) / CACHE_LINE_SIZE) - 1;
    let byte_offset = ((reg_value & mask) * CACHE_LINE_SIZE) + (8 * r as u64);
    let block_ix = byte_offset / ARGON_BLOCK_SIZE as u64;
    let block_v_ix = (byte_offset - (block_ix * ARGON_BLOCK_SIZE as u64)) / 8;
    seed_mem.blocks[block_ix as usize][block_v_ix as usize]
}

pub fn init_dataset_item(seed_mem: &SeedMemory, item_num: u64) -> [u64; 8] {
    let mut ds = [0; 8];

    let mut reg_value = item_num;
    ds[0] = (item_num + 1).wrapping_mul(SUPERSCALAR_MUL_0);
    ds[1] = ds[0] ^ SUPERSCALAR_ADD_1;
    ds[2] = ds[0] ^ SUPERSCALAR_ADD_2;
    ds[3] = ds[0] ^ SUPERSCALAR_ADD_3;
    ds[4] = ds[0] ^ SUPERSCALAR_ADD_4;
    ds[5] = ds[0] ^ SUPERSCALAR_ADD_5;
    ds[6] = ds[0] ^ SUPERSCALAR_ADD_6;
    ds[7] = ds[0] ^ SUPERSCALAR_ADD_7;

    for prog in &seed_mem.programs {
        prog.execute(&mut ds);

        for (r, v) in ds.iter_mut().enumerate() {
            let mix_value = mix_block_value(seed_mem, reg_value, r);
            *v ^= mix_value;
        }
        reg_value = ds[prog.address_reg];
    }
    ds
}

pub struct VmMemory {
    pub seed_memory: SeedMemory,
    pub dataset_memory: RwLock<Vec<Option<[u64; 8]>>>,
    pub cache: bool,
}

impl VmMemory {
    //only useful for testing
    pub fn no_memory() -> VmMemory {
        VmMemory {
            seed_memory: SeedMemory::no_memory(),
            cache: false,
            dataset_memory: RwLock::new(Vec::with_capacity(0)),
        }
    }

    pub fn light(key: &[u8]) -> VmMemory {
        VmMemory {
            seed_memory: SeedMemory::new_initialised(key),
            cache: false,
            dataset_memory: RwLock::new(Vec::with_capacity(0)),
        }
    }
    pub fn full(key: &[u8]) -> VmMemory {
        let seed_mem = SeedMemory::new_initialised(key);
        let mem = vec![None; DATASET_ITEM_COUNT];
        VmMemory {
            seed_memory: seed_mem,
            cache: true,
            dataset_memory: RwLock::new(mem),
        }
    }

    pub fn dataset_read(&self, offset: u64, reg: &mut [u64; 8]) {
        let item_num = offset / CACHE_LINE_SIZE;

        if self.cache {
            {
                let mem = self.dataset_memory.read().unwrap();
                let rl_cached = &mem[item_num as usize];
                if let Some(rl) = rl_cached {
                    for i in 0..8 {
                        reg[i] ^= rl[i];
                    }
                    return;
                }
            }
            {
                let rl = init_dataset_item(&self.seed_memory, item_num);
                let mut mem_mut = self.dataset_memory.write().unwrap();
                mem_mut[item_num as usize] = Some(rl);
                for i in 0..8 {
                    reg[i] ^= rl[i];
                }
            }
        } else {
            let rl = init_dataset_item(&self.seed_memory, item_num);
            for i in 0..8 {
                reg[i] ^= rl[i];
            }
        }
    }
}
