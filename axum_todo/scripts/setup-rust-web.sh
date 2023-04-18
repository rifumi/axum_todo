#!/bin/bash
#set -eux
if [ -z "$APP_ROOT_PATH" ];then
  APP_ROOT_PATH=$APP_EARTH_PATH/$APP_NAME
fi
if [ ! -d "$APP_ROOT_PATH" ];then
  echo "Please exec setup-devenv-rust.sh <app name> before exec this script." >> error.txt
  exit 2;
fi

TOML_PATH="$APP_ROOT_PATH/Cargo.toml"
if [ -z "$(grep thiserror $TOML_PATH)" ];then
  cd $APP_ROOT_PATH
  cargo add axum
  cargo add hyper --features full
  cargo add tokio --features full
  cargo add tower
  cargo add mime
  cargo add serde --features derive
  cargo add serde_json
  cargo add tracing
  cargo add tracing-subscriber --features env-filter
  cargo add anyhow
  cargo add thiserror
  cargo add axum-macros
fi
