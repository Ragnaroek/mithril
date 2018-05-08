extern crate mithril;

use mithril::timer;
use mithril::worker::worker_pool::{WorkerConfig};
use mithril::mithril_config::{DonationConfig};

#[test]
fn test_interval_mod_setup_donation_disabled_auto_tune_enabled() {
    let worker_conf = WorkerConfig{
        auto_tune: true,
        auto_tune_interval_minutes: 15,
        auto_tune_log: "/log/file".to_string(),
        num_threads: 8
    };
    let donation_conf = DonationConfig{
        percentage: 0.0
    };

    let (interval, donation_mod) = timer::interval_mod_setup(&worker_conf, &donation_conf);
    assert_eq!(interval, 60 * 15);
    assert_eq!(donation_mod, None);
}

#[test]
fn test_interval_mod_setup_donation_below_threshold_auto_tune_enabled() {
    let worker_conf = WorkerConfig{
        auto_tune: true,
        auto_tune_interval_minutes: 15,
        auto_tune_log: "/log/file".to_string(),
        num_threads: 8
    };
    let donation_conf = DonationConfig{
        percentage: 1.0/10.0 - std::f64::EPSILON
    };

    let (interval, donation_mod) = timer::interval_mod_setup(&worker_conf, &donation_conf);
    assert_eq!(interval, 60 * 15);
    assert_eq!(donation_mod, None);
}

#[test]
fn test_interval_mod_setup_donation_disabled_auto_tune_disabled() {
    let worker_conf = WorkerConfig{
        auto_tune: false,
        auto_tune_interval_minutes: 15,
        auto_tune_log: "/log/file".to_string(),
        num_threads: 8
    };
    let donation_conf = DonationConfig{
        percentage: 0.0
    };

    let (interval, donation_mod) = timer::interval_mod_setup(&worker_conf, &donation_conf);
    assert_eq!(interval, std::u64::MAX);
    assert_eq!(donation_mod, None);
}

#[test]
fn test_interval_mod_setup_donation_enabled_auto_tune_disabled() {
    let worker_conf = WorkerConfig{
        auto_tune: false,
        auto_tune_interval_minutes: 15,
        auto_tune_log: "/log/file".to_string(),
        num_threads: 8
    };
    let donation_conf = DonationConfig{
        percentage: 2.5
    };

    let (interval, donation_mod) = timer::interval_mod_setup(&worker_conf, &donation_conf);
    assert_eq!(interval, 100 * 60);
    assert_eq!(donation_mod, Some(1));
}

#[test]
fn test_interval_mod_setup_donation_enabled_auto_tune_enabled() {
    let worker_conf = WorkerConfig{
        auto_tune: true,
        auto_tune_interval_minutes: 15,
        auto_tune_log: "/log/file".to_string(),
        num_threads: 8
    };
    let donation_conf = DonationConfig{
        percentage: 2.5
    };

    let (interval, donation_mod) = timer::interval_mod_setup(&worker_conf, &donation_conf);
    assert_eq!(interval, 15 * 60);
    assert_eq!(donation_mod, Some(7));
}

#[test]
fn test_interval_mod_setup_donation_100_percent_enabled_auto_tune_enabled() {
    let worker_conf = WorkerConfig{
        auto_tune: true,
        auto_tune_interval_minutes: 15,
        auto_tune_log: "/log/file".to_string(),
        num_threads: 8
    };
    let donation_conf = DonationConfig{
        percentage: 100.0
    };

    let (interval, donation_mod) = timer::interval_mod_setup(&worker_conf, &donation_conf);
    assert_eq!(interval, 15 * 60);
    assert_eq!(donation_mod, Some(1));
}
