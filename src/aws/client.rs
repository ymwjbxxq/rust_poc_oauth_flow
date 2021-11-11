pub struct AWSConfig {
  config: aws_types::config::Config,
}

impl AWSConfig {
   pub fn set_config(config: aws_types::config::Config) -> Self {
    Self { 
      config: config 
    }
  }

  pub fn dynamo_client(&self) -> aws_sdk_dynamodb::Client {
    let dynamo_db_client = aws_sdk_dynamodb::Client::new(&self.config);
    return dynamo_db_client;
  }

  pub fn s3_client(&self) -> aws_sdk_s3::Client {
    let s3_client = aws_sdk_s3::Client::new(&self.config);
    return s3_client;
  }
}

#[derive(Clone)]
pub struct AWSClient {
  pub dynamo_db_client: Option<aws_sdk_dynamodb::Client>,
  pub s3_client: Option<aws_sdk_s3::Client>,
}
