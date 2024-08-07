FROM rust:1.78 as builder
ENV BUILD_DIR=/usr/src/init-service
WORKDIR ${BUILD_DIR}
COPY Cargo.toml Cargo.toml
COPY src src
RUN cargo build --release

FROM debian:12-slim

COPY --from=builder /usr/src/init-service/target/release/seed-invoice /usr/local/bin/seed-invoice
ENTRYPOINT ["seed-invoice"]

