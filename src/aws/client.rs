pub struct AWSConfig {
    config: aws_types::config::Config,
}

impl AWSConfig {
    pub fn set_config(config: aws_types::config::Config) -> Self {
        Self { config }
    }

    pub fn dynamo_client(&self) -> aws_sdk_dynamodb::Client {
        aws_sdk_dynamodb::Client::new(&self.config)
    }

    pub fn s3_client(&self) -> aws_sdk_s3::Client {
        aws_sdk_s3::Client::new(&self.config)
    }
}

#[derive(Clone)]
pub struct AWSClient {
    pub dynamo_db_client: Option<aws_sdk_dynamodb::Client>,
    pub s3_client: Option<aws_sdk_s3::Client>,
}
