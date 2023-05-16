#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


[ $# -eq 0 ] && { >&2 echo "Argument {branch} is required!"; exit 1; }

####
# ENV_File="$1"
cfg_name=$1
ENV_File=configs/$cfg_name.env
BuildLocal=$(printenv BuildLocal || true)

. frontend/$ENV_File
# BRANCH is defined in $ENV_File
TAG=$BRANCH
echo ">>> BRANCH: $BRANCH, TAG: $TAG, ENV_File: $ENV_File"

function on_exit {
    git checkout dev
}
trap on_exit EXIT

[[ "$BuildLocal" != "true" ]] && \
{
  git checkout $BRANCH
  git pull --no-edit
}

####
dfile=${_path}/Dockerfile.frontend
now=$(date +'%FT%T%:z')
name="registry.cn-shanghai.aliyuncs.com/d2jvkpn/rust-web-frontend"
image="$name:$TAG"
echo ">>> building image: $image..."

echo ">>> Pull base images..."

[[ "$BuildLocal" != "true" ]] && \
for base in $(awk '/^FROM/{print $2}' $dfile); do
    docker pull --quiet $base
    bn=$(echo $base | awk -F ":" '{print $1}')
    if [[ -z "$bn" ]]; then continue; fi

    docker images --filter "dangling=true" --quiet "$bn" | xargs -i docker rmi {} || true
done &> /dev/null

docker build --no-cache -f $dfile -t $image \
  --build-arg=ENV_File=$ENV_File            \
  --build-arg=BuildLocal="$BuildLocal"      \
  --build-arg=REACT_APP_BuildTime=$now      \
  ./

docker image prune --force --filter label=stage=rust-web-frontend_builder &> /dev/null

for img in $(docker images --filter=dangling=true $name --quiet); do
    docker rmi $img &> /dev/null
done

#### push to registry
echo ">>> pushing image: $image"
docker push $image
