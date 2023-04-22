#! /bin/bash
base=localhost:2333/api/v1
# base=https://api.realworld.io/api
method=$1

tokenFile=$PWD/token

if [ ! -e $tokenFile ]; then
echo "please execute 'touch $PWD/token' and write the token"
exit 1
fi

token=`cat $tokenFile`
slugFile=$PWD/slug
slug=$(cat $slugFile)
url=${base}/articles/$slug

compress="jq -c '' <"
jsonFile=$PWD/comment.json
json=$(eval $compress $jsonFile)

if [ $method = "POST" ] || [ $method = "GET" ]; then
url=${base}/articles/$slug/comments
curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -X $method \
    -d "$json"

elif [ $method = "DELETE" ]; then
commentFile=$PWD/comment
comment=$(cat $commentFile)
url=${base}/articles/$slug/comments/$comment
curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -X $method
fi