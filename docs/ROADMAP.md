# CherryDash Roadmap

Roadmap เรียงตาม dependency และความเสี่ยง ไม่เรียงตามความสวยของ UI ทุก phase ต้องมี tests, telemetry, security review, migration และ rollback path

## Phase 0 — Foundation

สถานะ: **in progress**

- [x] Rust workspace and canonical foundation envelope
- [x] Separate control plane, ingestion gateway and edge collector
- [x] Append-only development ingestion file
- [x] Authenticated development edge heartbeat
- [x] Enterprise dashboard shell
- [x] Industry-neutral dashboard definition v1, Rust validation and JSON Schema
- [x] Renderer and declarative Domain Pack manifest schemas
- [x] Docker Compose development topology
- [x] PostgreSQL and ClickHouse initial schemas
- [x] CI foundation
- [x] No-fork/clean-room policy
- [x] Initial security, identity, WAL, dashboard and readiness specifications
- [ ] Cargo/npm lockfiles and deterministic CI install
- [ ] End-to-end integration test

## Phase 0.5 — Trustworthy vertical slice

งานส่วนนี้เป็น P0 และต้องเสร็จก่อนเพิ่ม AI หรือประกาศ production readiness

### Identity and security

- Edge enrollment, unique certificate identity, mTLS, rotation and revocation
- Credential-derived tenant context across HTTP, gRPC, event fabric, storage and cache
- OIDC/service accounts, RBAC/ABAC and PostgreSQL RLS
- Secret-reference/vault interface
- Ingest, query, cardinality and storage quotas
- Monitor egress policy and SSRF protection

### Correct data model

- Canonical resource identity, observation evidence and merge/split history
- Versioned topology edges and ownership
- Typed metric, log, span, event, inventory and heartbeat payloads
- Schema compatibility, clock-skew and stale-data policy
- Cardinality budgets and deterministic series fingerprints

### Delivery and durability

- Bounded receiver queues and dedicated WAL writer
- Segmented checksummed WAL with recovery
- Explicit memory/WAL/durable acknowledgement modes
- Replay checkpoint, quarantine and idempotent consumers
- Edge local store-and-forward with disk-pressure policy
- Failure injection and power-loss test matrix

### Minimal monitoring loop

- PostgreSQL repository/migration layer
- Event publisher and ClickHouse batch writer
- Real host inventory and live overview queries
- Distributed scheduler with lease/fencing/run ID
- Native CPU, memory, disk and network collection
- Alert Pending/Firing/Resolved state machine
- Webhook notification with retry evidence
- Backup/restore and full component restart test

**Definition of done:** the complete collection-to-alert path survives WAN loss, restart and duplicate delivery; tenant A cannot read or write tenant B; backup restores onto a clean installation.

## Phase 1 — Telemetry spine

- Regional event-fabric topology, accounts, permissions and replication
- OTLP HTTP/gRPC receivers implemented as CherryDash services
- Prometheus remote-write and OpenMetrics support
- Typed ClickHouse tables, materialized rollups, TTL and cold-tier export
- Ingestion health, queue age, lag, retry, duplicate and storage-latency dashboards
- Query gateway with cancellation and tenant resource governance
- Universal frame transport with semantic metadata and version negotiation

**Definition of done:** telemetry survives component outage without silent loss and every stored signal is queryable through a tenant-safe typed contract.

## Phase 2 — Monitoring depth

- ICMP, TCP, DNS, TLS and HTTP synthetic checks
- SNMP v2/v3 polling and traps
- SSH, WMI/WinRM, IPMI/Redfish and database checks
- Network discovery, auto-registration and low-level discovery
- Versioned monitoring packs, variables/macros and preprocessing
- Maintenance windows, dependencies and availability calculation
- Signed edge configuration and staged fleet upgrades

**Definition of done:** a remote site monitors servers/network devices during central outage and reconciles safely after reconnect.

## Phase 3 — Universal query and dashboard runtime

### Query and semantic layer

- Metrics query API with PromQL-compatible public surface
- Log/event query language and trace waterfall contract
- Universal Data Frame API with scalar, table, time-series, event, topology, geospatial, media, document and custom shapes
- Field semantic metadata for unit, currency, timezone, resource, quality, sensitivity and state
- Shared versioned transformation pipeline for dashboard, alert and report
- Resource graph and service topology enrichment

### Native dashboard runtime

- Dashboard persistence, revision history, folders and promotion workflow
- Renderer registry and capability negotiation
- Built-in stat, time-series, table, status-grid, text and event renderers
- Variables, transformations, annotations, drilldown, cross-filter and linked time range
- Desktop, tablet, mobile, wallboard, kiosk, embed, print/PDF and accessibility layouts
- Server-side deterministic report/snapshot rendering
- Library panels, reusable queries and dashboard-as-code/GitOps API

### Extensibility and global coverage

- Renderer SDK with built-in, Web Worker, WASM and sandboxed-iframe execution modes
- Signed renderer manifest, permission policy, quotas and rollback
- Declarative Domain Pack registry, semantic dictionaries and conformance tests
- Adapter SDK and schema discovery
- Locale, timezone, currency, SI/IEC/imperial units, RTL and translation bundles
- Keyboard, screen-reader, table fallback, high-contrast and color-independent behavior
- Geospatial, topology, process-mimic, scientific, media and guarded workflow renderer families
- Isolated optional document importers with explicit conversion reports

**Definition of done:**

- Dashboards are tenant-safe, query-limited and shareable as code
- Dashboard, alert and report use the same query/transform semantics
- A new industry is delivered as a Domain Pack without modifying Core services or primary database schema
- A new renderer installs through a versioned sandboxed contract and cannot bypass authorization
- One dashboard definition renders correctly in desktop, mobile, wallboard and print modes
- Accessibility, locale, timezone, unit, masking and export-policy tests pass

## Phase 4 — Alert, incident and SLO

- Full alert state/history, NoData/Error/Stale and flapping behavior
- Deduplication, inhibition, dependency and maintenance suppression
- Notification routing, escalation, on-call and silence
- Incident evidence graph, ownership and correlated timeline
- SLI/SLO, error budget and multi-window burn-rate alerts
- Scheduled executive, technical and domain reports

**Definition of done:** every alert decision can be replayed from rule version, input window, query and evaluation timestamp.

## Phase 5 — Automation and AI operations

- Playbook definition and execution engine
- Approval, dry-run, timeout, verification and rollback
- Credential-vault references and separation of duties
- AI incident summary, anomaly explanation, probable root cause and remediation proposal
- Local/external model provider abstraction with data-redaction policy
- Tamper-evident audit and policy evaluation

**Definition of done:** AI recommends but cannot bypass policy or approval; every action has verification and rollback evidence.

## Phase 6 — Enterprise and proven scale

- Kubernetes operator and Helm charts
- ClickHouse sharding/replication automation
- Regional event-fabric clusters and disaster recovery
- PostgreSQL HA and online schema migration
- OIDC, SAML, LDAP, SCIM and fine-grained MSP hierarchy
- Tenant quotas, retention tiers and chargeback
- Air-gapped update bundles, SBOM, signing and upgrade orchestration
- Signed Domain Pack/renderer distribution and revocation
- Industry conformance suites with explicit supported-version matrix
- Repeatable backend, query, renderer and browser-memory benchmark suite

**Definition of done:** enterprise, industry and scale claims are backed by release evidence, production-like datasets and tested recovery procedures.
