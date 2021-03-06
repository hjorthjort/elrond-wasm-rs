#!/bin/sh

# helper script for checking that all contracts and debug projects compile

### EXAMPLES ###

cd examples/adder/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/adder_wasm.wasm output/adder.wasm
cd ../..

cd examples/crypto-bubbles/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/crypto_bubbles_wasm.wasm output/crypto-bubbles.wasm
cd ../..

cd examples/factorial/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/factorial_wasm.wasm output/factorial.wasm
cd ../..

cd examples/simple-erc20/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/simple_erc20_wasm.wasm output/simple-coin.wasm
cd ../..


### TEST CONTRACTS ###

cd test-contracts/basic-features/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/basic_features_wasm.wasm output/features.wasm
cd ../..

cd test-contracts/async-alice/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/async_alice_wasm.wasm output/alice.wasm
cd ../..

cd test-contracts/async-bob/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/async_bob_wasm.wasm output/bob.wasm
cd ../..
