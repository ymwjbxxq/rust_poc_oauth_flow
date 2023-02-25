use crate::error::ApplicationError;
use base64::engine::general_purpose;
use base64::Engine;
use base64_url;
use sha2::{Digest, Sha256};
use std::str;

pub struct CriptoHelper;

impl CriptoHelper {
    pub fn from_base64(input: String) -> Result<String, ApplicationError> {
        let bytes = general_purpose::STANDARD.decode(input)?;
        let result = str::from_utf8(&bytes)?;
        Ok(result.to_owned())
    }

    pub fn to_base64<T: AsRef<[u8]>>(input: T) -> String {
        base64_url::encode(&input)
    }

    pub fn to_sha256_string<T: AsRef<[u8]>>(input: T) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        CriptoHelper::to_base64(result)
    }

    pub fn random_bytes(length: usize) -> Vec<u8> {
        let random_bytes: Vec<u8> = (0..length).map(|_| rand::random::<u8>()).collect();
        random_bytes
    }

    // fn decrypt(input: &[u8], private_key_path: &str) -> String {
    //     let private_key_der = fs::read(private_key_path).unwrap();
    //     let private_key = RSAPrivateKey::from_pkcs1_der(&private_key_der).unwrap();
    //     let padding = PaddingScheme::new_pkcs1v15_encrypt();
    //     let decrypted = private_key.decrypt(padding, input).unwrap();
    //     String::from_utf8(decrypted).unwrap()
    // }
}
