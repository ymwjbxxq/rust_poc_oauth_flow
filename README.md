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

You can find more details [here](https://github.com/ymwjbxxq/protect-sensitive-data-with-aws-lambda-edge)

### What I have Learnt ###

* [Error handling](https://www.sheshbabu.com/posts/rust-error-handling/)
* [Safe JSON representations with Rust](https://n14n.dev/articles/2021/safe-json-representations-with-rust/)
* [Implementing a Trait on a Type](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)
* ["Type-Driven API Design in Rust" by Will Crichton](https://www.youtube.com/watch?v=bnnacleqg6k)

### MANUAL TEST WITH POSTMAN: ###
1. Upload the files to simulate a website
2. Register a user at - https://[your-oauth-domain]/{stage}/v2/signup?client_id=clientid1

Open PostMan:

1. SET: do not follow redirects
2. User click login link:  https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/login?client_id=clientid1
3. Authorization code quest + code challange to Oauth provider - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?response_type=code&state=e7761619-867d-4591-ab8f-f516afebc1aa&code_challenge_method=S256&client_id=clientid1&code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth
4. Redirec to login page - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/v2/login?code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth&code_challenge_method=S256&client_id=clientid1&response_type=code&state=e7761619-867d-4591-ab8f-f516afebc1aa
5. Authenticate and consent/optin - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/authorize?code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&response_type=code&state=e7761619-867d-4591-ab8f-f516afebc1aa&code_challenge_method=S256&client_id=clientid1&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth
6. Authorization code - https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/auth?redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth&code=0128cf82-808d-4949-bb77-a71fe2213750&state=e7761619-867d-4591-ab8f-f516afebc1aa&code_challenge=E6eArpYbPr7JJ12opY7fQ6r6fD-KfZcadk6VQIgeDls&client_id=clientid1
7. Request token - https://[your-oauth-domain].execute-api.eu-central-1.amazonaws.com/{stage}/token?grant_type=authorization_code&code_verifier=bSsHtJBHWBSduNeZ-LA03w1LtKQTTGRVWN76YH5uE4l92e5j6_ijnqASPobIwsNNBqyxlVa9aGbTFvwSVDBqfRa7efsgF25to1M0UzYNUtoNft0rUD3QSbvTYYFEUcOsSLePXLKZXbvbVPMArKt-sqyYRiazeCXReCjIfOKLRdg&redirect_uri=https%3A%2F%2F[your-app-domain].execute-api.eu-central-1.amazonaws.com%2F%2Fauth&client_id=clientid1&code=0128cf82-808d-4949-bb77-a71fe2213750
8. Now with the token we can call our protected API - eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.TXdR1GMY_5nqQLDTk3uSZlRjt7JeVdK8HUuTRo44-OU

### TEST FROM THE BROWSER: ###

1. Upload the files to simulate a website
2. Register a user at - https://[your-oauth-domain]/{stage}/v2/signup?client_id=clientid1
3. Open the page index.html
4. Insert https://[your-app-domain].execute-api.eu-central-1.amazonaws.com/{stage}/login?client_id=clientid1
5. Click login
6. Enter the data from point 2
7 Now with the token we can call our protected API - eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.TXdR1GMY_5nqQLDTk3uSZlRjt7JeVdK8HUuTRo44-OU

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
