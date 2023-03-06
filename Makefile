
build:
	make build --directory=services/oauth
	make build --directory=services/website
	make build --directory=services/edge

deploy:
	make deploy --directory=services/oauth
	make deploy --directory=services/website
	make deploy --directory=services/edge

delete:
	make delete --directory=services/edge
	make delete --directory=services/website
	make delete --directory=services/oauth