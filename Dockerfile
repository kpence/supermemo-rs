FROM mmozeiko/mingw-w64
FROM scottyhardy/docker-wine:latest
# TODO use variables for the file names and directory for easy change

COPY assets/i686-w64-mingw-deps/* /
ADD assets/sm18.exe /target.exe
RUN chmod a+x /target.exe

ADD src/lib/dll_hook/target/i686-pc-windows-gnu/debug/dll_hook.dll /hook.dll
ADD target/i686-pc-windows-gnu/debug/foobar.exe /main.exe
COPY assets/entrypoint.sh /usr/bin/entrypoint

ENTRYPOINT ["/usr/bin/entrypoint"]
