ARG TARGETARCH
FROM messense/rust-musl-cross:x86_64-musl AS builder_amd64
FROM messense/rust-musl-cross:aarch64-musl AS builder_arm64

FROM builder_${TARGETARCH} AS builder

WORKDIR /app

# Install protoc
ARG PROTOC_VERSION=31.1
ARG TARGETARCH

RUN if [ "${TARGETARCH}" = "amd64" ]; then \
    curl -sSL https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip -o protoc.zip; \
    elif [ "${TARGETARCH}" = "arm64" ]; then \
    curl -sSL https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-aarch_64.zip -o protoc.zip; \
    else echo "Unsupported arch: ${TARGETARCH}"; exit 1; fi \
    && unzip protoc.zip -d /usr/local \
    && rm -f protoc.zip \
    && chmod +x /usr/local/bin/protoc

# TODO: Caching
COPY vendor/ vendor/
COPY migrations/ migrations/
COPY build.rs build.rs
COPY Cargo.toml Cargo.toml
COPY src/ src/

RUN cargo build --release

FROM scratch AS final_amd64
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/creo-monitor /app/creo-monitor

FROM scratch AS final_arm64
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/creo-monitor /app/creo-monitor

ARG TARGETARCH
FROM final_${TARGETARCH}
WORKDIR /app
CMD ["/app/creo-monitor"]
