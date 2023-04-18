#!/bin/bash

source .env
export DOCKER_HOST=unix://$HOME/.lima/$ENV_NAME/sock/docker.sock
docker compose -f docker-compose.prod.yml up -d $@
