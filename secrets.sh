#!/bin/bash

echo "Deploying SSM parameters"

# this should be dynamic for each new client
aws ssm  put-parameter --name "/clientid1/public_key" --value TODO --type SecureString --region eu-central-1 --no-overwrite --tier Intelligent-Tiering --tags Key=Name,Value=clientid1/public_key --profile test
aws ssm put-parameter --name "/clientid1/private_key" --value TODO --type SecureString --region eu-central-1 --no-overwrite --tier Intelligent-Tiering --tags Key=Name,Value=clientid1/private_key --profile test
aws ssm put-parameter --name "/clientid1/secret_key" --value TODO --type SecureString --region eu-central-1 --no-overwrite --tier Intelligent-Tiering --tags Key=Name,Value=clientid1/private_key --profile test