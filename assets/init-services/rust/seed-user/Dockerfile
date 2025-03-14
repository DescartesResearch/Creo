FROM rust:1.85 AS builder
ARG BUILD_DIR=/usr/src/init-service
WORKDIR ${BUILD_DIR}
COPY Cargo.toml Cargo.toml
COPY src src
RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry\
    --mount=type=cache,sharing=private,target=${BUILD_DIR}/target \
    cargo build --release && \
    cp target/release/seed-user seed-user

FROM debian:12-slim
ARG BUILD_DIR=/usr/src/init-service

COPY --from=builder ${BUILD_DIR}/seed-user /usr/local/bin/seed-user
ENTRYPOINT ["seed-user"]
