#! /usr/bin/env bash
# set -eu -o pipefail
# _wd=$(pwd)
# _path=$(dirname $0 | xargs -i readlink -f {})

[ $# -eq 0 ] && { >&2 echo "Argument {format}(json or yaml) is required!"; exit 1; }
format=$1

if [[ "$format" != "yaml" && "$format" != "json" ]]; then
    >&2 echo "\!\!\! invalid format name: $format"; exit 1;
fi

build_time=$(date +'%FT%T%:z')
git_branch=$(git rev-parse --abbrev-ref HEAD)

git_commit_id=$(git rev-parse --verify HEAD) # git log --pretty=format:'%h' -n 1
git_commit_time=$(git log -1 --format="%at" | xargs -I{} date -d @{} +%FT%T%:z)
git_tree_state="clean"

uncommitted=$(git status --short)
unpushed=$(git diff origin $GIT_Branch..HEAD --name-status)
[ ! -z "$uncommitted$unpushed" ] && git_tree_state="dirty"

if [[ "$format" == "json" ]]; then
    jq -n \
      --arg build_time "${build_time}" \
      --arg git_branch "$git_branch" \
      --arg git_commit_id "$git_commit_id" \
      --arg git_commit_time "$git_commit_time" \
      --arg git_tree_state "$git_tree_state" \
      '{build_time: $build_time, git_branch: $git_branch, git_commit_id: $git_commit_id, git_commit_time: $git_commit_time, git_tree_state: $git_tree_state}'

    exit 0
fi

cat <<EOF
build_time: $build_time
git_branch: $git_branch
git_commit_id: $git_commit_id
git_commit_time: $git_commit_time
git_tree_state: $git_tree_state
EOF

# number=24
# --argjson number "${number:-42}"
