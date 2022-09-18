#!/usr/bin/bash
cd static/wasm/wasm-utils/
wasm-pack build --target web
cd ../../..
cargo run
