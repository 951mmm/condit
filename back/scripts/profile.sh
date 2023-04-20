#! /bin/bash
base=localhost:2333/api/v1
jsonFile=$PWD/login.json
compress="jq -c '' <"
method=$1

url="${base}/profiles/$2"

if [ $method != "GET" ]; then
url=${base}/profiles/$2/follow
fi

tokenFile=$PWD/token

if [ ! -e $tokenFile ]; then
echo "please execute 'touch $PWD/token' and write the token"
exit 1
fi

token=`cat $tokenFile`


curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

