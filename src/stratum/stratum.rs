extern crate serde;
extern crate serde_json;

use super::stratum_data;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender, SendError};
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::io::{BufReader, BufRead, BufWriter, Write};

/// command send to the stratum server
#[derive(Debug)]
pub enum StratumCmd {
    Login {},
    SubmitShare{
        share: stratum_data::Share
    }
}

/// something received from the stratum server
#[derive(Debug, Clone, PartialEq)]
pub enum StratumAction {
    Job {
        miner_id: String,
        blob: String,
        job_id: String,
        target: String
    },
    Error{
        err: String
    },
    Ok,
}

pub enum StratumError {
}

pub struct StratumClient {
    is_init: bool,
    tx_cmd: Option<Sender<StratumCmd>>,
    send_thread: Option<thread::JoinHandle<()>>,
    rcv_thread: Option<thread::JoinHandle<()>>,
    action_rcvs: Vec<Sender<StratumAction>>,
    pool_conf: stratum_data::PoolConfig,
    miner_id: Arc<Mutex<Option<String>>>
}

/// All operation in the client are async
impl StratumClient {
    pub fn new(pool_conf: stratum_data::PoolConfig, action_rcvs: Vec<Sender<StratumAction>>) -> StratumClient {
        return StratumClient{
            is_init: false,
            tx_cmd : Option::None,
            send_thread: Option::None,
            rcv_thread: Option::None,
            action_rcvs: action_rcvs,
            pool_conf: pool_conf,
            miner_id: Arc::new(Mutex::new(Option::None))
        };
    }

    fn init(self: &mut Self) {

        let stream = TcpStream::connect(self.pool_conf.clone().pool_address).unwrap();
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);

        let (tx, rx) = channel();

        let pool_conf = self.pool_conf.clone();
        let send_thread = thread::spawn(move || {
            handle_stratum_send(rx, writer, pool_conf);
        });
        self.tx_cmd = Option::Some(tx);
        self.send_thread = Option::Some(send_thread);

        let rcvs = self.action_rcvs.clone();
        let rcv_miner_id = self.miner_id.clone();
        let rcv_thread = thread::spawn(move || {
            handle_stratum_receive(reader, rcvs, rcv_miner_id);
        });
        self.rcv_thread = Option::Some(rcv_thread);
        self.is_init = true;
    }

    /// Initialises the StratumClient and performs the login that
    /// returns the first mining job.
    pub fn login(self: &mut Self) -> () {// Result<LoginResponse, StratumError> {

        info!("stratum client login");

        self.init();

        self.tx_cmd.clone().unwrap().send(StratumCmd::Login{}).unwrap();
        return;
    }

    pub fn join(self: Self) -> () {
        //TODO check send_thread optional
        self.send_thread.unwrap().join().unwrap();
    }

    /// Returns a new channel for sending commands to the stratum client
    pub fn new_cmd_channel(self: Self) -> Result<Sender<StratumCmd>, String> {
        if !self.is_init {
            return Err("stratum client not initialised, call login first".to_string());
        }
        let tx_clone = self.tx_cmd.clone();
        if tx_clone.is_some() {
            return Ok(self.tx_cmd.clone().unwrap());
        }
        return Err("Internal error, tx_clone.unwrap() failed although init was called".to_string());
    }
}

pub fn submit_share(tx: &Sender<StratumCmd>, share: stratum_data::Share) -> Result<(), SendError<StratumCmd>> {
    info!("submitting share: {:?}", share);
    return tx.send(StratumCmd::SubmitShare{share});
}

fn handle_stratum_send(rx: Receiver<StratumCmd>, mut writer: BufWriter<TcpStream>, pool_conf: stratum_data::PoolConfig) -> () {
    loop {
        match rx.recv().unwrap() {
            StratumCmd::Login{} => do_stratum_login(&mut writer, &pool_conf),
            StratumCmd::SubmitShare{share} => do_stratum_submit_share(&mut writer, share)
        }
    }
}

fn do_stratum_submit_share(writer: &mut BufWriter<TcpStream>, share: stratum_data::Share) {
    let submit_req = stratum_data::SubmitRequest{
        id: 1,
        method: "submit".to_string(),
        params: stratum_data::SubmitParams {
            id: share.miner_id,
            job_id: share.job_id,
            nonce: share.nonce,
            result: share.hash
        }
    };
    let json = serde_json::to_string(&submit_req).unwrap();
    write!(writer, "{}\n", json).unwrap();
    writer.flush().unwrap();
}

