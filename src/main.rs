#![feature(mpsc_select)]

#[macro_use]
extern crate log;

extern crate mithril;
extern crate env_logger;
extern crate bandit;

use mithril::stratum::{StratumClient, StratumAction};
use mithril::worker::worker_pool;
use mithril::worker::worker_pool::{WorkerPool};
use mithril::metric;
use mithril::cryptonight::hash;
use mithril::cryptonight::aes;
use mithril::cryptonight::aes::{AESSupport};
use mithril::byte_string;
use mithril::bandit_tools;
use mithril::mithril_config;
use mithril::timer;
use std::sync::mpsc::{channel, Select, Receiver};
use std::path::Path;
use std::io;
use std::io::{Error};
use std::thread;
use std::time::{Duration};

use bandit::MultiArmedBandit;

#[derive(Debug, PartialEq)]
enum MainLoopExit {
    DrawNewBanditArm,
    DonationHashing
}

fn main() {

    env_logger::init();

    //Read config
    let cwd_path = &format!("{}{}", "./", mithril_config::CONFIG_FILE_NAME);
    let config = mithril_config::read_config(Path::new(cwd_path), mithril_config::CONFIG_FILE_NAME).unwrap();

    sanity_check(config.hw_conf.aes_support);

    if config.donation_conf.percentage > 0.0 {
        print_donation_hint(config.donation_conf.percentage);
    }

    let mut bandit = if config.worker_conf.auto_tune {
        Some(bandit_tools::setup_bandit(config.worker_conf.auto_tune_log.clone()))
    } else {
        None
    };

    let timer_rx = timer::setup(&config.worker_conf, &config.donation_conf);
    let mut donation_hashing = false;

    loop {
        //Stratum start
        let (stratum_tx, stratum_rx) = channel();
        let (client_err_tx, client_err_rx) = channel();

        let conf = if donation_hashing {
            mithril_config::donation_conf()
        } else {
            config.pool_conf.clone()
        };

        let login_result = StratumClient::login(conf, client_err_tx, stratum_tx);
        if login_result.is_err() {
            error!("stratum login failed {:?}", login_result.err());
            await_timeout();
            continue;
        }
        let client = login_result.expect("stratum client");

        let share_tx = client.new_cmd_channel();

        let (arm, num_threads) = if bandit.is_some() {
            let selected_arm = bandit.as_ref().unwrap().select_arm();
            info!("trying arm with {} #threads", selected_arm.num_threads);
            (Some(selected_arm), selected_arm.num_threads)
        } else {
            (None, config.worker_conf.num_threads)
        };

        let (metric_tx, metric_rx) = channel();
        let metric = metric::start(config.metric_conf.clone(), metric_rx);

        //worker pool start
        let pool = worker_pool::start(num_threads, config.hw_conf.clone().aes_support,
            &share_tx, config.metric_conf.resolution, &metric_tx.clone());

        let term_result = start_main_event_loop(&pool, &client_err_rx, &stratum_rx, &timer_rx);

        pool.stop();
        client.stop();

        match term_result {
            Err(err) => {
                error!("error received, restarting connection after 60 seconds. err was {}", err);
                await_timeout();
            },
            Ok(ex) => {
                info!("main loop exit, next loop {:?}", ex);
                pool.join();

                metric.stop();
                let hashes = metric.hash_count();
                metric.join();

                if arm.is_some() && bandit.is_some() && !donation_hashing {
                    //do not save reward for donation hashing, it probably only runs for a short period
                    let bandit_ref = bandit.as_mut().unwrap();
                    let reward = (hashes as f64 / (config.worker_conf.auto_tune_interval_minutes as f64 * 60.0)) / 1000.0; /*kH/s*/
                    info!("adding reward {:?} for arm {:?}", reward, arm);
                    bandit_ref.update(arm.unwrap(), reward);
                    save_bandit_state(bandit_ref);
                }

                donation_hashing = ex == MainLoopExit::DonationHashing;
            }
        }
    }
}

fn await_timeout() {
    thread::sleep(Duration::from_secs(60))
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
    client_err_rx: &Receiver<Error>,
    stratum_rx: &Receiver<StratumAction>,
    timer_rx: &Receiver<timer::TickAction>) -> io::Result<MainLoopExit> {

    let select = Select::new();
    let mut err_hnd = select.handle(client_err_rx);
    unsafe {err_hnd.add()};
    let mut rcv_hnd = select.handle(stratum_rx);
    unsafe {rcv_hnd.add()};
    let mut clock_hnd = select.handle(timer_rx);
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
            let clock_res = timer_rx.recv();
            if clock_res.is_err() {
                return Err(io::Error::new(io::ErrorKind::Other, format!("error received {:?}", clock_res)));
            } else {
                let tick_action = clock_res.expect("tickAction");
                match tick_action {
                    timer::TickAction::ArmChange => {
                        info!("bandit clock signal received - time for new arm");
                        return Ok(MainLoopExit::DrawNewBanditArm)
                    },
                    timer::TickAction::DonationHashing => {
                        return Ok(MainLoopExit::DonationHashing)
                    }
                }
            }
        }
    }
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

fn print_donation_hint(percentage: f64) {
    println!("-------------------------------------------------------------------");
    println!("Donation Hashing enabled with {}%.", percentage);
    println!("Thank you for supporting the project with your donation hashes!");
    println!("-------------------------------------------------------------------");
}
