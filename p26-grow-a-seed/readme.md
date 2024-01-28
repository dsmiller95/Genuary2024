
## Profiling with Tracy

in one terminal, after downloading the tracy executables,
```shell
../../path/to/tracy/capture.exe -o output.tracy
```

in another terminal,
```shell
cargo run --release --features bevy/trace_tracy
```

view the output file with Tracy.exe