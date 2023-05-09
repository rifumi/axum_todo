#!/bin/bash

source .env
export DOCKER_HOST=unix://$HOME/.lima/$ENV_NAME/sock/docker.sock
devcontainer build
