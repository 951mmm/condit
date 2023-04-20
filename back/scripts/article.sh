#! /bin/bash
base=localhost:2333/api/v1
# base=https://api.realworld.io/api
method=$1
token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjkyOTgzZGM2LTRlNWUtNGFhNi1iYzdmLTJmODM4ZGMzOGNmOSIsInVzZXJuYW1lIjoiamFjayIsImV4cCI6MTY4MjE3MDMzNX0.JZFbkf73GLLjff7jb3GJVMJ-aUx40T9c-VoIOkfBmP8"


url="${base}/articles?$2"

# if [ $method = "GET" ]; then
# url=${base}/user
# fi


curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

