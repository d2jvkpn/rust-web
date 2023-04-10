#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
branch=$1
tag=$branch

function on_exit {
    git checkout dev
}
trap on_exit EXIT

git checkout $branch
git pull --no-edit

####
# --network=host
name=registry.cn-shanghai.aliyuncs.com/d2jvkpn/rust-web-backend
dfile=${_path}/Dockerfile.backend

bash deployments/git-build-info.sh > .git-build-info.yaml

for base in $(awk '/^FROM/{print $2}' $dfile); do
    echo ">>> docker pull $base"
    docker pull --quiet $base
    bn=$(echo $base | awk -F ":" '{print $1}')
    if [[ -z "$bn" ]]; then continue; fi
    docker images --filter "dangling=true" --quiet "$bn" | xargs -i docker rmi {} || true
done
# &> /dev/null

docker build --no-cache -f $dfile --tag $name:$tag ./
docker image prune --force --filter label=stage=rust-web-backend_builder &> /dev/null

docker push $name:$tag

for img in $(docker images --filter "dangling=true" --quiet $name); do
    docker rmi $img || true
done &> /dev/null
