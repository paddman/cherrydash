# CherryDash

**Unified Infrastructure Monitoring & Observability Platform**

CherryDash is a next-generation, open-standards monitoring platform that combines the operational depth of Zabbix with the dashboarding and exploratory experience of Grafana—without coupling the product to either codebase.

> Status: architecture foundation / pre-alpha

## Product direction

- Infrastructure monitoring: hosts, VMs, networks, storage, cloud, containers, applications
- Agent and agentless collection: OTLP, Prometheus, SNMP, syslog, HTTP, ICMP, SSH, WMI, IPMI
- Distributed edge collectors for branch, customer-edge, data-center, cloud, and air-gapped environments
- Unified metrics, logs, traces, events, inventory, topology, alerting, SLO/SLA, reports, and automation
- Dashboard-as-code, template packs, multi-tenancy, RBAC, audit trail, and enterprise identity integration
- AI-assisted anomaly detection, correlation, root-cause analysis, forecasting, and incident summaries

## Engineering principles

1. **Open standards first** — OpenTelemetry and Prometheus compatible at the ingestion boundary.
2. **Rust data plane** — predictable performance, memory safety, and efficient edge binaries.
3. **Scale-out by design** — tenant-aware partitioning, stateless APIs, durable event streaming, and horizontally scalable storage.
4. **Edge resilient** — local buffering, back-pressure, replay, remote configuration, and safe upgrades.
5. **One product experience** — inventory, queries, dashboards, alerts, incidents, and automation share one object model and one RBAC system.
6. **No AI directly executes risky actions** — proposals, approvals, expiration, rollback, and immutable audit evidence are required.

## Planned stack

- Core services and agents: Rust, Tokio, Axum, tonic, Apache Arrow/DataFusion
- Web application: TypeScript, React, Vite
- APIs: Protobuf/gRPC internally; REST and WebSocket externally
- Telemetry/event fabric: NATS JetStream
- Telemetry analytics: ClickHouse
- Control-plane state: PostgreSQL
- Cache/ephemeral coordination: Valkey
- Object storage and cold tier: S3/MinIO + Parquet
- Deployment: Docker Compose for single-node; Kubernetes/Helm for clustered installations

Detailed architecture and runnable services are being added on the `feat/foundation-v0.1` branch.

## License

A license will be selected before the first public release. Do not assume third-party dashboard, template, or protocol compatibility implies code or license inheritance from Zabbix or Grafana.
