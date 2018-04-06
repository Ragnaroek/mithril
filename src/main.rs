#![feature(mpsc_select)]

#[macro_use]
extern crate log;

extern crate mithril;
extern crate config;
extern crate env_logger;
extern crate bandit;

use mithril::stratum::stratum_data::{PoolConfig};
use mithril::stratum::{StratumClient, StratumAction};
use mithril::worker::worker_pool;
use mithril::worker::worker_pool::{WorkerConfig, WorkerPool};
use mithril::metric;
use mithril::metric::{MetricConfig};
use mithril::cryptonight::hash;
use mithril::cryptonight::aes;
use mithril::cryptonight::aes::{AESSupport};
use mithril::byte_string;
use mithril::bandit_tools;
use std::sync::mpsc::{channel, Select, Receiver};
use std::path::Path;
use std::io;
use std::io::{Error};
use std::thread;
use std::time::{Duration};
use config::{Config, ConfigError, File};
use bandit::MultiArmedBandit;

const CONFIG_FILE_NAME : &str = "config.toml";

enum MainLoopExit {
    DrawNewBanditArm
}

fn main() {

    env_logger::init().unwrap();

    //Read config
    let config = read_config().unwrap();
    let pool_conf = pool_config(&config).unwrap();
    let worker_conf = worker_config(&config).unwrap();
    let metric_conf = metric_config(&config).unwrap();
    let hw_conf = hardware_config(&config).unwrap();

    sanity_check(hw_conf.aes_support);

    let mut bandit = if worker_conf.auto_tune {
        Some(bandit_tools::setup_bandit(worker_conf.auto_tune_log.clone()))
    } else {
        None
    };

    loop {
        //Stratum start
        let (stratum_tx, stratum_rx) = channel();
        let (client_err_tx, client_err_rx) = channel();

        let mut client = StratumClient::new(pool_conf.clone(), client_err_tx, vec![stratum_tx]);
        client.login();
        let share_tx = client.new_cmd_channel().expect("command channel setup");

        let (arm, num_threads) = if bandit.is_some() {
            let selected_arm = bandit.as_ref().unwrap().select_arm();
            info!("trying arm with {} #threads", selected_arm.num_threads);
            (Some(selected_arm), selected_arm.num_threads)
        } else {
            (None, worker_conf.num_threads)
        };

        let (metric_tx, metric_rx) = channel();
        let metric = metric::start(metric_conf.clone(), metric_rx);

        //worker pool start
        let pool = worker_pool::start(num_threads, hw_conf.clone().aes_support,
            &share_tx, metric_conf.resolution, &metric_tx.clone());

        let term_result = start_main_event_loop(&pool, worker_conf.clone(), &client_err_rx, &stratum_rx);

        pool.stop();

        match term_result {
            Err(err) => {
                error!("error received, restarting connection after 60 seconds. err was {}", err);
                thread::sleep(Duration::from_secs(60));
            },
            Ok(MainLoopExit::DrawNewBanditArm) => {
                info!("main loop exit, drawing new bandit arm");
                pool.join();

                metric.stop();
                let hashes = metric.hash_count();
                metric.join();

                if arm.is_some() && bandit.is_some() {
                    let bandit_ref = bandit.as_mut().unwrap();
                    let reward = (hashes as f64 / (worker_conf.auto_tune_interval_minutes as f64 * 60.0)) / 1000.0; /*kH/s*/
                    info!("adding reward {:?} for arm {:?}", reward, arm);
                    bandit_ref.update(arm.unwrap(), reward);
                    save_bandit_state(bandit_ref);
                }
            }
        }
    }
}

fn save_bandit_state(bandit: &mut bandit::softmax::AnnealingSoftmax<bandit_tools::ThreadArm>) {

    let res = bandit_tools::ensure_mithril_folder_exists();
    if res.is_err() {
        error!("could not create folder for state file {:?}", res.err());
    }

    let save_result = bandit.save_bandit(&bandit_tools::state_file());
    if save_result.is_err() {
        error!("error saving bandit state {:?}", save_result.err());
    }

}

