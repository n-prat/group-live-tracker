#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
JWT_SECRET=123456789 CARGO_TARGET_DIR=../target-trunk trunk build --release --public-url "group-live-tracker/"
popd

# cargo run --bin server --release -- --port 8080 --static-dir ./docs
