FROM rust:1.85.0-slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev protobuf-compiler
RUN update-ca-certificates

WORKDIR /usr/src/app

RUN cargo init .

COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY .sqlx .

RUN cargo fetch

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/categories-service ./
CMD [ "./categories-service" ]
