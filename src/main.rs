extern crate mithril;
extern crate config;

//use mithril::byte_string;
//use mithril::cryptonight::hash;
use mithril::stratum::stratum_data::{PoolConfig};
use mithril::stratum::stratum::{StratumClient, StratumAction};
use mithril::worker::worker_pool;
use mithril::worker::worker_pool::{WorkerConfig};
use std::sync::mpsc::{channel};
use std::path::Path;
use config::{Config, ConfigError, File};

const CONFIG_FILE_NAME : &'static str = "config.toml";

fn main() {

    //Read config
    let config = read_config().unwrap();
    let pool_conf = pool_config(&config).unwrap();
    let worker_conf = worker_config(&config).unwrap();


    //Stratum start
    let (stratum_tx, stratum_rx) = channel();

    println!("Doing client login");
    let mut client = StratumClient::new(pool_conf, vec![stratum_tx]);
    client.login();

    let share_tx = client.new_cmd_channel().unwrap();

    //worker pool start
    let pool = &worker_pool::start(worker_conf, share_tx);

    loop {
        let received = stratum_rx.recv();
        if received.is_err() {
            println!("lost connection to stratum client: {:?}", received);
            return
        }
        match received.unwrap() {
            StratumAction::Job{miner_id, blob, job_id, target} => {
                pool.job_change(miner_id, blob, job_id, target);
            },
            StratumAction::Error{err} => {
                println!("received stratum error: {}", err);
            }
        }
    }

    //client.join();
    //return;
//===========================================
/*
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


*/

/*
let k = 0xa5;
let i = 0x01;
let j = 0;
let g = 0;
let mut nonce_b : [u8;4] = [0;4];
nonce_b[0] = k;
nonce_b[1] = i;
nonce_b[2] = j;
nonce_b[3] = g;
println!("nonce_b={:?}", nonce_b);
println!("hex={}", format!("{:02x}{:02x}{:02x}{:02x}", k, i, j, g));

/*
    let stream = TcpStream::connect("mine.moneropool.com:3335").unwrap();

    let r = stratum_data::login(&stream).unwrap();
    let blob = r.result.job.blob;
    let target = r.result.job.target;
*/
    let blob = "0606d8d390cf05d6de7e5bf6b2a6163f2843325a2fa46274e5ca5519cd7f488804af20abf4c57ea5010000907a17af0d0d48f2120a168daea8be20b8d33ba97f088e4f7c70c72813171af206".to_string();
    let target = "169f0200".to_string();
    let nonce = "a5010000".to_string();
    let miner_id = "983879881282337".to_string();
    let job_id = "335799611802212".to_string();


    println!("target received {:}", target);
    println!("blob received {:}", blob);

    let mut b = byte_string::string_to_u8_array(&blob);
    b[39] = 165;
    b[40] = 1;
    b[41] = 0;
    b[42] = 0;
    let num_target = stratum_data::target_u64(byte_string::hex2_u32_le(&target));
    println!("num_target={:}", num_target);

    //manual hash check

    let hash_result = hash::hash(&b);
    let hash_val = byte_string::hex2_u64_le(&hash_result[48..]);
    if hash_val < num_target {
        println!("found share {:?} {:?}", hash_result, hash_val);
        println!("b-hex {:?}", byte_string::u8_array_to_string(&b));
    } else {
        println!("share not found");
    }

    let share = stratum_data::Share{
        miner_id: miner_id,
        job_id: job_id,
        nonce: nonce,
        hash: hash_result
    };

    println!("share {:?}", share);
    stratum_data::submit_share_dummy(share);
//    {"id":1,"method":"submit","params":{"id":"983879881282337","job_id":"335799611802212","nonce":"a5010000","result":"56b5b9a2fa8e65888d48bd98a9289ab8b02478c5e4e8bbb75720a139daf70100"}}
//xmr {"id":1,"method":"submit","params":{"id":"983879881282337","job_id":"335799611802212","nonce":"a5010000","result":"56b5b9a2fa8e65888d48bd98a9289ab8b02478c5e4e8bbb75720a139daf70100"}}

/*
    //nonce permutation
    let mut hashes = 0;
    for k in 0..u8::max_value() {
        for i in 0..u8::max_value() {
            for j in 0..u8::max_value() {
                for g in 0..u8::max_value() {
                    b[39] = k;
                    b[40] = i;
                    b[41] = j;
                    b[42] = g;

                    let hash_result = hash::hash(&b);
                    hashes += 1;

                    if hashes % 1000 == 0 {
                        println!("computed 1000 hashes");
                    }

                    let hash_val = byte_string::hex2_u64_le(&hash_result[48..]);

                    if hash_val < num_target {
                        println!("found share {:?} {:?}", hash_result, hash_val);
                        println!("b-hex {:?}", byte_string::u8_array_to_string(&b));
                        println!("nonce-hex {:?}", format!("{:02x}{:02x}{:02x}{:02x}", k, i, j, g));

                        let share = stratum_data::Share{
                            miner_id: r.result.id.clone(),
                            job_id: r.result.job.job_id.clone(),
                            nonce: format!("{:02x}{:02x}{:02x}{:02x}", k, i, j, g),
                            hash: hash_result
                        };
                        let share_result = stratum_data::submit_share(&stream, share);
                        println!("share submit result {:?}", share_result);
                    }
                }
            }
        }
    }
*/
*/
}

fn pool_config(conf: &Config) -> Result<PoolConfig, ConfigError> {
    let pool_address = conf.get_str("pool.pool_address")?;
    let wallet_address = conf.get_str("pool.wallet_address")?;
    let pool_password = conf.get_str("pool.pool_password")?;
    return Ok(PoolConfig{pool_address, wallet_address, pool_password});
}

fn worker_config(conf: &Config) -> Result<WorkerConfig, ConfigError> {
    let num_threads = conf.get_int("worker.num_threads")?;
    if num_threads <= 0 {
        return Err(ConfigError::Message("num_threads hat to be > 0".to_string()));
    }
    return Ok(WorkerConfig{num_threads: num_threads as u64});
}

fn read_config() -> Result<Config, ConfigError> {

    let cwd_path = &format!("{}{}", "./", CONFIG_FILE_NAME);
    let cwd_config_file = Path::new(cwd_path);
    if cwd_config_file.exists() {
        let mut conf = Config::default();
        conf.merge(File::with_name(CONFIG_FILE_NAME))?;
        return Ok(conf);
    }
    return Err(ConfigError::Message("config file not found".to_string()));
}
