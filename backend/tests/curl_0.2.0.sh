#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3011

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@noreply.local", "password": "12QWas!@"}'


token="xxxx"

curl -i -X POST -H "content-type: application/json" -H "Authorization: Bearer $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12qwAS!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@noreply.local", "password": "12qwAS!@"}'

curl -i -X GET -H "Authorization: Bearer $token" "$address/api/auth/user/details"

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/register" \
  -d '{"email": "d2jvkpn@noreply.local", "name": "Rover", "birthday": "2006-01-02", "password": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/register" \
  -d '{"email": "alice@noreply.local", "name": "Alice", "birthday": "2006-01-02", "password": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "alice@noreply.local", "password": "12QWas!@"}'


####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@noreply.local", "password": "12QWas!@"}'

refresh_token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Nzg4NDUxNDEsImV4cCI6MTY3ODkzMTU0MSwidG9rZW5JZCI6IjM1MDczMDdlLTQ1NTgtNGUyNy1iYTJhLWQwN2YxYWQ1MjQ3OSIsInRva2VuS2luZCI6InJlZnJlc2giLCJ1c2VySWQiOjEsInJvbGUiOiJhZG1pbiIsInBsYXRmb3JtIjoidW5rbm93biJ9.ysl6CFuMv_WV_ERB46HXC0Tf6Rzy7ojnrJ0lpYa7Irc"

body=$(jq -cn --arg refresh_token $refresh_token '{refreshToken: $refresh_token}')

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/refresh_token" \
  -d $body
