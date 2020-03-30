#!/bin/bash

# In-depth explanation for each command available at:
# https://github.com/VioletVillain/Oxidized-Skies/wiki/Build-Instructions


set -euo pipefail


echo "
#############################################
#### Preparing the toolchain
#############################################
"
rustup toolchain install nightly
rustup +nightly target add wasm32-unknown-unknown
rustup +nightly component add llvm-tools-preview rust-src
cargo install wasm-bindgen-cli
export RUSTFLAGS="
    -C target-cpu=bleeding-edge
    -C target-feature=+atomics,+bulk-memory,+simd128
    -C linker=rust-lld
"

echo "
#############################################
#### Compiling & packing up
#############################################
"
cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=core,std,panic_abort
wasm-bindgen --target web --no-typescript --no-demangle --remove-producers-section --remove-name-section --out-dir pkg ./target/wasm32-unknown-unknown/release/oxidized_skies.wasm
cp src/index.html pkg

echo "
#############################################
#### Finished, resulting files have been placed
#### in ./pkg you can use a server with
#### application/wasm mime type to run them.
#############################################
"
ls -lh pkg