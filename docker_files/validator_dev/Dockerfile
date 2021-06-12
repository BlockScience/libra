FROM debian:buster-20200803@sha256:a44ab0cca6cd9411032d180bc396f19bc98f71972d2398d50460145cab81c5ab AS toolchain

# To use http/https proxy while building, use:
# docker build --build-arg https_proxy=http://fwdproxy:8080 --build-arg http_proxy=http://fwdproxy:8080

RUN apt-get update && apt-get install -y cmake curl clang git pkg-config libssl-dev lld

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
ENV PATH "$PATH:/root/.cargo/bin"

WORKDIR /libra
COPY rust-toolchain /libra/rust-toolchain
RUN rustup install $(cat rust-toolchain)

COPY cargo-toolchain /libra/cargo-toolchain
RUN rustup install $(cat cargo-toolchain)

FROM toolchain AS builder

ARG ENABLE_FAILPOINTS
COPY . /libra

RUN set -e

# This is a common compilation scripts across different docker file
# It unifies RUSFLAGS, compilation flags (like --release) and set of binary crates to compile in common docker layer

RUN export RUSTFLAGS="-Ctarget-cpu=skylake -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3"
# RUN rustup toolchain install 1.46.0
# We are using a pinned nightly cargo until feature resolver v2 is released (around 10/2020), but compiling with stable rustc
# RUN export RUST_NIGHTLY=$(cat cargo-toolchain)
# RUN export RUST_NIGHTLY="nightly-2020-07-08"
RUN export CARGO=$(rustup toolchain install 1.46.0)
RUN export CARGOFLAGS=$(cat cargo-flags)
RUN export CARGO_PROFILE_RELEASE_LTO=thin # override lto setting to turn on thin-LTO for release builds


# RUN ./docker/build-common.sh

# ### Production Image ###
# FROM debian:buster-20200803@sha256:a44ab0cca6cd9411032d180bc396f19bc98f71972d2398d50460145cab81c5ab AS prod

# RUN apt-get update && apt-get install -y libssl1.1 && apt-get clean && rm -r /var/lib/apt/lists/*

# RUN addgroup --system --gid 6180 libra && adduser --system --ingroup libra --no-create-home --uid 6180 libra

# RUN mkdir -p /opt/libra/bin /opt/libra/etc
# COPY docker/install-tools.sh /root
# COPY --from=builder /libra/target/release/libra-node /opt/libra/bin
# COPY --from=builder /libra/target/release/db-backup /opt/libra/bin
# COPY --from=builder /libra/target/release/db-bootstrapper /opt/libra/bin
# COPY --from=builder /libra/target/release/db-restore /opt/libra/bin

# # Admission control
# EXPOSE 8000
# # Validator network
# EXPOSE 6180
# # Metrics
# EXPOSE 9101
# # Backup
# EXPOSE 6186

# # Capture backtrace on error
# ENV RUST_BACKTRACE 1

# ARG BUILD_DATE
# ARG GIT_REV
# ARG GIT_UPSTREAM

# LABEL org.label-schema.schema-version="1.0"
# LABEL org.label-schema.build-date=$BUILD_DATE
# LABEL org.label-schema.vcs-ref=$GIT_REV
# LABEL vcs-upstream=$GIT_UPSTREAM
