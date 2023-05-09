#!/bin/bash

## cargo new $APP_NAMEを実行するディレクトリパス
EARTH_PATH=$APP_EARTH_PATH

if [ -d "$EARTH_PATH/$APP_NAME" ];then
  echo 'already setup finished.'
  exit 0
fi

if [ "$(whoami)" != "$USER_NAME" ];then
  echo "error. $(whoami) != $USER_NAME." >> error.txt
  exit 2
fi
if [ ! -d "$EARTH_PATH" ];then
  echo "failure, not found $EARTH_PATH" >> error.txt
  exit 3
fi
if [ -z "$(which cargo)" ];then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=stable -y
fi
source /usr/local/cargo/env
rustup update stable
rustup -V > ${EARTH_PATH}/verinfo.txt
cargo -V >> ${EARTH_PATH}/verinfo.txt
APP_ROOT_PATH=$EARTH_PATH/$APP_NAME
cd $EARTH_PATH
if [ ! -d $APP_ROOT_PATH ];then
  cargo new $APP_NAME
fi
rustup component add clippy
rustup component add rustfmt
