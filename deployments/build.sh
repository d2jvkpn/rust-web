#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# --network=host
name=registry.cn-shanghai.aliyuncs.com/d2jvkpn/rust-web
tag=dev

docker build --no-cache -f ${_path}/build.df --tag $name:$tag ./

docker image prune --force --filter label=stage=rust-web_builder &> /dev/null

docker push $name:$tag
