extern crate bandit;
extern crate num_cpus;
extern crate dirs;

use std::path::{PathBuf};
use std::fs::{DirBuilder};
use std::io;

use self::bandit::softmax::{AnnealingSoftmax, AnnealingSoftmaxConfig};
use self::bandit::{Identifiable, BanditConfig};

const MAX_THREADS_PER_CPU : usize = 4;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct ThreadArm {
    pub num_threads: u64
}

impl Identifiable for ThreadArm {
    fn ident(&self) -> String {
        format!("threads:{}", self.num_threads)
    }
}

pub fn setup_bandit(log_file: String) -> AnnealingSoftmax<ThreadArm> {
    let num_arms = num_cpus::get() * MAX_THREADS_PER_CPU;
    let mut arms = Vec::with_capacity(num_arms);
    for i in 1..num_arms {
        arms.push(ThreadArm{num_threads: i as u64})
    }

    let state_file = state_file();

    let bandit_config = BanditConfig{
        log_file: Some(PathBuf::from(log_file))
    };

    let softmax_config = AnnealingSoftmaxConfig{cooldown_factor: 0.7};

    if state_file.exists() {
        let loaded_state = AnnealingSoftmax::load_bandit(arms.clone(), bandit_config.clone(), &state_file);
        if loaded_state.is_err() {
            error!("loading bandit state failed, using new bandit. error {:?}", loaded_state);
            AnnealingSoftmax::new(arms, bandit_config, softmax_config)
        } else {
            info!("continuing with loaded bandit state");
            loaded_state.unwrap()
        }
    } else {
        info!("no bandit state file found, using new bandit");
        AnnealingSoftmax::new(arms, bandit_config, softmax_config)
    }
}

pub fn ensure_mithril_folder_exists() -> io::Result<()> {
    let folder = mithril_folder();
    DirBuilder::new().recursive(true).create(folder)
}

pub fn mithril_folder() -> PathBuf {
    let mut state_file = dirs::home_dir().expect("home dir");
    state_file.push(".mithril");
    state_file
}

pub fn state_file() -> PathBuf {
    let mut state_file = mithril_folder();
    state_file.push("bandit_state.json");
    state_file
}
