use u64x2::u64x2;

//TODO Rename to hex2_u8_array
pub fn string_to_u8_array(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => {
                error!("Problem with hex: {}", e);
                return bytes;
            }
        };
    }
    bytes
}

/// Converts the first 8 hex chars of the slice to a u32
/// number. The hex string is interpreted as litte-endian
/// (last two chars are most signifiant)
pub fn hex2_u32_le(hex: &str) -> u32 {
    let mut result : u32 = 0;
    for k in (0..8).step_by(2) {
        let p = u32::from_str_radix(&hex[(8-k-2)..(8-k)], 16).unwrap();
        result <<= 8;
        result |= p;
    }
    result
}

//TODO Write a test
pub fn hex2_u64_le(hex: &str) -> u64 {
    let mut result : u64 = 0;
    for k in (0..hex.len()).step_by(2) {
        let p = u64::from_str_radix(&hex[(hex.len()-k-2)..(hex.len()-k)], 16).unwrap();
        result <<= 8;
        result |= p;
    }
    result
}

pub fn hex2_u64x2_be(hex: &str) -> u64x2 {
    let u1 = hex2_u64_be(&hex[0..16]);
    let u2 = hex2_u64_be(&hex[16..32]);
    u64x2(u2, u1)
}

pub fn hex2_u64_be(hex: &str) -> u64 {
    u64::from_str_radix(hex, 16).unwrap()
}

pub fn u8_array_to_string(a: &[u8]) -> String {
    let mut str = String::new();
    for a_i in a {
        str.push_str(&format!("{:02x}", a_i));
    }
    str
}

pub fn u128_to_string(u: u128) -> String {
    return format!("{:016x}", u);
}

pub fn u64x2_to_string(u: u64x2) -> String {
    return format!("{:016x}{:016x}", u.1, u.0);
}
