
//extern crate tiny_keccak;
//use tiny_keccak::Keccak;

pub mod keccak;

fn main() {
    //let mut hasher = Keccak::new_sha3_512();

    let input = string_to_u8_array(&"5597a4dbcb54a459817bc0eacbc34b6b36d5c94c1524d2799b60c0a3e25cdd4be06bd32b0c00a8cba052ece2e7c9f84c21e6449d474051e5ff4fb9ebec473414134616a48665".to_string());
    println!("input: {:x}{:x}{:x}{:x}", input[0],input[1],input[2],input[3]);
    let a = keccak::keccak(&input);

    //println!("input {:x}", input[0]);
    //println!("input {:x}", input[1]);

    //hasher.update(&input);
    //let mut res : [u8; 200] = [0; 200];
    //hasher.finalize(&mut res);

    println!("len: {:?}", a.len());
    println!("{:x}{:x}{:x}", a[0], a[1], a[2]);

    //output:
    //4f2c6e752588279687af5229115c837a86b85483921fbd6
    //fc81a7a8536d64217163b838670d26053d185fc68709b96f8f8bf5fff0b2b8dd912ef2e9a6f17169b44123986949aa1f2be674cb9497cda05571457728afa9af94b1e7e8fe2e0bd8d1be49e3cf62ffae2b91a0e4a073219480d6a6fa3f967ff45c838e49718946a2fe8a69bbd465c3138a92030cc814aa24ef58f3f141223c9410bbbe9f3b5c50598c825a79bad8b54cd2ead48d40e019b6aabcd9d796d7aa23b67fc91e97e497451fabad80404ebd925238d68a5d6738342e8dceafba92aee359d
}

//source: http://blog.nella.org/my-first-ever-rust-program/
fn string_to_u8_array(hex: &String) -> Vec<u8> {
    // Make vector of bytes from octets
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => {
                println!("Problem with hex: {}", e);
                return bytes;
            }
        };
    }
    return bytes;
}
