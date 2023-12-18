if [ -z $2 ]; then
  curl -X GET "http://localhost:8082/api/${1}" -H "X-Remote-User: xilef"
else
  curl -X GET "http://localhost:8082/api/${1}/${2}" -H "X-Remote-User: xilef"
fi
