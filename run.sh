#!/bin/bash
cd src/lib/dll_hook && cargo build && cd -
cargo build
#docker build -t docker-wine .
docker build --no-cache -t docker-wine .
./docker-wine  --local=docker-wine wine /main.exe
#./docker-wine  --local=docker-wine wine C:/Users/wineuser/ollydbg.exe
