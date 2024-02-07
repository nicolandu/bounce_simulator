#!/bin/bash
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd "$parent_path"
cargo build --release
rm -r ./out/
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "bounce_simulator" \
    ./target/wasm32-unknown-unknown/release/bounce_simulator.wasm
wasm-opt -Oz -o ./out/bounce_simulator_bg.wasm ./out/bounce_simulator_bg.wasm
cp -a ./assets/ ./out/
cp index.html ./out/
mv ./out ./docs
