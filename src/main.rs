extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;
use mithril::stratum::stratum_data;

use mithril::stratum::stratum::{StratumClient};

use std::net::TcpStream;

fn main() {

    //Real impl test

    println!("Doing client login");
    let mut client = StratumClient::new();
    client.login();
    client.join();

    return;

    let stream = TcpStream::connect("mine.moneropool.com:3335").unwrap();

    let r = stratum_data::login(&stream).unwrap();
    let blob = r.result.job.blob;
    let target = r.result.job.target;

    println!("target received {:}", target);
    println!("blob received {:}", blob);

    let mut b = byte_string::string_to_u8_array(&blob);
    let num_target = stratum_data::target_u64(byte_string::hex2_u32_le(&target));
    println!("num_target={:}", num_target);


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
}
