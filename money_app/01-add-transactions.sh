#!/bin/bash

curl \
  -X POST \
  -H "X-Remote-User: xilefmusics" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d @01-transactions.json \
  "http://localhost:8082/api/transactions" \
  --fail
