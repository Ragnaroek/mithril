pub mod stratum_data;

extern crate serde;
extern crate serde_json;

use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender, SendError};
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::io::{BufReader, BufRead, BufWriter, Write, Error};
use std::time::{Duration};

/// command send to the stratum server
#[derive(Debug)]
pub enum StratumCmd {
    Login {},
    SubmitShare{
        share: stratum_data::Share
    },
    KeepAlive{
        miner_id: String
    },
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
    KeepAliveOk,
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
    miner_id: Arc<Mutex<Option<String>>>,
    err_receiver: Sender<Error>,
}

/// All operation in the client are async
impl StratumClient {
    pub fn new(pool_conf: stratum_data::PoolConfig, err_receiver: Sender<Error>, action_rcvs: Vec<Sender<StratumAction>>) -> StratumClient {
        StratumClient{
            is_init: false,
            tx_cmd : Option::None,
            send_thread: Option::None,
            rcv_thread: Option::None,
            action_rcvs,
            pool_conf,
            miner_id: Arc::new(Mutex::new(Option::None)),
            err_receiver
        }
    }

    fn init(self: &mut Self) {

        let stream = TcpStream::connect(self.pool_conf.clone().pool_address).unwrap();
        stream.set_read_timeout(None).unwrap();
        stream.set_write_timeout(Some(Duration::from_secs(10))).unwrap();

        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);

        let (tx, rx) = channel();

        let pool_conf = self.pool_conf.clone();
        let err_send_tx = self.err_receiver.clone();
        let send_thread = thread::Builder::new().name("Stratum send thread".to_string()).spawn(move || {
            let result = handle_stratum_send(&rx, writer, &pool_conf);
            if result.is_err() {
                err_send_tx.send(result.err().unwrap()).expect("sending error in send thread");
            }
        }).expect("Stratum send thread handle");

        self.send_thread = Option::Some(send_thread);

        let rcvs = self.action_rcvs.clone();
        let rcv_miner_id = self.miner_id.clone();
        let err_rcv_tx = self.err_receiver.clone();
        let rcv_thread = thread::Builder::new().name("Stratum receive thread".to_string()).spawn(move || {
            let result = handle_stratum_receive(reader, &rcvs, &rcv_miner_id);
            if result.is_err() {
                err_rcv_tx.send(result.err().unwrap()).expect("sending error in recv thread");
            }
        }).expect("Stratum received thread handle");
        self.rcv_thread = Option::Some(rcv_thread);

        //keep alive check thread
        let cmd_alive = tx.clone();
        let alive_miner_id = self.miner_id.clone();
        thread::Builder::new().name("keep alive thread".to_string()).spawn(move || {
            loop {

                thread::sleep(Duration::from_secs(60));

                let miner_id_guard = &*alive_miner_id.lock().unwrap();
                if miner_id_guard.is_some() {
                    let miner_id = miner_id_guard.clone().unwrap();
                    cmd_alive.send(StratumCmd::KeepAlive{miner_id}).expect("KeepAlive send failed");
                }
            }
        }).expect("keep alive thread handle");

        self.tx_cmd = Option::Some(tx);
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
        Err("Internal error, tx_clone.unwrap() failed although init was called".to_string())
    }
}

pub fn submit_share(tx: &Sender<StratumCmd>, share: stratum_data::Share) -> Result<(), SendError<StratumCmd>> {
    info!("submitting share: {:?}", share);
    tx.send(StratumCmd::SubmitShare{share})
}

fn handle_stratum_send(rx: &Receiver<StratumCmd>, mut writer: BufWriter<TcpStream>, pool_conf: &stratum_data::PoolConfig) -> Result<(), Error> {
    loop {
        match rx.recv().expect("stratum receiver") {
            StratumCmd::Login{} => do_stratum_login(&mut writer, pool_conf)?,
            StratumCmd::SubmitShare{share} => do_stratum_submit_share(&mut writer, share)?,
            StratumCmd::KeepAlive{miner_id} => do_stratum_keep_alive(&mut writer, miner_id)?,
        }
    }
}

fn do_stratum_keep_alive(writer: &mut BufWriter<TcpStream>, miner_id: String) -> Result<(), Error> {
    let keep_alive_req = stratum_data::KeepAliveRequest{
        id: 1,
        method: "keepalived".to_string(),
        params: stratum_data::KeepAliveParams {
            id: miner_id
        }
    };

    let json = serde_json::to_string(&keep_alive_req).unwrap();
    write!(writer, "{}\n", json)?;
    writer.flush().unwrap();
    Ok(())
}

fn do_stratum_submit_share(writer: &mut BufWriter<TcpStream>, share: stratum_data::Share) -> Result<(), Error> {
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
    write!(writer, "{}\n", json)?;
    writer.flush().unwrap();
    Ok(())
}

fn do_stratum_login(writer: &mut BufWriter<TcpStream>, pool_conf: &stratum_data::PoolConfig) -> Result<(), Error> {
    let login_req = stratum_data::LoginRequest {
        id: 1,
        method: "login".to_string(),
        params: stratum_data::LoginParams {
            login: pool_conf.wallet_address.clone(),
            pass: pool_conf.pool_password.clone()
        }
    };
    let json = serde_json::to_string(&login_req).unwrap();
    write!(writer, "{}\n",json)?;
    writer.flush().unwrap();
    Ok(())
}

fn handle_stratum_receive(mut reader: BufReader<TcpStream>, rcvs: &[Sender<StratumAction>], miner_id: &Arc<Mutex<Option<String>>>) -> Result<(), Error> {
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => {
                parse_line_dispatch_result(&line, rcvs, miner_id);
            },
            Err(e) => {
                //read_line fails (maybe connection lost, dispatch err to channel)
                //=> Terminate loop
                return Err(e);
            }
        };
    }
}

fn is_known_ok(result: Result<stratum_data::OkResponse, serde_json::Error>) -> Option<StratumAction> {
    if result.is_ok() {
        let unwrapped = result.unwrap();
        if unwrapped.result.status == "OK" && unwrapped.result.id.is_none() {
            return Some(StratumAction::Ok);
        } else if unwrapped.result.status == "KEEPALIVED" && unwrapped.result.id.is_none() {
            return Some(StratumAction::KeepAliveOk);
        }
    }
    None
}

//TODO Refactor this method (it is very ugly) - its probably better to use generic value parsing and not using struct for every case
pub fn parse_line_dispatch_result(line: &str, rcvs: &[Sender<StratumAction>], miner_id_mutx: &Arc<Mutex<Option<String>>>) {

    let action;

    let error : Result<stratum_data::ErrorResult, serde_json::Error> = serde_json::from_str(line);
    if error.is_ok() {
        match error.unwrap() {
            stratum_data::ErrorResult{error: err_details} => {
                action = StratumAction::Error{err: format!("error received: {} (code {}, raw json {})", err_details.message, err_details.code, line)}
            }
        }
    } else {
        let ok_result : Result<stratum_data::OkResponse, serde_json::Error> = serde_json::from_str(line);
        let known_ok = is_known_ok(ok_result);
        if known_ok.is_some() {
            action = known_ok.unwrap();
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
                    Ok(stratum_data::LoginResponse{result: stratum_data::LoginResult{status, job: stratum_data::Job{blob, job_id, target}, id: miner_id}, .. })
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
            StratumAction::Job{miner_id, blob, job_id, target}
        },
        _ => StratumAction::Error{err: "Error parsing job response".to_string()}
    }
}
