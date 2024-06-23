#!/bin/bash

set -eux

# move to
pushd ../solver/wasm

# see: https://betterprogramming.pub/deploying-a-wasm-powered-react-app-on-vercel-cf3cae2a75d6
echo "Installing Rustup..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

echo "Build WASM"
/vercel/.cargo/bin/wasm-pack build --release --target web

popd

# The following command is default install command...
npm install
