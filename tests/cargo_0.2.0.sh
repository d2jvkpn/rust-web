#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


####
cargo fmt

cargo check
cargo check --tests

####
cargo test --bin -- --show-output

cargo test --lib -- users::t_serde --show-output

cargo test --tests users -- --show-output

####
cargo clean
