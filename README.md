# lddfrom

Pass lddfrom an executable to generate a list of Docker COPY --from commands.

```shell
lddfrom /bin/bash
```

output
```Dockerfile
COPY --from=base /bin/bash /bin/bash
COPY --from=base /lib/x86_64-linux-gnu/libtinfo.so.6 /lib/x86_64-linux-gnu/libtinfo.so.6
COPY --from=base /lib/x86_64-linux-gnu/libdl.so.2 /lib/x86_64-linux-gnu/libdl.so.2
COPY --from=base /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=base /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
ENTRYPOINT [ "/bin/bash" ]
```