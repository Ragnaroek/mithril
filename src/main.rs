extern crate mithril;

//use mithril::byte_string;
//use mithril::cryptonight::hash;
//use mithril::stratum::stratum_data;

use mithril::stratum::stratum::{StratumClient};
use std::sync::mpsc::{channel};

//use std::net::TcpStream;

fn main() {
    //Real impl test

    //TODO read this from config file (use config crate file)
    let pool_url = "mine.moneropool.com:3335".to_string();

    let (test_tx, test_rx) = channel();

    println!("Doing client login");
    let mut client = StratumClient::new(vec![test_tx]);
    client.login(pool_url);

    loop {
        let received = test_rx.recv();
        println!("main received: {:?}", received);
    }

    //client.join();
    //return;
//===========================================


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
