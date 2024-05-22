# WASM Check Runner

This repo contains a sample "check" in `./smoke-a`.
This "check" is implemented in rust, but compiled to wasm.

Next, the compiled wasm is "component-ified" via `wasm-tools`.
See [./smoke-a/README.md](./smoke-a/README.md) for instructions on how to build.

An example of how to _run_ the check is available in `./wasm-check-runner`


The binary blob `wasi_snapshot_preview1.reactor.wasm` was downloaded from https://github.com/bytecodealliance/wasmtime/releases/tag/v21.0.1

