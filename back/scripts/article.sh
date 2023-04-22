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
url="${base}/articles?$2"

# get private feed
if [ $# -eq 2 ]; then
    func=$2
    if [ $func = "feed" ]; then
    url="${base}/articles/${func}"
    fi
fi

# json
compress="jq -c '' <"
jsonFile=$PWD/article.json
json=$(eval $compress $jsonFile)

if [ $method = "POST" ]; then

url=${base}/articles

curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -X $method \
    -d "$json"
elif 
    [ $method = "PUT" ] || 
    [ $method = "DELETE" ] || 
    [ $method = "GET" ]; 
then
# put article with `slug`
slugFile=$PWD/slug
slug=$(cat $slugFile)
url=${base}/articles/$slug
curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -X $method \
    -d "$json"

elif [ $method = "LIST" ]; then
curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X GET

elif [ $method = "FAV" ]; then 
slugFile=$PWD/slug
slug=$(cat $slugFile)
url=${base}/articles/$slug/favorite
curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -X POST \
    -d "$json"

elif [ $method = "UNFAV" ]; then
slugFile=$PWD/slug
slug=$(cat $slugFile)
url=${base}/articles/"$slug"/favorite
curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -X DELETE \
    -d "$json"

fi