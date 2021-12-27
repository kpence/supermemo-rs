#!/bin/bash
cd src/lib/dll_hook && cargo build && cd -
cargo build
docker build --no-cache -t docker-wine .
./docker-wine  --local=docker-wine wine /main.exe
