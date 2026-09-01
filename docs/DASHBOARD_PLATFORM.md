# CherryDash Universal Dashboard Platform

CherryDash Dashboard ถูกออกแบบให้รองรับงานจากทุกอุตสาหกรรมโดยใช้ **industry-neutral kernel** ไม่ใช่การเขียนเงื่อนไขแยกสำหรับโรงงาน โรงพยาบาล ธนาคาร หรือศูนย์ปฏิบัติการลงใน Core จนระบบกลายเป็นกอง `if industry == ...` ที่ไม่มีใครกล้าแก้

> คำว่า “รองรับทุกวงการ” หมายถึงวงการใหม่สามารถเพิ่ม semantic model, data adapter, renderer, dashboard, alert และ workflow ผ่าน contract ที่กำหนดไว้ โดยไม่ต้อง fork หรือแก้แกนหลัก ไม่ได้หมายความว่า CherryDash มี template สำเร็จรูปครบทุกอุตสาหกรรมตั้งแต่รุ่นแรก

## 1. Design invariant

Dashboard Platform ต้องแยกเป็นหกชั้น:

```text
Data Sources
    ↓
Query Gateway
    ↓
Universal Data Frames
    ↓
Semantic Model + Transform Pipeline
    ↓
Renderer Registry
    ↓
Layouts / Interactions / Outputs
```

ส่วนที่เกี่ยวกับวงการอยู่ใน **Domain Pack** ส่วนความสามารถแสดงผลเฉพาะทางอยู่ใน **Renderer Plugin** และทั้งสองต้องผ่าน version, provenance, permission และ signature policy

## 2. Universal data contract

Renderer ห้ามผูกกับฐานข้อมูลหรือ query language โดยตรง Query Gateway ต้องแปลงผลลัพธ์เป็น frame กลางก่อนส่งให้ renderer

### Primitive field types

- Boolean
- Signed/unsigned integer
- Decimal and floating point
- String and rich text
- Binary reference
- Timestamp, duration and interval
- Date, time and calendar period
- Currency and quantity
- Coordinate, geometry and geospatial feature
- Entity/resource reference
- Enum/state
- JSON/document
- Image, audio and video reference

### Semantic metadata

ทุก field สามารถระบุ:

```text
name
logical_type
semantic_role
unit
currency
locale
timezone
precision
quality
confidence
sensitivity
resource_type
resource_id
parent_resource_id
latitude / longitude / geometry
minimum / maximum / target
state_mapping
value_labels
```

Semantic metadata ทำให้ renderer เดียวกันใช้ได้กับ CPU, อุณหภูมิ, อัตราการเต้นหัวใจ, กำลังผลิต, ยอดขาย หรือการใช้พลังงาน โดยไม่ต้องรู้ชื่อวงการ

### Canonical data shapes

```text
scalar
record
key_value
table
pivot
matrix
time_series
histogram
distribution
state_series
event_stream
log_stream
trace_set
topology
hierarchy
flow
geospatial
route
calendar
schedule
media
document
form
custom
```

Data shape เป็น capability negotiation ระหว่าง query, transform และ renderer ไม่ใช่การรับประกันว่า renderer ทุกตัวอ่านข้อมูลทุกแบบได้

## 3. Renderer families

CherryDash ต้องรองรับ renderer registry ที่เพิ่มชนิดใหม่ได้ โดย built-in catalog มีเป้าหมายอย่างน้อยดังนี้

| Family | Renderer examples |
|---|---|
| KPI and summary | Stat, scorecard, sparkline, progress, target, gauge, bullet, SLA/SLO, health badge |
| Time and trend | Line, area, step, stacked series, horizon, candlestick, state timeline, anomaly band, forecast |
| Comparison | Bar, column, grouped/stacked bar, waterfall, Pareto, funnel, radar, scatter, bubble |
| Distribution | Histogram, heatmap, density, box plot, violin, quantile and correlation matrix |
| Tabular | Table, pivot, matrix, tree grid, virtual data grid, editable approval grid |
| Events | Event timeline, incident timeline, calendar, Gantt, schedule, log viewer and trace waterfall |
| Topology | Node graph, dependency graph, network map, Sankey, hierarchy, org chart and service map |
| Geospatial | Point/cluster map, choropleth, route, geofence, floor plan, indoor map and weather layer |
| Operations | Status grid, NOC/SOC wallboard, alarm banner, maintenance board and shift board |
| Industrial | Process mimic, equipment schematic, production line, digital-twin view and quality-control chart |
| Business | Cohort, retention, conversion, inventory, margin, budget, forecast and portfolio views |
| Scientific | Multi-axis plot, spectrum, waveform, experiment run, uncertainty interval and image annotation |
| Content | Markdown, sanitized rich text, image, document preview, media stream and signed external embed |
| Workflow | Form, annotation, checklist, approval, runbook and guarded action proposal |
| Advanced | Canvas, WebGL/3D, WebGPU, WASM renderer and custom signed renderer |

