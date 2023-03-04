"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.handler = void 0;
const client_ssm_1 = require("@aws-sdk/client-ssm");
const crypto_1 = __importDefault(require("crypto"));
const ssmHelper_1 = require("./modules/ssmHelper");
const s3Helper_1 = require("./modules/s3Helper");
const client_s3_1 = require("@aws-sdk/client-s3");
const ssm_client = new client_ssm_1.SSMClient({ region: "eu-central-1" });
const s3_client = new client_s3_1.S3Client({ region: 'eu-central-1' });
const ssm_helper = new ssmHelper_1.SimpleStorageManager(ssm_client);
const s3_helper = new s3Helper_1.S3Helper();
const handler = async (event) => {
    console.log(JSON.stringify(event));
    const request = event.Records[0].cf.request;
    const querystring = new URLSearchParams(request.querystring);
    if (request.uri.includes("/v2/signup")) {
        const decodedBase64 = Buffer.from(request.body.data, "base64").toString("utf8");
        const payload = JSON.parse(decodedBase64);
        const params = {
            Bucket: "oauth-sourcebucket-1j1q9b4k0za14",
            Key: `${querystring.get("client_id")}/config.json`,
        };
        let response = await s3_client.send(new client_s3_1.GetObjectCommand(params));
        console.log(JSON.stringify(response));
        const bodyContents = await s3_helper.streamToString(response.Body);
        console.log(JSON.stringify(bodyContents));
        let config = JSON.parse(bodyContents);
        console.log(JSON.stringify(config));
        let public_key = await ssm_helper.decrypt(config.public_key);
        console.log(public_key);
        config.PII_fields.forEach(prop => {
            if (payload.hasOwnProperty(prop)) {
                payload[prop] = hash(payload[prop], public_key);
            }
        });
        request.body.action = "replace";
        request.body.data = Buffer.from(JSON.stringify(payload)).toString("base64");
    }
    return request;
};
exports.handler = handler;
function hash(input, public_key) {
    return crypto_1.default
        .publicEncrypt({
        key: public_key,
    }, Buffer.from(input))
        .toString("base64");
}
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaGFuZGxlci5qcyIsInNvdXJjZVJvb3QiOiIiLCJzb3VyY2VzIjpbImhhbmRsZXIudHMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6Ijs7Ozs7O0FBQUEsb0RBQWdEO0FBQ2hELG9EQUE0QjtBQUM1QixtREFBMkQ7QUFDM0QsaURBQThDO0FBQzlDLGtEQUs0QjtBQUk1QixNQUFNLFVBQVUsR0FBRyxJQUFJLHNCQUFTLENBQUMsRUFBRSxNQUFNLEVBQUUsY0FBYyxFQUFFLENBQUMsQ0FBQztBQUM3RCxNQUFNLFNBQVMsR0FBRyxJQUFJLG9CQUFRLENBQUMsRUFBRSxNQUFNLEVBQUUsY0FBYyxFQUFFLENBQUMsQ0FBQztBQUMzRCxNQUFNLFVBQVUsR0FBRyxJQUFJLGdDQUFvQixDQUFDLFVBQVUsQ0FBQyxDQUFDO0FBQ3hELE1BQU0sU0FBUyxHQUFHLElBQUksbUJBQVEsRUFBRSxDQUFDO0FBRTFCLE1BQU0sT0FBTyxHQUFHLEtBQUssRUFBRSxLQUFVLEVBQWdCLEVBQUU7SUFDeEQsT0FBTyxDQUFDLEdBQUcsQ0FBQyxJQUFJLENBQUMsU0FBUyxDQUFDLEtBQUssQ0FBQyxDQUFDLENBQUM7SUFDbkMsTUFBTSxPQUFPLEdBQUcsS0FBSyxDQUFDLE9BQU8sQ0FBQyxDQUFDLENBQUMsQ0FBQyxFQUFFLENBQUMsT0FBTyxDQUFDO0lBQzVDLE1BQU0sV0FBVyxHQUFHLElBQUksZUFBZSxDQUFDLE9BQU8sQ0FBQyxXQUFXLENBQUMsQ0FBQztJQUU3RCxJQUFJLE9BQU8sQ0FBQyxHQUFHLENBQUMsUUFBUSxDQUFDLFlBQVksQ0FBQyxFQUFFO1FBQ3RDLE1BQU0sYUFBYSxHQUFHLE1BQU0sQ0FBQyxJQUFJLENBQUMsT0FBTyxDQUFDLElBQUksQ0FBQyxJQUFJLEVBQUUsUUFBUSxDQUFDLENBQUMsUUFBUSxDQUFDLE1BQU0sQ0FBQyxDQUFDO1FBQ2hGLE1BQU0sT0FBTyxHQUFHLElBQUksQ0FBQyxLQUFLLENBQUMsYUFBYSxDQUFDLENBQUM7UUFFMUMsTUFBTSxNQUFNLEdBQTBCO1lBQ3BDLE1BQU0sRUFBRSxrQ0FBa0M7WUFDMUMsR0FBRyxFQUFFLEdBQUcsV0FBVyxDQUFDLEdBQUcsQ0FBQyxXQUFXLENBQUMsY0FBYztTQUNuRCxDQUFDO1FBRUYsSUFBSSxRQUFRLEdBQTJCLE1BQU0sU0FBUyxDQUFDLElBQUksQ0FBQyxJQUFJLDRCQUFnQixDQUFDLE1BQU0sQ0FBQyxDQUFDLENBQUM7UUFDMUYsT0FBTyxDQUFDLEdBQUcsQ0FBQyxJQUFJLENBQUMsU0FBUyxDQUFDLFFBQVEsQ0FBQyxDQUFDLENBQUM7UUFDdEMsTUFBTSxZQUFZLEdBQUcsTUFBTSxTQUFTLENBQUMsY0FBYyxDQUFDLFFBQVEsQ0FBQyxJQUFnQixDQUFDLENBQUM7UUFDL0UsT0FBTyxDQUFDLEdBQUcsQ0FBQyxJQUFJLENBQUMsU0FBUyxDQUFDLFlBQVksQ0FBQyxDQUFDLENBQUM7UUFDMUMsSUFBSSxNQUFNLEdBQVcsSUFBSSxDQUFDLEtBQUssQ0FBQyxZQUFZLENBQUMsQ0FBQztRQUM5QyxPQUFPLENBQUMsR0FBRyxDQUFDLElBQUksQ0FBQyxTQUFTLENBQUMsTUFBTSxDQUFDLENBQUMsQ0FBQztRQUVwQyxJQUFJLFVBQVUsR0FBRyxNQUFNLFVBQVUsQ0FBQyxPQUFPLENBQUMsTUFBTSxDQUFDLFVBQVUsQ0FBQyxDQUFDO1FBQzdELE9BQU8sQ0FBQyxHQUFHLENBQUMsVUFBVSxDQUFDLENBQUM7UUFDeEIsTUFBTSxDQUFDLFVBQVUsQ0FBQyxPQUFPLENBQUMsSUFBSSxDQUFDLEVBQUU7WUFDL0IsSUFBSSxPQUFPLENBQUMsY0FBYyxDQUFDLElBQUksQ0FBQyxFQUFFO2dCQUNoQyxPQUFPLENBQUMsSUFBSSxDQUFDLEdBQUcsSUFBSSxDQUFDLE9BQU8sQ0FBQyxJQUFJLENBQUMsRUFBRSxVQUFVLENBQUMsQ0FBQzthQUNqRDtRQUNILENBQUMsQ0FBQyxDQUFDO1FBRUgsT0FBTyxDQUFDLElBQUksQ0FBQyxNQUFNLEdBQUcsU0FBUyxDQUFDO1FBQ2hDLE9BQU8sQ0FBQyxJQUFJLENBQUMsSUFBSSxHQUFHLE1BQU0sQ0FBQyxJQUFJLENBQUMsSUFBSSxDQUFDLFNBQVMsQ0FBQyxPQUFPLENBQUMsQ0FBQyxDQUFDLFFBQVEsQ0FBQyxRQUFRLENBQUMsQ0FBQztLQUM3RTtJQUVELE9BQU8sT0FBTyxDQUFDO0FBQ2pCLENBQUMsQ0FBQztBQWxDVyxRQUFBLE9BQU8sV0FrQ2xCO0FBRUYsU0FBUyxJQUFJLENBQUMsS0FBYSxFQUFFLFVBQWtCO0lBQzdDLE9BQU8sZ0JBQU07U0FDVixhQUFhLENBQUM7UUFDYixHQUFHLEVBQUUsVUFBVTtLQUNoQixFQUFFLE1BQU0sQ0FBQyxJQUFJLENBQUMsS0FBSyxDQUFDLENBQUM7U0FDckIsUUFBUSxDQUFDLFFBQVEsQ0FBQyxDQUFDO0FBQ3hCLENBQUMifQ==