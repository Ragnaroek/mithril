
use super::stratum_data::{LoginResponse};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

pub enum StratumError {

}

pub struct StratumClient {
    tx: Option<Sender<String>>,
    h_thread: Option<thread::JoinHandle<()>>
}

impl StratumClient {
    pub fn new() -> StratumClient {
        return StratumClient{
            tx: Option::None,
            h_thread: Option::None
        };
    }

    /// Initialises the StratumClient and performs the login that
    /// returns the first mining job.
    pub fn login(mut self: Self) -> () {// Result<LoginResponse, StratumError> {

        let (tx, rx) = channel();
        self.tx = Option::Some(tx);
        let h_thread = thread::spawn(move || {
            handle_stratum(rx);
        });
        self.h_thread = Option::Some(h_thread);

        self.tx.unwrap().send("Please do a login.".to_string()).unwrap();

        //TODO create background thread with channel
        //TODO add login job to channel
    }

    pub fn join(self: Self) -> () {
        //TODO check h_thread optional
        self.h_thread.unwrap().join().unwrap();
    }
}

fn handle_stratum(rx: Receiver<String>) -> () {
    let message = rx.recv();
    println!("Got: {}", message.unwrap());
}
