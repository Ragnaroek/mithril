extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;
use mithril::stratum::stratum;

fn main() {

    let r = stratum::test().unwrap();
    let blob = r.result.job.blob;
    let target = r.result.job.target;

    //let target = "169f0200";
    //let blob = "0606b99abece05661f0dd0a01298403dd9e4333c586dc31ec092b1c16c4135b4dabc4fff5d0a1000000000f316205a032da41df350a954cfb37931090426c1437acf84c0d4a4a2b909d98f03";

    println!("target received {:}", target);
    println!("blob received {:}", blob);

    let mut b = byte_string::string_to_u8_array(&blob);
    let num_target = stratum::target_u64(byte_string::hex2_u32_le(&target));
    println!("num_target={:}", num_target);

    for k in 0..u8::max_value() {
        for i in 0..u8::max_value() {
            for j in 0..u8::max_value() {
                for g in 0..u8::max_value() {
                    b[39] = k;
                    b[40] = i;
                    b[41] = j;
                    b[42] = g;

                    let hash_result = hash::hash(&b);
                    let hash_val = byte_string::hex2_u64_le(&hash_result[48..]);

                    if hash_val < num_target {
                        println!("found share {:?} {:?}", hash_result, hash_val);
                    }
                }
            }
        }
    }
}
