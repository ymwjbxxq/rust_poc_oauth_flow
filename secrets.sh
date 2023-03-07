#!/bin/bash

echo "Deploying SSM parameters"

# this should be dynamic for each new client
aws ssm  put-parameter --name "/f977ec729e094141b6c1d01f50cba6ce/public_key" --value TODO --type SecureString --region eu-central-1 --no-overwrite --tier Intelligent-Tiering --tags Key=Name,Value=f977ec729e094141b6c1d01f50cba6ce/public_key --profile test
aws ssm put-parameter --name "/f977ec729e094141b6c1d01f50cba6ce/private_key" --value TODO --type SecureString --region eu-central-1 --no-overwrite --tier Intelligent-Tiering --tags Key=Name,Value=f977ec729e094141b6c1d01f50cba6ce/private_key --profile test
aws ssm put-parameter --name "/f977ec729e094141b6c1d01f50cba6ce/secret_key" --value TODO --type SecureString --region eu-central-1 --no-overwrite --tier Intelligent-Tiering --tags Key=Name,Value=f977ec729e094141b6c1d01f50cba6ce/private_key --profile test