FUNCTIONS := app-random-api app-lambda-authorizer app-auth app-login oauth-consent-post oauth-login-post oauth-optin-post oauth-signup-post oauth-consent-get oauth-login-get oauth-optin-get oauth-signup-get oauth-token oauth-authorize

ARCH := aarch64-unknown-linux-gnu

build:
	rm -rf ./build
	rm -rf ./target
	cross build --release --target $(ARCH)
	mkdir -p ./build
	${MAKE} ${MAKEOPTS} $(foreach function,${FUNCTIONS}, build-${function})

build-%:
	mkdir -p ./build/$*
	cp -v ./target/$(ARCH)/release/$* ./build/$*/bootstrap
	# zip -j ./build/$*/$*.zip ./build/$*/bootstrap

deploy:
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name rust-oautflow-oauth --template-file template-oauth.yml  
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name rust-oautflow-app   --template-file template-app.yml

delete:
	sam delete --profile test --stack-name rust-oautflow-app
	sam delete --profile test --stack-name rust-oautflow-oauth