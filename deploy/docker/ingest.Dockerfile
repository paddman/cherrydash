# SPDX-License-Identifier: AGPL-3.0-only
FROM rust:1-bookworm AS builder
WORKDIR /workspace

COPY Cargo.toml rust-toolchain.toml ./
COPY crates ./crates
COPY services ./services
COPY agents ./agents

RUN cargo build --release --package cherrydash-ingest

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /var/lib/cherrydash/ingest \
    && chown -R 65532:65532 /var/lib/cherrydash

COPY --from=builder /workspace/target/release/cherrydash-ingest /usr/local/bin/cherrydash-ingest

USER 65532:65532
VOLUME ["/var/lib/cherrydash/ingest"]
EXPOSE 8081
ENTRYPOINT ["/usr/local/bin/cherrydash-ingest"]
