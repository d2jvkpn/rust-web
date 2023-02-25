#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

####
rustc --version
cargo --version

cargo install cargo-tarpaulin cargo-audit cargo-edit cargo-expand cargo-udeps cargo-vendor
rustup component add clippy rustfm

# cargo new rust-web && cd rust-web
mkdir -p rust-web && cd rust-web
cargo init

echo 'use_small_heuristics = "Max"' >> .rustfmt.toml

mkdir -p tests


git init
git remote add origin git@github.com:d2jvkpn/rust-web.git
# git add .
# git commit -am "init"
# git push -u origin main

####
cargo add actix-web@4 actix-rt@2 actix-service@2
cargo add tokio@1 --features=full
cargo add serde@1 --features=derive
cargo add uuid --features=v4,serde

cargo add anyhow@1 chrono@0.4 futures-util@0.3 serde_json@1 \
  structopt@0.3 thiserror@1 config@0.13 futures@0.3 \
  derive_more@0.99 once_cell@1

cargo add sqlx --features=runtime-actix-rustls,macros,postgres,uuid,chrono,migrate,offline

cargo add --dev reqwest

cat Cargo.toml

#
cargo clippy
cargo clippy -- -D warnings

cargo fmt
cargo fmt -- --check

cargo audit
cargo expand

cargo run
curl localhost:8000
