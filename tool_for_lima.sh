#!/bin/bash

source .env
export DOCKER_HOST=unix:///$HOME/.lima/$ENV_NAME/sock/docker.sock
echo "\$1=$1"
id=$(docker ps -aqf "name=$APP_NAME")

if [ "$1" == "curl" ] && [ "$2" == "root" ]; then
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
  exit 0;
fi
if [ "$1" == "curl" ] && [ "$2" == "users" ];then
  ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $id)
  send_json="{\"username\": \"Ohtani\"}" 
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://127.0.0.1:$HOST_PORT/users;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://127.0.0.1:3000/users;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://0.0.0.0:$HOST_PORT/users;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://0.0.0.0:3000/users;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://localhost:$HOST_PORT/users;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://localhost:3000/users;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://$ip:$HOST_PORT/users;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://$ip:3000/users;echo "";
  exit 0;
fi
if [ "$1" == "curl" ] && [ "$2" == "todos" ];then
  ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $id)
  send_json="{\"text\": \"todo title.\"}"
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://127.0.0.1:$HOST_PORT/todos;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://127.0.0.1:3000/todos;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://0.0.0.0:$HOST_PORT/todos;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://0.0.0.0:3000/todos;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://localhost:$HOST_PORT/todos;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://localhost:3000/todos;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://$ip:$HOST_PORT/todos;echo "";
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://$ip:3000/todos;echo "";
  exit 0;
fi
if [ "$1" == "test" ] && [ "$2" == "todos" ] && [ "$3" == "create" ] && [ -n "$4" ];then
  #ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $id)
  send_json="{\"text\": \"$4\"}"
  curl -X POST --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://localhost:$HOST_PORT/todos;echo "";
  exit 0;
elif [ "$1" == "test" ] && [ "$2" == "todos" ] && [ "$3" == "read" ] && [ -n "$4" ];then
  #ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $id)
  if [ "$4" == "all" ];then
    curl -X GET --connect-timeout 3 -H "Content-Type: application/json" http://localhost:$HOST_PORT/todos;echo "";
  else
    expr "$4" + 1 >&/dev/null
    if [ $? -ge 2 ];then
      echo "error. \$4 invalid.";
      exit 1;
    fi
    # $4 is number
    curl -X GET --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://localhost:$HOST_PORT/todos/$4;echo "";
  fi
  exit 0;
elif [ "$1" == "test" ] && [ "$2" == "todos" ] && [ "$3" == "update" ] && [ -n "$4" ] && [ -n "$5" ];then
  expr "$4" + 1 >&/dev/null
  if [ $? -ge 2 ];then
    echo "\$4 invalid.";
    exit 1;
  fi
  send_json="{\"text\": \"$5.\"}"
  curl -X PATCH --connect-timeout 3 -H "Content-Type: application/json" -d "$send_json" http://localhost:$HOST_PORT/todos/$4;echo "";
  exit 0;
elif [ "$1" == "test" ] && [ "$2" == "todos" ] && [ "$3" == "delete" ] && [ -n "$4" ];then
  expr "$4" + 1 >&/dev/null
  if [ $? -ge 2 ];then
    echo "error. \$4 invalid.";
    exit 1;
  fi
  # $4 is number
  curl -X DELETE --connect-timeout 3 http://localhost:$HOST_PORT/todos/$4;echo "";
  exit 0;
fi

if [ "$1" == "/bin/bash" ];then
  real_c_name=$(docker ps --format "{{.Names}}" | grep "$APP_NAME" | head -n 1)
  if [ -z "$real_c_name" ];then
    echo "starting container not found."
    exit 1
  fi
  echo "$@"
  docker container exec -it $real_c_name $@
elif [ "$1" == "log" ];then
  real_c_name=$(docker ps --format "{{.Names}}" | grep "$APP_NAME" | head -n 1)
  if [ -z "$real_c_name" ];then
    echo "starting container not found."
    exit 1
  fi
  echo "\$APP_NAME:$APP_NAME"
  echo "\$real_c_name:$real_c_name"
  docker container logs $real_c_name
fi
