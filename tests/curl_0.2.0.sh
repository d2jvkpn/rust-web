#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12QWas!@"}'

# {"code":0,"data":{"tokens":{"aliveMins":10,"refreshHrs":24,"refreshToken":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Nzg3OTM3MDYsImV4cCI6MTY3ODg4MDEwNiwidG9rZW5JZCI6ImE5NjU1YWIxLWM4YTItNGE2Ni1iMmU2LTUzMTU3ZGFkNjRjMiIsInRva2VuS2luZCI6InJlZnJlc2giLCJ1c2VySWQiOjEsInJvbGUiOiJhZG1pbiIsInBsYXRmb3JtIjoidW5rbm93biJ9.Fb1-xucN8sw9mUMCqEqXWGrLbIbqmylhUYO5b7rp3R8","accessToken":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Nzg3OTM3MDYsImV4cCI6MTY3ODc5NDMwNiwidG9rZW5JZCI6ImE5NjU1YWIxLWM4YTItNGE2Ni1iMmU2LTUzMTU3ZGFkNjRjMiIsInRva2VuS2luZCI6InRlbXAiLCJ1c2VySWQiOjEsInJvbGUiOiJhZG1pbiIsInBsYXRmb3JtIjoidW5rbm93biJ9.DK8vj0I42O9WXDN6jq0dI3eX9tQGzVQWLrdiBqbjixU"},"user":{"birthday":"2006-01-02","createdAt":"2023-03-08T03:59:59.426363Z","email":"admin@users.noreply.github.com","id":1,"name":"admin","role":"admin","status":"oK","updatedAt":"2023-03-08T03:59:59.426363Z"}},"msg":"ok","requestId":"f3d294ca-0417-4742-a981-3bba1db16e0c"}

token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Nzg3OTM3MDYsImV4cCI6MTY3ODc5NDMwNiwidG9rZW5JZCI6ImE5NjU1YWIxLWM4YTItNGE2Ni1iMmU2LTUzMTU3ZGFkNjRjMiIsInRva2VuS2luZCI6InRlbXAiLCJ1c2VySWQiOjEsInJvbGUiOiJhZG1pbiIsInBsYXRmb3JtIjoidW5rbm93biJ9.DK8vj0I42O9WXDN6jq0dI3eX9tQGzVQWLrdiBqbjixU"

curl -i -X POST -H "content-type: application/json" -H "Authorization: Bearer $token" \
  "$address/api/auth/user/change_password" \
  -d '{"oldPassword": "12QWas!@", "newPassword": "12qwAS!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12qwAS!@"}'

curl -i -X GET -H "Authorization: Bearer $token" "$address/api/auth/user/details"

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/register" \
  -d '{"email": "d2jvkpn@users.noreply.github.com", "name": "Rover", "birthday": "2006-01-02", "password": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/register" \
  -d '{"email": "alice@users.noreply.github.com", "name": "Alice", "birthday": "2006-01-02", "password": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "alice@users.noreply.github.com", "password": "12QWas!@"}'


####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12QWas!@"}'

refresh_token="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Nzg4NDUxNDEsImV4cCI6MTY3ODkzMTU0MSwidG9rZW5JZCI6IjM1MDczMDdlLTQ1NTgtNGUyNy1iYTJhLWQwN2YxYWQ1MjQ3OSIsInRva2VuS2luZCI6InJlZnJlc2giLCJ1c2VySWQiOjEsInJvbGUiOiJhZG1pbiIsInBsYXRmb3JtIjoidW5rbm93biJ9.ysl6CFuMv_WV_ERB46HXC0Tf6Rzy7ojnrJ0lpYa7Irc"

body=$(jq -cn --arg refresh_token $refresh_token '{refreshToken: $refresh_token, "password": "12QWas!@"}')

curl -i -X POST -H "content-type: application/json" "$address/api/open/user/refresh_token" \
  -d $body
