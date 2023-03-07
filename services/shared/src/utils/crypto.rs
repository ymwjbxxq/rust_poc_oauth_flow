use crate::error::ApplicationError;
use base64::{
    engine::general_purpose,
    Engine as _,
};
use base64_url;
use openssl::rsa::{Padding, Rsa};
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

    pub fn decrypt(data: &str, private_key_pem: &str) -> String {
        let rsa = Rsa::private_key_from_pem(private_key_pem.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];

        let bytes = general_purpose::STANDARD.decode(data).unwrap();
        let size = rsa
            .private_decrypt(&bytes, &mut buf, Padding::PKCS1_OAEP)
            .unwrap();

        buf.truncate(size);

        String::from_utf8(buf)
            .unwrap()
    }

    pub fn encrypt(data: &str, public_key_pem: &str) -> Vec<u8> {
        let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa
            .public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1)
            .unwrap();

        buf
    }
}

pub struct Test {
    pub temp: String,
}

#[cfg(test)]
mod tests {
    use crate::utils::crypto::CriptoHelper;

    #[test]
    fn test_decrypt() {
        // AARANGE
        let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEApl23EO002XmcyJ+ztbgSrhsR8IyPhfu+V9V84jleUSEaVBYR
Php5UNmKOgvmWKe1fLqZ3M/bjMehvUd72oLFCIDPWMfmrU51+1CijNLWp8/VnbO0
p6kbhybbH2uuJLiBXCYiT6pjNbgef9n9ABkqTbuCdpdU5m8eJd0le+8KKEWTYVbq
vGxIVcLd4uVtyJRvPVe2oIddcotqK4wdEajgU68M9ruJ4Ilr88aUczVcRxP253rI
9Haza4PwcNsomHLCSJ1ymReA8sevbA7tzWPwlYMwAO2NAnIOmqrUwkoT94lCbuTc
aB2D0/Z0LQY7BsVPbMYh5xFm3hI3FjX+cmCuewIDAQABAoIBAGrlgIFxySmLyL/o
TdKPigExB5/m0Tmn/i/1zx6U+hNrD73DyCR9YkIe5YBSsRl5+VVBmSeWr12P0E8M
pXpL2EqUaaaEG6Zz6b8nmqqdtqtxEbMZCxVHxZZb0yQnTmft3cDWB+nkc4bK3V4N
NVFg2hvERhnpNvYxo890f2dYutAQioTssdNDo628QKrZBjz+CqWEkjp0F5LAska5
rhRX25x5R0ih7CMrTLZt+Dzk2JbhRPVAlkj3d8a7amLjWoRyN2CD90tQOCzbNiZD
UzvlV4s3idVAiaSClZwrkvL6uiC7qnSwRXCjJlMOeFdJy9JRMiAtX9L63OTQ+onC
Z3VGTLECgYEA38A9lioJ3v0op9S0sdoq83cig4VWIVkDDDV/ofQH/xZB6D/tY9PS
n80yQhi7tc+dae0AZYVoh4ifVsw0FfcwtoUDPL9cYi0GInhnAlTn7UuppWI2pOlG
2XazBFB26BNskvCYsKbsMiS+E5BoQHecmZOcBv251w4XZv2WVhBSLX0CgYEAvlgj
yD/RguhIB0x+OgQmZPSGWQ8qXW7+e109RMQ59RGTsBCQq38FUZGrIJIbRBSmlybr
r3PJXHiSFbt52KWIivN5k+bjuJKYpJEpfDlRG8Kp3HFZLrWTd5U00cdZxd8LDFvc
INrhWI2GGVn8qdZdrszTlZPpS8AW1I3Nhl15bVcCgYEAxEYtgBlWWV53mGmVLGJ1
xOZfx0FioZQkgUQ4tseLcC+FFwdk5Wn93CIzERoDJ2R88FtvOp8BZ8roA0rT8eTJ
vYIGqfYvQwu90uUNb1UtsdHqeeIijxz3AnIGbSVseP35Axi8yFFU5lOmzSCi4tJJ
88oxV0yhBc4dp0GR6+MbQz0CgYBFUCFPlXW8vssj5UX96G72ylh16+DYf0eqMqzR
8sbMKCdosM+Ns8aDCpGPXcUSCJcVabXfgUFtK/a+dTOModLUDo9SPXzlRHTTUI0T
0GdpvXxPavM34CUgIbRHQ9m8BVmnmXfSewIeVgLkDnHEguxAcBQIXwFQdVWa9zxF
VpqWJwKBgDSnlhgBDzM7E3rqfJLtxFd8/QX6k6ZhtNabO25MyBgwblBmHq2EIOrt
jvwjJz2q+EpWR2iptUpJaqNTYJP4e8J+8nSotuVC1A022SBnWZvm+V2sEiCNc5+f
ytszGwriSCNPW7m2PVhgC74NS9u+MOOqpRn9qSb4b3Zdm9kEhVng
-----END RSA PRIVATE KEY-----";

        let encrypted_data = "gNJwKE0eSLHh0+kCkUEoOseKMAwJV2Ks8gUV8XxhnY26dy1n86J62E6L9JecWeKWK/pXJqj2mKunEFoGJmvdZgI5U4xiWvXch65owkRd+rpbwS6Ram0LP4vA9b5Owl3oBHsCah7wOnnNxEz5lEyYAeg+ebKdpVthUJuUlh7HCkYYdfgITxuS3kwxfyKOZzVVRs+eQcjKoh0A1+DtRp7N2uAJdSIba/Rjz6u9kKNGN5oQQOlARC0Hth0j0iFPnwG88lSIj07Q3mVJr8jyetava0lIJ6wevNKpMCRhE2b54M+B0zKHQXbGHC367URqWNEwaPuuoigmYeZ9ZDjgy3wQ8g==";

        // ACT
        let decrypted_data = CriptoHelper::decrypt(&encrypted_data, private_key_pem);

        // ASSERT
        assert_eq!(decrypted_data, "ciao".to_string());
    }
}
