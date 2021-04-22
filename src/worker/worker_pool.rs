extern crate crossbeam_channel;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use self::crossbeam_channel::{unbounded, Receiver, Sender};
use super::super::byte_string;
use super::super::randomx::memory::VmMemory;
use super::super::randomx::vm::new_vm;
use super::super::stratum;
use super::super::stratum::stratum_data;

pub struct WorkerPool {
    thread_chan: Vec<Sender<WorkerCmd>>,
    thread_hnd: Vec<thread::JoinHandle<()>>,
    vm_memory_seed: String,
    vm_memory: Arc<VmMemory>,
}

#[derive(Clone)]
pub struct WorkerConfig {
    pub num_threads: u64,
    pub auto_tune: bool,
    pub auto_tune_interval_minutes: u64,
    pub auto_tune_log: String,
}

pub struct JobData {
    pub miner_id: String,
    pub seed_hash: String,
    pub memory: Arc<VmMemory>,
    pub blob: String,
    pub job_id: String,
    pub target: String,
    pub nonce: Arc<AtomicU32>,
}

pub enum WorkerCmd {
    NewJob { job_data: JobData },
    Stop,
}

enum WorkerExit {
    NonceSpaceExhausted,
    NewJob { job_data: JobData },
    Stopped,
}

pub fn start(
    num_threads: u64,
    share_sndr: &Sender<stratum::StratumCmd>,
    metric_resolution: u64,
    metric_sndr: &Sender<u64>,
) -> WorkerPool {
    let mut thread_chan: Vec<Sender<WorkerCmd>> = Vec::with_capacity(num_threads as usize);
    let mut thread_hnd: Vec<thread::JoinHandle<()>> = Vec::with_capacity(num_threads as usize);
    for i in 0..num_threads {
        let (sndr, rcvr) = unbounded();
        let share_sndr_thread = share_sndr.clone();
        let metric_sndr_thread = metric_sndr.clone();

        let hnd = thread::Builder::new()
            .name(format!("worker thread {}", i))
            .spawn(move || {
                work(
                    &rcvr,
                    &share_sndr_thread,
                    metric_resolution,
                    &metric_sndr_thread,
                )
            })
            .expect("worker thread handle");
        thread_chan.push(sndr);
        thread_hnd.push(hnd);
    }
    WorkerPool {
        thread_chan,
        thread_hnd,
        vm_memory_seed: "".to_string(),
        vm_memory: Arc::new(VmMemory::no_memory()),
    }
}

impl WorkerPool {
    pub fn job_change(
        &mut self,
        miner_id: &str,
        seed_hash: &str,
        blob: &str,
        job_id: &str,
        target: &str,
    ) {
        info!("job change, blob {}", blob);

        if seed_hash != self.vm_memory_seed {
            let mem_init_start = Instant::now();
            self.vm_memory = Arc::new(VmMemory::full(&byte_string::string_to_u8_array(seed_hash)));
            self.vm_memory_seed = seed_hash.to_string();
            info!(
                "memory init took {}ms with seed_hash: {}",
                mem_init_start.elapsed().as_millis(),
                seed_hash
            );
        }

        let nonce = Arc::new(AtomicU32::new(0));

        for (_, tx) in self.thread_chan.iter().enumerate() {
            tx.send(WorkerCmd::NewJob {
                job_data: JobData {
                    miner_id: miner_id.to_string(),
                    seed_hash: seed_hash.to_string(),
                    memory: self.vm_memory.clone(),
                    blob: blob.to_string(),
                    job_id: job_id.to_string(),
                    target: target.to_string(),
                    nonce: nonce.clone(),
                },
            })
            .expect("sending new job command");
        }
    }

    pub fn stop(&self) {
        info!("stopping workers");

        for tx in &self.thread_chan {
            let _ = tx.send(WorkerCmd::Stop);
        }
    }

    //Waits for completing of all threads in the pool
    pub fn join(self) {
        for hnd in self.thread_hnd {
            let join_result = hnd.join();
            if join_result.is_err() {
                error!("thread join failed {:?}, waiting for next", join_result)
            }
        }
    }
}

