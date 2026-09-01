# Production Readiness Gates

CherryDash is production-eligible only when every blocking gate for the intended deployment profile is satisfied with evidence. A green unit-test badge is useful, but it is not a magic amulet.

## Gate A: End-to-end functional slice

- [ ] create organization and tenant
- [ ] enroll an edge with unique identity
- [ ] collect CPU, memory, disk and network metrics
- [ ] buffer during WAN outage and replay without silent loss
- [ ] authenticate, validate and durably receive centrally
- [ ] publish, normalize and write typed telemetry
- [ ] create/update canonical resource inventory
- [ ] query and display live data with freshness
- [ ] evaluate Pending → Firing → Resolved alert lifecycle
- [ ] route one notification with retry/evidence
- [ ] restart every component without corrupting state

## Gate B: Security

- [ ] mTLS edge/service identity, rotation and revocation
- [ ] SSO and scoped service accounts
- [ ] RBAC/ABAC and PostgreSQL RLS
- [ ] automated cross-tenant isolation tests
- [ ] ingest/query/cardinality quotas
- [ ] secret references and encrypted secret storage
- [ ] monitor egress/SSRF controls
- [ ] sanitized log/event rendering and CSP
- [ ] tamper-evident audit trail
- [ ] threat model reviewed for each exposed protocol

## Gate C: Data correctness and durability

- [ ] typed telemetry schema and schema-compatibility tests
- [ ] segmented checksummed WAL
- [ ] explicit acknowledgement modes
- [ ] replay checkpoint, quarantine and idempotency
- [ ] retention, rollup and deletion policies
- [ ] backup, point-in-time recovery and restore test
- [ ] clock-skew, stale data and duplicate semantics
- [ ] no silent loss on conversion/import

## Gate D: Operability

- [ ] liveness and dependency-aware readiness
- [ ] self-monitoring dashboards and alerts
- [ ] capacity limits and saturation signals
- [ ] structured logs, traces and request correlation
- [ ] upgrade, downgrade and rollback runbooks
- [ ] disaster-recovery runbook and ownership
- [ ] configuration validation and safe reload
- [ ] support bundle with secret redaction

## Gate E: Supply chain and release

- [ ] project license selected
- [ ] lockfiles committed and CI uses deterministic install
- [ ] dependency/license/secret/SAST/container scans
- [ ] SBOM and provenance generated
- [ ] images pinned by digest for release
- [ ] artifacts and agents signed
- [ ] reproducible or independently verifiable builds
- [ ] third-party notices and source/asset provenance review
- [ ] release notes include migrations and rollback

## Gate F: Performance claims

- [ ] benchmark dataset and generator published
- [ ] ingest throughput and latency tested by durability mode
- [ ] query latency tested by cardinality/time range/concurrency
- [ ] WAL/replay, restart and failure scenarios included
- [ ] resource usage and cost model recorded
- [ ] hardware, configuration and software versions preserved
- [ ] no marketing scale number exceeds tested evidence

## Required release evidence

Each release candidate stores:

```text
release manifest
commit and dependency locks
migration plan
security scan results
SBOM and signatures
benchmark artifact
backup/restore result
upgrade/rollback result
known limitations
operator runbook links
```
