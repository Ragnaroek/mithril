use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use super::super::cryptonight::hash;
use super::super::byte_string;

pub struct WorkerPool {
    thread_chan : Vec<Sender<WorkerCmd>>,
    num_threads: u64
}

pub struct WorkerConfig {
    pub num_threads: u64
}

#[derive(Debug)]
pub enum WorkerCmd {
    NewJob {
        blob: String,
        job_id: String,
        target: String,
        nonce_partition: u8,
        nonce_partition_num_bits: u8
    },
}

//TODO Wire stratum channels for sending share result with workers

pub fn start(conf: WorkerConfig) -> WorkerPool {
    let mut thread_chan : Vec<Sender<WorkerCmd>> = Vec::with_capacity(conf.num_threads as usize);
    for _ in 0..conf.num_threads {
        let (tx, rx) = channel();
        thread_chan.push(tx);
        thread::spawn(move || {
            work(rx)
        });
    }
    return WorkerPool{thread_chan, num_threads: conf.num_threads};
}

impl WorkerPool {
    pub fn job_change(&self, blob: String, job_id: String, target: String) {
        let mut partition_ix = 0;
        let num_bits = num_bits(self.num_threads);
        for tx in self.thread_chan.clone() {
            tx.send(WorkerCmd::NewJob{blob: blob.clone(),
                job_id: job_id.clone(),
                target: target.clone(),
                nonce_partition: partition_ix,
                nonce_partition_num_bits: num_bits}).unwrap();
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

fn work(rcv: Receiver<WorkerCmd>) {

    loop {
        let job_blocking = rcv.recv();
        if job_blocking.is_err() {
            //TODO proper logging
            println!("err: {:?}", job_blocking);
            //channel was dropped, terminate thread
            return;
        } else {
            work_job(job_blocking.unwrap(), &rcv);
            //if work_job returns the received WorkerCmd was not a job cmd
            //or the nonce space was exhausted. We have to wait blocking for
            //a new job and "idle".
        }
    }
}

fn work_job(job: WorkerCmd, rcv: &Receiver<WorkerCmd>) {
    match job {
        WorkerCmd::NewJob{blob, job_id, target, nonce_partition, nonce_partition_num_bits} => {

            println!("Starting job: {}", job_id); //TODO proper logging

            let num_target = target_u64(byte_string::hex2_u32_le(&target));
            let mut b = byte_string::string_to_u8_array(&blob);
            let first_byte = nonce_partition << (8 - nonce_partition_num_bits);

            for i in 0..2^(8 - nonce_partition_num_bits) {
                for j in 0..u8::max_value() {
                    for k in 0..u8::max_value() {
                        for l in 0..u8::max_value() {
                            b[39] = first_byte | i;
                            b[40] = j;
                            b[41] = k;
                            b[42] = l;

                            let hash_result = hash::hash(&b);
                            let hash_val = byte_string::hex2_u64_le(&hash_result[48..]);

                            if hash_val < num_target {
                                println!("found share {}", hash_val);
                                //TODO submit share
                                /*
                                let share = stratum_data::Share{
                                    miner_id: r.result.id.clone(),
                                    job_id: r.result.job.job_id.clone(),
                                    nonce: format!("{:02x}{:02x}{:02x}{:02x}", i, j, k, l),
                                    hash: hash_result
                                };
                                let share_result = stratum_data::submit_share(&stream, share);
                                println!("share submit result {:?}", share_result);
                                */
                            }
                        }
                    }
                }
            }
            //TODO check for job change after each job (is try_recv expensive?)
        },
        _ => return
    }
}

pub fn target_u64(t: u32) -> u64 {
    return u64::max_value() / (u32::max_value() as u64 / t as u64)
}
