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

## Sources created in the project

|                |                                          |
| :------------- | :--------------------------------------- |
| src/hello.rs   | Sample element that generate data.       |
| src/hexdump.rs | Sample element that dump data to stdout. |
