FROM debian:buster-20200803@sha256:a44ab0cca6cd9411032d180bc396f19bc98f71972d2398d50460145cab81c5ab AS toolchain

# To use http/https proxy while building, use:
# docker build --build-arg https_proxy=http://fwdproxy:8080 --build-arg http_proxy=http://fwdproxy:8080


WORKDIR /libra
# COPY rust-toolchain /libra/rust-toolchain
# COPY cargo-toolchain /libra/cargo-toolchain


FROM toolchain AS builder

ARG ENABLE_FAILPOINTS
COPY . /libra

RUN set -e

Run apt update && apt install -y cmake sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo
RUN make deps
RUN . ~/.cargo/env
RUN export RUSTC_WRAPPER=sccache
RUN . ~/.bashrc


# RUN ./docker/build-common.sh
RUN $HOME/.cargo/bin/cargo build --release \
    -p libra-genesis-tool \
    -p libra-operational-tool \
    -p libra-node \
    -p libra-key-manager \
    -p safety-rules \
    -p db-bootstrapper \
    -p backup-cli


# ### Production Image ###
FROM debian:buster-20200803@sha256:a44ab0cca6cd9411032d180bc396f19bc98f71972d2398d50460145cab81c5ab AS prod

RUN apt-get update && apt-get install -y libssl1.1 && apt-get clean && rm -r /var/lib/apt/lists/*

RUN addgroup --system --gid 6180 libra && adduser --system --ingroup libra --no-create-home --uid 6180 libra

RUN mkdir -p /opt/libra/bin /opt/libra/etc
COPY docker/install-tools.sh /root
COPY --from=builder /libra/target/release/libra-node /opt/libra/bin
COPY --from=builder /libra/target/release/db-backup /opt/libra/bin
COPY --from=builder /libra/target/release/db-bootstrapper /opt/libra/bin
COPY --from=builder /libra/target/release/db-restore /opt/libra/bin

# Admission control
EXPOSE 8000
# Validator network
EXPOSE 6180
# Metrics
EXPOSE 9101
# Backup
EXPOSE 6186

# Capture backtrace on error
ENV RUST_BACKTRACE 1

ARG BUILD_DATE
ARG GIT_REV
ARG GIT_UPSTREAM

LABEL org.label-schema.schema-version="1.0"
LABEL org.label-schema.build-date=$BUILD_DATE
LABEL org.label-schema.vcs-ref=$GIT_REV
LABEL vcs-upstream=$GIT_UPSTREAM