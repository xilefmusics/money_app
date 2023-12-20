#!/bin/bash

curl \
  -X DELETE \
  -H "X-Remote-User: xilefmusics" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d @04-transactions.json \
  "http://localhost:8082/api/transactions" \
  --fail
