# ADR-0001: Core platform stack

- Status: Accepted for foundation
- Date: 2026-09-01

## Context

CherryDash must combine deep infrastructure monitoring with a modern dashboard and observability experience. It must run on one server for evaluation, scale horizontally for enterprise/MSP use, operate at remote sites with unreliable links, and remain suitable for on-premise and air-gapped environments.

A literal merge or fork of Zabbix and Grafana would inherit two object models, two permission models, separate alert semantics, separate upgrade lifecycles and restrictive coupling to upstream implementation choices.

## Decision

1. Use **Rust** for CherryDash central services, native collectors and edge agent.
2. Use **Tokio + Axum + Tower** for asynchronous network services and HTTP APIs.
3. Use **Protobuf/gRPC** for versioned internal streaming contracts; expose REST/WebSocket for web and external integrations.
4. Use **React + TypeScript + Vite** for the web application.
5. Use **NATS JetStream** as durable event fabric and regional/edge messaging layer.
6. Use **ClickHouse** as hot analytical telemetry store.
7. Use **PostgreSQL** as authoritative control-plane database.
8. Use **S3-compatible object storage + Parquet** for cold data and evidence.
9. Use **OpenTelemetry and Prometheus compatibility** at ingestion boundaries.
10. Implement Zabbix/Grafana compatibility as clean adapters and importers, not source-code coupling.

## Consequences

### Positive

- Performance-sensitive paths share one memory-safe language
- Services can be statically distributed to edge environments
- Data plane, query plane and control plane scale independently
- Open standards reduce migration friction and vendor lock-in
- One CherryDash object model can unify dashboard, alert, incident, RBAC and audit behavior

### Negative

- Rust has a steeper learning curve than Go or TypeScript for some contributors
- A native dashboard/query engine is substantial product work
- PromQL and Zabbix expression compatibility require careful semantic testing
- Operating ClickHouse, PostgreSQL and NATS adds distributed-system complexity
- Early versions should reuse OpenTelemetry Collector adapters rather than rewrite every protocol receiver

## Rejected alternatives

- **Fork Zabbix and embed Grafana:** fast initial demo but creates long-term upgrade, UI, authorization and license coupling
- **All-TypeScript backend:** development speed is attractive but less suitable for high-rate ingestion and small edge binaries
- **All-Go backend:** excellent operational ecosystem, but Rust is selected for native data plane and edge safety/performance
- **Kafka as the only event fabric:** proven at very large scale but heavier for a single-node and remote-edge product; event transport remains abstracted so Kafka-compatible backends can be added if benchmark evidence requires it
- **Store everything in PostgreSQL:** simple but unsuitable for the target telemetry volume and analytical query patterns
