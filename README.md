# PoC - RUST Authorization Code Flow with Proof Key for Code Exchange (PKCE) #
Usually, my hello version of a serverless example combines services like SQS, DynamoDB etc. 
This time I wanted to build something where the flow is a bit more complex, and this is why I used the [Authorization Code Flow with Proof Key for Code Exchange (PKCE)](https://auth0.com/docs/flows/authorization-code-flow-with-proof-key-for-code-exchange-pkce)

### VERY IMPORTANT ###
This code is not for production and contains security issues. However, some choice is there just to make the flow easier. 
For example, some GET should be a POST, and some elements should not be saved in the cookie.
It is just a PoC and so take it for what it is.

### HOW IT WORKS ###

![picture](https://github.com/ymwjbxxq/rust_poc_oauth_flow/blob/main/readme/auth-sequence-auth-code-pkce.png)

I have created 3 APIs:

* App endpoint protected by Lambda Authorizer
* App endpoints not protected
* OAuth endpoints

If, for example, you need to add extra steps at the original flow, you can, and this is why I have added two more:

* Consent page
* Optin page

At the end of the flow, we should be able to see the JWT token:
```App
Authorization: eyJhbGciOiJIUzI1NiJ9.T0RBVUxDQVk0V0k1S1ZQU01DUkZCTExBOEs3QURNUEQzWUM4WFdUNVQ1UDRVUElRREw.956xdJUWC4mfDJlohbqP2kqFUNoAPlZ8nRRJCfzo1KI
```
To add more latency and authenticity, I load some UI from a configuration file hosted in S3. This file could be custom made for each clientId that the OAuth provider is supporting. You can find an example on the file clientid1.json  (you should upload manually on the S3).

### EXTRA: ###

To reduce the exposure in clear of sensitive data, you can combine AWS CloudFront with Lambda@Edge to:

* Intercept data
* Encrypt at the edge
* Pass it through before the application can process it, reducing exposure.

Lambda edge will load the client config and from there take SSM key to load RSA public key to encrypt sensitive data

Example:
```
// -----BEGIN PUBLIC KEY-----
// MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApl23EO002XmcyJ+ztbgS
// rhsR8IyPhfu+V9V84jleUSEaVBYRPhp5UNmKOgvmWKe1fLqZ3M/bjMehvUd72oLF
// CIDPWMfmrU51+1CijNLWp8/VnbO0p6kbhybbH2uuJLiBXCYiT6pjNbgef9n9ABkq
// TbuCdpdU5m8eJd0le+8KKEWTYVbqvGxIVcLd4uVtyJRvPVe2oIddcotqK4wdEajg
// U68M9ruJ4Ilr88aUczVcRxP253rI9Haza4PwcNsomHLCSJ1ymReA8sevbA7tzWPw
// lYMwAO2NAnIOmqrUwkoT94lCbuTcaB2D0/Z0LQY7BsVPbMYh5xFm3hI3FjX+cmCu
// ewIDAQAB
// -----END PUBLIC KEY-----


// -----BEGIN RSA PRIVATE KEY-----
// MIIEowIBAAKCAQEApl23EO002XmcyJ+ztbgSrhsR8IyPhfu+V9V84jleUSEaVBYR
// Php5UNmKOgvmWKe1fLqZ3M/bjMehvUd72oLFCIDPWMfmrU51+1CijNLWp8/VnbO0
// p6kbhybbH2uuJLiBXCYiT6pjNbgef9n9ABkqTbuCdpdU5m8eJd0le+8KKEWTYVbq
// vGxIVcLd4uVtyJRvPVe2oIddcotqK4wdEajgU68M9ruJ4Ilr88aUczVcRxP253rI
// 9Haza4PwcNsomHLCSJ1ymReA8sevbA7tzWPwlYMwAO2NAnIOmqrUwkoT94lCbuTc
// aB2D0/Z0LQY7BsVPbMYh5xFm3hI3FjX+cmCuewIDAQABAoIBAGrlgIFxySmLyL/o
// TdKPigExB5/m0Tmn/i/1zx6U+hNrD73DyCR9YkIe5YBSsRl5+VVBmSeWr12P0E8M
// pXpL2EqUaaaEG6Zz6b8nmqqdtqtxEbMZCxVHxZZb0yQnTmft3cDWB+nkc4bK3V4N
// NVFg2hvERhnpNvYxo890f2dYutAQioTssdNDo628QKrZBjz+CqWEkjp0F5LAska5
// rhRX25x5R0ih7CMrTLZt+Dzk2JbhRPVAlkj3d8a7amLjWoRyN2CD90tQOCzbNiZD
// UzvlV4s3idVAiaSClZwrkvL6uiC7qnSwRXCjJlMOeFdJy9JRMiAtX9L63OTQ+onC
// Z3VGTLECgYEA38A9lioJ3v0op9S0sdoq83cig4VWIVkDDDV/ofQH/xZB6D/tY9PS
// n80yQhi7tc+dae0AZYVoh4ifVsw0FfcwtoUDPL9cYi0GInhnAlTn7UuppWI2pOlG
// 2XazBFB26BNskvCYsKbsMiS+E5BoQHecmZOcBv251w4XZv2WVhBSLX0CgYEAvlgj
// yD/RguhIB0x+OgQmZPSGWQ8qXW7+e109RMQ59RGTsBCQq38FUZGrIJIbRBSmlybr
// r3PJXHiSFbt52KWIivN5k+bjuJKYpJEpfDlRG8Kp3HFZLrWTd5U00cdZxd8LDFvc
// INrhWI2GGVn8qdZdrszTlZPpS8AW1I3Nhl15bVcCgYEAxEYtgBlWWV53mGmVLGJ1
// xOZfx0FioZQkgUQ4tseLcC+FFwdk5Wn93CIzERoDJ2R88FtvOp8BZ8roA0rT8eTJ
// vYIGqfYvQwu90uUNb1UtsdHqeeIijxz3AnIGbSVseP35Axi8yFFU5lOmzSCi4tJJ
// 88oxV0yhBc4dp0GR6+MbQz0CgYBFUCFPlXW8vssj5UX96G72ylh16+DYf0eqMqzR
// 8sbMKCdosM+Ns8aDCpGPXcUSCJcVabXfgUFtK/a+dTOModLUDo9SPXzlRHTTUI0T
// 0GdpvXxPavM34CUgIbRHQ9m8BVmnmXfSewIeVgLkDnHEguxAcBQIXwFQdVWa9zxF
// VpqWJwKBgDSnlhgBDzM7E3rqfJLtxFd8/QX6k6ZhtNabO25MyBgwblBmHq2EIOrt
// jvwjJz2q+EpWR2iptUpJaqNTYJP4e8J+8nSotuVC1A022SBnWZvm+V2sEiCNc5+f
// ytszGwriSCNPW7m2PVhgC74NS9u+MOOqpRn9qSb4b3Zdm9kEhVng
// -----END RSA PRIVATE KEY-----
```

### MANUAL TEST WITH POSTMAN: ###
1. Upload the files to simulate a website
2. Upload the RSA public and private key into SSM
3. Register a user at - https://[your-oauth-domain]/{stage}/v2/signup?client_id=clientid1

Open PostMan:

1. SET: do not follow redirects
2. User click login link:  https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/login?client_id=clientid1
3. Authorization code quest + code challenge to Oauth provider - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?response_type=code&state=e7761619-867d-4591-ab8f-f516afebc1aa&code_challenge_method=S256&client_id=clientid1&code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth
4. Redirec to login page - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/v2/login?code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth&code_challenge_method=S256&client_id=clientid1&response_type=code&state=e7761619-867d-4591-ab8f-f516afebc1aa
5. Authenticate and consent/optin - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&response_type=code&state=e7761619-867d-4591-ab8f-f516afebc1aa&code_challenge_method=S256&client_id=clientid1&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth
6. Authorization code - https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/auth?redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth&code=0128cf82-808d-4949-bb77-a71fe2213750&state=e7761619-867d-4591-ab8f-f516afebc1aa&code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&client_id=clientid1
7. Request token - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/token?grant_type=authorization_code&code_verifier=bSsHtJBHWBSduNeZ-LA03w1LtKQTTGRVWN76YH5uE4l92e5j6_ijnqASPobIwsNNBqyxlVa9aGbTFvwSVDBqfRa7efsgF25to1M0UzYNUtoNft0rUD3QSbvTYYFEUcOsSLePXLKZXbvbVPMArKt-sqyYRiazeCXReCjIfOKLRdg&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth&client_id=clientid1&code=0128cf82-808d-4949-bb77-a71fe2213750
8. Now with the token we can call our protected API - eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.TXdR1GMY_5nqQLDTk3uSZlRjt7JeVdK8HUuTRo44-OU

### TEST FROM THE BROWSER: ###

1. Upload the files to simulate a website
2. Upload the RSA public and private key into SSM
3. Register a user at - https://[your-oauth-domain]/{stage}/v2/signup?client_id=clientid1
4. Open the page index.html
5. Insert https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/login?client_id=clientid1
6. Click login
7. Enter the data from point 2
8 Now with the token we can call our protected API - eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.TXdR1GMY_5nqQLDTk3uSZlRjt7JeVdK8HUuTRo44-OU

### LOAD TEST WITH POSTMAN: ###

As I am not the most excellent automation tester, I could not find a better and easier way. So, I created a collection of steps in POSTMAN, simulating the registration and login for each user generating 5K registration and 5K login. 
I run the same collection in an iteration setting the iteration and in parallel. 
Each Lambda has some latency. For example, the GET for login and signup requests are loading a page from S3 while the POST is doing a query to DynamoDB. 

In the graph below, you can see:

* Average
* P99
* P90

![picture](https://github.com/ymwjbxxq/rust_poc_oauth_flow/blob/main/readme/first-test.png)

### WHAT IS MISSING: ###
Many things :)

### Deploy ###

```bash
# Compile and prepare Lambda functions
make build

# Deploy the functions on AWS
make deploy

```

### Cleanup ###
```
make delete
```
