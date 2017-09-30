extern crate mithril;

use mithril::byte_string;
use mithril::cryptonight::hash;
use mithril::stratum::stratum;

fn main() {

    //let r = stratum::test().unwrap();
    //let blob = r.result.job.blob;
    //let target = r.result.job.target;

    let target = "169f0200";
    let blob = "0606b99abece05661f0dd0a01298403dd9e4333c586dc31ec092b1c16c4135b4dabc4fff5d0a1000000000f316205a032da41df350a954cfb37931090426c1437acf84c0d4a4a2b909d98f03";

    //TODO parse target to u64

    println!("target received {:}", target);
    println!("blob received {:}", blob);

    let mut b = byte_string::string_to_u8_array(&blob);
    println!("b.len {:}", b.len());
    println!("b[39]={:}", b[39]);

    //TODO hash until we find a value that is smaller than target_u64
    //look at value in hash (result.bResult + 24) u64 ;
    // uint8_t		bResult[32];
    //TODO WHY WHY WHY at pos 24 ??

    //let input = byte_string::string_to_u8_array("54686973206973206120746573743636");
    //let result = hash::hash(&input);
    //println!("result={}", result);

}