fn work(
    rcv: &Receiver<WorkerCmd>,
    share_tx: &Sender<stratum::StratumCmd>,
    metric_resolution: u64,
    metric_tx: &Sender<u64>,
) {
    let first_job = rcv.recv();
    if first_job.is_err() {
        error!("job channel was dropped");
        return;
    }
    let mut job = match first_job.unwrap() {
        WorkerCmd::NewJob { job_data } => job_data,
        WorkerCmd::Stop => {
            info!("Worker immediately stopped");
            return;
        }
    };

    loop {
        let exit_reason = work_job(&job, rcv, share_tx, metric_resolution, metric_tx);
        //if work_job returns the nonce space was exhausted or a new job was received.
        //In case the nonce space was exhausted, we have to wait blocking for a new job and "idle".
        match exit_reason {
            WorkerExit::NonceSpaceExhausted => {
                warn!("nonce space exhausted, thread idle");
                let job_blocking = rcv.recv();
                if job_blocking.is_err() {
                    error!("job channel was dropped");
                    return;
                }
                job = match job_blocking.unwrap() {
                    WorkerCmd::NewJob { job_data } => job_data,
                    WorkerCmd::Stop => break, //Terminate thread
                };
            }
            WorkerExit::NewJob { job_data } => {
                job = job_data;
            }
            WorkerExit::Stopped => break, //Terminate thread
        }
    }

    info!("Worker stopped")
}

fn work_job<'a>(
    job: &'a JobData,
    rcv: &'a Receiver<WorkerCmd>,
    share_tx: &Sender<stratum::StratumCmd>,
    metric_resolution: u64,
    metric_tx: &Sender<u64>,
) -> WorkerExit {
    let num_target = job_target_value(&job.target);
    let mut nonce = job.nonce.fetch_add(1, Ordering::SeqCst);

    let mut hash_count: u64 = 0;
    let mut vm = new_vm(job.memory.clone());

    while nonce <= 65535 {
        let nonce_hex = nonce_hex(nonce);
        let hash_in = with_nonce(&job.blob, &nonce_hex);
        let bytes_in = byte_string::string_to_u8_array(&hash_in);

        let hash_result = vm.calculate_hash(&bytes_in).to_hex();
        let hash_val = hash_target_value(&hash_result);

        if hash_val < num_target {
            let share = stratum_data::Share {
                miner_id: job.miner_id.clone(),
                job_id: job.job_id.clone(),
                nonce: nonce_hex,
                hash: hash_result.to_string(),
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

        let cmd = check_command_available(rcv);
        if let Some(cmd_value) = cmd {
            match cmd_value {
                WorkerCmd::NewJob { job_data } => {
                    let send_result = metric_tx.send(hash_count);
                    if send_result.is_err() {
                        //flush hash_count
                        error!("metric submit failed {:?}", send_result);
                    }
                    return WorkerExit::NewJob { job_data };
                }
                WorkerCmd::Stop => return WorkerExit::Stopped,
            }
        }

        nonce = job.nonce.fetch_add(1, Ordering::SeqCst);
    }
    WorkerExit::NonceSpaceExhausted
}

pub fn nonce_hex(nonce: u32) -> String {
    format!("{:08x}", nonce)
}

pub fn with_nonce(blob: &str, nonce: &str) -> String {
    let (a, _) = blob.split_at(78);
    let (_, b) = blob.split_at(86);
    return format!("{}{}{}", a, nonce, b);
}

fn check_command_available(rcv: &Receiver<WorkerCmd>) -> Option<WorkerCmd> {
    let try_result = rcv.try_recv();
    match try_result {
        Ok(cmd) => Some(cmd),
        _ => None,
    }
}

pub fn job_target_value(hex_str: &str) -> u64 {
    let t = byte_string::hex2_u32_le(hex_str);
    u64::max_value() / (u64::from(u32::max_value()) / u64::from(t))
}

pub fn hash_target_value(hex_str: &str) -> u64 {
    byte_string::hex2_u64_le(&hex_str[48..])
}
