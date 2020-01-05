#!/bin/bash
cargo build --release
mkdir -p ./built
cp ./target/release/cli.exe ./built/cv.exe
cp ./target/release/cli ./built/cv
