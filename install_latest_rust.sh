#!/bin/bash

echo "🔍 Checking for rustup..."

if ! command -v rustup &> /dev/null
then
    echo "🚀 rustup not found. Installing rustup + Rust..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y

    echo "🔁 Sourcing environment..."
    source "$HOME/.cargo/env"
else
    echo "✅ rustup is already installed."
fi

echo "📦 Updating to the latest Rust version..."
rustup update

echo "🔧 Ensuring rustc and cargo are in PATH..."
source "$HOME/.cargo/env"

echo "🧪 Rust version:"
rustc --version
cargo --version

echo "🎉 Rust is set up and ready!"

