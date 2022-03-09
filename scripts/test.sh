#!/bin/bash

# https://doc.rust-lang.org/cargo/commands/cargo-test.html
cargo test
cargo test \
    --all-features \
    --all-targets \
    --benches \
    --bins \
    --color always \
    --examples \
    --future-incompat-report \
    --locked \
    --manifest-path Cargo.toml \
    --no-fail-fast \
    --release \
    --tests \
    --verbose \
    --workspace \
    -Z unstable-options
# --frozen \
# --offline
# --timings html
# --unit-graph \
