use std::thread;
use std::time;
use std::sync::mpsc::{Receiver};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc};

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
                println!("reading hash count failed"); //TODO Logging
            } else {
                let cnt = cnt_rcv.unwrap();
                thread_count.fetch_add(cnt, Ordering::SeqCst);
            }
        }
    });

    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_secs(conf.sample_interval_seconds));
            let sample_cnt = count.load(Ordering::SeqCst);

            //TODO write result to csv file
            println!("sampled hash count {}", sample_cnt);
        }
    });
}
