# CherryDash Product Scope

CherryDash เป็น unified operations platform ที่พัฒนาขึ้นใหม่ทั้งหมด เพื่อรวม infrastructure monitoring, observability, dashboard, alert, incident, SLO/report และ guardrailed automation ไว้บน object model และ authorization model ชุดเดียว

## Universal dashboard commitment

Dashboard Core ต้องเป็นกลางต่ออุตสาหกรรม วงการใหม่ต้องเพิ่มได้ผ่าน:

```text
Data-source adapter
Universal data frame
Semantic model
Renderer
Domain Pack
Dashboard / Alert / Report definitions
```

ห้ามเพิ่มวงการด้วยการ fork product, copy UI หรือกระจาย business condition เข้าไปใน Core

คำว่า “รองรับทุกวงการ” เป็น architectural capability: วงการใหม่เพิ่ม contract และ pack ได้โดยไม่แก้แกนระบบ ไม่ใช่คำกล่าวว่ามี template สำเร็จรูปครบทุกอุตสาหกรรมแล้ว

## Capability map

| Domain | Operational expectation | Analytical/visual expectation | CherryDash direction |
|---|---|---|---|
| Inventory | Host, device, asset, service, location and ownership | Resource metadata and relationships | Native resource identity and topology graph |
| Collection | Agent, edge, polling, push, stream and discovery | Query external/native sources | Native edge plus versioned public adapters |
| Discovery | Network/resource discovery and registration | Dynamic variables and schema discovery | Unified discovery with evidence, approval and lifecycle |
| Templates | Checks, preprocessing, states and thresholds | Dashboard, report and reusable panels | Versioned packs containing monitoring, semantic and presentation objects |
| Telemetry | Metrics, availability, events, logs and traces | Multi-signal exploration | Typed multi-signal schema plus operational checks |
| Dashboards | Live operations, wallboard and control-room views | Flexible visualization, transformations and drilldown | Industry-neutral dashboard kernel and renderer registry |
| Domain behavior | Units, states, KPIs, workflows and compliance evidence | Industry terminology and presentation | Signed declarative Domain Packs |
| Alerts | Stateful rules, dependencies, maintenance and escalation | Multi-source alert views | One rule model, correlation, suppression and incident grouping |
| Distributed operation | Remote sites and unreliable links | Regional views | Edge collectors, regional ingestion and replay |
| Reporting | Availability, SLA and evidence | Scheduled and governed exports | SLO/SLA, executive, technical and domain reports |
| Automation | Proposed and approved remediation | Action/approval panels | Guardrailed workflow with verification, rollback and audit |
| AI | Optional advisory analysis | Query, summary and explanation | Provider-neutral AI with redaction and policy boundaries |

## Native product modules

1. **Infrastructure** — hosts, VMs, network, storage, cloud, Kubernetes, databases, facilities, devices and applications
2. **Collectors** — edge enrollment, capability, status, queue, version and remote configuration
3. **Explore** — unified metrics/logs/traces/events query and correlation timeline
4. **Dashboards** — versioned definition, universal frames, renderer registry, variables, transformations, interactions, annotations, layouts, revisions and dashboard-as-code
5. **Domain Packs** — semantic models, units, state dictionaries, dashboards, monitor/alert/SLO/report templates, localization and conformance tests
6. **Alerts** — rule evaluation, dependency, inhibition, maintenance, contact point and routing policy
7. **Incidents** — evidence graph, timeline, ownership, status, playbook and post-incident record
8. **SLO/SLA** — SLI definition, error budget, availability, burn-rate alert and customer report
9. **Automation** — proposal, approval, execution, verification, rollback and audit
10. **AI Operations** — anomaly, correlation, probable root cause, impact analysis and natural-language summary
11. **Administration** — tenants, sites, RBAC, identity, audit, quotas, retention, pack/plugin governance and license management

## Dashboard presentation coverage

Core renderer families target:

- KPI, stat, scorecard, gauge, progress and SLA/SLO
- Time-series, state timeline, forecast and anomaly
- Table, pivot, matrix, tree and virtual data grid
- Event, log, trace, calendar, Gantt and schedule
- Topology, dependency, hierarchy, flow and network map
- Geospatial map, route, geofence, floor plan and indoor map
- Process mimic, equipment schematic and digital-twin presentation
- Scientific plot, spectrum, waveform, distribution and uncertainty
- Document, media, annotation, form and approval
- Sandboxed custom renderer including WASM/WebGL/WebGPU classes

Output modes include desktop, tablet, mobile, wallboard, kiosk, embedded, print/PDF, email snapshot and accessibility table fallback

## Industry expansion model

Domain Packs can cover infrastructure, cybersecurity, telecom, data center, manufacturing, energy, utilities, transportation, logistics, healthcare, finance, insurance, retail, agriculture, government, education, construction, mining, facilities, hospitality, media, sports, legal, HR, scientific and future domains without creating separate product forks

Pack content remains declarative Extension code is separately reviewed, permissioned, sandboxed, signed and reversible

## Product constraints

- Core functionality must continue to work without an external AI service
- Air-gapped deployment is a first-class deployment mode
- No unsafe automation action may be executed directly by an LLM or renderer
- Dashboard refresh must not bypass query quotas
- Dashboard definitions and packs must not contain plaintext secrets
- Third-party renderer/adapter code must not run in the main application context without built-in review
- Alert evaluation must not depend on a browser session
- A remote site must continue collecting during WAN outage
- Import compatibility must produce a conversion report; silent loss of unsupported fields is forbidden
- New industries must use semantic/domain contracts rather than Core conditionals
- Accessibility, locale, timezone, units, masking and export policy are part of correctness
