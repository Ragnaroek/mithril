
pub fn string_to_u8_array(hex: String) -> Vec<u8> {
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

pub fn u8_array_to_string(a: [u8; 200]) -> String {
    let mut str = String::new();
    for i in 0..200 {
        str.push_str(&format!("{:02x}", a[i]));
    }
    return str;
}
