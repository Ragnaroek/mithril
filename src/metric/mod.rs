
extern crate crossbeam_channel;

use std::thread;
use std::time;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc};
use std::io::Write;
use std::fs::OpenOptions;
use self::crossbeam_channel::{unbounded, Sender, Receiver, select, RecvTimeoutError};

#[derive(Clone)]
pub struct MetricConfig {
    pub enabled: bool,
    pub resolution: u64,
    pub sample_interval_seconds: u64,
    pub report_file: String,
}

pub struct Metric {
    /// This is the total hash count since the construction of the
    /// metric struct.
    total_hashes: Arc<AtomicU64>,
    cnt_hnd: thread::JoinHandle<()>,
    tick_hnd: thread::JoinHandle<()>,
    stop_tick_sndr: Sender<()>,
    stop_cnt_sndr: Sender<()>,
}

pub fn start(conf: MetricConfig, hash_cnt_rcvr: Receiver<u64>) -> Metric {

    let log_count = Arc::new(AtomicU64::new(0));
    let total_count = Arc::new(AtomicU64::new(0));

    let thread_log_count = log_count.clone();
    let thread_total_count = total_count.clone();
    let (stop_cnt_sndr, stop_cnt_rcvr) = unbounded();

    let cnt_hnd = thread::Builder::new().name("metric counting thread".to_string()).spawn(move || {
        'select_loop: loop {
            select! {
                recv(stop_cnt_rcvr) -> _ => {
                    info!("stopping metric counting thread");
                    break 'select_loop;
                },
                recv(hash_cnt_rcvr) -> cnt_msg => {
                    let cnt = cnt_msg.expect("Hash channel unexpectedly closed");
                    thread_log_count.fetch_add(cnt, Ordering::SeqCst);
                    thread_total_count.fetch_add(cnt, Ordering::SeqCst);
                }
            }
        }
    }).expect("metric counting thread handle");

    let (stop_tick_sndr, stop_tick_rcvr) = unbounded();

    let tick_hnd = thread::Builder::new().name("metric sample thread".to_string()).spawn(move || {
        loop {
            let recv_result = stop_tick_rcvr.recv_timeout(time::Duration::from_secs(conf.sample_interval_seconds));
            match recv_result {
                Ok(()) | Err(RecvTimeoutError::Disconnected) => {
                    info!("metric sample thread stopped");
                    break;
                },
                Err(RecvTimeoutError::Timeout) => {}, //continue with next loop
            }

            let sample_cnt = log_count.swap(0, Ordering::SeqCst);

            let timestamp_result = time::SystemTime::now().duration_since(time::UNIX_EPOCH);
            if timestamp_result.is_err() {
                error!("error getting metric timestamp");
                return;
            }
            let timestamp = timestamp_result.unwrap();
            let millis = timestamp.as_secs() * 1_000 + u64::from(timestamp.subsec_millis());

            let file_result = OpenOptions::new()
                             .create(true)
                             .append(true)
                             .open(conf.report_file.clone());
            if let Ok(mut file) = file_result {
                let write_result = writeln!(file, "{};{}", millis, sample_cnt);
                if write_result.is_err() {
                    error!("could not write metric file");
                }
                if file.flush().is_err() {
                    error!("err flushing metric file");
                }
            } else {
                error!("could not open metric file");
            }
        }
    }).expect("metric sample thread handle");

    Metric{total_hashes: total_count, cnt_hnd, tick_hnd, stop_tick_sndr, stop_cnt_sndr}
}

impl Metric {
    pub fn hash_count(&self) -> u64 {
        self.total_hashes.load(Ordering::SeqCst)
    }

    pub fn stop(&self) {
        info!("stopping metrics");

        let res_tick = self.stop_tick_sndr.send(());
        if res_tick.is_err() {
            error!("sending tick stop failed {:?}", res_tick);
        }
        let res_cnt = self.stop_cnt_sndr.send(());
        if res_cnt.is_err() {
            error!("sending cnt stop failed {:?}", res_cnt);
        }

        info!("metrics stopped");
    }

    pub fn join(self) {
        let _ = self.tick_hnd.join();
        let _ = self.cnt_hnd.join();
    }
}
