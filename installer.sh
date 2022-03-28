#!/bin/bash

echo "What version of permscan do you need ? (1, 2, 3, 4)
1. linux-gnu
2. linux-musl
3. macos-arm
4. macos-x86_64"
read -r version
if [ "$version" = "1" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-gnu.tar.gz
    tar -xzvf permscan-x86_64-unknown-linux-gnu.tar.gz
    sudo mv permscan-x86_64-unknown-linux-gnu/permscan /bin
    rm -rf permscan-x86_64-unknown-linux-gnu.tar.gz
    rm -rf permscan-x86_64-unknown-linux-gnu
    rm -f installer.sh
elif [ "$version" = "2" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-musl.tar.gz
    tar -xzvf permscan-x86_64-unknown-linux-musl.tar.gz
    sudo mv permscan-x86_64-unknown-linux-musl/permscan /bin
    rm -rf permscan-x86_64-unknown-linux-musl.tar.gz
    rm -rf permscan-x86_64-unknown-linux-musl
    rm -f installer.sh
elif [ "$version" = "3" ]; then
    wget https://github.com/Pythack/permscan/releases/download/v2.2.6/permscan-aarch64-apple-darwin.zip
    unzip permscan-aarch64-apple-darwin.zip
    sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin
    rm -rf permscan-aarch64-apple-darwin.zip
    rm -rf permscan-aarch64-apple-darwin
    rm -rf __MACOSX
    rm -f installer.sh
elif [ "$version" = "4" ]; then
    wget https://github.com/Pythack/permscan/releases/download/v2.2.6/permscan-x86_64-apple-darwin.zip
    unzip permscan-x86_64-apple-darwin.zip
    sudo mv permscan-x86_64-apple-darwin/permscan /usr/local/bin
    rm -rf permscan-x86_64-apple-darwin.zip
    rm -rf permscan-x86_64-apple-darwin
    rm -rf __MACOSX
    rm -f installer.sh
else
    echo "permscan: installer: unknown version"
fi
