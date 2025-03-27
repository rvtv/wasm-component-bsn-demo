#! /bin/bash

# install:
# npm install -g @bytecodealliance/jco

echo "command: jco transpile ./target/wasm32-wasip2/release/bsn_nineproof_validator.wasm --out-dir ./js/components/bsn-nineproof-validator"
jco transpile \
  ./target/wasm32-wasip2/release/bsn_nineproof_validator.wasm \
  --out-dir ./js/components/bsn-nineproof-validator