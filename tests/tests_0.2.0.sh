#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


address=http://localhost:3000

####
curl -i -X POST -H "content-type: application/json" "$address/api/open/user/login" \
  -d '{"email": "admin@users.noreply.github.com", "password": "12QWas!@"}'
