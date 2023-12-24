#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

[ $# -lt 3 ] && { echo "Arguments {tag}, {app_env} and {port} are required!"; exit 1; }

TAG="$1"
APP_ENV="$2"
# APP_ENV="$TAG"
PORT="$3"

#### deploy
export TAG="${TAG}" APP_ENV="${APP_ENV}" PORT="${PORT}"
[ -f docker-compose.yaml ] || envsubst < ${_path}/docker_deploy.yaml > docker-compose.yaml

docker-compose pull
docker-compose up -d

docker logs rust-web-db_${APP_ENV}
docker logs rust-web-backend_${APP_ENV}
docker logs rust-web-frontend_${APP_ENV}
