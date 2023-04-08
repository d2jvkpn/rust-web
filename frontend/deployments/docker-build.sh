#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# BRANCH="$1"
# ENV_File="$2"
# TAG=$3

BRANCH=$1
# "./configs/$APP_ENV.env"
ENV_File="$2"
TAG=$BRANCH

echo ">>> BRANCH: $BRANCH, TAG: $TAG, ENV_File: $ENV_File"
. $ENV_File

function on_exit {
    git checkout dev
}
trap on_exit EXIT

####
git checkout $BRANCH
git pull --no-edit

####
df=${_path}/Dockerfile

name="registry.cn-shanghai.aliyuncs.com/d2jvkpn/rust-web-frontend"
image="$name:$TAG"
echo ">>> building image: $image..."

echo ">>> Pull base images..."
for base in $(awk '/^FROM/{print $2}' $df); do
    docker pull --quiet $base
    bn=$(echo $base | awk -F ":" '{print $1}')
    if [[ -z "$bn" ]]; then continue; fi

    docker images --filter "dangling=true" --quiet "$bn" |
      xargs -i docker rmi {}
done &> /dev/null

docker build --no-cache -f $df -t $image  \
  --build-arg=ENV_File=$ENV_File          \
  --build-arg=REACT_APP_BuildTime=$(date +'%FT%T%:z')  \
  ./

docker image prune --force --filter label=stage=rust-web-frontend_builder &> /dev/null

for img in $(docker images --filter=dangling=true $name --quiet); do
    docker rmi $img &> /dev/null
done

#### push to registry
echo ">>> pushing image: $image"
docker push $image
