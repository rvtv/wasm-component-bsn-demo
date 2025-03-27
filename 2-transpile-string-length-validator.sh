#! /bin/bash

# install:
# npm install -g @bytecodealliance/jco

echo "command: jco transpile \
./target/wasm/string_length_validator.wasm \
--out-dir ./js/components/string-length-validator
"

jco transpile \
  ./target/wasm/string_length_validator.wasm \
  --out-dir ./js/components/string-length-validator