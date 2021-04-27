#[macro_use]
extern crate log;

extern crate bandit;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate mithril;

use self::crossbeam_channel::{select, unbounded, Receiver};
use mithril::bandit_tools;
use mithril::metric;
use mithril::mithril_config;
use mithril::randomx::memory::VmMemoryAllocator;
use mithril::stratum::{StratumAction, StratumClient};
use mithril::timer;
use mithril::worker::worker_pool;
use mithril::worker::worker_pool::WorkerPool;
use std::io;
use std::io::Error;
use std::path::Path;
use std::thread;
use std::time::Duration;

use bandit::MultiArmedBandit;

#[derive(Debug, PartialEq)]
enum MainLoopExit {
    DrawNewBanditArm,
    DonationHashing,
}

#[allow(clippy::unnecessary_unwrap)]
fn main() {
    env_logger::init();

    //Read config
    let cwd_path = &format!("{}{}", "./", mithril_config::CONFIG_FILE_NAME);
    let config =
        mithril_config::read_config(Path::new(cwd_path), mithril_config::CONFIG_FILE_NAME).unwrap();

    if config.donation_conf.percentage > 0.0 {
        print_donation_hint(config.donation_conf.percentage);
    }

    let mut bandit = if config.worker_conf.auto_tune {
        Some(bandit_tools::setup_bandit(
            config.worker_conf.auto_tune_log.clone(),
        ))
    } else {
        None
    };

    let timer_rcvr = timer::setup(&config.worker_conf, &config.donation_conf);
    let mut donation_hashing = false;
    let mut vm_memory_allocator = VmMemoryAllocator::initial();

    loop {
        //Stratum start
        let (stratum_sndr, stratum_rcvr) = unbounded();
        let (client_err_sndr, client_err_rcvr) = unbounded();

        let conf = if donation_hashing {
            mithril_config::donation_conf()
        } else {
            config.pool_conf.clone()
        };

        let login_result = StratumClient::login(conf, client_err_sndr, stratum_sndr);
        if login_result.is_err() {
            error!("stratum login failed {:?}", login_result.err());
            await_timeout();
            continue;
        }
        let client = login_result.expect("stratum client");
        let share_sndr = client.new_cmd_channel();
        let (arm, num_threads) = if bandit.is_some() {
            let selected_arm = bandit.as_ref().unwrap().select_arm();
            info!("trying arm with {} #threads", selected_arm.num_threads);
            (Some(selected_arm), selected_arm.num_threads)
        } else {
            (None, config.worker_conf.num_threads)
        };

        let (metric_sndr, metric_rcvr) = unbounded();
        let metric = metric::start(config.metric_conf.clone(), metric_rcvr);

        //worker pool start
        let mut pool = worker_pool::start(
            num_threads,
            &share_sndr,
            config.metric_conf.resolution,
            &metric_sndr.clone(),
            vm_memory_allocator,
        );

        let term_result =
            start_main_event_loop(&mut pool, &client_err_rcvr, &stratum_rcvr, &timer_rcvr);

        vm_memory_allocator = pool.vm_memory_allocator.clone();
        pool.stop();
        client.stop();

        match term_result {
            Err(err) => {
                error!(
                    "error received, restarting connection after 60 seconds. err was {}",
                    err
                );
                await_timeout();
            }
            Ok(ex) => {
                info!("main loop exit, next loop {:?}", ex);
                pool.join();

                metric.stop();
                let hashes = metric.hash_count();
                metric.join();

                if arm.is_some() && bandit.is_some() && !donation_hashing {
                    //do not save reward for donation hashing, it probably only runs for a short period
                    let bandit_ref = bandit.as_mut().unwrap();
                    let reward = (hashes as f64
                        / (config.worker_conf.auto_tune_interval_minutes as f64 * 60.0))
                        / 1000.0; /*kH/s*/
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
fn start_main_event_loop(
    pool: &mut WorkerPool,
    client_err_rcvr: &Receiver<Error>,
    stratum_rcvr: &Receiver<StratumAction>,
    timer_rcvr: &Receiver<timer::TickAction>,
) -> io::Result<MainLoopExit> {
    loop {
        select! {
            recv(stratum_rcvr) -> stratum_msg => {
                if stratum_msg.is_err() {
                    return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "received error"));
                }
                match stratum_msg.unwrap() {
                    StratumAction::Job{miner_id, seed_hash, blob, job_id, target} => {
                        pool.job_change(&miner_id, &seed_hash, &blob, &job_id, &target);
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
            },
            recv(timer_rcvr) -> timer_msg => {
                if timer_msg.is_err() {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("error received {:?}", timer_msg)));
                } else {
                    let tick_action = timer_msg.expect("tickAction");
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
            },
            recv(client_err_rcvr) -> client_err_msg => {
                return Err(io::Error::new(io::ErrorKind::Other, format!("error received {:?}", client_err_msg)));
            }
        }
    }
}

fn print_donation_hint(percentage: f64) {
    println!("-------------------------------------------------------------------");
    println!("Donation Hashing enabled with {}%.", percentage);
    println!("Thank you for supporting the project with your donation hashes!");
    println!("-------------------------------------------------------------------");
}
