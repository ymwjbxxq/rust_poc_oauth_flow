build:
	sam build --template-file ./deployment/template.yml

deploy:
	sam deploy --no-fail-on-empty-changeset --no-confirm-changeset --profile test --region us-east-1 --stack-name edge --template-file ./deployment/template.yml

delete:
	sam delete --profile test --stack-name edge --template-file ./deployment/template.yml