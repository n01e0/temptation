# temptation
detect fileless malware on linux

## sample

```console
make
make test
LD_PRELOAD=./target/debug/libtemptation.so ./fileless_sample
[2020-09-12T16:58:42Z WARN  temptation] detected fileless fexecve!!
hello fileless
```

![demo](demo/temptation_demo.gif)
