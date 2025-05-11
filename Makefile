include .env

all: dev

dev:
	@TELOXIDE_TOKEN=${TELOXIDE_TOKEN} \
		cargo watch -x run