Arbitrary HTML หรือ JavaScript ห้ามรันใน main application context เนื้อหาที่ต้อง embed ใช้ sandbox และ permission policy เท่านั้น

## 4. Layout and output modes

Dashboard definition เดียวสามารถมี placement หลาย layout:

```text
desktop
tablet
mobile
wallboard
control_room
kiosk
embedded
print
pdf
email_snapshot
accessibility_table
```

ข้อกำหนด:

- Breakpoint และ column grid เป็น declarative
- Panel สามารถจัดตำแหน่งต่างกันต่อ layout
- Renderer ต้องประกาศ minimum size และ responsive behavior
- Print/PDF ใช้ server-side deterministic rendering
- Wallboard/Kiosk ต้องมี burn-in protection, stale-data indicator และ connection status
- Mobile ต้องรองรับ touch, limited bandwidth และ progressive loading
- Accessibility fallback ต้องแสดงข้อมูลสำคัญแบบตารางหรือข้อความได้

## 5. Dashboard definition v1

Canonical model อยู่ที่:

```text
crates/cherrydash-core/src/dashboard.rs
schemas/dashboard/v1/dashboard.schema.json
```

องค์ประกอบหลัก:

```text
DashboardDefinition
├── metadata: key, title, tags, locale, timezone
├── domainProfiles[]
├── dataSources[]
│   ├── adapter
│   ├── non-secret configuration
│   └── secretRefs
├── variables[]
├── layouts[]
├── panels[]
│   ├── renderer
│   ├── queries
│   ├── placements
│   ├── transformations
│   ├── interactions
│   ├── options
│   └── accessibility
└── settings
```

Validation รุ่นแรกตรวจ:

- Schema version
- Identifier format
- Duplicate IDs
- Data-source references
- Layout bounds
- Query and panel limits
- JSON configuration shape
- Inline secret-like fields
- Invalid refresh/concurrency values

ตัวอย่างอยู่ที่ `examples/dashboards/universal-operations.dashboard.json` ตัวอย่างนี้เป็น contract demonstration ไม่ใช่การประกาศว่า renderer ทุกตัวทำงานแล้ว

## 6. Domain Pack architecture

Domain Pack เป็นแพ็กเกจ declarative ที่เพิ่มความหมายทางธุรกิจโดยไม่เพิ่มเงื่อนไขใน Core

```text
Domain Pack
├── manifest
├── semantic models / ontology mappings
├── unit and state dictionaries
├── dashboard definitions
├── monitor templates
├── alert/SLO templates
├── report definitions
├── workflow proposals
├── icons and assets with provenance
├── localization bundles
└── compatibility and validation tests
```

Domain Pack ห้ามมีสิทธิ์ execute native code โดยปริยาย ถ้าต้องการ renderer หรือ adapter ใหม่ต้องอ้างถึง extension ที่ได้รับอนุมัติแยกต่างหาก

### Industry coverage model

ตัวอย่างวงการที่ใช้ pack แยกกันได้:

- Infrastructure, cloud, DevOps, NOC and SRE
- Cybersecurity and SOC
- Telecom and ISP
- Data center and hosting
- Manufacturing, OT and quality control
- Energy, oil and gas, renewable energy and utilities
- Water, wastewater and environmental monitoring
- Transportation, logistics, fleet, aviation, rail and maritime
- Healthcare, hospital operations, laboratory and medical-device operations
- Banking, finance, insurance, payments and risk
- Retail, e-commerce, supply chain and warehouse
- Agriculture, livestock, aquaculture and food production
- Government, public safety and smart city
- Education and research
- Construction, engineering, mining and heavy equipment
- Real estate, facilities, building management and hospitality
- Media, broadcasting, gaming, sports and live events
- Legal, compliance, audit, HR and workforce operations
- Scientific, climate, earth observation and space operations

รายการนี้ไม่ใช่ allowlist วงการใหม่เพิ่ม pack ได้โดยใช้ contract เดียวกัน

## 7. Data-source adapters

Adapter ต้องประกาศ:

```text
adapter_id
api_version
supported_query_types
supported_data_shapes
streaming capability
push/pull behavior
authentication modes
tenant isolation behavior
rate and concurrency limits
schema discovery
health and diagnostics
```

Adapter categories สามารถครอบคลุม native telemetry, SQL, REST, GraphQL, WebSocket, message bus, object storage, files, geospatial services, industrial protocols, healthcare/public standards และ vendor APIs โดย adapter ทุกตัวต้องแปลงผลเป็น universal frame

Secret ห้ามอยู่ใน dashboard definition ให้ใช้ `secretRefs` ที่ resolve ฝั่ง server ตาม tenant และ policy

## 8. Query and transform pipeline

Query engine ต้องรองรับ:

- Parameter binding โดยไม่ต่อ string แบบเสี่ยง injection
- Time-range propagation
- Server-side aggregation and downsampling
- Streaming incremental updates
- Join, filter, group, pivot and window transforms
- Unit conversion and normalization
- Quality/staleness propagation
- Resource identity enrichment
- Query cancellation and timeout
- Per-tenant cost budget
- Result cache with authorization-aware keys
- Deterministic query semanticsสำหรับ alert และ report

