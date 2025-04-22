FROM amd64/debian:9

ARG RUST_TOOLCHAIN=stable
ARG DISTRIBUTION=debian

RUN echo 'deb http://archive.debian.org/debian stretch main' >/etc/apt/sources.list && \
    echo 'deb http://archive.debian.org/debian-security stretch/updates main' >>/etc/apt/sources.list && \
    apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    gcc-aarch64-linux-gnu \
    gcc-arm-linux-gnueabihf \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    lsb-release \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /etc/apt/keyrings
RUN curl -fsSL https://repository.aptpod.jp/intdash-edge/linux/${DISTRIBUTION}/gpg | \
    gpg --dearmor -o /etc/apt/keyrings/intdash-edge.gpg
RUN echo "deb [signed-by=/etc/apt/keyrings/intdash-edge.gpg] \
    https://repository.aptpod.jp/intdash-edge/linux/${DISTRIBUTION} $(lsb_release -cs) \
    stable" \
    | tee /etc/apt/sources.list.d/intdash-edge.list

ARG TARGET
ENV TARGET=${TARGET}

RUN <<EOF
    case "${TARGET}" in
        "aarch64-unknown-linux-gnu") DEB_ARCH=arm64;;
        "x86_64-unknown-linux-gnu") DEB_ARCH=amd64;;
        "armv7-unknown-linux-gnueabihf") DEB_ARCH=armhf;;
    esac

    dpkg --add-architecture ${DEB_ARCH}
    apt-get update
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
        libdc-core-dev:${DEB_ARCH}
    apt-get clean
    rm -rf /var/lib/apt/lists/*
EOF

ENV CARGO_HOME=/usr/local/cargo
ENV PATH="${CARGO_HOME}/bin:$PATH"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none

SHELL ["/bin/bash", "-c"]
WORKDIR /usr/local/src

RUN echo "[target.x86_64-unknown-linux-gnu]" >>$CARGO_HOME/config.toml && \
    echo "linker = \"gcc\"" >>$CARGO_HOME/config.toml && \
    echo "objcopy = { path = \"objcopy\" }" >>$CARGO_HOME/config.toml && \
    echo "strip = { path = \"strip\" }" >>$CARGO_HOME/config.toml && \
    echo "[target.aarch64-unknown-linux-gnu]" >>$CARGO_HOME/config.toml && \
    echo "linker = \"aarch64-linux-gnu-gcc\"" >>$CARGO_HOME/config.toml && \
    echo "objcopy = { path = \"aarch64-linux-gnu-objcopy\" }" >>$CARGO_HOME/config.toml && \
    echo "strip = { path = \"aarch64-linux-gnu-strip\" }" >>$CARGO_HOME/config.toml && \
    echo "[target.armv7-unknown-linux-gnueabihf]" >>$CARGO_HOME/config.toml && \
    echo "linker = \"arm-linux-gnueabihf-gcc\"" >>$CARGO_HOME/config.toml && \
    echo "objcopy = { path = \"arm-linux-gnueabihf-objcopy\" }" >>$CARGO_HOME/config.toml && \
    echo "strip = { path = \"arm-linux-gnueabihf-strip\" }" >>$CARGO_HOME/config.toml && \
    echo "[net]" >>$CARGO_HOME/config.toml && \
    echo "git-fetch-with-cli = true" >>$CARGO_HOME/config.toml && \
    rustup install ${RUST_TOOLCHAIN} && \
    rustup target add --toolchain ${RUST_TOOLCHAIN} ${TARGET}
