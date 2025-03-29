#! /bin/bash

# action: build a wasm component that performs the nine proof validation on a bsn (coded in Rust)

echo "command: cargo build --target wasm32-wasip2 --release"

cargo build \
  --package bsn-nineproof-validator \
  --target wasm32-wasip2 \
  --release

# before wasm32-p2 it was "cargo component build"