
//extern crate tiny_keccak;
//use tiny_keccak::Keccak;

pub mod keccak;

fn main() {
    //let mut hasher = Keccak::new_sha3_512();

    println!("{:02x}", 5);

    let input = string_to_u8_array("05059a8ce9cb05e7ca1d7fcc5ba3d697889fb5ee874ea5ba9528d88cb64ca7d2ee8240e20b25b31b0000009a1d8e009b431f0901a085b1be8b4a8a83ad4e1d75a5e1205b7d94085071bafc05".to_string());
    let a = keccak::keccak(&input);
    println!("len: {:?}", a.len());

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
