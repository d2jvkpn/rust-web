#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


addr=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" $addr/open/user/register \
  -d '{"email": "d2jvkpn@users.noreply.github.com", "name": "Rover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $addr/open/user/update/4 \
  -d '{"name": "Rover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $addr/open/user/update/4 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $addr/open/user/update_v2a/4 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'

curl -i -X POST -H "content-type: application/json" $addr/open/user/update_v2b?id=4 \
  -d '{"name": "RoverRover", "birthday": "2006-01-02"}'
