#! /bin/bash

# action: componentize a JavaScript module that checks for sting length

# install:
# npm install -g @bytecodealliance/componentize-js
mkdir -p target/wasm
echo "command: componentize-js ./js/packages/string-length-validator/src/stringLengthValidator.js --wit ./wit/validator.wit --world-name string-length-validator --out ./target/wasm/string_length_validator.wasm"
componentize-js \
  ./js/packages/string-length-validator/src/stringLengthValidator.js \
  --wit ./wit/validator.wit --world-name string-length-validator \
  --out ./target/wasm/string_length_validator.wasm

# alternative

# install:
# npm install -g @bytecodealliance/jco
# npm install -g @bytecodealliance/componentize-js
#
# run:
# jco componentize ./js/packages/string-length-validator/src/stringLengthValidator.js --wit ./wit/validator.wit --world-name string-length-validator --out ./target/wasm/string_length_validator.wasm