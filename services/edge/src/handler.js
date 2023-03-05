"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.handler = void 0;
const client_ssm_1 = require("@aws-sdk/client-ssm");
const ssmHelper_1 = require("./modules/ssmHelper");
const client_s3_1 = require("@aws-sdk/client-s3");
const encryption_1 = require("./modules/encryption");
const jwt_1 = require("./modules/jwt");
const ssm_client = new client_ssm_1.SSMClient({ region: "eu-central-1" });
const s3_client = new client_s3_1.S3Client({ region: 'eu-central-1' });
const ssm_helper = new ssmHelper_1.SimpleStorageManager(ssm_client);
const handler = async (event) => {
    console.log("Event: ", JSON.stringify(event));
    const request = event.Records[0].cf.request;
    const querystring = new URLSearchParams(request.querystring);
    if (request.method === "POST"
        && (request.uri.includes("/v2/signup") || request.uri.includes("/v2/login"))) {
        console.log("dentro");
        const jsonString = Buffer.from(request.body.data, "base64").toString("utf8");
        const payload = JSON.parse(jsonString);
        const params = {
            Bucket: "oauth-sourcebucket-1j1q9b4k0za14",
            Key: `${querystring.get("client_id")}/config.json`,
        };
        let response = await s3_client.send(new client_s3_1.GetObjectCommand(params));
        const bodyContents = await response.Body?.transformToString();
        let config = JSON.parse(bodyContents);
        let [ssm_private_key, ssm_secret_key] = await Promise.all([
            ssm_helper.decrypt(config.private_key),
            ssm_helper.decrypt(config.secret_key)
        ]);
        const secret = jwt_1.JWT.decrypt_token(ssm_secret_key).payload;
        console.log("secret", secret);
        let hash_salt = encryption_1.Encryption.rsa_decrypt(secret.hash_salt, ssm_private_key);
        let aes_securitykey = encryption_1.Encryption.rsa_decrypt(secret.aes_securitykey, ssm_private_key);
        let aes_initVector = encryption_1.Encryption.rsa_decrypt(secret.aes_initVector, ssm_private_key);
        config.PII_fields.forEach(prop => {
            if (payload.hasOwnProperty(prop)) {
                if (prop === "password") {
                    payload[prop] = encryption_1.Encryption.bcrypt_hash(payload[prop], hash_salt);
                }
                else {
                    payload[prop] = encryption_1.Encryption.aes_encrypt(payload[prop], aes_securitykey, aes_initVector);
                }
            }
        });
        request.body.action = "replace";
        request.body.data = Buffer.from(JSON.stringify(payload)).toString("base64");
    }
    return request;
};
exports.handler = handler;
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaGFuZGxlci5qcyIsInNvdXJjZVJvb3QiOiIiLCJzb3VyY2VzIjpbImhhbmRsZXIudHMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6Ijs7O0FBQUEsb0RBQWdEO0FBQ2hELG1EQUEyRDtBQUMzRCxrREFLNEI7QUFFNUIscURBQWtEO0FBQ2xELHVDQUFvQztBQUdwQyxNQUFNLFVBQVUsR0FBRyxJQUFJLHNCQUFTLENBQUMsRUFBRSxNQUFNLEVBQUUsY0FBYyxFQUFFLENBQUMsQ0FBQztBQUM3RCxNQUFNLFNBQVMsR0FBRyxJQUFJLG9CQUFRLENBQUMsRUFBRSxNQUFNLEVBQUUsY0FBYyxFQUFFLENBQUMsQ0FBQztBQUMzRCxNQUFNLFVBQVUsR0FBRyxJQUFJLGdDQUFvQixDQUFDLFVBQVUsQ0FBQyxDQUFDO0FBRWpELE1BQU0sT0FBTyxHQUFHLEtBQUssRUFBRSxLQUFVLEVBQWdCLEVBQUU7SUFDeEQsT0FBTyxDQUFDLEdBQUcsQ0FBQyxTQUFTLEVBQUUsSUFBSSxDQUFDLFNBQVMsQ0FBQyxLQUFLLENBQUMsQ0FBQyxDQUFDO0lBQzlDLE1BQU0sT0FBTyxHQUFHLEtBQUssQ0FBQyxPQUFPLENBQUMsQ0FBQyxDQUFDLENBQUMsRUFBRSxDQUFDLE9BQU8sQ0FBQztJQUM1QyxNQUFNLFdBQVcsR0FBRyxJQUFJLGVBQWUsQ0FBQyxPQUFPLENBQUMsV0FBVyxDQUFDLENBQUM7SUFFN0QsSUFBSSxPQUFPLENBQUMsTUFBTSxLQUFLLE1BQU07V0FDeEIsQ0FBQyxPQUFPLENBQUMsR0FBRyxDQUFDLFFBQVEsQ0FBQyxZQUFZLENBQUMsSUFBSSxPQUFPLENBQUMsR0FBRyxDQUFDLFFBQVEsQ0FBQyxXQUFXLENBQUMsQ0FBQyxFQUFFO1FBQzlFLE9BQU8sQ0FBQyxHQUFHLENBQUMsUUFBUSxDQUFDLENBQUM7UUFFdEIsTUFBTSxVQUFVLEdBQUcsTUFBTSxDQUFDLElBQUksQ0FBQyxPQUFPLENBQUMsSUFBSSxDQUFDLElBQUksRUFBRSxRQUFRLENBQUMsQ0FBQyxRQUFRLENBQUMsTUFBTSxDQUFDLENBQUM7UUFDN0UsTUFBTSxPQUFPLEdBQUcsSUFBSSxDQUFDLEtBQUssQ0FBQyxVQUFVLENBQUMsQ0FBQztRQUd2QyxNQUFNLE1BQU0sR0FBMEI7WUFDcEMsTUFBTSxFQUFFLGtDQUFrQztZQUMxQyxHQUFHLEVBQUUsR0FBRyxXQUFXLENBQUMsR0FBRyxDQUFDLFdBQVcsQ0FBQyxjQUFjO1NBQ25ELENBQUM7UUFDRixJQUFJLFFBQVEsR0FBMkIsTUFBTSxTQUFTLENBQUMsSUFBSSxDQUFDLElBQUksNEJBQWdCLENBQUMsTUFBTSxDQUFDLENBQUMsQ0FBQztRQUMxRixNQUFNLFlBQVksR0FBRyxNQUFNLFFBQVEsQ0FBQyxJQUFJLEVBQUUsaUJBQWlCLEVBQUUsQ0FBQztRQUM5RCxJQUFJLE1BQU0sR0FBVyxJQUFJLENBQUMsS0FBSyxDQUFDLFlBQVksQ0FBQyxDQUFDO1FBRzlDLElBQUksQ0FBQyxlQUFlLEVBQUUsY0FBYyxDQUFDLEdBQUcsTUFBTSxPQUFPLENBQUMsR0FBRyxDQUFDO1lBQ3hELFVBQVUsQ0FBQyxPQUFPLENBQUMsTUFBTSxDQUFDLFdBQVcsQ0FBQztZQUN0QyxVQUFVLENBQUMsT0FBTyxDQUFDLE1BQU0sQ0FBQyxVQUFVLENBQUM7U0FDdEMsQ0FBQyxDQUFDO1FBRUgsTUFBTSxNQUFNLEdBQVcsU0FBRyxDQUFDLGFBQWEsQ0FBQyxjQUFjLENBQUMsQ0FBQyxPQUFPLENBQUM7UUFDakUsT0FBTyxDQUFDLEdBQUcsQ0FBQyxRQUFRLEVBQUUsTUFBTSxDQUFDLENBQUM7UUFDOUIsSUFBSSxTQUFTLEdBQUcsdUJBQVUsQ0FBQyxXQUFXLENBQUMsTUFBTSxDQUFDLFNBQVMsRUFBRSxlQUFlLENBQUMsQ0FBQztRQUMxRSxJQUFJLGVBQWUsR0FBRyx1QkFBVSxDQUFDLFdBQVcsQ0FBQyxNQUFNLENBQUMsZUFBZSxFQUFFLGVBQWUsQ0FBQyxDQUFDO1FBQ3RGLElBQUksY0FBYyxHQUFHLHVCQUFVLENBQUMsV0FBVyxDQUFDLE1BQU0sQ0FBQyxjQUFjLEVBQUUsZUFBZSxDQUFDLENBQUM7UUFHcEYsTUFBTSxDQUFDLFVBQVUsQ0FBQyxPQUFPLENBQUMsSUFBSSxDQUFDLEVBQUU7WUFDL0IsSUFBSSxPQUFPLENBQUMsY0FBYyxDQUFDLElBQUksQ0FBQyxFQUFFO2dCQUNoQyxJQUFJLElBQUksS0FBSyxVQUFVLEVBQUU7b0JBQ3ZCLE9BQU8sQ0FBQyxJQUFJLENBQUMsR0FBRyx1QkFBVSxDQUFDLFdBQVcsQ0FBQyxPQUFPLENBQUMsSUFBSSxDQUFDLEVBQUUsU0FBUyxDQUFDLENBQUM7aUJBQ2xFO3FCQUFNO29CQUNMLE9BQU8sQ0FBQyxJQUFJLENBQUMsR0FBRyx1QkFBVSxDQUFDLFdBQVcsQ0FBQyxPQUFPLENBQUMsSUFBSSxDQUFDLEVBQUUsZUFBZSxFQUFFLGNBQWMsQ0FBQyxDQUFDO2lCQUN4RjthQUNGO1FBQ0gsQ0FBQyxDQUFDLENBQUM7UUFHSCxPQUFPLENBQUMsSUFBSSxDQUFDLE1BQU0sR0FBRyxTQUFTLENBQUM7UUFDaEMsT0FBTyxDQUFDLElBQUksQ0FBQyxJQUFJLEdBQUcsTUFBTSxDQUFDLElBQUksQ0FBQyxJQUFJLENBQUMsU0FBUyxDQUFDLE9BQU8sQ0FBQyxDQUFDLENBQUMsUUFBUSxDQUFDLFFBQVEsQ0FBQyxDQUFDO0tBQzdFO0lBRUQsT0FBTyxPQUFPLENBQUM7QUFDakIsQ0FBQyxDQUFDO0FBbERXLFFBQUEsT0FBTyxXQWtEbEIifQ==