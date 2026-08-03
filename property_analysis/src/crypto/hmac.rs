use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn sign(data: &[u8], secret: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");

    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

pub fn verify(data: &[u8], secret: &[u8], signature: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");

    mac.update(data);
    mac.verify_slice(signature).is_ok()
}
