extern crate mithril;

use mithril::mithril_config;
use mithril::cryptonight::aes::AESSupport;

use std::time::{Duration, Instant};
use std::path::Path;

#[test]
fn test_read_default_config() {

    let config = read_default_config();

    assert_eq!(config.pool_conf.pool_address, "xmrpool.eu:3333");
    assert_eq!(config.pool_conf.wallet_address, "");
    assert_eq!(config.pool_conf.pool_password, "");

    assert_eq!(config.worker_conf.num_threads, 8);
    assert_eq!(config.worker_conf.auto_tune, true);
    assert_eq!(config.worker_conf.auto_tune_interval_minutes , 15);
    assert_eq!(config.worker_conf.auto_tune_log, "./bandit.log");

    assert_eq!(config.metric_conf.enabled, false);
    assert_eq!(config.metric_conf.resolution, std::u32::MAX as u64);
    assert_eq!(config.metric_conf.sample_interval_seconds, std::u32::MAX as u64);
    assert_eq!(config.metric_conf.report_file, "/dev/null");

    assert_eq!(config.hw_conf.aes_support, AESSupport::HW);

    assert_eq!(config.donation_conf.percentage, 2.5);
}

#[test] //Bugfix test, there should be some "room" so that this value can be added to a time instant
fn test_disabled_metric_value_should_be_addable_to_now() {
    let config = read_default_config();

    let now = Instant::now();
    let _ = now + Duration::from_secs(config.metric_conf.sample_interval_seconds);
    //Ok if it doesn't panic
}

//helper

fn read_default_config() -> mithril_config::MithrilConfig {
    let path = &format!("{}{}", "./", "default_config.toml");
    return mithril_config::read_config(Path::new(path), "default_config.toml").unwrap();
}
