# CherryDash

**Universal Monitoring, Observability & Dashboard Platform**

CherryDash คือแพลตฟอร์ม monitoring และ observability ที่พัฒนาขึ้นใหม่ทั้งหมด พร้อม dashboard engine แบบ industry-neutral ซึ่งใช้ object model, tenant, RBAC, query, alert, incident และ lifecycle ชุดเดียว วงการใหม่เพิ่มผ่าน semantic contract, renderer และ Domain Pack โดยไม่แก้หรือ fork แกนระบบ

> Status: foundation / pre-alpha. ยังไม่พร้อมใช้ production และยังไม่มีผล benchmark ที่รับรอง scale claim

## Non-fork commitment

CherryDash เป็น clean-room implementation: ไม่ fork, copy, embed, rebrand หรือใช้ฐานข้อมูลภายในของผลิตภัณฑ์ monitoring/dashboard อื่นเป็นแกนระบบ การเชื่อมต่อภายนอกทำผ่าน public protocol, open standard และ isolated migration adapter ที่ผ่านการตรวจ license เท่านั้น ดู [`docs/NO_FORK_POLICY.md`](docs/NO_FORK_POLICY.md)

## เป้าหมายผลิตภัณฑ์

- Monitor server, VM, network, storage, cloud, Kubernetes, database, application, facility, device และ API
- Agent + agentless collection ผ่าน public/open protocols และ versioned adapters
- Distributed edge collectors สำหรับ branch, customer edge, data center, cloud และ air-gapped environment
- Unified metrics, logs, traces, events, inventory, topology, alerting, incident, SLO/SLA และ report
- Universal dashboard kernel สำหรับ KPI, time series, table, event, topology, geospatial, industrial, scientific, media, workflow และ custom renderer
- Responsive definitions สำหรับ desktop, mobile, wallboard, kiosk, embed, print, PDF และ accessibility fallback
- Versioned Domain Pack สำหรับความหมาย หน่วย สถานะ dashboard alert report และ workflow ของแต่ละวงการ
- Dashboard-as-code, versioned template packs, multi-tenancy, RBAC และ tamper-evident audit
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
| Open ecosystem | OpenTelemetry and Prometheus public protocols |

## Current runnable slice

- `cherrydash-server`: health, system information and foundation overview API
- `cherrydash-ingest`: credential-bound tenant ingestion with append-only local WAL
- `cherrydash-edge`: authenticated heartbeat and basic Linux host snapshot collector
- `cherrydash-core`: telemetry contract plus dashboard definition v1 and validation
- `web`: enterprise CherryDash dashboard shell connected to overview API
- JSON schemas for dashboard definitions, renderer manifests and declarative Domain Packs
- Docker Compose topology for services plus PostgreSQL, ClickHouse, NATS, Valkey and MinIO

NATS publishing, WAL replay, ClickHouse writing, PostgreSQL-backed APIs, universal frame query path and renderer runtime are subsequent implementation slices; infrastructure and schemas are not falsely presented as complete production functionality.

## Quick start

```bash
git clone https://github.com/paddman/cherrydash.git
cd cherrydash
git switch feat/foundation-v0.1
cp .env.example .env
docker compose -f deploy/compose/docker-compose.yml up --build
```

Development services bind to `127.0.0.1` by default. Change `CHERRYDASH_BIND_HOST` only when network exposure is intentional and protected.

Open:

- Web UI: `http://localhost:3000`
- Control API: `http://localhost:8080/healthz`
- Ingest API: `http://localhost:8081/healthz`
- NATS monitoring: `http://localhost:8222`
- MinIO console: `http://localhost:9003`

The example environment uses a development-only bearer token. Replace it before any shared or non-development deployment.

Send a test event:

```bash
curl -sS -X POST http://localhost:8081/api/v1/events \
  -H 'content-type: application/json' \
  -H 'authorization: Bearer development-only-change-me' \
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

`durableWal=false` means the gateway acknowledged after userspace/OS flush only. Set `CHERRYDASH_INGEST_SYNC_WRITES=true` to require `sync_data()` before a durable acknowledgement. Segmented WAL, batching and replay remain P0 work.

## Dashboard contract example

```bash
cat examples/dashboards/universal-operations.dashboard.json
```

The example demonstrates the schema only. A renderer identifier in a definition does not mean that renderer has been implemented or approved.

## Native development

Requirements: current stable Rust toolchain and Node.js 22+

```bash
cargo run -p cherrydash-ingest
cargo run -p cherrydash-server
CHERRYDASH_INGEST_TOKEN=development-only-change-me \
CHERRYDASH_INGEST_URL=http://127.0.0.1:8081 \
  cargo run -p cherrydash-edge

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
crates/        shared Rust types, validation and libraries
services/      central control/data-plane services
web/           native CherryDash dashboard application
deploy/        containers, Compose, storage schemas and proxy config
docs/          architecture, dashboard, scope, security, identity, delivery and roadmap
schemas/       versioned public dashboard, renderer and Domain Pack contracts
examples/      non-production examples used to demonstrate contracts
proto/         versioned RPC contracts
```

## Design rules

1. Clean-room implementation; no fork, copied source, embedded upstream UI or inherited private schema
2. Open standards at every ingestion boundary
3. Rust on performance-sensitive and edge paths
4. Tenant identity comes from authenticated credentials, not a client-selected tenant header
5. Durable receipt is reported only when the configured durability boundary has completed
6. Scale data plane, query plane and control plane independently
7. One product object model for dashboards, alerts, incidents, RBAC and audit
8. Dashboard Core remains industry-neutral; domain behavior arrives through versioned declarative packs and approved extensions
9. Renderer and adapter extensions require sandbox, permission, provenance, compatibility and rollback contracts
10. No LLM may directly execute a risky infrastructure action
11. No silent loss during import, conversion, buffering or replay
12. No feature, compliance or scale claim without repeatable evidence

## Documentation

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`docs/DASHBOARD_PLATFORM.md`](docs/DASHBOARD_PLATFORM.md)
- [`docs/NO_FORK_POLICY.md`](docs/NO_FORK_POLICY.md)
- [`docs/SECURITY_ARCHITECTURE.md`](docs/SECURITY_ARCHITECTURE.md)
- [`docs/RESOURCE_IDENTITY.md`](docs/RESOURCE_IDENTITY.md)
- [`docs/WAL_DELIVERY.md`](docs/WAL_DELIVERY.md)
- [`docs/PRODUCTION_READINESS.md`](docs/PRODUCTION_READINESS.md)
- [`docs/PRODUCT_SCOPE.md`](docs/PRODUCT_SCOPE.md)
- [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md)
- [`docs/ROADMAP.md`](docs/ROADMAP.md)
- [`docs/adr/0001-core-platform-stack.md`](docs/adr/0001-core-platform-stack.md)
- [`docs/adr/0002-universal-dashboard-kernel.md`](docs/adr/0002-universal-dashboard-kernel.md)

## Licensing note

A CherryDash project license has not yet been selected. Third-party libraries, public protocols, templates and assets require explicit dependency/license review before inclusion. Product compatibility never grants permission to copy another implementation.
