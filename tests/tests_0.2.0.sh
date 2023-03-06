#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12QWas!@"}'

token="Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2NzgxMDY4MzIsImV4cCI6MTY3ODEwODYzMiwidG9rZW5JZCI6ImQ5MmNiZGUwLTEwZDctNDJkMi1iOTg3LTEwZDQxZmJlY2ZjYSIsInVzZXJJZCI6MSwicm9sZSI6ImFkbWluIiwicGxhdGZvcm0iOiJ1bmtub3duIn0.MJawh8Ju34OQ0dur3TRBRTOYh6HDh-IGFBrb-moLKYo"

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12qwAS!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12qwAS!@"}'
