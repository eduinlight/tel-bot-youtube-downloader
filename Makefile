include .env

all: dev

dev:
	@TELOXIDE_TOKEN=${TELOXIDE_TOKEN} \
		ALLOWED_USERS=${ALLOWED_USERS} \
		cargo watch -x run
