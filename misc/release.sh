#!/bin/sh
cargo build --release
strip target/release/cakeybar
help2man target/release/cakeybar > target/release/cakeybar.man
