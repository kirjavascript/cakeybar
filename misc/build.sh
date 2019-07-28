#!/bin/sh
cargo build --release
strip -s target/release/cakeybar
du -h target/release/cakeybar
du target/release/cakeybar
# help2man -N target/release/cakeybar > target/release/cakeybar.1
# gzip target/release/cakeybar.1
