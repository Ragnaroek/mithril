use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use super::super::cryptonight::hash;
use super::super::cryptonight::hash::{MEM_SIZE};
use super::super::cryptonight::aes;
use super::super::cryptonight::aes::{AES, AESSupport};
use super::super::stratum::stratum;
use super::super::stratum::stratum_data;
use super::super::byte_string;
use super::super::u64x2::{u64x2};

pub struct WorkerPool {
    thread_chan : Vec<Sender<WorkerCmd>>,
    num_threads: u64
}

pub struct WorkerConfig {
    pub num_threads: u64
}

#[derive(Debug)]
pub struct JobData {
    pub miner_id: String,
    pub blob: String,
    pub job_id: String,
    pub target: String,
    pub nonce_partition: u8,
    pub nonce_partition_num_bits: u8
}

#[derive(Debug)]
pub enum WorkerCmd {
    NewJob {
        job_data: JobData
    },
}

pub fn start(conf: WorkerConfig,
             aes_support: AESSupport,
             share_tx: Sender<stratum::StratumCmd>,
             metric_resolution: u64,
             metric_tx: Sender<u64>) -> WorkerPool {
    let mut thread_chan : Vec<Sender<WorkerCmd>> = Vec::with_capacity(conf.num_threads as usize);
    for _ in 0..conf.num_threads {
        let (tx, rx) = channel();
        let share_tx_thread = share_tx.clone();
        let metric_tx_thread = metric_tx.clone();
        let aes_support_thread = aes_support.clone();
        thread_chan.push(tx);
        thread::spawn(move || {
            work(rx, share_tx_thread, aes_support_thread, metric_resolution, metric_tx_thread)
        });
    }
    return WorkerPool{thread_chan, num_threads: conf.num_threads};
}

impl WorkerPool {
    pub fn job_change(&self, miner_id: String, blob: String, job_id: String, target: String) {
        info!("job change, blob {}", blob);
        let mut partition_ix = 0;
        let num_bits = num_bits(self.num_threads);
        for tx in self.thread_chan.clone() {
            tx.send(WorkerCmd::NewJob{
                job_data: JobData {
                    miner_id: miner_id.clone(),
                    blob: blob.clone(),
                    job_id: job_id.clone(),
                    target: target.clone(),
                    nonce_partition: partition_ix,
                    nonce_partition_num_bits: num_bits
                }}).unwrap();
            partition_ix += 1;
        }
    }
}

pub fn num_bits(num_threads: u64) -> u8 {
    match num_threads {
        0 => return 0,
        1 => return 1,
        x => return (x as f64).log2().ceil() as u8
    }
}

//TODO pub fn stop() //stop all workers, for controlled shutdown

fn work(rcv: Receiver<WorkerCmd>,
        share_tx: Sender<stratum::StratumCmd>,
        aes_support: AESSupport,
        metric_resolution: u64,
        metric_tx: Sender<u64>) {

    let aes = aes::new(aes_support);
    let mut scratchpad : Box<[u64x2; MEM_SIZE]> = box [u64x2(0,0); MEM_SIZE];

    loop {
        let job_blocking = rcv.recv();
        if job_blocking.is_err() {
            error!("job channel was droppped: {:?}", job_blocking);
            //channel was dropped, terminate thread
            return;
        } else {
            work_job(&mut scratchpad, job_blocking.unwrap(), &rcv, &share_tx, &aes, metric_resolution, &metric_tx);
            //if work_job returns the received WorkerCmd was not a job cmd
            //or the nonce space was exhausted. We have to wait blocking for
            //a new job and "idle".
            warn!("nonce space exhausted, thread idle");
        }
    }
}

pub fn with_nonce(blob: String, nonce: String) -> String {
    let (a, _) = blob.split_at(78);
    let (_, b) = blob.split_at(86);
    return format!("{}{}{}", a, nonce, b);
}

fn work_job(scratchpad : &mut Box<[u64x2; MEM_SIZE]>,
    job: WorkerCmd,
    rcv: &Receiver<WorkerCmd>,
    share_tx: &Sender<stratum::StratumCmd>,
    aes: &AES,
    metric_resolution: u64,
    metric_tx: &Sender<u64>) {
    match job {
        WorkerCmd::NewJob{job_data} => {
            let num_target = target_u64(byte_string::hex2_u32_le(&job_data.target));
            let first_byte = job_data.nonce_partition << (8 - job_data.nonce_partition_num_bits);

            let mut hash_count : u64 = 0;

            for i in 0..2^(8 - job_data.nonce_partition_num_bits) {
                for j in 0..u8::max_value() {
                    for k in 0..u8::max_value() {
                        for l in 0..u8::max_value() {

                            let nonce = format!("{:02x}{:02x}{:02x}{:02x}", first_byte | i, j, k, l);
                            let hash_in = with_nonce(job_data.blob.clone(), nonce.clone());
                            let bytes_in = byte_string::string_to_u8_array(&hash_in);

                            let hash_result = hash::hash(scratchpad, &bytes_in, aes);
                            let hash_val = byte_string::hex2_u64_le(&hash_result[48..]);

                            if hash_val < num_target {
                                let share = stratum_data::Share{
                                    miner_id: job_data.miner_id.clone(),
                                    job_id: job_data.job_id.clone(),
                                    nonce: nonce,
                                    hash: hash_result
                                };

                                let submit_result = stratum::submit_share(share_tx, share);
                                if submit_result.is_err() {
                                    error!("submitting share failed: {:?}", submit_result);
                                }
                            }

                            hash_count += 1;
                            if hash_count % metric_resolution == 0 {
                                let send_result = metric_tx.send(hash_count);
                                if send_result.is_err() {
                                    error!("metric submit failed {:?}", send_result);
                                }
                                hash_count = 0;
                            }

                            if new_job_available(rcv) {
                                let send_result = metric_tx.send(hash_count);
                                if send_result.is_err() { //flush hash_count
                                    error!("metric submit failed {:?}", send_result);
                                }
                                return
                            }
                        }
                    }
                }
            }
        }
    }
}

fn new_job_available(rcv: &Receiver<WorkerCmd>) -> bool {
    let try_result = rcv.try_recv();
    match try_result {
        Ok(WorkerCmd::NewJob{job_data: _job_data}) => return true,
        _ => return false
    }
}

pub fn target_u64(t: u32) -> u64 {
    return u64::max_value() / (u32::max_value() as u64 / t as u64)
}
