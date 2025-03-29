#! /bin/bash

# action: combine (= compose) the bsn validator wasm component with the wasm components that tests the string length
# performs the bsn nine proof

# install:
# npm install -g @bytecodealliance/jco

echo "command: jco transpile \
./target/wasm/bsn_validator_composed.wasm \
--out-dir ./js/components/bsn-validator"

jco transpile \
  ./target/wasm/bsn_validator_composed.wasm \
  --out-dir ./js/components/bsn-validator