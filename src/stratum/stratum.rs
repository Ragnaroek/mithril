
use super::stratum_data::{LoginResponse};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;


const DEFAULT_CMD_TIMEOUT : u64 = 100; //ms

//TODO setup two thread => one reading, one writing
//  reading thread handles responses and propagates them to listeners
//  writing thread just writes commands


#[derive(Debug)]
pub enum StratumCmd {
    Login {url: String}
}

pub enum StratumError {
}

pub struct StratumClient {
    tx_cmd: Option<Sender<StratumCmd>>,
    send_thread: Option<thread::JoinHandle<()>>,
    rcv_thread: Option<thread::JoinHandle<()>>
}

/// All operation in the client are async
impl StratumClient {
    pub fn new() -> StratumClient {
        return StratumClient{
            tx_cmd : Option::None,
            send_thread: Option::None,
            rcv_thread: Option::None
        };
    }

    fn init(self: &mut Self) {
        let (tx, rx) = channel();

        let send_thread = thread::spawn(move || {
            handle_stratum_send(rx);
        });
        self.tx_cmd = Option::Some(tx);
        self.send_thread = Option::Some(send_thread);

        let rcv_thread = thread::spawn(move || {
            handle_stratum_receive();
        });
        self.rcv_thread = Option::Some(rcv_thread);

        //TODO create Read and write buffers, TcpStream here
    }

    /// Initialises the StratumClient and performs the login that
    /// returns the first mining job.
    pub fn login(self: &mut Self, url: String) -> () {// Result<LoginResponse, StratumError> {

        //TODO Init socket connection here and move read/writer buffer to threads

        self.init();

        self.tx_cmd.clone().unwrap().send(StratumCmd::Login{url}).unwrap();
        return;
    }

    pub fn join(self: Self) -> () {
        //TODO check send_thread optional
        self.send_thread.unwrap().join().unwrap();
    }
}

fn handle_stratum_send(rx: Receiver<StratumCmd>) -> () {
    let message = rx.recv_timeout(Duration::from_millis(DEFAULT_CMD_TIMEOUT));
    println!("Got: {:?}", message.unwrap());
}

fn handle_stratum_receive() -> () {

}
