#!/bin/sh
set -e
set -x
cd /home/navn/workspace/rust/mqjs
cargo build --release
cp --debug -u target/release/mqjs ~/bin
# mkdir -p ~/bin/lib/mqjs/modules/so
# cp target/release/libfs.so ~/bin/lib/mqjs/modules/so
