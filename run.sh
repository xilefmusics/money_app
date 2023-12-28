#!/bin/bash
if [ "${1}" = "db" ]; then
  surreal start --log debug --user root --pass root memory --allow-scripting
elif [ "${1}" = "backend" ]; then
  cd backend && cargo watch -cqx run
elif [ "${1}" = "frontend" ]; then
  cd frontend && npm run dev
elif [ "${1}" = "frontend-install" ]; then
  cd frontend && npm install
elif [ "${1}" = "proxauth" ]; then
  CONFIG_FILE="./proxauth-config.yaml" proxauth
elif [ "${1}" = "add-transactions" ]; then
  curl \
    -X POST \
    -H "X-Remote-User: xilef" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d @transactions.json \
    "http://localhost:8082/api/transactions" \
    --fail
elif [ "${1}" = "get" ]; then
  if [ -z "${2}" ]; then
    echo "usage: run.sh get <endpoint>"
    exit 1
  fi
  curl -X GET "http://localhost:8082/api/${2}" -H "X-Remote-User: xilef"
elif [ "${1}" = "docker-build" ]; then
  docker build -t money-app .
else
  echo "usage: run.sh <command>"
    exit 1
fi
