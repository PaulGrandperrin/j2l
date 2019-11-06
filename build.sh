#!/bin/sh
set -xe

if ! hash cargo 2>/dev/null; then
  if [ "`uname`" = "Linux" ] ;then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal -t x86_64-unknown-linux-musl
  else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal
  fi
fi

cargo build --release

# the resulting binary is at ./target/release/j2l
