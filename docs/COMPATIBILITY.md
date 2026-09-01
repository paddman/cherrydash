# Compatibility Strategy

Compatibility is implemented at protocol and document boundaries. CherryDash must not depend on embedding another product's frontend or database schema.

## Ingestion compatibility

Planned receivers, ordered by implementation priority:

1. OpenTelemetry Protocol: OTLP/gRPC and OTLP/HTTP for metrics, logs and traces
2. Prometheus remote write and OpenMetrics scrape
3. Syslog TCP/UDP/TLS and common structured event formats
4. SNMP polling and traps
5. ICMP, TCP, TLS, DNS and HTTP synthetic checks
6. Zabbix sender/trapper-compatible input for migration and coexistence
7. Cloud and Kubernetes APIs
8. Windows WMI/WinRM, Linux SSH, IPMI/Redfish and vendor APIs

## Query and dashboard compatibility

- PromQL-compatible query surface for metric workloads
- OpenTelemetry semantic conventions preserved where possible
- Grafana dashboard JSON importer with an explicit conversion report
- Variables, time range, annotations, common panel types and transformations mapped to CherryDash objects
- External data-source adapters remain possible, but native CherryDash storage is the default path

## Monitoring migration

- Zabbix host, group, tag, template, item, trigger, discovery and maintenance import is implemented as a migration pipeline
- Unsupported preprocessing, macro semantics or expression functions are reported as blocking or lossy conversions
- Imported content is converted to a versioned CherryDash template pack rather than executed directly
- Coexistence mode can receive Zabbix sender data while migration is in progress

## Export and automation

- Dashboard, monitor template, alert rule and SLO definitions are exportable as versioned YAML/JSON
- APIs are described with OpenAPI and Protobuf
- Terraform provider and CLI are planned after object schemas stabilize

## Licensing boundary

Current Grafana and current Zabbix releases use AGPLv3 licensing. CherryDash therefore treats compatibility as clean protocol/document integration and does not copy their source code, proprietary assets or documentation into the CherryDash core. Any future reuse of third-party source or templates requires an explicit license review and attribution record.

This document is an engineering boundary, not legal advice.
