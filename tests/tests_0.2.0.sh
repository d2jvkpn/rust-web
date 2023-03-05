#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12QWas!@"}'

token="Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2NzgwMDUyMTksImV4cCI6MTY3ODAwNzAxOSwidXNlcklkIjoxLCJyb2xlIjoiYWRtaW4ifQ.QYv_JDycmL-Ob0zveRObjwjqE5KAQUON_95a9uk2F8k"

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12qwAS!@"}'
