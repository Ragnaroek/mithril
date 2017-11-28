use std::thread;
use std::time;
use std::sync::mpsc::{Receiver};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc};
use std::io::Write;
use std::fs::OpenOptions;

#[derive(Clone)]
pub struct MetricConfig {
    pub enabled: bool,
    pub resolution: u64,
    pub sample_interval_seconds: u64,
    pub report_file: String
}

pub fn start(conf: MetricConfig, hash_cnt_receiver: Receiver<u64>) {

    let count = Arc::new(AtomicU64::new(0));

    let thread_count = count.clone();
    thread::spawn(move || {
        loop {
            let cnt_rcv = hash_cnt_receiver.recv();
            if cnt_rcv.is_err() {
                error!("reading hash count failed");
            } else {
                let cnt = cnt_rcv.unwrap();
                thread_count.fetch_add(cnt, Ordering::SeqCst);
            }
        }
    });

    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_secs(conf.sample_interval_seconds));
            let sample_cnt = count.swap(0, Ordering::SeqCst);

            let timestamp_result = time::SystemTime::now().duration_since(time::UNIX_EPOCH);
            if timestamp_result.is_err() {
                error!("error getting metric timestamp");
                return;
            }
            let timestamp = timestamp_result.unwrap();
            let millis = timestamp.as_secs() * 1_000 + (timestamp.subsec_nanos() / 1_000_000) as u64;

            let file_result = OpenOptions::new()
                             .create(true)
                             .append(true)
                             .open(conf.report_file.clone());
            if file_result.is_ok() {
                let mut file = file_result.unwrap();
                let write_result = write!(file, "{};{}\n", millis, sample_cnt);
                if write_result.is_err() {
                    error!("could not write metric file"); //TODO Log
                }
                if file.flush().is_err() {
                    error!("err flushing metric file"); //TODO Log
                }
            } else {
                error!("could not open metric file"); //TODO Log
            }
        }
    });
}
