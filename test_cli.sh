#!/bin/bash

CLI=./target/release/zkwasm-cli

set -e
set -x

test_continuation_cli() {
    cargo build --release --features continuation,perf,profile,cuda
    rm -rf params/*.data params/*.config output
    $CLI --params ./params mongomerkle setup --host standard
    $CLI --params ./params mongomerkle dry-run --wasm crates/zkwasm/wasm/mongomerkle.wasm --output ./output
    CUDA_VISIBLE_DEVICES=0 $CLI --params ./params mongomerkle prove --wasm crates/zkwasm/wasm/mongomerkle.wasm --output ./output
    $CLI --params ./params mongomerkle verify --output ./output
}


#x=50
#while [ $x -gt 0 ]; do
#    test_phantom_cli
    test_continuation_cli
#    x=$(($x-1))
#done
