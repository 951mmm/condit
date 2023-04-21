#! /bin/bash
base=localhost:2333/api/v1
# base=https://api.realworld.io/api
method=$1
func=$2

tokenFile=$PWD/token

if [ ! -e $tokenFile ]; then
echo "please execute 'touch $PWD/token' and write the token"
exit 1
fi

token=`cat $tokenFile`

url="${base}/articles?$2"

# if [ $func = "feed" ]; then
# url="${base}/articles/${func}"
# fi

curlCli="curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method"

compress="jq -c '' <"
jsonFile=$PWD/article.json
json=$(eval $compress $jsonFile)
url=${base}/articles

if [ $method = "POST" ]; then
curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -X $method \
    -d "$json"
fi
