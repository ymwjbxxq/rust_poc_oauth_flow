import { SSMClient } from "@aws-sdk/client-ssm";
import { SimpleStorageManager } from "./modules/ssmHelper";
import {
  S3Client,
  GetObjectCommand,
  GetObjectCommandInput,
  GetObjectCommandOutput,
} from '@aws-sdk/client-s3';
import { Config } from "./dtos/config";
import { Encryption } from "./modules/encryption";
import { JWT } from "./modules/jwt";
import { Secret } from "./dtos/secret";

const ssm_client = new SSMClient({ region: "eu-central-1" });
const s3_client = new S3Client({ region: 'eu-central-1' });
const ssm_helper = new SimpleStorageManager(ssm_client);

export const handler = async (event: any): Promise<any> => {
  console.log("Event: ", JSON.stringify(event));
  const request = event.Records[0].cf.request;
  const querystring = new URLSearchParams(request.querystring);

  if (request.method === "POST"
    && (request.uri.includes("/v2/signup") || request.uri.includes("/v2/login"))) {
    // decrypt from base64
    const jsonString = Buffer.from(request.body.data, "base64").toString("utf8");
    const payload = JSON.parse(jsonString);

    // load the config from S3 it should be s3 multi-region endpoint
    const params: GetObjectCommandInput = {
      Bucket: "oauth-sourcebucket-1j1q9b4k0za14", //The function cannot have environment variables. Set a static bucket name
      Key: `${querystring.get("client_id")}/config.json`,
    };
    let response: GetObjectCommandOutput = await s3_client.send(new GetObjectCommand(params));
    const bodyContents = await response.Body?.transformToString();
    let config: Config = JSON.parse(bodyContents);

    // take secret and public keys from SSM
    let [ssm_private_key, ssm_secret_key] = await Promise.all([
      ssm_helper.decrypt(config.private_key),
      ssm_helper.decrypt(config.secret_key)
    ]);

    const token = JWT.decrypt_token(ssm_secret_key);
    let secret: Secret = token.payload;
    let hash_salt = Encryption.rsa_decrypt(secret.hash_salt, ssm_private_key);
    let aes_securitykey = Encryption.rsa_decrypt(secret.aes_securitykey, ssm_private_key);
    let aes_initVector = Encryption.rsa_decrypt(secret.aes_initVector, ssm_private_key);

    // encrypt the PII fields based on the config loaded from S3
    config.PII_fields.forEach(prop => {
      if (payload.hasOwnProperty(prop)) {
        if (prop === "password") {
          payload[prop] = Encryption.password_hash(payload[prop], hash_salt);
        } else {
          payload[prop] = Encryption.aes_encrypt(payload[prop], aes_securitykey, aes_initVector);
        }
      }
    });

    // replace the request body with the encrypted payload
    request.body.action = "replace";
    request.body.data = Buffer.from(JSON.stringify(payload)).toString("base64");
  }

  return request;
};

