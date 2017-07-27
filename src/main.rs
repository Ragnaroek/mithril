
//extern crate tiny_keccak;
//use tiny_keccak::Keccak;

pub mod keccak;

fn main() {
    //let mut hasher = Keccak::new_sha3_512();

    println!("{:02x}", 5);

    let input = string_to_u8_array("050583f0e5cb050185585032c45c659af5ec4c57cc30c74430b379e357f34e420571a5f55ae5aa2800c0003f3db7b5e62851cca56dec091d319536155cbeedfb37c36c371885311838e7980b".to_string());
    let a = keccak::keccak(&input);

    //println!("input {:x}", input[0]);
    //println!("input {:x}", input[1]);

    //hasher.update(&input);
    //let mut res : [u8; 200] = [0; 200];
    //hasher.finalize(&mut res);

    println!("len: {:?}", a.len());
    println!("{:x}{:x}{:x}", a[0], a[1], a[2]);

    //output:
    //23e61f7a89c55df61b41e44d2ceb0d4214cc7fcdbe66cadc081d092e6a488775f82917926cc5d38f9d13362c84bbd8a2fc5ec44bfff8bc56c7371654e23c3f42c163958b649dca89dc7f309d8570c38d7fa3dbff99ab238066abea6bae2b2a3fd2662fa0272ea9724c571c746c202afc4d8e2b1bc6f49d156c94113207c83fceaeccba8e162597953bb3b89f5c1bb96fbdbc943187912543b222a5e9a7fa4b7863e1786d4492b3a079fd97f6cde24ea55d03dc1e93d5859153cc78fee9fa62b7f3ae62


    //c1f6286319dadd1bdcfccca082ab1972528bc3153688dbe6
}

//source: http://blog.nella.org/my-first-ever-rust-program/
fn string_to_u8_array(hex: String) -> Vec<u8> {
    // Make vector of bytes from octets
    println!("len: {:?}", hex.len());
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
