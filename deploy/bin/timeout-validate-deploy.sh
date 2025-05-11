#!/usr/bin/env bash

# ARGUMENTS
# same arguments than validate-service.sh

SERVICE_NAME=${1}
SERVICE_IMAGE_NAME=${2}

echoerr() {
  awk " BEGIN { print \"$@\" > \"/dev/fd/2\" }"
}

timeout 5m ./deploy/bin/validate-deploy.sh ${@}
if [ $? -eq 124 ]; then
  echoerr "\n✘ Timeout after 5 munites. The image for the service [${SERVICE_NAME}] does not match with [${SERVICE_IMAGE_NAME}]"
  exit 1
fi
