use crate::error::ApplicationError;
use sha2::{Digest, Sha256};
use base64_url;
use base64;
use std::str;
// use openssl::rsa::{Rsa, Padding}; do not work with AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR

pub struct CriptoHelper;

impl CriptoHelper {
  pub fn from_base64(input: String) -> Result<String, ApplicationError> {
    let bytes = base64::decode(input)?;
    let result = str::from_utf8(&bytes)?;
    Ok(result.to_owned())
  }

  pub fn to_base64<T: AsRef<[u8]>>(input: T) -> String {
    return base64_url::encode(&input);
  }

  pub fn to_sha256_string<T: AsRef<[u8]>>(input: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    return CriptoHelper::to_base64(result);
  }

  pub fn random_bytes(length: usize) -> Vec<u8> {
    let random_bytes: Vec<u8> = (0..length).map(|_| { rand::random::<u8>() }).collect();
    return random_bytes;
  }

  // pub fn rsa_decrypt(data: &Vec<u8>, private_key: String) -> Vec<u8> {
  //   // Decrypt with private key
  //   let rsa = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
  //   let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
  //   let _ = rsa.private_decrypt(&data, &mut buf, Padding::PKCS1_OAEP).unwrap();
  //   return buf;
  // }
}
