#!/bin/bash

source .env
export DOCKER_HOST=unix:///$HOME/.lima/$ENV_NAME/sock/docker.sock
echo "\$1=$1"

if [ "$2" == "curl" ] && [ "$2" == "root" ]; then
  ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $id)
  echo "curl 127.0.0.1:$HOST_PORT"
  curl --connect-timeout 3 http://127.0.0.1:$HOST_PORT
  echo "curl 127.0.0.1:3000"
  curl --connect-timeout 3 http://127.0.0.1:3000
  echo ""
  echo "curl 0.0.0.0:$HOST_PORT"
  curl --connect-timeout 3 http://0.0.0.0:$HOST_PORT
  echo "curl 0.0.0.0:3000"
  curl --connect-timeout 3 http://0.0.0.0:3000
  echo ""
  echo "curl localhost:$HOST_PORT"
  curl --connect-timeout 3 http://localhost:$HOST_PORT
  echo "curl localhost:3000"
  curl --connect-timeout 3 http://localhost:3000
  echo ""
  echo "curl ip always fail?"
  echo "curl $ip:$HOST_PORT"
  curl --connect-timeout 3 http://$ip:$HOST_PORT
  echo "curl $ip:3000"
  curl --connect-timeout 3 http://$ip:3000
  echo ""
fi
if [ "$2" == "curl" ] && [ "$2" == "users" ];then
  ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $id)
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://127.0.0.1:$HOST_PORT/users;echo "";
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://127.0.0.1:3000/users;echo "";
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://0.0.0.0:$HOST_PORT/users;echo "";
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://0.0.0.0:3000/users;echo "";
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://localhost:$HOST_PORT/users;echo "";
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://localhost:3000/users;echo "";
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://$ip:$HOST_PORT/users;echo "";
  curl --connect-timeout 3 -H "Content-Type: application/json" -d '{"username": "Ohtani"}' http://$ip:3000/users;echo "";
fi

if [ "$2" == "/bin/bash" ];then
  real_c_name=$(docker ps --format "{{.Names}}" | grep "$APP_NAME")
  if [ -z "$real_c_name" ];then
    echo "starting container not found."
    exit 1
  fi
  echo "$@"
  docker container exec -it $real_c_name $@
fi
