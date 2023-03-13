#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# --network=host
name=registry.cn-shanghai.aliyuncs.com/d2jvkpn/rust-web
tag=dev

bash deployments/git-build-info.sh > .env_git-build-info

for base in $(awk '/^FROM/{print $2}' ${_path}/Dockerfile); do
    docker pull --quiet $base
    bn=$(echo $base | awk -F ":" '{print $1}')
    if [[ -z "$bn" ]]; then continue; fi
    docker images --filter "dangling=true" --quiet "$bn" | xargs -i docker rmi {}
done &> /dev/null

docker build --no-cache -f ${_path}/Dockerfile --tag $name:$tag ./
docker image prune --force --filter label=stage=rust-web_builder &> /dev/null

docker push $name:$tag

for img in $(docker images --filter "dangling=true" --quiet $image); do
    docker rmi $img || true
done &> /dev/null