/// This function terminates if a non-recoverable error was detected (i.e. connection lost)
fn start_main_event_loop(pool: &WorkerPool,
    worker_conf: WorkerConfig,
    client_err_rx: &Receiver<Error>,
    stratum_rx: &Receiver<StratumAction>) -> io::Result<MainLoopExit> {

    let bandit_clock_rx = bandit_tools::setup_bandit_arm_select_clock(&worker_conf);

    let select = Select::new();
    let mut err_hnd = select.handle(client_err_rx);
    unsafe {err_hnd.add()};
    let mut rcv_hnd = select.handle(stratum_rx);
    unsafe {rcv_hnd.add()};
    let mut clock_hnd = select.handle(&bandit_clock_rx);
    unsafe {clock_hnd.add()};

    loop {
        let id = select.wait();
        if id == rcv_hnd.id() {
            let received = rcv_hnd.recv();
            if received.is_err() {
                return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "received error"));
            }
            match received.unwrap() {
                StratumAction::Job{miner_id, blob, job_id, target} => {
                    pool.job_change(&miner_id, &blob, &job_id, &target);
                },
                StratumAction::Error{err} => {
                    error!("Received stratum error: {}", err);
                },
                StratumAction::Ok => {
                    info!("Received stratum ok");
                },
                StratumAction::KeepAliveOk => {
                    info!("Received keep alive ok");
                }
            }
        } else if id == err_hnd.id() {
            let err_received = client_err_rx.recv();
            return Err(io::Error::new(io::ErrorKind::Other, format!("error received {:?}", err_received)));
        } else if id == clock_hnd.id() {
            let clock_res = bandit_clock_rx.recv();
            if clock_res.is_err() {
                return Err(io::Error::new(io::ErrorKind::Other, format!("error received {:?}", clock_res)));
            } else {
                info!("bandit clock signal received - time for new arm");
                return Ok(MainLoopExit::DrawNewBanditArm)
            }
        }
    }
}

#[derive(Clone)]
pub struct HardwareConfig {
    pub aes_support: AESSupport
}

fn pool_config(conf: &Config) -> Result<PoolConfig, ConfigError> {
    let pool_address = conf.get_str("pool.pool_address")?;
    let wallet_address = conf.get_str("pool.wallet_address")?;
    let pool_password = conf.get_str("pool.pool_password")?;
    Ok(PoolConfig{pool_address, wallet_address, pool_password})
}

fn worker_config(conf: &Config) -> Result<WorkerConfig, ConfigError> {
    let num_threads = conf.get_int("worker.num_threads")?;
    if num_threads <= 0 {
        return Err(ConfigError::Message("num_threads hat to be > 0".to_string()));
    }

    let auto_tune = conf.get_bool("worker.auto_tune")?;

    let auto_tune_interval_minutes = conf.get_int("worker.auto_tune_interval_minutes")?;
    if auto_tune_interval_minutes <= 0 {
        return Err(ConfigError::Message("auto_tune_interval_minutes hat to be > 0".to_string()));
    }

    let auto_tune_log = conf.get_str("worker.auto_tune_log")?;

    Ok(WorkerConfig{num_threads: num_threads as u64,
                    auto_tune,
                    auto_tune_interval_minutes: auto_tune_interval_minutes as u64,
                    auto_tune_log})
}

fn metric_config(conf: &Config) -> Result<MetricConfig, ConfigError> {
    let enabled = conf.get_bool("metric.enabled")?;
    if enabled {
        let resolution = get_u64_no_zero(conf, "metric.resolution")?;
        let sample_interval_seconds = get_u64_no_zero(conf, "metric.sample_interval_seconds")?;
        let report_file = conf.get_str("metric.report_file")?;
        Ok(MetricConfig{enabled, resolution, sample_interval_seconds, report_file})
    } else {
        Ok(MetricConfig{enabled: false, resolution: std::u64::MAX,
                        sample_interval_seconds: std::u64::MAX, report_file: "/dev/null".to_string()})
    }
}

fn hardware_config(conf: &Config) -> Result<HardwareConfig, ConfigError> {
    let has_aes = conf.get_bool("hardware.has_aes")?;
    let aes_support = if has_aes {
        AESSupport::HW
    } else {
        warn!("software AES enabled: hashing performance will be low");
        AESSupport::SW
    };
    Ok(HardwareConfig{aes_support})
}

fn get_u64_no_zero(conf: &Config, field: &str) -> Result<u64, ConfigError> {
    let val = conf.get_int(field)?;
    if val <= 0 {
        return Err(ConfigError::Message(format!("{} has to be > 0", field)));
    }
    Ok(val as u64)
}

fn read_config() -> Result<Config, ConfigError> {
    let cwd_path = &format!("{}{}", "./", CONFIG_FILE_NAME);
    let cwd_config_file = Path::new(cwd_path);
    if cwd_config_file.exists() {
        let mut conf = Config::default();
        conf.merge(File::with_name(CONFIG_FILE_NAME))?;
        return Ok(conf);
    }
    Err(ConfigError::Message("config file not found".to_string()))
}

fn sanity_check(aes_support: AESSupport) {

    let aes = aes::new(aes_support);

    let result0 = hash::hash_alloc_scratchpad(&byte_string::string_to_u8_array(""), &aes, hash::HashVersion::Version6);
    let result1 = hash::hash_alloc_scratchpad(&b"This is a test"[0..], &aes, hash::HashVersion::Version6);
    if result0 != "eb14e8a833fac6fe9a43b57b336789c46ffe93f2868452240720607b14387e11" ||
       result1 != "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605" {
        panic!("hash sanity check failed, please report this at https://github.com/Ragnaroek/mithril/issues");
    }
}
