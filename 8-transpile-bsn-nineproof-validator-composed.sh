#! /bin/bash

# action: create a JavaScript module that calls the bsn_nineproof_composed_validator.wasm component

# install:
# npm install -g @bytecodealliance/jco

echo "command: jco transpile ./target/wasm/bsn_nineproof_validator_composed.wasm --out-dir ./js/components/bsn-nineproof-validator-composed"

jco transpile \
  ./target/wasm/bsn_nineproof_validator_composed.wasm \
  --out-dir ./js/components/bsn-nineproof-validator-composed