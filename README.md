# WebAssembly Components Citizen Service Number Validation Demo

## Introduction

This project contains a (somewhat contrived) example of validating Dutch Citizen Service Numbers (BSN) with WebAssembly Components.

By running the shell scripts in the root folder of this project in the order that they are numbered, the demo can be completed. See the next paragraph for the tooling that needs to be installed to run the demo successfully.

## Tooling

To run the demo the following tools need to be installed on the machine.

### NodeJS

You will need a working Node.js installation, version 22.x or higher. See the Node.js [site](https://nodejs.org/) for instructions.

### Rust

A Rust 1.85.1 installation or newer is needed. See the Rust [site](https://www.rust-lang.org/tools/install) for details on how to install.

### Rust wasm32-wasip2 compilation target

```shell
rustup target add wasm32-wasip2
```

### Cargo Component

```shell
cargo install cargo-component
```

### Wac

```shell
cargo install wac-cli
```

### JCO

```shell
npm install -g @bytecodealliance/jco
```

### Componentize-JS

```shell
npm install -g @bytecodealliance/componentize-js
```

## WebAssembly (Component) resources

- [webassembly.org](https://webassembly.org/)
- [Bytecode Alliance](https://bytecodealliance.org/)
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
- [The WebAssembly Component Model](https://component-model.bytecodealliance.org/introduction.html)
- [GitHub - WebAssembly/WASI: WebAssembly System Interface](https://github.com/WebAssembly/WASI)
- [The `wasm-bindgen` Guide](https://rustwasm.github.io/wasm-bindgen/introduction.html)
- [GitHub - bytecodealliance/wit-bindgen: A language binding generator for WebAssembly interface types](https://github.com/bytecodealliance/wit-bindgen)
- [GitHub - bytecodealliance/cargo-component: A Cargo subcommand for creating WebAssembly components based on the component model proposal.](https://github.com/bytecodealliance/cargo-component)
- [GitHub - bytecodealliance/wac: WebAssembly Composition (WAC) tooling](https://github.com/bytecodealliance/wac)
- [GitHub - bytecodealliance/jco: JavaScript toolchain for working with WebAssembly Components](https://github.com/bytecodealliance/jco)
- [GitHub - bytecodealliance/ComponentizeJS: JS -> WebAssembly Component](https://github.com/bytecodealliance/ComponentizeJS)
- [Wasmtime](https://wasmtime.dev/)

