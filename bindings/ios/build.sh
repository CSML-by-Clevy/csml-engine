#!/usr/bin/env bash

cargo install cargo-lipo
cargo install cbindgen

cbindgen src/lib.rs -l c > csmllib.h

cargo lipo --release

cp ../../target/universal/release/libcsmllib.a ./
