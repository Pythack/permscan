#!/bin/bash

echo "What version of permscan do you need ? (linux-gnu, linux-musl macos-arm, macos-w86_64)"
read version
if [ $version = "linux-gnu" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-gnu.tar.gz
    tar -xzvf permscan-x86_64-unknown-linux-gnu.tar.gz
    sudo mv permscan-x86_64-unknown-linux-gnu/permscan /bin
    rm -rf permscan-x86_64-unknown-linux-gnu.tar.gz
    rm -rf permscan-x86_64-unknown-linux-gnu
    rm -f installer.sh
elif [ $version = "linux-musl" ]; then
    wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-musl.tar.gz
    tar -xzvf permscan-x86_64-unknown-linux-musl.tar.gz
    sudo mv permscan-x86_64-unknown-linux-musl/permscan /bin
    rm -rf permscan-x86_64-unknown-linux-musl.tar.gz
    rm -rf permscan-x86_64-unknown-linux-musl
    rm -f installer.sh
elif [ $version = "macos-arm" ]; then
    wget https://github.com/Pythack/permscan/releases/download/v2.2.6/permscan-aarch64-apple-darwin.zip
    unzip permscan-aarch64-apple-darwin.zip
    sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin
    rm -rf permscan-aarch64-apple-darwin.zip
    rm -rf permscan-aarch64-apple-darwin
    rm -rf __MACOSX
    rm -f installer.sh
elif [ $version = "macos-x86_64" ]; then
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
