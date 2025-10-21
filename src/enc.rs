use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
    aead::{Aead, OsRng, consts::U12},
};

use sha2::{Digest, Sha256};

pub fn encrypt(data: &[u8], passwd: &str) -> Result<Vec<u8>, String> {
    let key: Key<Aes256Gcm> = get_hash(passwd); // 密码的哈希值
    let cipher: Aes256Gcm = Aes256Gcm::new(&key); // 加密器

    let nonce: Nonce<U12> = Aes256Gcm::generate_nonce(&mut OsRng);
    let data_enc: Vec<u8> = cipher.encrypt(&nonce, data).map_err(|e| e.to_string())?;

    Ok([nonce.to_vec(), data_enc].concat())
}

pub fn decrypt(data: &[u8], passwd: &str) -> Result<Vec<u8>, String> {
    let key: Key<Aes256Gcm> = get_hash(passwd);
    let cipher: Aes256Gcm = Aes256Gcm::new(&key);

    let (nonce, data) = (&data[0..12], &data[12..]);
    let nonce: &Nonce<U12> = Nonce::<U12>::from_slice(nonce);
    cipher.decrypt(nonce, data).map_err(|e| e.to_string())
}

fn get_hash(x: &str) -> Key<Aes256Gcm> {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(x);
    let key: Key<Aes256Gcm> = hasher.finalize();
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSWD: & 'static str = "2345vor";
    const DATA_PATH: & 'static str = "./color.jpeg";
    const DEC_DATA_PATH: &'static str = "./color_dec.jpeg";

    #[test]
    fn test_enc_dec() {
        let data = std::fs::read(DATA_PATH).unwrap();
        let enc_data = encrypt(&data, PASSWD).unwrap();
        let dec_data = decrypt(&enc_data, PASSWD).unwrap();
        std::fs::write(DEC_DATA_PATH, &dec_data).unwrap();
    }
}
