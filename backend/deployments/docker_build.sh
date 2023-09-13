#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
[ $# -eq 0 ] && { >&2 echo "Argument {branch} is required!"; exit 1; }

branch=$1
tag=${Tag:-$branch}
# BUILD_Vendor=$(printenv BUILD_Vendor || true)
BUILD_Vendor=${BUILD_Vendor:-"true"}

function on_exit {
    git checkout dev
}
trap on_exit EXIT

git checkout $branch
[[ "$BUILD_Vendor" != "true" ]] && git pull --no-edit

####
# --network=host
name=registry.cn-shanghai.aliyuncs.com/d2jvkpn/rust-web-backend
dfile=${_path}/Dockerfile

bash ${_path}/build_info.sh yaml > src/_build_info.yaml
mkdir -p vendor

[[ "$BUILD_Vendor" == "true" ]] && cargo vendor --versioned-dirs && cd -

[[ "$BUILD_Vendor" != "true" ]] && \
for base in $(awk '/^FROM/{print $2}' $dfile); do
    echo ">>> docker pull $base"
    docker pull --quiet $base
    bn=$(echo $base | awk -F ":" '{print $1}')
    if [[ -z "$bn" ]]; then continue; fi
    docker images --filter "dangling=true" --quiet "$bn" | xargs -i docker rmi {} || true
done
# &> /dev/null

docker build --no-cache --build-arg=BUILD_Vendor="$BUILD_Vendor" -f $dfile --tag $name:$tag ./

docker image prune --force --filter label=stage=rust-web-backend_builder &> /dev/null

docker push $name:$tag

for img in $(docker images --filter "dangling=true" --quiet $name); do
    docker rmi $img || true
done &> /dev/null
