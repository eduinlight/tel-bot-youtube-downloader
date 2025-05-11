#!/usr/bin/env bash

# ARGUMENTS
# $1 => the name of the docker swarm service
# $2 => the image that the $1 service should have after the deployment
# $3 => min number of replicas

SERVICE_NAME=${1}
SERVICE_IMAGE_NAME=${2}
MIN_REPLICAS=${3:-0}

echoerr() {
  awk " BEGIN { print \"$@\" > \"/dev/fd/2\" }"
}

get-service-real-image-name() {
  docker service inspect ${SERVICE_NAME} | jq -r '.[].Spec.Labels."com.docker.stack.image"'
}

get-service-replicas() {
  docker service inspect ${SERVICE_NAME} | jq -r '.[].Spec.Mode.Replicated.Replicas'
}

printf "⚙︎ Velidating that the service ${SERVICE_NAME} exists: "
while ! docker service inspect ${SERVICE_NAME} &> /dev/null; do
  sleep 1
done
echo "✅"

printf "⚙︎ Validating that the service ${SERVICE_NAME} has the image ${SERVICE_IMAGE_NAME}: "
while [ $(get-service-replicas) -lt ${MIN_REPLICAS} ] || [ $(get-service-real-image-name) != ${SERVICE_IMAGE_NAME} ]; do
  sleep 1
done
echo "✅"
