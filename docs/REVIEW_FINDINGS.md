# Foundation Review Findings

Review date: 2026-09-01

## Verdict

The stack and repository boundaries are suitable for continuing, but the branch remains a foundation rather than a functional monitoring platform. Identity, durability, typed schemas, scheduling and tenant isolation are the critical path.

## P0: blocks a trustworthy vertical slice

1. Edge enrollment, mTLS and credential-derived tenant identity
2. Segmented WAL, bounded queues, replay checkpoint and idempotency
3. Edge local store-and-forward
4. Typed telemetry payloads and schema evolution
5. Canonical resource identity and topology
6. ClickHouse writer, retention, rollups and cardinality controls
7. PostgreSQL migrations, RLS and control-plane repositories
8. Distributed scheduler with leases, fencing and deterministic run IDs
9. Alert state machine with evidence and reproducible evaluation
10. End-to-end restart, isolation, backup and restore tests

## P1: required before enterprise pilot

- OIDC/SAML/LDAP and fine-grained RBAC
- maintenance, dependency, inhibition, silence and escalation
- query cost governance and workload isolation
- SNMP, synthetic checks and secure credential references
- dashboard persistence and real query engine
- self-observability, support bundle and operator runbooks
- signed edge updates with staged rollout and rollback
- retention/cold-tier lifecycle and legal deletion workflow

## P2: defer until the spine is reliable

- AI root-cause analysis
- natural-language dashboard generation
- broad migration importers
- marketplace/plugin ecosystem
- mobile application
- multi-region control plane
- billing and chargeback UI

## Immediate implementation order

```text
P0-1 Resource identity ADR/schema
P0-2 Telemetry contract v2
P0-3 Edge enrollment and mTLS
P0-4 WAL v2 and edge queue
P0-5 Event publisher and typed ClickHouse writer
P0-6 PostgreSQL repository/RLS
P0-7 Real inventory and overview query
P0-8 Scheduler and native host checks
P0-9 Alert lifecycle and webhook
P0-10 Failure/isolation/restore test suite
```

## Runtime hardening already applied on the foundation branch

- removed permissive CORS from Rust APIs
- tenant is selected by an authenticated ingest credential rather than a tenant header
- default ports bind to loopback
- NATS development authentication is enabled
- WAL status distinguishes buffered flush from durable fsync
- security headers and CSP were added to the web proxy

These are bootstrap improvements, not completion of the P0 security design.
