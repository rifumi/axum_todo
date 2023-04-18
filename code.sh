#!/bin/bash

source .env
if [ "$2" != "check" ];then
  export UNAME="$UNAME"
  export APP_EARTH_PATH="$APP_EARTH_PATH"
  export APP_NAME="$APP_NAME"
  export HOST_PORT="$HOST_PORT"
  username="$(whoami)"
  # using lima
  # this is the first priority DOCKER_HOST (in my system, if you think it is not working, check your ~/.profile or~/.bash_profile or ~/.bashrc or similar this case.)
  export DOCKER_HOST="unix:///$HOME/.lima/$ENV_NAME/sock/docker.sock"
  if [ "$2" != "test" ];then
    code .
  fi
fi
echo "UNAME:$UNAME"
echo "APP_EARTH_PATH:$APP_EARTH_PATH"
echo "APP_NAME:$APP_NAME"
echo "HOST_PORT:$HOST_PORT"
echo "DOCKER_HOST:$DOCKER_HOST"
echo "DOCKER_CONTEXT:$DOCKER_CONTEXT"
echo "docker context:$(docker context ls|grep ' \* ')"
