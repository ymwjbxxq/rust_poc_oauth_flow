# PRUST Authorization Code Flow with Proof Key for Code Exchange (PKCE) #

This project try to replicate the [Authorization Code Flow with Proof Key for Code Exchange (PKCE)](https://auth0.com/docs/flows/authorization-code-flow-with-proof-key-for-code-exchange-pkce)

### HOW IT WORKS ###

![picture](https://github.com/ymwjbxxq/rust_poc_oauth_flow/blob/main/readme/auth-sequence-auth-code-pkce.png)

The project is divided in the following parts:

* OAuth service
* Website service that could be any client that wants to use the OAuth service
* Edge service

The OAuth service could have N wesites that want to use its services and so each website has itws own configuration.
The configuration contains

- Custom PII_fields that OAuth service encrypt at edge for maximum security
- RSA certificate generate for each website
- Custom pages like the login

Once the user register, can obviously loing and if it is all successfully At the end of the flow, we should be able to see the JWT token:
```App
Authorization: eyJhbGciOiJIUzI1NiJ9.T0RBVUxDQVk0V0k1S1ZQU01DUkZCTExBOEs3QURNUEQzWUM4WFdUNVQ1UDRVUElRREw.956xdJUWC4mfDJlohbqP2kqFUNoAPlZ8nRRJCfzo1KI
```


### Security: ###

To reduce the exposure of sensitive data, I use AWS CloudFront with Lambda@Edge to:

* Intercept data
* Encrypt at the edge with the website RSA
* Pass it through before the application can process it, reducing exposure.

Lambda edge will load the client config from S3 and from there take SSM key to load RSA public key to encrypt sensitive data.

To avoid [Cross Site Request Forgery (CSRF) attacks](https://owasp.org/www-community/attacks/csrf) the 
This project try to replicate the [Authorization Code Flow with Proof Key for Code Exchange (PKCE) use two parameters:

- state: is sent with the authorization request, and compare it with the state value that is returned by the authorization server in the authorization response
- code_challange: It ensures that the client that receives the authorization code is the same client that initially requested the authorization

They can be stored in the "HTTP-only" cookies but they can be blocked by broswer (Cognito mode)
They can be stored in the "localStorage" but they could be altered by client-side code running in the browser

So to avoid all of them I store them into DynamoDB but i could be any cache service and I compared against them setting also a relative low TTL of 1 minute.

### MANUAL SETUP: ###

Lambda@Edge does not support envriment variables and so I cannot pass with the CI the OAuth S3 bucket name where the config is stored.

**S3**:

- In reality this could be the S3 multi-region endpoitn or a custom domain that point to a specific place. So for this test I must harcode the name in the edge lambda

**SSM**:

- Run `./secrets.sh` and after upload an RSA certificate. For the sake of the test I generate one here https://cryptotools.net/rsagen


### MANUAL TEST WITH POSTMAN using Lambda@edge (all encrypted): ###

Register a user at - https://[cloudfront-domain]/{stage}/v2/signup?client_id=clientid1 
Body:
```
{"email":"a@a.com","password":"aaaa","family_name":"aaaa","given_name":"aaa","is_consent":"true","is_optin":"true","remember":"true"}
```

**NOTE**:

- All the URL are visibile in the response header section under Location.
- Copy the Location and paste for the nexst step

1. SET in Postman do not follow redirects
2. GET -> user click login -  https://[your-app-domain]/{stage}/login?client_id=clientid1
3. GET -> Authorization code quest + code challenge to Oauth provider - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?....
4. POST -> Redirec to login page - https://[cloudfront-domain]/{stage}/v2/login?.....
5. GET -> Authenticate and consent/optin - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?....
6. GET -> Authorization code - https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/auth?.....
7. GET -> Request token - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/token?....
8. Now with the token we can call our protected API - eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.TXdR1GMY_5nqQLDTk3uSZlRjt7JeVdK8HUuTRo44-OU

### TEST FROM THE BROWSER without Lambda@edge (not encrypted): ###

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
