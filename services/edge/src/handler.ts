import { SSMClient } from "@aws-sdk/client-ssm";
import crypto from "crypto";
import { SimpleStorageManager } from "./modules/ssmHelper";
import {
  S3Client,
  GetObjectCommand,
  GetObjectCommandInput,
  GetObjectCommandOutput,
} from '@aws-sdk/client-s3';
import { Config } from "./dtos/config";

const ssm_client = new SSMClient({ region: "eu-central-1" });
const s3_client = new S3Client({ region: 'eu-central-1' });
const ssm_helper = new SimpleStorageManager(ssm_client);

export const handler = async (event: any): Promise<any> => {
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

    // take public from SSM
    let public_key = await ssm_helper.decrypt(config.public_key);

    // encrypt the PII fields based on the config loaded from S3
    config.PII_fields.forEach(prop => {
      if (payload.hasOwnProperty(prop)) {
        payload[prop] = hash(payload[prop], public_key);
      }
    });

    // replace the request body with the encrypted payload
    request.body.action = "replace";
    request.body.data = Buffer.from(JSON.stringify(payload)).toString("base64");
  }

  return request;
};

function hash(input: string, public_key: string): string {
  return crypto
    .publicEncrypt({
      key: public_key,
    }, Buffer.from(input))
    .toString("base64");
}