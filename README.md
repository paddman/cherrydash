# CherryDash

**Unified Infrastructure Monitoring & Observability Platform**

CherryDash คือแพลตฟอร์ม monitoring รุ่นใหม่ที่รวมความลึกด้าน infrastructure monitoring แบบ Zabbix เข้ากับประสบการณ์ dashboard, Explore และ multi-signal observability แบบ Grafana โดยสร้างเป็นผลิตภัณฑ์เดียวบน object model, tenant, RBAC, alert และ incident lifecycle ชุดเดียว

> Status: foundation / pre-alpha. ยังไม่พร้อมใช้ production และยังไม่มีผล benchmark ที่รับรอง scale claim

## เป้าหมายผลิตภัณฑ์

- Monitor server, VM, network, storage, cloud, Kubernetes, database, application และ API
- Agent + agentless collection ผ่าน OTLP, Prometheus, SNMP, syslog, HTTP, ICMP, SSH, WMI, IPMI/Redfish
- Distributed edge collectors สำหรับ branch, customer edge, data center, cloud และ air-gapped environment
- Unified metrics, logs, traces, events, inventory, topology, alerting, incident, SLO/SLA และ report
- Dashboard-as-code, versioned template packs, multi-tenancy, RBAC และ immutable audit trail
- AI-assisted anomaly, correlation, root-cause suggestion, forecasting และ incident summary
- Guardrailed automation: proposal, approval, expiration, verification, rollback และ evidence

## Core stack

| Layer | Technology |
|---|---|
| Core services and edge | Rust, Tokio, Axum, Tower |
| API contracts | Protobuf/gRPC internally; REST/WebSocket externally |
| Web | TypeScript, React, Vite |
| Event fabric | NATS JetStream |
| Hot telemetry | ClickHouse |
| Control state | PostgreSQL |
| Cache/coordination | Valkey |
| Cold tier/evidence | S3/MinIO + Parquet |
| Open ecosystem | OpenTelemetry, Prometheus and migration adapters |

## Current runnable slice

- `cherrydash-server`: health, system information and overview API
- `cherrydash-ingest`: tenant-aware JSON ingestion with append-only local WAL
- `cherrydash-edge`: heartbeat and basic Linux host snapshot collector
- `web`: enterprise CherryDash dashboard shell connected to overview API
- Docker Compose topology for services plus PostgreSQL, ClickHouse, NATS, Valkey and MinIO

NATS publishing, WAL replay, ClickHouse writing and PostgreSQL-backed APIs are the next implementation slice; infrastructure containers are scaffolded but not falsely presented as wired production functionality.

## Quick start

```bash
git clone https://github.com/paddman/cherrydash.git
cd cherrydash
git switch feat/foundation-v0.1
cp .env.example .env
docker compose -f deploy/compose/docker-compose.yml up --build
```

Open:

- Web UI: `http://localhost:3000`
- Control API: `http://localhost:8080/healthz`
- Ingest API: `http://localhost:8081/healthz`
- NATS monitoring: `http://localhost:8222`
- MinIO console: `http://localhost:9003`

Send a test event:

```bash
curl -sS -X POST http://localhost:8081/api/v1/events \
  -H 'content-type: application/json' \
  -H 'x-cherrydash-tenant-id: default' \
  -d '{
    "signal": "event",
    "source": "manual/quickstart",
    "attributes": {"environment": "lab"},
    "body": {"message": "hello from CherryDash"}
  }'
```

Inspect ingest state:

```bash
curl -sS http://localhost:8081/api/v1/ingest/status
```

## Native development

Requirements: current stable Rust toolchain and Node.js 22+

```bash
cargo run -p cherrydash-ingest
cargo run -p cherrydash-server
CHERRYDASH_INGEST_URL=http://127.0.0.1:8081 cargo run -p cherrydash-edge

cd web
npm install
npm run dev
```

Quality checks:

```bash
make check
make web-build
make compose-config
```

## Repository layout

```text
agents/        CherryDash edge and future native collectors
crates/        shared Rust types and libraries
services/      central control/data-plane services
web/           native CherryDash dashboard application
deploy/        containers, Compose, storage schemas and proxy config
docs/          architecture, scope, compatibility, ADR and roadmap
proto/         versioned RPC contracts
```

## Design rules

1. Open standards at every ingestion boundary
2. Rust on performance-sensitive and edge paths
3. Durable receipt before acknowledgement when durable mode is enabled
4. Scale data plane, query plane and control plane independently
5. One product object model for dashboards, alerts, incidents, RBAC and audit
6. No LLM may directly execute a risky infrastructure action
7. No silent loss during import, conversion, buffering or replay
8. No scale claim without a repeatable benchmark artifact

## Documentation

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`docs/PRODUCT_SCOPE.md`](docs/PRODUCT_SCOPE.md)
- [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md)
- [`docs/ROADMAP.md`](docs/ROADMAP.md)
- [`docs/adr/0001-core-platform-stack.md`](docs/adr/0001-core-platform-stack.md)

## Licensing note

A CherryDash project license has not yet been selected. Current Grafana and Zabbix releases are AGPLv3; CherryDash compatibility is therefore designed as protocol/document integration rather than copying or coupling their source code. Third-party code, templates and assets require explicit license review before inclusion.
