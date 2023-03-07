#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

[ $# -eq 0 ] && { 2&>1 echo "please provide branch"; exit 1; }

GIT_BRANCH=$1
git checkout $GIT_BRANCH # --force
# git pull --no-edit

#### build
BUILD_TIME=$(date +'%FT%T%:z')
GIT_COMMIT=$(git rev-parse --verify HEAD) # git log --pretty=format:'%h' -n 1
GIT_TIME=$(git log -1 --format="%at" | xargs -I{} date -d @{} +%FT%T%:z)
# git tag $git_tag
# git push origin $git_tag
GIT_TREE_STATE="clean"

uncommitted=$(git status --short)
unpushed=$(git diff origin/$GIT_BRANCH..HEAD --name-status)
[[ ! -z "$uncommitted$unpushed" ]] && GIT_TREE_STATE="dirty"
