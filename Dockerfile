# FROM debian:bookworm-slim
# WORKDIR /verisense
# COPY ./target/release/verisense /usr/local/bin/

FROM rust:1.81 as builder
WORKDIR /verisense
COPY . .
RUN apt update
RUN apt install --assume-yes clang libssl-dev llvm libudev-dev make protobuf-compiler
RUN rustup target add wasm32-unknown-unknown --toolchain 1.81.0-x86_64-unknown-linux-gnu
RUN rustup component add rust-src --toolchain 1.81.0-x86_64-unknown-linux-gnu
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /verisense
COPY --from=builder /verisense/target/release/verisense /usr/local/bin/
