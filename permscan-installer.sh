#!/bin/bash

BUILD="false"
while getopts 'b' flag; do
    case "${flag}" in
    b) BUILD="true" ;;
    *) exit 1 ;;
    esac
done

if [ "$BUILD" = "false" ]; then
    OS=$(uname)
    if [ "$OS" = "Linux" ]; then
        OSTYPE=$(uname -o)
        if [ "$OSTYPE" = "GNU/Linux" ]; then
            wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-gnu.tar.gz
            tar -xzvf permscan-x86_64-unknown-linux-gnu.tar.gz
            sudo mv permscan-x86_64-unknown-linux-gnu/permscan /bin
            rm -rf permscan-x86_64-unknown-linux-gnu.tar.gz
            rm -rf permscan-x86_64-unknown-linux-gnu
        elif [ "$OSTYPE" = "Linux" ]; then
            wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-musl.tar.gz
            tar -xzvf permscan-x86_64-unknown-linux-musl.tar.gz
            sudo mv permscan-x86_64-unknown-linux-musl/permscan /bin
            rm -rf permscan-x86_64-unknown-linux-musl.tar.gz
            rm -rf permscan-x86_64-unknown-linux-musl
        else
            echo "permscan: installer: os not supported. try building it (run this installer with the -b flag if installing permscan or run permscan -ub if updating)"
            exit 1
        fi
    elif [ "$OS" = "Darwin" ]; then
        OSTYPE=$(uname -m)
        if [ "$OSTYPE" = "arm64" ]; then
            wget https://github.com/Pythack/permscan/releases/latest/download/permscan-aarch64-apple-darwin.zip
            unzip permscan-aarch64-apple-darwin.zip
            sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin
            rm -rf permscan-aarch64-apple-darwin.zip
            rm -rf permscan-aarch64-apple-darwin
            rm -rf __MACOSX
        elif [ "$OSTYPE" = "x86_64" ]; then
            wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-apple-darwin.zip
            unzip permscan-x86_64-apple-darwin.zip
            sudo mv permscan-x86_64-apple-darwin/permscan /usr/local/bin
            rm -rf permscan-x86_64-apple-darwin.zip
            rm -rf permscan-x86_64-apple-darwin
            rm -rf __MACOSX
        else
            echo "permscan: installer: os not supported. try building it (run this installer with the -b flag if installing permscan or run permscan -ub if updating)"
            exit 1
        fi
    else
        echo "permscan: installer: os not supported. try building it (run this installer with the -b flag if installing permscan or run permscan -ub if updating)"
        exit 1
    fi
elif [ "$BUILD" = "true" ]; then
    if ! command -v cargo &>/dev/null; then
        echo "permscan: installer: build failed. make sure the rust programming language is installed"
        exit 1
    else
        git clone https://github.com/Pythack/permscan
        cd permscan || exit
        cargo build --release
        sudo mv ./target/release/permscan /usr/local/bin
        cd ..
        rm -rf permscan
    fi
fi
rm -f permscan-installer.sh
