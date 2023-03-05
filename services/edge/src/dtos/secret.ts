export interface Secret {
  hash_salt: string;
  aes_initVector: string;
  aes_securitykey: string;
}