FROM rust:1.93-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev libclang-dev clang cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() { println!("dummy"); }' > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 app

WORKDIR /app
COPY --from=builder /app/target/release/axum-docker-deployment /app/server
RUN chown -R app:app /app

USER app
ENV RUST_LOG=info
EXPOSE 3000
ENTRYPOINT ["/app/server"]
