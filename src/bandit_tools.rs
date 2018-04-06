extern crate bandit;
extern crate num_cpus;

use std;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration};
use std::path::{PathBuf};
use std::fs::{DirBuilder};
use std::io;

use self::bandit::softmax::{AnnealingSoftmax, DEFAULT_CONFIG};
use self::bandit::{Identifiable, BanditConfig};
use worker::worker_pool::{WorkerConfig};

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

    if state_file.exists() {
        let loaded_state = AnnealingSoftmax::load_bandit(arms.clone(), bandit_config.clone(), &state_file);
        if loaded_state.is_err() {
            error!("loading bandit state failed, using new bandit. error {:?}", loaded_state);
            AnnealingSoftmax::new(arms, bandit_config, DEFAULT_CONFIG)
        } else {
            info!("continuing with loaded bandit state");
            loaded_state.unwrap()
        }
    } else {
        info!("no bandit state file found, using new bandit");
        AnnealingSoftmax::new(arms, bandit_config, DEFAULT_CONFIG)
    }
}

pub fn ensure_mithril_folder_exists() -> io::Result<()> {
    let folder = mithril_folder();
    DirBuilder::new().recursive(true).create(folder)
}

pub fn mithril_folder() -> PathBuf {
    let mut state_file = std::env::home_dir().expect("home dir");
    state_file.push(".mithril");
    state_file
}

pub fn state_file() -> PathBuf {
    let mut state_file = mithril_folder();
    state_file.push("bandit_state.json");
    state_file
}

pub fn setup_bandit_arm_select_clock(worker_conf: &WorkerConfig) -> Receiver<()>{
    let (clock_tx, clock_rx) = channel();

    let interval = if worker_conf.auto_tune {
        info!("auto_tune enabled, starting arm clock signaling");
        60 * worker_conf.auto_tune_interval_minutes
    } else {
        info!("auto_tune disabled");
        std::u64::MAX
    };

    //if auto_tune is not enabled, never send the clock signal for drawing
    //a new arm, effectively disabling auto tuning
    thread::Builder::new().name("clock signal thread".to_string()).spawn(move ||{
        thread::sleep(Duration::from_secs(interval));
        clock_tx.send(()).expect("sending clock signal");
    }).expect("clock signal thread handle");

    clock_rx
}