Transform pipeline ต้องเป็น versioned และ reproducible Dashboard, alert และ report ที่ใช้ query/transform เดียวกันต้องได้ semantic result เดียวกัน

## 9. Interaction model

Panel สามารถประกาศ event และ action เช่น:

```text
select → dashboard_filter
brush → set_time_range
hover → linked_cursor
click → drilldown
open → resource_details
annotate → create_annotation
propose → automation_proposal
export → governed_export
```

Interaction ที่เปลี่ยนระบบจริงห้าม execute จาก renderer โดยตรง ต้องผ่าน authorization, policy, approval, timeout, verification, rollback และ audit ของ Automation Engine

## 10. Renderer plugin security

Renderer manifest อยู่ที่ `schemas/dashboard/v1/renderer-manifest.schema.json`

Execution modes:

- `built_in`: build และ review พร้อม CherryDash
- `web_worker`: isolated computation ไม่มี DOM โดยตรง
- `wasm`: capability-limited runtime
- `sandboxed_iframe`: isolated UI พร้อม explicit allowlist

ข้อกำหนดทุก extension:

- Versioned API contract
- Signed artifact and integrity hash
- Permission manifest
- No implicit network access
- No direct database credential
- No cross-tenant storage
- CPU, memory and execution-time quota
- Sanitized output
- Provenance and license record
- Compatibility tests
- Disable/uninstall/rollback path

## 11. Global readiness

เพื่อใช้ทั่วโลก Dashboard Platform ต้องรองรับ:

- Locale-aware number/date formatting
- IANA timezone and browser timezone
- Currency and exchange-rate provenance
- SI, IEC, imperial and domain units
- Gregorian and configurable business calendars
- Right-to-left layout
- Translation bundles and fallback locale
- Unicode-safe search and labels
- Color-independent status and high-contrast mode
- Keyboard navigation and screen-reader semantics
- Data sensitivity labels, masking and export policy
- Regional retention and data-residency policy

การรองรับข้อกำหนดทางกฎหมายหรือมาตรฐานของวงการต้องประกาศต่อ pack/version และมีหลักฐานทดสอบ ห้ามใช้คำว่า compliant เพียงเพราะมี dashboard สีเข้ากับโลโก้หน่วยงาน

## 12. Performance and scale

- Query budget is enforced before execution
- Progressive and incremental rendering
- Virtualized tables and event streams
- Server-side aggregation and rollups
- Automatic resolution based on pixel width and time range
- Cardinality and series limits
- Back-pressure for live streams
- Shared query fan-out when authorization context matches
- Panel isolation so one failed renderer does not crash the dashboard
- Stale and partial-result indicators
- Load, soak and browser-memory benchmarks

Dashboard refresh ห้ามสร้าง query storm และห้าม bypass tenant quota

## 13. Lifecycle and governance

Dashboard, renderer และ domain pack ทุกชนิดต้องมี:

```text
stable ID
semantic version
schema version
owner
provenance
license
compatibility range
migration path
rollback path
deprecation policy
signature status
```

Dashboard-as-code รองรับ JSON/YAML representation, API, GitOps, revision history, diff, review, promotion between environments และ immutable published revision

## 14. Delivery phases

### Foundation

- Dashboard definition v1
- Rust validation
- JSON Schema
- Example dashboard
- Architecture and security constraints

### Minimal native runtime

- Renderer registry
- Stat, time-series, table, status-grid and text renderers
- Desktop/mobile/wallboard layouts
- Native query binding
- Dashboard persistence and revision history
- Cross-filter and drilldown

### Extensibility

- Universal frame API
- Transform pipeline
- Renderer SDK and sandbox
- Domain Pack manifest and validation
- Localization and accessibility tests

### Advanced coverage

- Geospatial, topology, process mimic, scientific and 3D renderers
- Server-side export
- Signed pack registry
- Industry conformance suites

## 15. Acceptance criteria

CherryDash จะถือว่า Dashboard Platform รองรับวงการใหม่ได้เมื่อ:

1. เพิ่ม domain pack ใหม่โดยไม่แก้ Core service หรือ database schema หลัก
2. เพิ่ม renderer ใหม่ผ่าน versioned manifest และ sandbox
3. Data source ใหม่ส่ง frame ตาม contract โดยไม่ทำให้ renderer ผูกกับ vendor
4. Dashboard เดียวมี desktop, mobile, wallboard และ print layout
5. Locale, timezone, unit, currency และ accessibility fallback ทำงาน
6. Query quota, tenant isolation, field masking และ export policy ผ่านการทดสอบ
7. Dashboard, alert และ report ใช้ query semantics เดียวกัน
8. Pack และ plugin มี provenance, signature, compatibility และ rollback

สถานะปัจจุบันคือ **schema and validation foundation** ตัว renderer runtime, query engine, pack registry และ industry templates ยังเป็นงานใน Roadmap
