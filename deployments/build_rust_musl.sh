#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
name=registry.cn-shanghai.aliyuncs.com/d2jvkpn/rust-musl
tag=1
dfile=${_path}/Dockerfile.rust-musl

docker build --no-cache -f $dfile --tag $name:$tag ./
docker push $name:$tag

for img in $(docker images --filter "dangling=true" --quiet $name); do
    docker rmi $img || true
done &> /dev/null
