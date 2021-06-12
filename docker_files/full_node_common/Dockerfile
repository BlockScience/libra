FROM debian:buster-20200803@sha256:a44ab0cca6cd9411032d180bc396f19bc98f71972d2398d50460145cab81c5ab AS toolchain

RUN mkdir /app
WORKDIR /app
ADD . /app/
# COPY . .


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


# RUN make bins
RUN ~/.cargo/bin/cargo run -p stdlib --release
RUN ~/.cargo/bin/cargo build -p miner -p ol -p txs -p onboard --release
# RUN $HOME/.cargo/bin/cargo build -p libra-node -p miner -p backup-cli -p ol -p txs -p onboard --release

RUN make install
RUN $HOME/.cargo/bin/cargo run -p ol restore

CMD ["libra-node", "--config", "$HOME/.0L/fullnode.node.yaml"]