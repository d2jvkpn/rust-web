#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# --network=host
docker build --no-cache -f ${_path}/build.df --tag rust-web:dev ./

docker image prune --force --filter label=stage=rust-web_builder &> /dev/null
