#!/bin/bash

echo "What version of permscan do you need ? (1, 2, 3, 4)
1. linux-gnu
2. linux-musl
3. macos-arm
4. macos-x86_64
5. build it yourself (recommended if your architecture is not yet supported) (rustlang must be installed)"
read -r version
if [ "$version" = "1" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-gnu.tar.gz
    tar -xzvf permscan-x86_64-unknown-linux-gnu.tar.gz
    sudo mv permscan-x86_64-unknown-linux-gnu/permscan /bin
    rm -rf permscan-x86_64-unknown-linux-gnu.tar.gz
    rm -rf permscan-x86_64-unknown-linux-gnu
elif [ "$version" = "2" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-musl.tar.gz
    tar -xzvf permscan-x86_64-unknown-linux-musl.tar.gz
    sudo mv permscan-x86_64-unknown-linux-musl/permscan /bin
    rm -rf permscan-x86_64-unknown-linux-musl.tar.gz
    rm -rf permscan-x86_64-unknown-linux-musl
elif [ "$version" = "3" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-aarch64-apple-darwin.zip
    unzip permscan-aarch64-apple-darwin.zip
    sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin
    rm -rf permscan-aarch64-apple-darwin.zip
    rm -rf permscan-aarch64-apple-darwin
    rm -rf __MACOSX
elif [ "$version" = "4" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-apple-darwin.zip
    unzip permscan-x86_64-apple-darwin.zip
    sudo mv permscan-x86_64-apple-darwin/permscan /usr/local/bin
    rm -rf permscan-x86_64-apple-darwin.zip
    rm -rf permscan-x86_64-apple-darwin
    rm -rf __MACOSX
elif [ "$version" = "5" ]; then
    git clone https://github.com/Pythack/permscan
    cd permscan || exit
    cargo build --release
    sudo mv ./target/release/permscan /usr/local/bin
    cd ..
    rm -rf permscan
else
    echo "permscan: installer: unknown version"
    exit 1
fi
rm -f permscan-installer.sh
