# NagiOS
## build
```
rustup install night;y
rustup default night;y
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
rustup component add llvm-tools-preview
cargo install bootimage
cargo bootimage
```

## run
```
#qemu-system-x86_64 -nographic -drive format=raw,file=target/x86_64-nagios/debug/bootimage-nagios.bin
cargo run -- -nographic
```

## test
```
cargo test
```
