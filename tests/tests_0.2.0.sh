#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12QWas!@"}'

token="Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2NzgxNjI4MTEsImV4cCI6MTY3ODE2NDYxMSwidG9rZW5JZCI6Ijk1OGMyNmRiLTYxOGEtNDM1MC1hNTQ2LWM4NTRjYmEwYTZiYiIsInVzZXJJZCI6MSwicm9sZSI6ImFkbWluIiwicGxhdGZvcm0iOiJ1bmtub3duIn0.ePyZkG91NKmeV95-a_3jFcWzsnxtxTsXfdcllcSkQIM"

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12qwAS!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12qwAS!@"}'

curl -i -X GET -H "Authorization: $token" "$address/api/auth/user/details"
