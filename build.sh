#!/bin/bash
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd "$parent_path"
cargo build --release
rm -r ./docs/
wasm-bindgen --no-typescript --target web \
    --out-dir ./docs/ \
    --out-name "bounce_simulator" \
    ./target/wasm32-unknown-unknown/release/bounce_simulator.wasm
wasm-opt -Oz -o ./docs/bounce_simulator_bg.wasm ./docs/bounce_simulator_bg.wasm
cp -r ./deploy_src/* ./docs
cp ./art/player_red.svg ./docs/favicon.svg
