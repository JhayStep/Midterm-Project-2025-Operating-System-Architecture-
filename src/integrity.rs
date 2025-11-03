use sha2::{Digest, Sha256};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

pub fn hash_i64(v: i64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(v.to_le_bytes());
    let out = hasher.finalize();
    hex_encode(&out)
}
