#Authorization Code Flow with Proof Key for Code Exchange (PKCE) #

This project tries to replicate the [Authorization Code Flow with Proof Key for Code Exchange (PKCE)](https://auth0.com/docs/flows/authorization-code-flow-with-proof-key-for-code-exchange-pkce)

### HOW IT WORKS ###

![picture](https://github.com/ymwjbxxq/rust_poc_oauth_flow/blob/main/readme/auth-sequence-auth-code-pkce.png)

The project is divided into the following parts:

* OAuth service
* Website service that could be any client that wants to use the OAuth service
* Edge service

The OAuth service could have N websites that want to use its services, so each has its configuration.
The configuration contains

- Custom PII_fields that the OAuth service encrypts at the edge for maximum security
- RSA certificate generates for each website
- Custom pages like the Login

Once the user register, we can Login and if it is all successful and at the end of the flow, we should be able to see the JWT token:
```App
Authorization: eyJhbGciOiJIUzI1NiJ9.T0RBVUxDQVk0V0k1S1ZQU01DUkZCTExBOEs3QURNUEQzWUM4WFdUNVQ1UDRVUElRREw.956xdJUWC4mfDJlohbqP2kqFUNoAPlZ8nRRJCfzo1KI
```


### Security: ###

To reduce the exposure of sensitive data, I use AWS CloudFront with Lambda@Edge to:

* Intercept data
* Encrypt at the edge with the website RSA
* Pass it through before the application can process it, reducing exposure.

Lambda edge will load the client config from S3 and, from there, take the SSM key to load secrets to encrypt sensitive data.

To avoid [Cross Site Request Forgery (CSRF) attacks](https://owasp.org/www-community/attacks/csrf) the 
This project tries to replicate the [Authorization Code Flow with Proof Key for Code Exchange (PKCE) using two parameters:

- state: is sent with the authorization request and compared with the state value that is returned by the authorization server in the authorization response
- code_challange: It ensures that the client that receives the authorization code is the same client that initially requested the authorization

They can be stored in the "HTTP-only" cookies, but they can be blocked by the browser (Cognito mode)
They can be stored in the "localStorage" but could be altered by client-side code running in the browser.

So to avoid all of them, I stored them in DynamoDB, but it could be any cache service, and I compared against them, setting a relatively low TTL of 1 minute.

Each client will have a secret_key made of:

```
{
  "hash_salt": "some bcrypt hash encrypeted with the RSA Public key and converted to base64",
  "aes_initVector": "some random 16 bytes encrypeted with the RSA Public key and converted to base64",
  "aes_securitykey": "some random 32 bytes encrypted with the RSA Public key and converted to base64."
}
```
- public_key - RSA public key to encrypt the values of the secret_key
- private_key - RSA private key to decrypt the encrypted secret_key values

### MANUAL SETUP: ###

Lambda@Edge does not support environment variables, so I cannot pass with the CI the OAuth S3 bucket name where the config is stored.

**S3**:

- Change the bucket name inside the edge lambda
- This could be the S3 multi-region endpoint or a custom domain pointing to a specific place. So for this test, I must hardcode the name in the edge lambda

**SSM**:

- Run `./secrets.sh` 
- Upload an RSA certificate public key into public_key
- Upload an RSA certificate private key into private_key

For the sake of the test, I generate one here https://cryptotools.net/rsagen

The secret_key is a token made of: 

```
{
  "hash_salt": "some hash encrypeted with the RSA Public key and converted to base64",
  "aes_initVector": "some random 16 bytes encrypeted with the RSA Public key and converted to base64",
  "aes_securitykey": "some random 32 bytes encrypted with the RSA Public key and converted to base64."
}
```

Once it is all done, we have something like

**/clientid1/public_key**

```
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApl23EO002XmcyJ+ztbgS
rhsR8IyPhfu+V9V84jleUSEaVBYRPhp5UNmKOgvmWKe1fLqZ3M/bjMehvUd72oLF
CIDPWMfmrU51+1CijNLWp8/VnbO0p6kbhybbH2uuJLiBXCYiT6pjNbgef9n9ABkq
TbuCdpdU5m8eJd0le+8KKEWTYVbqvGxIVcLd4uVtyJRvPVe2oIddcotqK4wdEajg
U68M9ruJ4Ilr88aUczVcRxP253rI9Haza4PwcNsomHLCSJ1ymReA8sevbA7tzWPw
lYMwAO2NAnIOmqrUwkoT94lCbuTcaB2D0/Z0LQY7BsVPbMYh5xFm3hI3FjX+cmCu
ewIDAQAB
-----END PUBLIC KEY-----
```

**/clientid1/private_key**
```
-----BEGIN RSA PRIVATE KEY-----
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
-----END RSA PRIVATE KEY-----
```

**/clientid1/secret_key**
```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJwYXlsb2FkIjp7Imhhc2hfc2FsdCI6ImQ3aWI5aEpTR0pod3crSWJGellGZ2ZaNkMydTVrTGRCSEpvTHorb2VNalhiQ1RXc2NJQkJobkY0dkZ3Qk5SZzZIYlE3Z1ZSejAzQTZHZzJGdFZ4ZlcrSWIzTGFKUjBCSEJ5VURCeEZWcHdnUUFGd1VrWkNqSzRoMkF1TXN5RmE3c3FLQmlydEE3cGVXTWovd3Jwc3paQkhCQ2Y4T1J1REtMQ1ZqMGdUd0RVeERGbE1LRUhzM05hSlRKMDdtRkY5N0RHMWdBNlZmS3ZIcFFlMEpFQ29nVzcxNzJ6TFVmNW16YmFNTXY0YVdZM3lGdEN3ODkvaUU5UVVmeUZnelNRK0tQNU5uWUJnUmJnb2FsTHpoZkRQcVdCREU1dkRSVEhyc1MrV3pDa3EvaVJiNmZJRmZLT0plOUFIcW9yczZEVkx0MjRoU2FhK3dlL0U1cjRoWDBqMzQ1dz09IiwiYWVzX2luaXRWZWN0b3IiOiJmYnBoR0RvWUkwMGVRNFdqZlFFQStUR3dxT1Q4MWJzanM3SC9VU3dkZE9QSlFIeFlSeHZVd3hWTGhqR0tPS0lHdU5GZzNUZktrc1BjZ3lrUEJxOXZyVEdzRFFKRWRWeWtSVTRwLy9ZSU5jSEdTVXZnd00zYVZKb1FUOHVPMjd0MjJwZlU3ZFlubU1KdGVONDNzZXJYME94S2RSdThRZ2JrbzNUSEJaY3ZoY1dFYy9mMURkZXpYRk5Ddlo4dm10R1libGNNNWN0U25XN00ybTZRZmR4cGJjMkEyeXlvQlJFeFBMU2VyOWNsQjdFNExYdlhwTXcra3F1TE14R2VWbklONDViQlBRdWJEdUduN284QkJOVGhuendNcFFXU1ZOeG9RRGwxSkdxM0N4SkJramlEckFFWXJ3UjEvUGw0U1pRUnFnQmx1MytwenhCU1NVZUozdmtJdlE9PSIsImFlc19zZWN1cml0eWtleSI6ImxCaVRHMEdRcVNVNFJXQTkyWi9wUmtlSGdiV3hlZFhhdUVIVVNwSjR1ZlZFTGQ4OWNLdXFENWc4MHZ3ZXc5VWtrMGtzbFZHc0JqTTdQQU5ka3NHOVB2NlZVMGNPb050TnM4eE5jWDV4Q1RNUlNqTURoN1p0NWwydnZUdS8vVVhVejhjTDhnb29WemVKWE5Dc1hJWnpScko5WCs2RjlVOU96SnRzaHVkVktSUUR1WnR5bUNWWURNa09jUXEzL25nRDNiUnRWVUU3b1B3T2cxVFVuMTRmaGdScW03aHhXYlJtcHRpeUdobHZoWmtpMFVPc2M0bFJhRmhqWU9XVDVLWndDN1FKRml5Mk1wTTVYL1ZLNU1TUG5rVHVwWFVGYmw3WmVFeHZIc2RuWTNzOFhBVlN1V3VkUnNrTHdtK2VBM282VmlqUTl2WndicFcxZWR4aytaOVFQUT09In0sImlhdCI6MTY3ODAxMTg1OX0.NnyGQhjjUyV-oKpSKeK9T-irwRrThI2u-rQM25Tizq8

```


### MANUAL TEST WITH POSTMAN using Lambda@edge (all encrypted): ###

Register a user at - https://[cloudfront-domain]/{stage}/v2/signup?client_id=clientid1

POST:
```
{
  "email":"a@a.com",
  "password":"aaaa",
  "family_name":"aaaa",
  "given_name":"aaa",
  "is_consent":"true",
  "is_optin":"true",
  "remember":"true"
}
```

**NOTE**:

- All the URLs are visible in the response header section under Location.
- Copy the Location and paste it for the next step

1. SET in Postman do not follow redirects
2. GET -> user click login -  https://[your-app-domain]/{stage}/login?client_id=clientid1
3. GET -> Authorization code quest + code challenge to Oauth provider - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?....
4. POST -> Redirec to login page - https://[cloudfront-domain]/{stage}/v2/login?.....
5. GET -> Authenticate and consent/optin - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?....
6. GET -> Authorization code - https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/auth?.....
7. GET -> Request token - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/token?....
8. Now, with the token, we can call our protected API - eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.TXdR1GMY_5nqQLDTk3uSZlRjt7JeVdK8HUuTRo44-OU

### TEST FROM THE BROWSER without Lambda@edge (not encrypted): ###

1. Upload the files to simulate a website
2. Upload the RSA public and private keys into SSM
3. Register a user at - https://[your-oauth-domain]/{stage}/v2/signup?client_id=clientid1
4. Open the page index.html
5. Insert https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/login?client_id=clientid1
6. Click Login
7. Enter the data from point 2
8 Now, with the token, we can call our protected API - eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.TXdR1GMY_5nqQLDTk3uSZlRjt7JeVdK8HUuTRo44-OU

**DB RESULTS**:

Data can be in clear based on the test with Lambda@edge or without.

![picture](https://github.com/ymwjbxxq/rust_poc_oauth_flow/blob/main/readme/database.png)

The reality is that with a custom domain etc., you will not access APIGW directly, so this situation will not happen. 


### LOAD TEST WITH POSTMAN: ###

TODO


### Deploy ###

```bash
coming soon

```

### Cleanup ###
```
coming soon
```
