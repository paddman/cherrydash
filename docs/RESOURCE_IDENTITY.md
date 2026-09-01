# Canonical Resource Identity

Monitoring data is useful only when CherryDash can determine which real resource produced it. Hostname and IP address are attributes, not durable identity.

## Goals

- correlate the same asset discovered through agent, SNMP, cloud, Kubernetes and API sources
- survive IP, hostname, location and parent changes
- preserve merge/split history and evidence
- prevent duplicate resources from fragmenting alerts and SLOs
- support topology, ownership, dependency and blast-radius analysis

## Core model

```text
Organization
└── Tenant
    └── Scope: Region / Site / Zone / Cluster
        └── Resource
            ├── Physical asset
            ├── Virtual machine
            ├── Network device
            ├── Cloud resource
            ├── Kubernetes resource
            ├── Database
            ├── Application / Service
            ├── Endpoint
            └── Process / Workload
```

Every resource has a CherryDash-generated immutable `resource_id`. External identifiers are evidence attached to the resource, not the primary key.

## Identity evidence

Examples, ordered roughly from stronger to weaker:

- cloud provider resource ID plus account and region
- hardware serial/UUID with vendor and scope
- Kubernetes UID with cluster identity
- agent enrollment identity and machine boot identity
- hypervisor VM UUID
- certificate public-key fingerprint
- stable MAC address within a network scope
- hostname/domain within a site
- IP address within a time-bounded network scope

Each observation records source, first/last seen, confidence and normalization version.

## Resolution process

```text
Observation
   ↓ normalize
Candidate lookup
   ↓ score evidence
No match ───────────────► create resource
Unique strong match ───► attach observation
Ambiguous match ───────► quarantine/review
Conflicting identity ──► split or merge workflow
```

Automatic merge is allowed only above a configured confidence threshold and when no strong identifier conflicts. Every merge/split is versioned and reversible.

## Required fields

```text
resource_id
organization_id
tenant_id
resource_type
lifecycle_state
canonical_name
scope_id
external_identifiers[]
aliases[]
attributes
labels
owners[]
first_seen_at
last_seen_at
identity_version
```

## Topology edges

Relationships are first-class versioned objects:

```text
runs_on
contains
connected_to
depends_on
serves
backs
routes_to
managed_by
owned_by
```

Each edge stores source, confidence, valid-from and valid-until. Current topology is a projection; history must remain queryable for incident reconstruction.

## P0 implementation slice

1. Define `Resource`, `ResourceObservation`, `ExternalIdentifier` and `TopologyEdge` schemas.
2. Create deterministic normalization for identifiers and labels.
3. Implement exact strong-identifier matching only.
4. Record unresolved observations without creating endless duplicates.
5. Add explicit merge/split API and immutable history.
6. Bind telemetry to `resource_id` during enrichment, not by dashboard guesswork.
7. Test IP/hostname changes, cloned VM UUIDs, cross-site name collisions and container recreation.
