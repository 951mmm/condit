#! /bin/bash
base=localhost:2333/api/v1
jsonFile=./register.json
compress="jq -c '' <"
method=POST

token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjQzYTZhNWM2LTI0NzUtNDJkZC04NzQ4LWZlZDU1ZGZhZDZjYyIsInVzZXJuYW1lIjoiamFjayIsImV4cCI6MTY4MTgxODM4Nn0.WQBsl2PWXnu9S-5K3AxEQPalYOlSijreQW9OFqhD_nI


curl $cfg\
    -d $(eval $compress $jsonFile) \
    "${base}/users" \
    -H 'Authorization:Token '${token}'' \
    -H "Authorization:Fxxk" \
    -X $method 

