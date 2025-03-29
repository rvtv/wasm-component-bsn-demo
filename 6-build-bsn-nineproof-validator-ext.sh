#! /bin/bash

# action: build a wasm component that performs the nine proof validation on a bsn that itself calls a wasm component to
# validate the string length
# (coded in Rust)

echo "command: cargo build --target wasm32-wasip2 --release"

cargo build \
  --package bsn-nineproof-validator-ext \
  --target wasm32-wasip2 \
  --release
