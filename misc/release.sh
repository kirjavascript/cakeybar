#!/bin/sh
cargo build --release
strip target/release/cakeybar
du -h target/release/cakeybar
help2man -N target/release/cakeybar > target/release/cakeybar.1
# cp -r examples target/release/examples
