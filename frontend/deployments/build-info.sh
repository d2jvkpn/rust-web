#! /usr/bin/env bash
# set -eu -o pipefail
# _wd=$(pwd)
# _path=$(dirname $0 | xargs -i readlink -f {})

BUILD_Time=$(date +'%FT%T%:z')
GIT_Branch=$(git rev-parse --abbrev-ref HEAD)

GIT_Commit_ID=$(git rev-parse --verify HEAD) # git log --pretty=format:'%h' -n 1
GIT_Commit_Time=$(git log -1 --format="%at" | xargs -I{} date -d @{} +%FT%T%:z)
GIT_Tree_State="clean"

uncommitted=$(git status --short)
unpushed=$(git diff origin/$GIT_Branch..HEAD --name-status)
[ ! -z "$uncommitted$unpushed" ] && GIT_Tree_State="dirty"

cat <<EOF
build_time: $BUILD_Time
git_branch: $GIT_Branch
git_commit_id: $GIT_Commit_ID
git_commit_time: $GIT_Commit_Time
git_tree_state: $GIT_Tree_State
EOF
