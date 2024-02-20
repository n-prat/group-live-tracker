#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd frontend; CARGO_TARGET_DIR=../target-trunk trunk serve --address 0.0.0.0 --port 8080 --tls-key-path ../key.pem --tls-cert-path ../cert.pem' & \
 JWT_SECRET=123456789 bash -c 'cd server; cargo watch -- cargo run -- --port 8081')
