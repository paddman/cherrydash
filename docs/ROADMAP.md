# CherryDash Roadmap

Roadmap เรียงตาม dependency และความเสี่ยง ไม่เรียงตามความสวยของ UI ทุก phase ต้องมี tests, telemetry, security review, migration และ rollback path

## Phase 0 — Foundation

สถานะ: **in progress**

- [x] Rust workspace and canonical foundation envelope
- [x] Separate control plane, ingestion gateway and edge collector
- [x] Append-only development ingestion file
- [x] Authenticated development edge heartbeat
- [x] Enterprise dashboard shell
- [x] Docker Compose development topology
- [x] PostgreSQL and ClickHouse initial schemas
- [x] CI foundation
- [x] No-fork/clean-room policy
- [x] Initial security, identity, WAL and readiness specifications
- [ ] Cargo/npm lockfiles and deterministic CI install
- [ ] End-to-end integration test

## Phase 0.5 — Trustworthy vertical slice

งานส่วนนี้เป็น P0 และต้องเสร็จก่อนเพิ่ม AI หรือประกาศ production readiness

### Identity and security

- Edge enrollment, unique certificate identity, mTLS, rotation and revocation
- Credential-derived tenant context across HTTP, gRPC, NATS, storage and cache
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

## Phase 3 — Query and dashboard engine

- Metrics query API with PromQL-compatible public surface
- Log/event query language and trace waterfall
- Resource graph and service topology
- Dashboard persistence, folders, variables, transformations and annotations
- Library panels and reusable queries
- Dashboard/template GitOps API
- Isolated optional document importers with explicit conversion reports

**Definition of done:** dashboards are tenant-safe, query-limited, shareable as code and use the same query semantics as alert evaluation.

## Phase 4 — Alert, incident and SLO

- Full alert state/history, NoData/Error/Stale and flapping behavior
- Deduplication, inhibition, dependency and maintenance suppression
- Notification routing, escalation, on-call and silence
- Incident evidence graph, ownership and correlated timeline
- SLI/SLO, error budget and multi-window burn-rate alerts
- Scheduled executive and technical reports

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
- Repeatable benchmark suite and published capacity model

**Definition of done:** enterprise and scale claims are backed by release evidence, production-like datasets and tested recovery procedures.
