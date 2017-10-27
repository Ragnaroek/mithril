use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct WorkerPool {
    thread_chan : Vec<Sender<WorkerCmd>>
}

pub struct WorkerConfig {
    pub num_threads: u64
}

pub enum WorkerCmd {
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
    return WorkerPool{thread_chan};
}

//TODO pub fn job_change() //change work to new job
//TODO pub fn stop() //stop all workers, for controlled shutdown

fn work(rcv: Receiver<WorkerCmd>) {
    //TODO Wait blocking for first job
    //TODO check for job change after each job (is try_recv expensive?)
}
