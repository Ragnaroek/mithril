extern crate serde;
extern crate serde_json;

use std::net::TcpStream;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::{thread, time};

#[derive(Deserialize)]
pub struct Job {
    pub blob: String,
    pub job_id: String,
    pub target: String,
}

#[derive(Deserialize)]
pub struct LoginResult {
    pub id: String,
    pub job: Job,
    pub status: String
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub id: u32,
    pub result: LoginResult
}

#[derive(Serialize)]
pub struct SubmitParams {
    pub id: String,
    pub job_id: String,
    pub nonce: String,
    pub result: String
}

#[derive(Serialize)]
pub struct SubmitRequest {
    pub id: u32,
    pub method: String,
    pub params: SubmitParams
}

#[derive(Debug)]
pub struct Share {
    pub job_id: String,
    pub nonce: String,
    pub hash: String
}

pub fn parse_login_response(str: &str) -> Result<LoginResponse, serde_json::Error> {
    serde_json::from_str(str)
}

pub fn login(stream: &TcpStream) -> Result<LoginResponse, serde_json::Error> {
    println!("connecting");

    //let stream = TcpStream::connect("mine.moneropool.com:3335").unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);

    write!(writer, "{{\"id\": 1, \"method\": \"login\", \"params\": {{\"login\": \"4715wx51k4w2m7dgCcyX9LjYgzBnASbTEJg35r7Wta2b8yxTDTrjpuT8JDhz42oA7Q2dumz7Evb876NbVD8PDLkv4Jqd6Cj\", \"pass\":\"\"}}}}\n").unwrap();
    writer.flush().unwrap();

    let tout = time::Duration::from_millis(50);
    thread::sleep(tout);

    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(length) => println!("read_line ok, length {:?}", length),
        Err(e) => println!("read_line error: {:?}", e),
    };

    return parse_login_response(&line);
}

//TODO parse result, once we knew how it looks like
pub fn submit_share(stream: &TcpStream, share: Share) -> String {

    println!("submitting share: {:?}", share);

    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);

    let submit_req = SubmitRequest{
        id: 1,
        method: "submit".to_string(),
        params: SubmitParams {
            id: "1".to_string(),
            job_id: share.job_id,
            nonce: share.nonce,
            result: share.hash
        }
    };

    let json = serde_json::to_string(&submit_req).unwrap();
    println!("json={:?}", json);
    write!(writer, "{}\n", json).unwrap();

    let tout = time::Duration::from_millis(50);
    thread::sleep(tout);

    let mut line = "".to_string();
    loop {
        match reader.read_line(&mut line) {
            Ok(length) => println!("read_line ok, length {:?}", length),
            Err(e) => println!("read_line error: {:?}", e),
        };
        println!("read_line: {:?}", line);
        if line == "" {
            break;
        }
        line = "".to_string();
    }

    return line;
}

pub fn target_u64(t: u32) -> u64 {
    return u64::max_value() / (u32::max_value() as u64 / t as u64)
}
