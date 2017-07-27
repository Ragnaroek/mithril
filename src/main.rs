
//extern crate tiny_keccak;
//use tiny_keccak::Keccak;

pub mod keccak;

fn main() {
    //let mut hasher = Keccak::new_sha3_512();

    println!("{:02x}", 5);

    let input = string_to_u8_array("0505dde3e8cb05fe5bbf018a7f44c0d0fe0703e29c774651f7b6f138f7bb91b64ccd19904734751f008000c5f5ec8a51c8f4548ec4f366f532d31bc22c35de4ba58a87d7cb5cab40fb22fb02".to_string());
    let a = keccak::keccak(&input);
    println!("len: {:?}", a.len());
    println!("{:x}{:x}{:x}", a[0], a[1], a[2]);

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
