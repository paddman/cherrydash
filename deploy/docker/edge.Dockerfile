# SPDX-License-Identifier: AGPL-3.0-only
FROM rust:1-bookworm AS builder
WORKDIR /workspace

COPY Cargo.toml rust-toolchain.toml ./
COPY crates ./crates
COPY services ./services
COPY agents ./agents

RUN cargo build --release --package cherrydash-edge

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/target/release/cherrydash-edge /usr/local/bin/cherrydash-edge

USER 65532:65532
ENTRYPOINT ["/usr/local/bin/cherrydash-edge"]
