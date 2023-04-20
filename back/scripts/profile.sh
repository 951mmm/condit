#! /bin/bash
base=localhost:2333/api/v1
jsonFile=./login.json
compress="jq -c '' <"
method=$1

url="${base}/profiles/$2"

if [ $method != "GET" ]; then
url=${base}/profiles/$2/follow
fi

token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjkyOTgzZGM2LTRlNWUtNGFhNi1iYzdmLTJmODM4ZGMzOGNmOSIsInVzZXJuYW1lIjoiamFjayIsImV4cCI6MTY4MjAwOTEzOH0.LWcdhjuqHIAhQY8SpL0cwzyhQTJQEsN2j3hSAD_PYUM"


curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

