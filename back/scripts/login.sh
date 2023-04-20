#! /bin/bash
base=localhost:2333/api/v1
jsonFile=./login.json
compress="jq -c '' <"
method=$1

url="${base}/users/login"

if [ $method = "GET" ]; then
url=${base}/user
fi

# token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjkyOTgzZGM2LTRlNWUtNGFhNi1iYzdmLTJmODM4ZGMzOGNmOSIsInVzZXJuYW1lIjoiamFjayIsImV4cCI6MTY4MjE3MDMzNX0.JZFbkf73GLLjff7jb3GJVMJ-aUx40T9c-VoIOkfBmP8"

token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjkyOTgzZGM2LTRlNWUtNGFhNi1iYzdmLTJmODM4ZGMzOGNmOSIsInVzZXJuYW1lIjoiamFjayIsImV4cCI6MTY4MjAwODY4NH0.JSaaG_P2fUBx9ibyKGD9wHRVxALtmvepu4jndn4IeVA"


curl $cfg\
    -d $(eval $compress $jsonFile) \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

