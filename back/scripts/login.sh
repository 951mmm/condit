#! /bin/bash
base=localhost:2333/api/v1
jsonFile=$PWD/login.json

if [ ! -e $jsonFile ]; then
echo "please execute 'touch $jsonFile' and write the token"
exit 1
fi

compress="jq -c '' <"
method=$1

url="${base}/users/login"

if [ $method = "GET" ]; then
url=${base}/user
fi

tokenFile=$PWD/token

if [ ! -e $tokenFile ]; then
echo "please execute 'touch $PWD/token' and write the token"
exit 1
fi

token=`cat $tokenFile`


curl $cfg\
    -d $(eval $compress $jsonFile) \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

