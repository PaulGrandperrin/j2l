#!/bin/sh
set -xe

if [ "`uname`" = "Linux" ] ;then
  if ! hash cargo 2>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal -t x86_64-unknown-linux-musl # MUSL is for static linking on Linux
    . $HOME/.cargo/env
  fi

  apt install -y --no-install-recommends build-essential musl-tools

  cargo build --release --target x86_64-unknown-linux-musl
  echo "the resulting binary is at ./target/x86_64-unknown-linux-musl/release/j2l"
else
  if ! hash cargo 2>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
    . $HOME/.cargo/env
  fi

  cargo build --release
  echo "the resulting binary is at ./target/release/j2l"
fi

