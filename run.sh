#!/bin/bash
cargo build
docker build --no-cache -t docker-wine .
./docker-wine  --local=docker-wine wine /main.exe
