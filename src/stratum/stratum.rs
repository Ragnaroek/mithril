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

pub fn parse_login_response(str: &str) -> Result<LoginResponse, serde_json::Error> {
    serde_json::from_str(str)
}

pub fn test() -> Result<LoginResponse, serde_json::Error> {
    println!("connecting");

    let stream = TcpStream::connect("mine.moneropool.com:3335").unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    write!(writer, "{{\"id\": 1, \"method\": \"login\", \"params\": {{\"login\": \"4715wx51k4w2m7dgCcyX9LjYgzBnASbTEJg35r7Wta2b8yxTDTrjpuT8JDhz42oA7Q2dumz7Evb876NbVD8PDLkv4Jqd6Cj\", \"pass\":\"\"}}}}\n").unwrap();
    writer.flush().unwrap();

    let tout = time::Duration::from_millis(100);
    thread::sleep(tout);

    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(length) => println!("read_line ok, length {:?}", length),
        Err(e) => println!("read_line error: {:?}", e),
    };

    return parse_login_response(&line);
}
