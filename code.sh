#!/bin/bash

if [ -z "$1" ];then
  arg1=""
else
  arg1="$1"
fi
source .env
if [ "$arg1" != "check" ];then
  export UNAME="$UNAME"
  export APP_EARTH_PATH="$APP_EARTH_PATH"
  export APP_NAME="$APP_NAME"
  export HOST_PORT="$HOST_PORT"
  export ENV_NAME="$ENV_NAME"
  username="$(whoami)"
  # using lima
  limalist=$(limactl list | grep $ENV_NAME)
  if [ -z "$limalist" ];then
    limactl start ${ENV_NAME}.yaml
  elif [ -n "$(echo $limalist | grep Stopped)" ];then
    limactl start $ENV_NAME
  fi
  # this is the first priority DOCKER_HOST (in my system, if you think it is not working, check your ~/.profile or~/.bash_profile or ~/.bashrc or similar this case.)
  export DOCKER_HOST="unix:///$HOME/.lima/$ENV_NAME/sock/docker.sock"
  if [ "$arg1" != "test" ];then
    code .
  fi
fi
echo "UNAME:$UNAME"
echo "APP_EARTH_PATH:$APP_EARTH_PATH"
echo "APP_NAME:$APP_NAME"
echo "HOST_PORT:$HOST_PORT"
echo "DOCKER_HOST:$DOCKER_HOST"
echo "ENV_NAME:$ENV_NAME"
echo "docker context:$(docker context ls|grep ' \* ')"
