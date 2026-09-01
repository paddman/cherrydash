# Security Architecture

> Status: design baseline. The current development stack does not satisfy the production controls below.

## Trust zones

```text
Untrusted monitored networks
        │
        ▼
CherryDash Edge
  collection sandbox
  local durable queue
  credential references
        │ mTLS + enrolled identity
        ▼
Regional/Central Ingest
  authentication
  tenant binding
  schema/quota enforcement
  central WAL
        │ authenticated event fabric
        ▼
Processors and Storage
  tenant-scoped consumers
  idempotent writes
  retention and redaction
        │ authorized query context
        ▼
Control, Query and Web
  SSO/RBAC/ABAC
  audit and policy
  human-approved automation
```

## Security invariants

1. A client cannot choose another tenant by changing a header, label, subject or payload.
2. Every stored record has a validated tenant and source identity.
3. Edge and service credentials are unique, revocable and rotatable.
4. No plaintext secret is stored in telemetry, dashboard or template documents.
5. Query and alert evaluation share the same authorization and quota model.
6. Imported content and collected text are untrusted and never rendered/executed without sanitization.
7. AI output is untrusted advisory data until policy and human approval allow an action.
8. Audit evidence cannot be silently updated or deleted.

## Primary threats

### Collection and monitor abuse

- SSRF and internal port scanning through HTTP/TCP checks
- DNS rebinding
- credential theft through SSH, WinRM, SNMP or database monitors
- command injection in preprocessing or scripts
- malicious device responses and parser vulnerabilities

Controls: network-zone policies, destination allow/deny rules, credential vault references, parser isolation, execution sandbox, timeout/output limits and signed monitor definitions.

### Telemetry abuse

- payload or compression bombs
- label/cardinality explosions
- forged timestamps and sequence numbers
- duplicate/replay floods
- stored XSS, ANSI escape injection and log forging
- poison records targeting processors

Controls: compressed and decompressed size limits, typed schemas, attribute budgets, clock-skew policy, event identity, rate limits, quarantine queues, output escaping and fuzzed parsers.

### Tenant boundary attacks

- caller-selected tenant identifiers
- cross-tenant NATS wildcard subscriptions
- missing SQL tenant predicates
- cache-key collisions
- cross-tenant dashboard links or alert evidence

Controls: credential-derived tenant context, NATS accounts/subject permissions, PostgreSQL RLS, tenant-prefixed cache keys, ClickHouse query policy and automated isolation tests.

### Query denial of service

- unbounded time ranges
- high-cardinality group-by
- dashboard refresh storms
- concurrent export/report jobs
- regex and transformation abuse

Controls: scan-byte limits, time-range limits, concurrency pools, cancellation, per-tenant quotas, result limits, cached rollups and cost estimates.

### Supply-chain threats

- compromised dependency or container image
- mutable image tags
- leaked build credentials
- unsigned agent updates
- malicious plugin packages

Controls: lockfiles, dependency/license policy, SBOM, provenance, digest pinning, signed artifacts, reproducible builds, isolated builders and plugin signatures/sandboxing.

## Identity target

Development bearer-token JSON is a temporary bootstrap mechanism only. Production requires:

- one-time edge enrollment token
- issued edge certificate and private key
- mTLS with certificate rotation and revocation
- service accounts and scoped short-lived credentials
- OIDC/SAML/LDAP for users
- role and attribute policy bindings
- break-glass account with explicit audit and alerting

## Production-blocking P0 controls

- [ ] Edge enrollment and mTLS
- [ ] Credential-derived tenant context across HTTP, gRPC and NATS
- [ ] PostgreSQL RLS and tenant-isolation tests
- [ ] Rate, payload, attribute and query quotas
- [ ] Secret-reference and vault interface
- [ ] Append-only tamper-evident audit sink
- [ ] CSP/output sanitization and stored-XSS tests
- [ ] Network monitor SSRF/egress policy
- [ ] Dependency, secret, SAST and container scanning
- [ ] Signed release and agent update verification
- [ ] Backup/restore and incident response runbooks
