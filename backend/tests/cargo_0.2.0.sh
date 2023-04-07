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

#### integration tests only (tests/user.rs)
cargo test --test user -- --show-output

cargo test --test user -- users_refresh_token --show-output

####
cargo clean
