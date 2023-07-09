# Device Connector Template

Device Connector Template is a template for quickly generating a new [Device Connector Framework](https://github.com/aptpod/device-connector-framework) RUST project.

## How to try

### Generate New Project

```
$ cargo install cargo-generate
$ cargo generate --git https://github.com/aptpod/device-connector-template.git
```

### Build

```
$ cargo build --release
```

### Run `hello-src`

```
$ ./target/release/test-run --config conf.yml
```

### Cross-compilation (docker)

```
$ docker build -t rust-cross-builder .
$ docker run --rm -it -v ${PWD}:/usr/local/src rust-cross-builder
# cargo build --release --target <x86_64-unknown-linux-gnu|aarch64-unknown-linux-gnu|armv7-unknown-linux-gnueabihf>
```

## Sources created in the project

|                |                                          |
| :------------- | :--------------------------------------- |
| src/hello.rs   | Sample element that generate data.       |
| src/hexdump.rs | Sample element that dump data to stdout. |
