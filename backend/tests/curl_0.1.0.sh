#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3011

####
curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "d2jvkpn@noreply.local", "name": "Rover", "birthday": "2006-01-02", "password": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "alice@noreply.local", "name": "Alice", "birthday": "2006-01-02", "password": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "bob@noreply.local", "name": "Bob", "birthday": "2006-01-02", "password": "12QWas!@"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "evol@noreply.local", "name": "Evol", "birthday": "2006-01-02", "password": "12QWas!@"}'

####
curl -i -X POST -H "content-type: application/json" $address/api/open/user/update/30 \
  -d '{"name": "Rover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/update/30 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/update_v2a/30 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/update_v2b?id=30 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

####
curl -i -X GET $address/api/open/user/query

curl -i -X GET "$address/api/open/user/query?page_no=1&page_size=5&order_by=name&asc=true"

curl -i -X GET "$address/api/open/user/find?id=4"

curl -i -X GET "$address/api/open/user/find?email=d2jvkpn@noreply.local"

curl -i -X GET "$address/api/open/user/update_status?id=30&status=blocked"

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "d2jvkpn@noreply.local", "password": "12QWas!@"}'

token="Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Nzc5MTE0MzUsImV4cCI6MTY3NzkxMzIzNSwidXNlcklkIjo0MCwicm9sZSI6Im1lbWJlciJ9.UsrirBjc_hCvXHeOqHWG89hsfPYQ0-tNkaEuKcVUP-8"

curl -i -X POST -H "content-type: application/json" $address/api/auth/user/update/40 \
  -d '{"name": "Rover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  $address/api/auth/user/update/40 \
  -d '{"name": "Rover", "birthday": "2023-01-02"}'

curl -i -X POST -H "content-type: application/json" -H "Authorization: $token" \
  $address/api/auth/user/update \
  -d '{"name": "Rover", "birthday": "2023-01-02"}'
