FROM mmozeiko/mingw-w64
FROM scottyhardy/docker-wine:latest
# TODO use variables for the file names and directory for easy change

#RUN apt-get update \
#    && DEBIAN_FRONTEND="noninteractive" apt-get install -y --no-install-recommends \
#        mingw-w64 \
#        gcc-mingw-w64 \
#    && rm -rf /var/lib/apt/lists/*

COPY assets/i686-w64-mingw-deps/* /
ADD assets/ConsoleApplication1.exe /target.exe
RUN chmod a+x /target.exe

ADD src/lib/dll_hook/target/i686-pc-windows-gnu/debug/dll_hook.dll /hook.dll
ADD target/i686-pc-windows-gnu/debug/foobar.exe /main.exe
COPY assets/entrypoint.sh /usr/bin/entrypoint

ENTRYPOINT ["/usr/bin/entrypoint"]
