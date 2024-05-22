cargo build --target wasm32-wasi
wasm-tools component new ./target/wasm32-wasi/debug/smoke_a.wasm -o smoke_a_component.wasm --adapt ../wasi_snapshot_preview1.reactor.wasm
