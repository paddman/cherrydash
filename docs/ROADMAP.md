# CherryDash Roadmap

Roadmap นี้เรียงตาม dependency ของระบบ ไม่เรียงตามความสวยของหน้า UI ทุก phase ต้องมี tests, telemetry, security review และ rollback path

## Phase 0 — Foundation

สถานะ: **in progress**

- [x] Rust workspace and canonical telemetry envelope
- [x] Separate control plane, ingestion gateway and edge collector
- [x] Append-only local ingestion WAL
- [x] Edge heartbeat and basic Linux host snapshot
- [x] Enterprise dashboard shell
- [x] Docker Compose development topology
- [x] PostgreSQL and ClickHouse initial schemas
- [x] CI foundation
- [ ] Reproducible lock files and signed release artifacts
- [ ] End-to-end integration test

## Phase 1 — Telemetry spine

- NATS JetStream stream topology, account permissions and replication policy
- WAL replay worker with back-pressure, checkpoint and poison-message quarantine
- ClickHouse batch writer and materialized rollups
- OTLP HTTP and gRPC ingestion
- Prometheus remote-write ingestion
- API keys, mTLS edge identity, quotas and rate limiting
- Live ingestion health, lag, dropped/retried record and storage latency dashboards

**Definition of done:** telemetry survives component restart, WAN interruption and consumer outage without silent loss; replay and duplicate handling are tested.

## Phase 2 — Monitoring depth

- Scheduler and distributed check assignment
- ICMP, TCP, DNS, TLS and HTTP checks
- SNMP v2/v3 polling and traps
- SSH, WMI/WinRM, IPMI/Redfish and database checks
- Network discovery, auto-registration and low-level discovery
- Versioned monitoring template packs, macros and preprocessing pipeline
- Maintenance windows, dependencies and availability calculation

**Definition of done:** a remote site can monitor servers and network devices during central outage and reconcile when connectivity returns.

## Phase 3 — Query and dashboard engine

- Metrics query API with PromQL-compatible layer
- Log/event query language and trace waterfall
- Resource graph and service topology
- Dashboard persistence, folders, variables, transformations and annotations
- Library panels and reusable queries
- Dashboard/template GitOps API
- Grafana JSON import with conversion report

**Definition of done:** dashboards are tenant-safe, query-limited, shareable as code and usable by the alert engine without semantic drift.

## Phase 4 — Alert, incident and SLO

- Stateful alert evaluator, pending/firing/resolved lifecycle
- Deduplication, inhibition, dependency and maintenance suppression
- Notification routing, escalation, on-call and silence
- Incident evidence graph and correlated timeline
- SLI/SLO, error budget and multi-window burn-rate alerts
- Scheduled executive and technical reports

**Definition of done:** every alert decision is reproducible from stored query, rule version and evidence.

## Phase 5 — Automation and AI operations

- Playbook definition and execution engine
- Approval, dry-run, timeout, verification and rollback
- Credential vault references and separation of duties
- AI incident summary, anomaly explanation, probable root cause and remediation proposal
- Local-model and external-model provider abstraction
- Immutable audit evidence and policy evaluation

**Definition of done:** AI can recommend but cannot bypass policy or approval; every action has verification and rollback evidence.

## Phase 6 — Enterprise and massive scale

- Kubernetes operator and Helm charts
- ClickHouse sharding/replication automation
- NATS regional clusters, gateways and disaster recovery
- PostgreSQL HA and online schema migration
- OIDC, SAML, LDAP, SCIM and fine-grained RBAC
- MSP hierarchy, tenant quotas, chargeback and retention tiers
- Air-gapped update bundles, SBOM, signing and upgrade orchestration
- Performance benchmark suite and published capacity model

**Definition of done:** scale claims are backed by repeatable benchmark artifacts and production-like datasets.
