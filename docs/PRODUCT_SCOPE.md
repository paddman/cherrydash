# CherryDash Product Scope

CherryDash ไม่ใช่การเอา UI ของ Grafana วางบน Zabbix และไม่ใช่ fork ของสองระบบ แต่เป็น unified operations platform ที่รักษาความสามารถสำคัญของทั้งคู่และเพิ่ม observability/automation รุ่นใหม่

## Capability map

| Domain | Monitoring platform expectation | Dashboard platform expectation | CherryDash direction |
|---|---|---|---|
| Inventory | Host, interface, group, tag, macro, asset fields | Data-source metadata | Native inventory and resource graph |
| Collection | Agent, proxy, SNMP, ICMP, HTTP, SSH, WMI, IPMI, traps | Query external sources | Native edge plus open-standard adapters |
| Discovery | Network discovery, low-level discovery, auto-registration | Dynamic variables | Unified discovery with approval and lifecycle |
| Templates | Items, triggers, discovery rules, macros | Dashboard/library panels | Versioned template packs containing collection, dashboard, alert and SLO objects |
| Telemetry | Metrics, availability, events, logs | Metrics, logs, traces, profiles | OTel-native multi-signal schema plus infrastructure checks |
| Dashboards | Operational screens and graphs | Flexible panels, variables, transformations, Explore | Native dashboard engine and query workbench |
| Alerts | Trigger expressions, dependencies, maintenance, escalation | Multi-source alert rules and notification policies | One rule model, correlation, suppression and incident grouping |
| Distributed operation | Proxy and central server | Distributed data sources | Edge collectors, regional ingest and multi-region fabric |
| Reporting | Availability, SLA, scheduled reports | Dashboard sharing and reporting | SLA/SLO, executive report and evidence export |
| Automation | Remote command and media actions | Webhooks/integrations | Guardrailed workflow with approval, rollback and audit |
| AI | External/custom | External/custom | RCA suggestions, anomaly detection, incident summary and query assistant |

## Native product modules

1. **Infrastructure** — hosts, VMs, network, storage, cloud, Kubernetes, databases and applications
2. **Collectors** — edge enrollment, capability, status, queue, version and remote configuration
3. **Explore** — unified metrics/logs/traces/events query and correlation timeline
4. **Dashboards** — panels, variables, transformations, annotations, library objects and dashboard-as-code
5. **Alerts** — rule evaluation, dependency, inhibition, maintenance, contact point and routing policy
6. **Incidents** — evidence graph, timeline, ownership, status, playbook and post-incident record
7. **SLO/SLA** — SLI definition, error budget, availability, burn-rate alert and customer report
8. **Automation** — proposal, approval, execution, verification, rollback and audit
9. **AI Operations** — anomaly, correlation, probable root cause, impact analysis and natural-language summary
10. **Administration** — tenants, sites, RBAC, identity, audit, quotas, retention and license management

## Product constraints

- Core functionality must continue to work without an external AI service
- Air-gapped deployment is a first-class deployment mode
- No unsafe automation action may be executed directly by an LLM
- Dashboard refresh must not bypass query quotas
- Alert evaluation must not depend on a browser session
- A remote site must continue collecting during WAN outage
- Import compatibility must produce a conversion report; silent loss of unsupported fields is forbidden
