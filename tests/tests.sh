#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "d2jvkpn@users.noreply.github.com", "name": "Rover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "alice@users.noreply.github.com", "name": "Alice", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "bob@users.noreply.github.com", "name": "Bob", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/register \
  -d '{"email": "evol@users.noreply.github.com", "name": "Evol", "birthday": "2006-01-02"}'

####
curl -i -X POST -H "content-type: application/json" $address/api/open/user/update/4 \
  -d '{"name": "Rover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/update/4 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/update_v2a/4 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $address/api/open/user/update_v2b?id=4 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

####
curl -i -X GET $address/api/open/user/query

curl -i -X GET "$address/api/open/user/query?page_no=1&page_size=5&order_by=name&asc=true"

curl -i -X GET "$address/api/open/user/find?id=4"

curl -i -X GET "$address/api/open/user/update_status?id=4&status=blocked"
