extern crate crossbeam_channel;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::worker::worker_metrics::HashCompletionReport;

use self::crossbeam_channel::{unbounded, Receiver, Sender};
use super::super::byte_string;
use super::super::randomx::memory::{VmMemory, VmMemoryAllocator};
use super::super::randomx::vm::new_vm;
use super::super::stratum;
use super::super::stratum::stratum_data;

pub struct WorkerPool {
    thread_chan: Vec<Sender<WorkerCmd>>,
    thread_hnd: Vec<thread::JoinHandle<()>>,
    pub vm_memory_allocator: VmMemoryAllocator,
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
    metric_sndr: &Sender<HashCompletionReport>,
    vm_memory_allocator: VmMemoryAllocator,
) -> WorkerPool {
    let mut thread_chan: Vec<Sender<WorkerCmd>> = Vec::with_capacity(num_threads as usize);
    let mut thread_hnd: Vec<thread::JoinHandle<()>> = Vec::with_capacity(num_threads as usize);
    for thread_id in 0..num_threads {
        let (sndr, rcvr) = unbounded();
        let share_sndr_thread = share_sndr.clone();
        let metric_sndr_thread = metric_sndr.clone();

        let hnd = thread::Builder::new()
            .name(format!("worker thread {}", thread_id))
            .spawn(move || work(&rcvr, &share_sndr_thread, &metric_sndr_thread, thread_id))
            .expect("worker thread handle");
        thread_chan.push(sndr);
        thread_hnd.push(hnd);
    }
    WorkerPool {
        thread_chan,
        thread_hnd,
        vm_memory_allocator,
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
        self.vm_memory_allocator.reallocate(seed_hash.to_string());
        let nonce = Arc::new(AtomicU32::new(0));

        for (_, tx) in self.thread_chan.iter().enumerate() {
            tx.send(WorkerCmd::NewJob {
                job_data: JobData {
                    miner_id: miner_id.to_string(),
                    seed_hash: seed_hash.to_string(),
                    memory: self.vm_memory_allocator.vm_memory.clone(),
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
    metric_tx: &Sender<HashCompletionReport>,
    thread_id: u64,
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
        let exit_reason = work_job(&job, rcv, share_tx, metric_tx, thread_id);
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

    info!("Worker stopped (thread {thread_id}");
}

fn work_job<'a>(
    job: &'a JobData,
    rcv: &'a Receiver<WorkerCmd>,
    share_tx: &Sender<stratum::StratumCmd>,
    metric_tx: &Sender<HashCompletionReport>,
    thread_id: u64,
) -> WorkerExit {
    let num_target = job_target_value(&job.target);
    let mut nonce = job.nonce.fetch_add(1, Ordering::SeqCst);

    let mut local_hash_count: u64 = 0;
    let mut last_hash_report: Instant = Instant::now();
    let hash_report_interval: Duration = Duration::from_secs_f64(0.5);

    let mut vm = new_vm(job.memory.clone());

    let mut blob_bytes = byte_string::string_to_u8_array(&job.blob);

    while nonce < u32::MAX {
        // Write nonce directly into blob_bytes at the fixed byte offset (bytes 39–42).
        write_nonce_into_blob(&mut blob_bytes, nonce);

        let hash_result = vm.calculate_hash(&blob_bytes);
        let hash_val = hash_to_target_comparable_value(hash_result.as_array());

        if hash_val < num_target {
            // Only allocate the nonce hex string on the (rare) share-found path.
            let nonce_hex = byte_string::u32_to_hex(nonce);
            let share = stratum_data::Share {
                miner_id: job.miner_id.clone(),
                job_id: job.job_id.clone(),
                nonce: nonce_hex.clone(),
                hash: hash_result.to_hex().to_string(),
            };

            info!(
                "Share found in thread {thread_id}: nonce={nonce_hex}, hash={}",
                hash_result.to_hex().to_string()
            );

            let submit_result = stratum::submit_share(share_tx, share);
            if submit_result.is_err() {
                error!("submitting share failed: {:?}", submit_result);
            }
        }

        local_hash_count += 1;
        if local_hash_count > 15 || last_hash_report.elapsed() > hash_report_interval {
            let send_result = metric_tx.send(HashCompletionReport {
                at: Instant::now(),
                count: local_hash_count,
            });
            if send_result.is_err() {
                error!("metric submit failed {:?}", send_result);
            }

            debug!(
                "Reported hash_count: {}, last report: {:?} ago",
                local_hash_count,
                last_hash_report.elapsed()
            );

            local_hash_count = 0;
            last_hash_report = Instant::now();
        }

        let cmd = check_command_available(rcv);
        if let Some(cmd_value) = cmd {
            match cmd_value {
                WorkerCmd::NewJob { job_data } => {
                    let send_result = metric_tx.send(HashCompletionReport {
                        at: Instant::now(),
                        count: local_hash_count,
                    });
                    if send_result.is_err() {
                        error!("metric submit failed {:?}", send_result);
                    }
                    return WorkerExit::NewJob { job_data };
                }
                WorkerCmd::Stop => return WorkerExit::Stopped,
            }
        }

        nonce = job.nonce.fetch_add(1, Ordering::SeqCst); // TODO: Assign separate nonce range per thread to avoid atomic stalls.
    }
    WorkerExit::NonceSpaceExhausted
}

/// Writes `nonce` as 4 little-endian bytes directly into the pre-decoded blob buffer.
/// The nonce occupies hex chars 78–85 in the original blob string, i.e. bytes 39–42.
#[inline(always)]
fn write_nonce_into_blob(blob: &mut [u8], nonce: u32) {
    let bytes: [u8; 4] = nonce.to_le_bytes();
    blob[39] = bytes[0];
    blob[40] = bytes[1];
    blob[41] = bytes[2];
    blob[42] = bytes[3];
}

fn check_command_available(rcv: &Receiver<WorkerCmd>) -> Option<WorkerCmd> {
    match rcv.try_recv() {
        Ok(cmd) => Some(cmd),
        _ => None,
    }
}

pub fn job_target_value(hex_str: &str) -> u64 {
    let t = byte_string::hex2_u32_le(hex_str);
    u64::max_value() / (u64::from(u32::max_value()) / u64::from(t))
}

/// Convert the hash value to a value that can be compared with the target value.
#[inline(always)]
pub fn hash_to_target_comparable_value(hash_val: &[u8]) -> u64 {
    u64::from_le_bytes(hash_val[24..32].try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_nonce() {
        let blob_hex = "0606cbe692d005ecfebc7d2249d2b43535c237c02359e888b8b05d2e980c1405779241ac3ab48500000000e62a06e71559c98a37e7b6743465f4f72e42784c5719411c935dc002e347826b05";
        let nonce_hex = "12345678";

        let mut blob = byte_string::string_to_u8_array(blob_hex);
        let nonce = byte_string::hex2_u32_le(nonce_hex);

        write_nonce_into_blob(&mut blob, nonce);

        assert_eq!(byte_string::u8_array_to_string(&blob),
            "0606cbe692d005ecfebc7d2249d2b43535c237c02359e888b8b05d2e980c1405779241ac3ab48512345678e62a06e71559c98a37e7b6743465f4f72e42784c5719411c935dc002e347826b05");
    }

    #[test]
    fn test_hash_to_target_comparable_value() {
        let input_hex = "c5c49db95a9da3f0802a34c6f97c364e7455fca7e41f72254fd4624dd2f91578";
        let input_bytes: [u8; 32] = byte_string::string_to_u8_array(input_hex)
            .try_into()
            .unwrap();

        assert_eq!(
            hash_to_target_comparable_value(&input_bytes),
            0x7815f9d24d62d44f
        );
    }

    #[test]
    fn test_job_target_value() {
        assert_eq!(job_target_value("8b4f0100"), 368934881474191);
    }
}
