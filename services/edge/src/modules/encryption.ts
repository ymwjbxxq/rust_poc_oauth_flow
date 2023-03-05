
import crypto from "crypto";

export class Encryption {
  public static rsa_encrypt(input: string, public_key: string): string {
    return crypto
      .publicEncrypt({
        key: public_key,
      }, Buffer.from(input))
      .toString("base64");
  }

  public static rsa_decrypt(input: string, private_key: string): string {
    return crypto
      .privateDecrypt({
        key: private_key,
      }, Buffer.from(input, "base64"))
      .toString("utf8");
  }

  public static aes_encrypt(input: string, secret_key: string, my_iv: string): string {
    const cipher = crypto.createCipheriv(
      "aes-256-cbc",
      Buffer.from(secret_key, 'base64'),
      Buffer.from(my_iv, 'base64'));
    let encryptedData = cipher.update(input, "utf-8", "hex");
    encryptedData += cipher.final("hex");
    return encryptedData;
  }

  public static aes_decrypt(input: string, secret_key: string, my_iv: string): string {
    const decipher = crypto.createDecipheriv(
      "aes-256-cbc",
      Buffer.from(secret_key, 'base64'),
      Buffer.from(my_iv, 'base64'));
    let decryptedData = decipher.update(input, "hex", "utf-8");
    decryptedData += decipher.final("utf8");

    return decryptedData;
  }

  public static password_hash(input: string, salt: string): string {
    const hash = crypto.pbkdf2Sync(input, salt, 1000, 64, `sha512`).toString(`base64`);

    return hash;
  }
}