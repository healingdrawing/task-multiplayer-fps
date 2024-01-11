#!/bin/bash

# build release executable
cargo build --release

# copy release executable to the root folder level with nice name
cp ./target/release/hybrid ./hybrid
