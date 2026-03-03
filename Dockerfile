# syntax=docker/dockerfile:1
FROM rust:1-slim-bookworm AS builder
WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY xtask ./xtask
COPY agents ./agents
COPY packages ./packages
# Build ochi binary (primary); openfang binary removed — use symlink shim below
RUN cargo build --release --bin ochi

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/ochi /usr/local/bin/
# Legacy shim: openfang → ochi (backward compat for existing scripts/tooling)
RUN ln -s /usr/local/bin/ochi /usr/local/bin/openfang
COPY --from=builder /build/agents /opt/ochi/agents
# Legacy path symlink for existing volume mounts
RUN ln -s /opt/ochi /opt/openfang
EXPOSE 4200
VOLUME /data
ENV OCHI_HOME=/data
# Legacy env var still accepted by kernel (deprecated, will be removed in v1.0)
ENV OPENFANG_HOME=/data
ENTRYPOINT ["ochi"]
CMD ["start"]
