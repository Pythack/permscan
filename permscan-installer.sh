#!/bin/bash

VERSION="latest"
BUILD="false"
while getopts 'bv:' flag; do
    case "${flag}" in
    b) BUILD="true" ;;
    v) VERSION="${OPTARG}" ;;
    *) exit 1 ;;
    esac
done

if [ "$BUILD" = "false" ]; then
    OS=$(uname)
    if [ "$OS" = "Linux" ]; then
        OSTYPE=$(uname -o)
    elif [ "$OS" = "Darwin" ]; then
        OSTYPE=$(uname -m)
        if [ "$OSTYPE" = "arm64" ]; then
            if [ "$VERSION" = "latest" ]; then
                wget https://github.com/Pythack/permscan/releases/latest/download/permscan-aarch64-apple-darwin.zip
                unzip permscan-aarch64-apple-darwin.zip
                sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin
                rm -rf permscan-aarch64-apple-darwin.zip
                rm -rf permscan-aarch64-apple-darwin
                rm -rf __MACOSX
            else
                wget https://github.com/Pythack/permscan/releases/download/v"${VERSION}"/permscan-aarch64-apple-darwin.zip
                unzip permscan-aarch64-apple-darwin.zip
                sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin
                rm -rf permscan-aarch64-apple-darwin.zip
                rm -rf permscan-aarch64-apple-darwin
                rm -rf __MACOSX
            fi
        elif [ "$OSTYPE" = "x86_64" ]; then
            if [ "$VERSION" = "latest" ]; then
                wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-apple-darwin.zip
                unzip permscan-x86_64-apple-darwin.zip
                sudo mv permscan-x86_64-apple-darwin/permscan /usr/local/bin
                rm -rf permscan-x86_64-apple-darwin.zip
                rm -rf permscan-x86_64-apple-darwin
                rm -rf __MACOSX
            else
                wget https://github.com/Pythack/permscan/releases/download/v"${VERSION}"/permscan-x86_64-apple-darwin.zip
                unzip permscan-x86_64-apple-darwin.zip
                sudo mv permscan-x86_64-apple-darwin/permscan /usr/local/bin
                rm -rf permscan-x86_64-apple-darwin.zip
                rm -rf permscan-x86_64-apple-darwin
                rm -rf __MACOSX
            fi
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
        if [ "$VERSION" = "latest" ]; then
            git clone https://github.com/Pythack/permscan
            cd permscan || exit
            cargo build --release
            sudo mv ./target/release/permscan /usr/local/bin
            cd ..
            rm -rf permscan
        else
            wget https://github.com/Pythack/permscan/archive/refs/tags/v"${VERSION}".tar.gz
            tar -xzvf v"${VERSION}".tar.gz
            cd permscan-"${VERSION}" || exit
            cargo build --release
            sudo mv ./target/release/permscan /usr/local/bin
            cd ..
            rm -rf v"${VERSION}".tar.gz
            rm -rf permscan-"${VERSION}"
            rm -rf permscan
        fi
    fi
fi
rm -f permscan-installer.sh
