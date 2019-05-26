#!/bin/bash
cargo build --release --features generate-api-description --target=wasm32-unknown-unknown
wasm-build target trackvestor --target-runtime=substrate --final=trackvestor --save-raw=./target/trackvestor-deployed.wasm --target wasm32-unknown-unknown
