#! /bin/bash

echo "command: wac plug \
./target/wasm/bsn_validator.wasm \
--plug ./target/wasm/string_length_validator.wasm \
--plug ./target/wasm/bsn_nineproof_validator_composed.wasm \
--output ./target/wasm/bsn_validator_composed.wasm"

wac plug \
  ./target/wasm/bsn_validator.wasm \
  --plug ./target/wasm/string_length_validator.wasm \
  --plug ./target/wasm/bsn_nineproof_validator_composed.wasm \
  --output ./target/wasm/bsn_validator_composed.wasm
