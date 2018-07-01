#!/bin/sh
cargo build --release
strip target/release/cakeybar
help2man -N target/release/cakeybar > target/release/cakeybar.man
