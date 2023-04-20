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


curl $cfg \
    $url \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

