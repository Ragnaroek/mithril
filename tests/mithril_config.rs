extern crate mithril;

use mithril::mithril_config;
use mithril::cryptonight::aes::AESSupport;

use std::path::Path;

#[test]
fn test_read_default_config() {

    let path = &format!("{}{}", "./", "default_config.toml");
    let config = mithril_config::read_config(Path::new(path), "default_config.toml").unwrap();

    assert_eq!(config.pool_conf.pool_address, "iron-pool.com:5555");
    assert_eq!(config.pool_conf.wallet_address, "");
    assert_eq!(config.pool_conf.pool_password, "");

    assert_eq!(config.worker_conf.num_threads, 8);
    assert_eq!(config.worker_conf.auto_tune, true);
    assert_eq!(config.worker_conf.auto_tune_interval_minutes , 15);
    assert_eq!(config.worker_conf.auto_tune_log, "./bandit.log");

    assert_eq!(config.metric_conf.enabled, false);
    assert_eq!(config.metric_conf.resolution, std::u64::MAX);
    assert_eq!(config.metric_conf.sample_interval_seconds, std::u64::MAX);
    assert_eq!(config.metric_conf.report_file, "/dev/null");

    assert_eq!(config.hw_conf.aes_support, AESSupport::HW);

    assert_eq!(config.donation_conf.percentage, 2.5);
}
