#!/bin/bash
#set -eux
if [ -z "$APP_ROOT_PATH" ];then
  APP_ROOT_PATH=$APP_EARTH_PATH/$APP_NAME
fi
if [ ! -d "$APP_ROOT_PATH" ];then
  echo "Please exec setup-devenv-rust.sh <app name> before exec this script." >> error.txt
  exit 2;
fi

cd $APP_ROOT_PATH
TOML_PATH="$APP_ROOT_PATH/Cargo.toml"
if [ -z "$(grep axum $TOML_PATH)" ];then
  cargo add axum
fi
if [ -z "$(grep hyper $TOML_PATH)" ];then
  cargo add hyper --features full
fi
if [ -z "$(grep tokio $TOML_PATH)" ];then
  cargo add tokio --features full
fi
if [ -z "$(grep tower $TOML_PATH)" ];then
  cargo add tower
fi
if [ -z "$(grep mime $TOML_PATH)" ];then
  cargo add mime
fi
if [ -z "$(grep serde $TOML_PATH)" ];then
  cargo add serde --features derive
fi
if [ -z "$(grep serde_json $TOML_PATH)" ];then
  cargo add serde_json
fi
if [ -z "$(grep tracing $TOML_PATH)" ];then
  cargo add tracing
fi
if [ -z "$(grep tracing-subscriber $TOML_PATH)" ];then
  cargo add tracing-subscriber --features env-filter
fi
if [ -z "$(grep anyhow $TOML_PATH)" ];then
  cargo add anyhow
fi
if [ -z "$(grep thiserror $TOML_PATH)" ];then
  cargo add thiserror
fi
if [ -z "$(grep http-body $TOML_PATH)" ];then
  cargo add http-body
fi
if [ -z "$(grep validator $TOML_PATH)" ];then
  cargo validator --features derive
fi
if [ -z "$(grep axum-macros $TOML_PATH)" ];then
  cargo add axum-macros
fi
if [ -z "$(grep tower_http $TOML_PATH)" ];then
  cargo add tower_http
fi
