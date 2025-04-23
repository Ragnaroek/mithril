extern crate crossbeam_channel;

use crate::mithril_config::DonationConfig;
use crate::worker::worker_pool::WorkerConfig;

use self::crossbeam_channel::{unbounded, Receiver};
use std;
use std::thread;
use std::time::Duration;

const DONATION_THRESHOLD: f64 = 1.0 / 10.0;

#[derive(Debug, PartialEq)]
pub enum TickAction {
    ArmChange,
    DonationHashing,
}

pub fn interval_mod_setup(
    worker_conf: &WorkerConfig,
    donation_conf: &DonationConfig,
) -> (u64, Option<u64>) {
    if donation_conf.percentage >= DONATION_THRESHOLD && !worker_conf.auto_tune {
        return (100 * 60, Some(1));
    }

    let interval = if worker_conf.auto_tune {
        info!("auto_tune enabled, starting arm clock signaling");
        60 * worker_conf.auto_tune_interval_minutes
    } else {
        info!("auto_tune disabled");
        std::u64::MAX
    };
    let donation_mod = if donation_conf.percentage >= DONATION_THRESHOLD {
        if donation_conf.percentage >= 100.0 {
            Some(1)
        } else {
            Some((100.0 / worker_conf.auto_tune_interval_minutes as f64).ceil() as u64)
        }
    } else {
        None
    };
    (interval, donation_mod)
}

/// clock for bandit arm change and donation
pub fn setup(worker_conf: &WorkerConfig, donation_conf: &DonationConfig) -> Receiver<TickAction> {
    let (clock_sndr, clock_rcvr) = unbounded();

    let (reg_interval, donation_mod) = interval_mod_setup(worker_conf, donation_conf);
    let mut interval = reg_interval;

    let donation_percentage = donation_conf.percentage;
    //if auto_tune is not enabled, never send the clock signal for drawing
    //a new arm, effectively disabling auto tuning
    thread::Builder::new()
        .name("clock signal thread".to_string())
        .spawn(move || {
            let mut arm_changes = 1;
            loop {
                thread::sleep(Duration::from_secs(interval));

                let action = if let Some(d_mod) = donation_mod {
                    if arm_changes % d_mod == 0 {
                        TickAction::DonationHashing
                    } else {
                        TickAction::ArmChange
                    }
                } else {
                    TickAction::ArmChange
                };

                interval = if action == TickAction::DonationHashing {
                    (donation_percentage * 60.0).ceil() as u64
                } else {
                    reg_interval
                };

                clock_sndr.send(action).expect("sending clock signal");
                arm_changes += 1;
            }
        })
        .expect("clock signal thread handle");

    clock_rcvr
}
