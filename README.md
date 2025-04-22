# Device Connector Template

Device Connector Template is a template for quickly generating a new [Device Connector Framework](https://github.com/aptpod/device-connector-framework) Rust project.

For detailed documentation, please refer to the [Device Connector Developer Guide](https://docs.intdash.jp/manual/device-connector-developer-guide/latest/ja/index.html).


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

### Cross-compilation (docker running on amd64)

Setup rust-cross-builder.

```
# for aarch64-unknown-linux-gnu
$ docker build --build-arg TARGET=aarch64-unknown-linux-gnu -t rust-cross-builder-arm64 .
$ docker run --rm -it -v ${PWD}:/usr/local/src rust-cross-builder-arm64

# for x86_64-unknown-linux-gnu
$ docker build --build-arg TARGET=x86_64-unknown-linux-gnu -t rust-cross-builder-amd64 .
$ docker run --rm -it -v ${PWD}:/usr/local/src rust-cross-builder-amd64

# for armv7-unknown-linux-gnueabihf
$ docker build --build-arg TARGET=armv7-unknown-linux-gnueabihf -t rust-cross-builder-armhf .
$ docker run --rm -it -v ${PWD}:/usr/local/src rust-cross-builder-armhf
```

Run commands below in the Docker container.

Cross-compiling for custom plugin development using Rust.

```
cargo build --release --target $TARGET
```

#### C

The Dockerfile can be used for cross-compiling for custom plugin development in C.

Setup target variables

```
# for aarch64-unknown-linux-gnu
export TARGET=aarch64-unknown-linux-gnu
export CROSS_COMPILE="aarch64-linux-gnu-"
export CFLAGS="-I/usr/aarch64-linux-gnu/include"
export LDFLAGS="-L/usr/aarch64-linux-gnu/lib"

# for armv7-unknown-linux-gnueabihf
export TARGET=armv7-unknown-linux-gnueabihf
export CROSS_COMPILE="arm-linux-gnueabihf-"
export CFLAGS="-I/usr/arm-linux-gnueabihf/include"
export LDFLAGS="-L/usr/arm-linux-gnueabihf/lib"

# for x86_64-unknown-linux-gnu
export TARGET=x86_64-unknown-linux-gnu
export CROSS_COMPILE=""
```

Cross-compiling

```
${CROSS_COMPILE}gcc $CFLAGS -Wall -O2 -fPIC -shared $LDFLAGS \
  -o libdc_example_plugin.so src/example_plugin.c
```

## Sources created in the project

|                |                                          |
| :------------- | :--------------------------------------- |
| src/hello.rs   | Sample element that generate data.       |
| src/hexdump.rs | Sample element that dump data to stdout. |
