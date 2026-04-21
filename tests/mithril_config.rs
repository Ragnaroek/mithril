extern crate mithril;

use mithril::mithril_config;

use std::path::Path;

#[test]
fn test_read_default_config() {
    let config = read_default_config();

    assert_eq!(config.pool_conf.pool_address, "xmrpool.eu:3333");
    assert_eq!(config.pool_conf.wallet_address, "48y3RCT5SzSS4jumHm9rRL91eWWzd6xcVGSCF1KUZGWYJ6npqwFxHee4xkLLNUqY4NjiswdJhxFALeRqzncHoToeJMg2bhL");
    assert_eq!(config.pool_conf.pool_password, "x");

    assert_eq!(config.worker_conf.num_threads, 8);
    assert_eq!(config.worker_conf.auto_tune, true);
    assert_eq!(config.worker_conf.auto_tune_interval_minutes, 15);
    assert_eq!(config.worker_conf.auto_tune_log, "./bandit.log");

    assert_eq!(config.donation_conf.percentage, 2.5);
}

//helper

fn read_default_config() -> mithril_config::MithrilConfig {
    let path = &format!("{}{}", "./", "default_config.toml");
    return mithril_config::read_config(Path::new(path), "default_config.toml").unwrap();
}
