#!/bin/sh
sh ./misc/build.sh
cp target/release/cakeybar /usr/bin/cakeybar
cp target/release/cakeybar.1.gz /usr/share/man/man1/cakeybar.1.gz
cp -r examples $HOME/.config/cakeybar/
