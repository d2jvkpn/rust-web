#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12QWas!@"}'

token="Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2NzgxNTU0ODEsImV4cCI6MTY3ODE1NzI4MSwidG9rZW5JZCI6IjdlNTljMjQ4LTUyYzMtNDZiNC04MzY3LWQzZTZlNjcxMTNkZiIsInVzZXJJZCI6MSwicm9sZSI6ImFkbWluIiwicGxhdGZvcm0iOiJ1bmtub3duIn0.wmHgq7tMcj59nsqHthv-_ywUFw7aWOMuBJ_BT7q9gGg"

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12qwAS!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12qwAS!@"}'

curl -i -X GET -H "Authorization: $token" "$address/api/auth/user/details"
