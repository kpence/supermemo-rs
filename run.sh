#!/bin/bash
cd src/lib/dll_hook && cargo build && cd -
cargo build

#docker build -t docker-wine .
#docker build --no-cache -t docker-wine .
#./docker-wine  \
#  --local=docker-wine \
#  --home-volume=/media/data/supermemo \
#  /run.sh

  #wine C:/Users/wineuser/ollydbg.exe & && wine /main.exe
#./docker-wine  --local=docker-wine wine C:/Users/wineuser/ollydbg.exe
  #wine C:/Users/wineuser/ollydbg.exe

cp src/lib/dll_hook/target/i686-pc-windows-gnu/debug/dll_hook.dll /media/data/supermemo/.wine/drive_c/users/wineuser/hook.dll
cp target/i686-pc-windows-gnu/debug/foobar.exe /media/data/supermemo/.wine/drive_c/users/wineuser/main.exe

export WINEPREFIX=/media/data/supermemo/.wine; wine /media/data/supermemo/.wine/drive_c/users/wineuser/main.exe
