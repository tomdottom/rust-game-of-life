#!/bin/bash -e

PATH=~/.cargo/bin/:$PATH

mkdir -p ./dist/{rust,python,wasm}

cargo +nightly build --release \
    --package python-cgol \
    --package rust-cgol
cargo +nightly build --release \
    --package wasm-cgol --target "wasm32-unknown-unknown"

#wasm cgol
BUILD=release
wasm-bindgen \
    target/wasm32-unknown-unknown/${BUILD}/wasm_cgol.wasm \
    --out-dir wasm-src/

# Rust cgol
cp ./target/release/rust-cgol ./dist/rust/cgol

# Python cgol
cp ./target/release/libcgol.so ./dist/python/cgol.so
cp ./python-src/* ./dist/python

# WASM cgol
pushd wasm-src && \
npm install && \
npm run bundle && \
cp ./dist/* ../dist/wasm/
popd
