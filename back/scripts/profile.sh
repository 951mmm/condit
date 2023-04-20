#! /bin/bash
base=localhost:2333/api/v1
jsonFile=./login.json
compress="jq -c '' <"
method=$1

url="${base}/profiles/$2"

if [ $method != "GET" ]; then
url=${base}/profiles/$2/follow
fi

token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjkyOTgzZGM2LTRlNWUtNGFhNi1iYzdmLTJmODM4ZGMzOGNmOSIsInVzZXJuYW1lIjoiamFjayIsImV4cCI6MTY4MjE3MDMzNX0.JZFbkf73GLLjff7jb3GJVMJ-aUx40T9c-VoIOkfBmP8"


curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

