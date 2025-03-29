#! /bin/bash

# action: combine (= compose) the nineproof wasm component with the wasm component that tests the string length

echo "command: wac plug \
./target/wasm32-wasip2/release/bsn_nineproof_validator_ext.wasm \
--plug ./target/wasm/string_length_validator.wasm \
--output ./target/wasm/bsn_nineproof_validator_composed.wasm"

wac plug \
  ./target/wasm32-wasip2/release/bsn_nineproof_validator_ext.wasm \
  --plug ./target/wasm/string_length_validator.wasm \
  --output ./target/wasm/bsn_nineproof_validator_composed.wasm
