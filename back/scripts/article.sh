#! /bin/bash
base=localhost:2333/api/v1
# base=https://api.realworld.io/api
method=$1



url="${base}/articles?$2"

# if [ $method = "GET" ]; then
# url=${base}/user
# fi


curl $cfg \
    $url \
    -X $method \

