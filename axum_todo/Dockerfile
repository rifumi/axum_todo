# syntax=docker/dockerfile:1
FROM rust:1.67

ARG USER_ID
ARG GROUP_ID
ARG USER_NAME
ARG APP_EARTH_PATH
ARG APP_NAME

EXPOSE 3000

# setup the debugger port
ENV DEBUG_PORT=9229
EXPOSE ${DEBUG_PORT}

ENV LANG C.UTF-8

RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN <<EOT
  groupadd -g $GROUP_ID $USER_NAME || :
  useradd -l -u $USER_ID -g $GROUP_ID -s /bin/bash -m $USER_NAME || :
  chown -R $USER_NAME /home/$USER_NAME
  #if [ -z "$(ls -ld /home/$USER_NAME | grep $USER_NAME)" ];then
  if [[ "$(find /home/$USER_NAME ! -user $USER_NAME)" ]];then
    echo "Failure chown /home/$USER_NAME $USER_NAME."
    exit 1
  fi
  #if [ ! -d ${APP_EARTH_PATH} ];then
  #  mkdir -p ${APP_EARTH_PATH}
  #fi
  #chown -R $USER_NAME ${APP_EARTH_PATH}
EOT

# for Rust develop-environment
RUN <<EOT
  set -eux
  apt-get update
  apt-get upgrade -y
  apt-get -y install sudo
EOT

# add vim and tools
RUN apt-get install -y vim curl lsof iproute2 nodejs
# copy script for CMD
COPY scripts/setup-rust-devenv.sh /home/${USER_NAME}/
COPY scripts/setup-rust-web.sh  /home/${USER_NAME}/
RUN chown $USER_NAME /home/${USER_NAME}/setup-rust-devenv.sh
RUN chown $USER_NAME /home/${USER_NAME}/setup-rust-web.sh
# ${USER_NAME}にsudo権限を追加する
RUN usermod -aG sudo "$USER_NAME"
RUN <<EOT
  #!/bin/bash
  set -eux
  echo "${USER_NAME}:${USER_NAME}"|chpasswd
EOT

USER $USER_NAME
# vi settings
RUN <<EOT
  o=/home/$USER_NAME/.vimrc
  {
    echo 'set fenc=utf-8'
    echo 'set number'
    echo 'set nowrap'
    echo 'set list'
    echo 'set listchars=tab:>\ ,trail:\ '
    echo 'set tabstop=4'
    echo 'set shiftwidth=4'
    echo 'set expandtab'
    echo 'set autoindent'
    echo 'set smartindent'
    echo 'set hlsearch'
    echo 'set clipboard=unnamed'
    echo 'set modeline'
    echo 'set modelines=5'
    echo 'syntax on'
  } >> $o
EOT

WORKDIR /home/${USER_NAME}
RUN <<EOT
#!/bin/bash
  o="./setup.sh"
  {
    echo '#!/bin/bash'
    echo "# $(pwd)"
    echo 'set -eux'
    echo "USER_NAME=$USER_NAME"
    echo "APP_NAME=$APP_NAME"
    echo "APP_EARTH_PATH=$APP_EARTH_PATH"
    cat "./setup-rust-devenv.sh"
    cat "./setup-rust-web.sh"
  } > $o
  chmod u+x $o
EOT

RUN cat /home/${USER_NAME}/setup.sh

CMD ["/bin/bash", "-c", "./setup.sh;/bin/bash"]
