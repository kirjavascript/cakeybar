#!/bin/sh
cargo build --release
strip target/release/cakeybar
