
extern crate config;

use metric::{MetricConfig};
use stratum::stratum_data::{PoolConfig};
use worker::worker_pool::{WorkerConfig};
use cryptonight::aes::{AESSupport};

use std;
use std::path::{Path};
use self::config::{Config, ConfigError, File};

pub const CONFIG_FILE_NAME : &str = "config.toml";

/// contains all configurations for mithril
#[derive(Clone)]
pub struct MithrilConfig {
    pub pool_conf: PoolConfig,
    pub worker_conf: WorkerConfig,
    pub metric_conf: MetricConfig,
    pub hw_conf: HardwareConfig,
    pub donation_conf: DonationConfig,
}

#[derive(Clone)]
pub struct DonationConfig {
    pub percentage: f64
}

#[derive(Clone)]
pub struct HardwareConfig {
    pub aes_support: AESSupport
}

pub fn read_config(conf_file: &Path, filename: &str) -> Result<MithrilConfig, config::ConfigError> {
    let config = parse_conf(conf_file, filename)?;

    let pool_conf = pool_config(&config)?;
    let worker_conf = worker_config(&config)?;
    let metric_conf = metric_config(&config)?;
    let hw_conf = hardware_config(&config)?;
    let donation_conf = donation_config(&config)?;

    Ok(MithrilConfig{pool_conf, worker_conf, metric_conf, hw_conf, donation_conf})
}

fn donation_config(conf: &Config) -> Result<DonationConfig, ConfigError> {
    let percentage = conf.get_float("donation.percentage")?;
    Ok(DonationConfig{percentage})
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
        return Err(ConfigError::Message("num_threads has to be > 0".to_string()));
    }

    let auto_tune = conf.get_bool("worker.auto_tune")?;

    let auto_tune_interval_minutes = conf.get_int("worker.auto_tune_interval_minutes")?;
    if auto_tune_interval_minutes <= 0 {
        return Err(ConfigError::Message("auto_tune_interval_minutes has to be > 0".to_string()));
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
        Ok(MetricConfig{enabled: false, resolution: std::u32::MAX as u64,
                        sample_interval_seconds: std::u32::MAX as u64, report_file: "/dev/null".to_string()})
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

fn parse_conf(conf_file: &Path, filename: &str) -> Result<Config, ConfigError> {
    if conf_file.exists() {
        let mut conf = Config::default();
        conf.merge(File::with_name(filename))?;
        return Ok(conf);
    }
    Err(ConfigError::Message("config file not found".to_string()))
}

pub fn donation_conf() -> PoolConfig {
    PoolConfig {
        pool_address: "xmrpool.eu:3333".to_string(),
        pool_password: "x".to_string(),
        wallet_address: "48y3RCT5SzSS4jumHm9rRL91eWWzd6xcVGSCF1KUZGWYJ6npqwFxHee4xkLLNUqY4NjiswdJhxFALeRqzncHoToeJMg2bhL".to_string()
    }
}
