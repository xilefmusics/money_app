#!/bin/bash

curl \
  -X POST \
  -H "X-Remote-User: xilefmusics" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  "http://localhost:8082/api/import" \
  -d "$(cat ./06-import-n26.csv)" \
  --fail
