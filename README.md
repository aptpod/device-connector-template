# Device Connector Template

Device Connector Template is a template for quickly generating a new [Device Connector Framework](https://github.com/aptpod/device-connector-framework) RUST project.

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

### Cross-compilation (docker)

Setup rust-cross-builder.

```
$ docker build -t rust-cross-builder .
$ docker run --rm -it -v ${PWD}:/usr/local/src rust-cross-builder
```

Run commands below in the Docker container.

#### Rust

Cross-compiling for custom plugin development using Rust.

```
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target armv7-unknown-linux-gnueabihf
cargo build --release --target x86_64-unknown-linux-gnu
```

#### C

Cross-compiling for custom plugin development in C.

1. Setup target variables

```
# for aarch64-unknown-linux-gnu
export TARGET=aarch64-unknown-linux-gnu
export CROSS_COMPILE="aarch64-linux-gnu-"

# for armv7-unknown-linux-gnueabihf
export TARGET=armv7-unknown-linux-gnueabihf
export CROSS_COMPILE="arm-linux-gnueabihf-"

# for x86_64-unknown-linux-gnu
export TARGET=x86_64-unknown-linux-gnu
export CROSS_COMPILE=""
```

2. Setup library directory

```
mkdir -p library_dir/${TARGET}
cargo build -p device-connector-common --release --target ${TARGET}
cp target/${TARGET}/release/libdevice_connector_common.a library_dir/${TARGET}
export LIBRARY_DIR="$(readlink -f library_dir/${TARGET})"
```

3. Setup include directory

```
export INCLUDE_DIR="$(readlink -f /usr/local/cargo/git/checkouts/device-connector-framework-*/*/common/include)"
```

4. Cross-compiling

```
${CROSS_COMPILE}gcc -I${INCLUDE_DIR} -Wall -O2 -fPIC -shared -L${LIBRARY_DIR} \
  -o libdc_example_plugin.so src/example_plugin.c -ldevice_connector_common
```

## Sources created in the project

|                |                                          |
| :------------- | :--------------------------------------- |
| src/hello.rs   | Sample element that generate data.       |
| src/hexdump.rs | Sample element that dump data to stdout. |
