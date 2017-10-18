
use super::stratum_data::{LoginResponse};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::net::TcpStream;
use std::io::{BufReader, BufRead, BufWriter, Write};

/// command send to the stratum server
#[derive(Debug)]
enum StratumCmd {
    Login {}
}

/// something received from the stratum server
#[derive(Debug, Clone)]
pub enum StratumAction {
    Test {}
}

pub enum StratumError {
}

pub struct StratumClient {
    tx_cmd: Option<Sender<StratumCmd>>,
    send_thread: Option<thread::JoinHandle<()>>,
    rcv_thread: Option<thread::JoinHandle<()>>,
    action_rcvs: Vec<Sender<StratumAction>>,
}

/// All operation in the client are async
impl StratumClient {
    pub fn new(action_rcvs: Vec<Sender<StratumAction>>) -> StratumClient {
        return StratumClient{
            tx_cmd : Option::None,
            send_thread: Option::None,
            rcv_thread: Option::None,
            action_rcvs: action_rcvs,
        };
    }

    fn init(self: &mut Self, url: String) {

        let stream = TcpStream::connect(url).unwrap();
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);

        let (tx, rx) = channel();

        let send_thread = thread::spawn(move || {
            handle_stratum_send(rx, writer);
        });
        self.tx_cmd = Option::Some(tx);
        self.send_thread = Option::Some(send_thread);

        let rcvs = self.action_rcvs.clone();
        let rcv_thread = thread::spawn(move || {
            handle_stratum_receive(reader, rcvs);
        });
        self.rcv_thread = Option::Some(rcv_thread);
    }

    /// Initialises the StratumClient and performs the login that
    /// returns the first mining job.
    pub fn login(self: &mut Self, url: String) -> () {// Result<LoginResponse, StratumError> {

        //TODO Init socket connection here and move read/writer buffer to threads

        self.init(url);

        self.tx_cmd.clone().unwrap().send(StratumCmd::Login{}).unwrap();
        return;
    }

    pub fn join(self: Self) -> () {
        //TODO check send_thread optional
        self.send_thread.unwrap().join().unwrap();
    }
}

fn handle_stratum_send(rx: Receiver<StratumCmd>, mut writer: BufWriter<TcpStream>) -> () {
    loop {
        match rx.recv().unwrap() { //TODO Err handling
            StratumCmd::Login{} => do_stratum_login(&mut writer)
        }
    }
}

fn do_stratum_login(writer: &mut BufWriter<TcpStream>) {

    //TODO create login json with serde
    //TODO take address from config
    write!(writer, "{{\"id\": 1, \"method\": \"login\", \"params\": {{\"login\": \"4715wx51k4w2m7dgCcyX9LjYgzBnASbTEJg35r7Wta2b8yxTDTrjpuT8JDhz42oA7Q2dumz7Evb876NbVD8PDLkv4Jqd6Cj\", \"pass\":\"\"}}}}\n").unwrap();
    writer.flush().unwrap();
}

fn handle_stratum_receive(mut reader: BufReader<TcpStream>, rcvs: Vec<Sender<StratumAction>>) -> () {
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => parse_line_dispatch_result(line, &rcvs),
            Err(e) => println!("read_line error: {:?}", e), //TODO Err handling??
        };
    }
}

fn parse_line_dispatch_result(line: String, rcvs: &Vec<Sender<StratumAction>>) {
    //TODO parse result and notify via listener channel
    println!("received {}", line);

    for rcv in rcvs {
        let result = rcv.send(StratumAction::Test{});
        println!("send result: {:?}", result);
    }
}
