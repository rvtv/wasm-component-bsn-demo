#! /bin/bash

# action: make a wasm component from a JavaScript module that checks for string length and performs the bsn nineproof

# install:
# npm install -g @bytecodealliance/componentize-js

mkdir -p target/wasm
echo "command: jco componentize \
./js/packages/bsn-validator/src/bsnValidator.js \
--wit ./wit \
--world-name bsn-validator \
--disable all \
--out ./target/wasm/bsn_validator.wasm"

jco componentize \
  ./js/packages/bsn-validator/src/bsnValidator.js \
  --wit ./wit \
  --world-name bsn-validator \
  --disable all \
  --out ./target/wasm/bsn_validator.wasm
