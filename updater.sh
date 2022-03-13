#!/bin/bash
cargo build --release
sudo rm /bin/permscan -f
sudo cp ./target/release/permscan /bin
