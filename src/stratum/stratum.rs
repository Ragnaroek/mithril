extern crate serde;
extern crate serde_json;

use super::stratum_data;
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
    Job {
        blob: String,
        job_id: String,
        target: String
    },
    Error{
        err: String
    }
}

//{"jsonrpc":"2.0","method":"job","params":{"blob":"0606fcb29bcf051b9c7bfc
//60c98885de404ef48f721f09b8f51d37faf280470880bd120d4e9e0500000000577192c076fed53a24372bc43a3bed1d448a061ad06
//a262ac5e7f6803a28ccc705","job_id":"878440772206522","target":"169f0200"}}


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
            Ok(_) => parse_line_dispatch_result(&line, &rcvs),
            Err(e) => println!("read_line error: {:?}", e), //TODO Err handling??
        };
    }
}

pub fn parse_line_dispatch_result(line: &str, rcvs: &Vec<Sender<StratumAction>>) {
    let result : Result<stratum_data::Method, serde_json::Error> = serde_json::from_str(line);
    let action;
    if result.is_ok() {
        match result.unwrap() {
            stratum_data::Method{method} => {
                match method.as_ref() {
                    "job" => action = parse_job(line),
                    _ => action = StratumAction::Error{err: format!("unknown method received: {}", method)}
                }
            },
            _ => action = StratumAction::Error{err: format!("unknown method received")}
        }
    } else {
        //try parsing intial job
        let initial : Result<stratum_data::LoginResponse, serde_json::Error> = serde_json::from_str(line);
        match initial {
            Ok(stratum_data::LoginResponse{id: _, result: stratum_data::LoginResult{status, job: stratum_data::Job{blob, job_id, target}, id: _}})
                => {
                      if status == "OK" {
                          action = StratumAction::Job{blob, job_id, target}
                      } else {
                          action = StratumAction::Error{err: format!("Not OK initial job received, status was {}", status)}
                      }
                   },
            Err(e) => action = StratumAction::Error{err: format!("error: {:?}", e)}
        }
    }

    for rcv in rcvs {
        let result = rcv.send(action.clone());
    }
}

fn parse_job(line: &str) -> StratumAction {
    let result : Result<stratum_data::JobResponse, serde_json::Error> = serde_json::from_str(line);
    match result {
        Ok(stratum_data::JobResponse{params: stratum_data::Job{blob, job_id, target}}) => {
            return StratumAction::Job{blob, job_id, target};
        },
        _ => return StratumAction::Error{err: "Error parsing job response".to_string()}
    }
}
