FROM lukemathwalker/cargo-chef:latest-rust-1.56.0-alpine as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# install system dependencies
ARG RUSTFLAGS='-C target-feature=-crt-static'

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --offline --bin big_cheese_server

FROM alpine:edge AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/big_cheese_server /usr/local/bin
CMD ["/usr/local/bin/big_cheese_server"]
