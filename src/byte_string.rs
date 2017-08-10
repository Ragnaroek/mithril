
use cryptonight::aes::u64x2;

pub fn string_to_u8_array(hex: &str) -> Vec<u8> {
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

pub fn u8_array_to_string(a: &[u8]) -> String {
    let mut str = String::new();
    for i in 0..a.len() {
        str.push_str(&format!("{:02x}", a[i]));
    }
    return str;
}

pub fn u128_to_string(u: u128) -> String {
    return format!("{:016x}", u);
}

pub fn u64x2_to_string(u: u64x2) -> String {
    return format!("{:08x}{:08x}", u.1, u.0);
}
