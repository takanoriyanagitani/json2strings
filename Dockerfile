## wasm builder
FROM rust:1.60.0 AS builder
RUN rustup target add wasm32-wasi

## download/build 3rd party
WORKDIR /usr/src/json2strings
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
RUN mkdir --parents ./src
RUN echo 'fn main(){}' > ./src/main.rs
RUN cargo check
RUN cargo build --release --target wasm32-wasi

## build wasm
COPY ./src/ ./src/
RUN cargo build --release --target wasm32-wasi
RUN cp target/wasm32-wasi/release/json2strings.wasm /usr/local/bin/json2strings.wasm

## get wasm runtime(wasmtime)
FROM rust:1.60.0 AS extractor
WORKDIR /usr/local/bin/wasmtime
RUN curl \
  --location \
  https://github.com/bytecodealliance/wasmtime/releases/download/v0.35.2/wasmtime-v0.35.2-x86_64-linux.tar.xz \
  | xzcat \
  | tar --extract --verbose

## main container
FROM debian:buster-slim
WORKDIR /usr/local/bin/wasmtime.d
COPY --from=extractor /usr/local/bin/wasmtime/wasmtime-v0.35.2-x86_64-linux/ ./
WORKDIR /usr/local/bin
RUN ln -s ./wasmtime.d/wasmtime ./

COPY --from=builder /usr/local/bin/json2strings.wasm /usr/local/bin/
CMD ["wasmtime", "/usr/local/bin/json2strings.wasm"]
