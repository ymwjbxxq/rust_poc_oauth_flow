FUNCTIONS := app-random-api app-lambda-authorizer app-auth app-login oauth-consent-post oauth-login-post oauth-optin-post oauth-signup-post oauth-get-page oauth-token oauth-authorize

ARCH := aarch64-unknown-linux-gnu
ARCH_SPLIT = $(subst -, ,$(ARCH))

build:
ifeq ("$(shell zig targets | jq -r .native.cpu.arch)-$(shell zig targets | jq -r .native.os)-$(shell zig targets | jq -r .native.abi)", "$(word 1,$(ARCH_SPLIT))-$(word 3,$(ARCH_SPLIT))-$(word 4,$(ARCH_SPLIT))")
	@echo "Same host and target. Using native build"
	cargo build --release --target $(ARCH)
else
	@echo "Different host and target. Using zigbuild"
	cargo zigbuild --release --target $(ARCH)
endif

	rm -rf ./build
	mkdir -p ./build
	${MAKE} ${MAKEOPTS} $(foreach function,${FUNCTIONS}, build-${function})

build-%:
	mkdir -p ./build/$*
	cp -v ./target/$(ARCH)/release/$* ./build/$*/bootstrap

deploy:
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name oauth --template-file template-oauth.yml  
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --profile test --stack-name app   --template-file template-app.yml

delete:
	sam delete --profile test --stack-name app
	sam delete --profile test --stack-name oauth