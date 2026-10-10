#!/usr/bin/env bash
# Reproduce the S0 cloud toolchain without writing outside /workspace.
set -euo pipefail
cd "$(dirname "$0")/.."

export XDG_DATA_HOME=/workspace/.cache/share
export XDG_CACHE_HOME=/workspace/.cache
export npm_config_cache=/workspace/.cache/npm
export RUSTUP_HOME=/workspace/.toolchains/rustup
export CARGO_HOME=/workspace/.toolchains/cargo
export CARGO_TARGET_DIR=/workspace/.cache/cargo-target
export PATH="/workspace/.toolchains/cargo/bin:/workspace/.toolchains/mingw/usr/bin:$PATH"
mkdir -p "$XDG_DATA_HOME" "$npm_config_cache" "$RUSTUP_HOME" "$CARGO_HOME" "$CARGO_TARGET_DIR" /workspace/.cache/pnpm

if ! command -v rustup >/dev/null; then
  curl --fail --silent --show-error --location https://sh.rustup.rs -o /workspace/.cache/rustup-init.sh
  sh /workspace/.cache/rustup-init.sh -y --profile minimal --default-toolchain stable --no-modify-path
fi
rustup toolchain install stable --profile minimal
rustup component add rustfmt clippy
rustup target add x86_64-pc-windows-gnu
python3 scripts/install-mingw.py
pnpm install --frozen-lockfile --store-dir /workspace/.cache/pnpm
cargo fetch --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
