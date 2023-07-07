FROM rust:1.70-slim-bullseye as build

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release

FROM scratch

COPY --from=build /target/wasm32-wasi/release/wasm-on-docker.wasm hello.wasm

ENTRYPOINT [ "/hello.wasm" ]