fn do_stratum_login(writer: &mut BufWriter<TcpStream>, pool_conf: &stratum_data::PoolConfig) {
    //TODO create login json with serde
    write!(writer, "{{\"id\": 1, \"method\": \"login\", \"params\": {{\"login\": \"{}\", \"pass\":\"{}\"}}}}\n",
        pool_conf.wallet_address, pool_conf.pool_password).unwrap();
    writer.flush().unwrap();
}

fn handle_stratum_receive(mut reader: BufReader<TcpStream>, rcvs: Vec<Sender<StratumAction>>, miner_id: Arc<Mutex<Option<String>>>) -> () {
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => parse_line_dispatch_result(&line, &rcvs, &miner_id),
            Err(e) => error!("read_line error: {:?}", e), //TODO Err handling??
        };
    }
}

fn is_generic_ok(result: Result<stratum_data::OkResponse, serde_json::Error>) -> bool {
    if result.is_ok() {
        let unwrapped = result.unwrap();
        return unwrapped.result.status == "OK" && unwrapped.result.id.is_none()
    }
    return false;
}

//TODO Refactor this method (it is very ugly) - its probably better to use generic value parsing and not using struct for every case
pub fn parse_line_dispatch_result(line: &str, rcvs: &Vec<Sender<StratumAction>>, miner_id_mutx: &Arc<Mutex<Option<String>>>) {

    let action;

    let error : Result<stratum_data::ErrorResult, serde_json::Error> = serde_json::from_str(line);
    if error.is_ok() {
        match error.unwrap() {
            stratum_data::ErrorResult{error: err_details} => {
                action = StratumAction::Error{err: format!("error received: {} (code {})", err_details.message, err_details.code)}
            }
        }
    } else {
        let ok_result : Result<stratum_data::OkResponse, serde_json::Error> = serde_json::from_str(line);
        if is_generic_ok(ok_result) {
            action = StratumAction::Ok
        } else {
            let result : Result<stratum_data::Method, serde_json::Error> = serde_json::from_str(line);
            if result.is_ok() {
                match result.unwrap() {
                    stratum_data::Method{method} => {
                        match method.as_ref() {
                            "job" => action = parse_job(line, miner_id_mutx),
                            _ => action = StratumAction::Error{err: format!("unknown method received: {}", method)}
                        }
                    }
                }
            } else {
                //try parsing intial job
                let initial : Result<stratum_data::LoginResponse, serde_json::Error> = serde_json::from_str(line);
                match initial {
                    Ok(stratum_data::LoginResponse{id: _, result: stratum_data::LoginResult{status, job: stratum_data::Job{blob, job_id, target}, id: miner_id}})
                        => {
                              if status == "OK" {
                                  action = StratumAction::Job{miner_id: miner_id.clone(), blob, job_id, target};
                                  let mut miner_id_guard = miner_id_mutx.lock().unwrap();
                                  *miner_id_guard = Option::Some(miner_id.clone());
                              } else {
                                  action = StratumAction::Error{err: format!("Not OK initial job received, status was {}", status)}
                              }
                           },
                    Err(e) => action = StratumAction::Error{err: format!("{:?}, json received {}", e, line)}
                }
            }
        }
    }

    for rcv in rcvs {
        rcv.send(action.clone()).unwrap();
        // TODO Log instead of panic + remove faulty rcv_er
    }
}

fn parse_job(line: &str, miner_id_mutx: &Arc<Mutex<Option<String>>>) -> StratumAction {
    let result : Result<stratum_data::JobResponse, serde_json::Error> = serde_json::from_str(line);
    let miner_id_guard = &*miner_id_mutx.lock().unwrap();

    if miner_id_guard.is_none() {
        return StratumAction::Error{err: "miner_id not available for first mining job (login failed previously, this is a bug)".to_string()}
    }
    let miner_id = miner_id_guard.clone().unwrap();

    match result {
        Ok(stratum_data::JobResponse{params: stratum_data::Job{blob, job_id, target}}) => {
            return StratumAction::Job{miner_id, blob, job_id, target};
        },
        _ => return StratumAction::Error{err: "Error parsing job response".to_string()}
    }
}
