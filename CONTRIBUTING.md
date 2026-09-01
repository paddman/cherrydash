# Contributing to CherryDash

CherryDash is an original, clean-room implementation. Contributions are accepted only when their origin, license and behavior can be reviewed.

## Contributor attestation

By submitting a contribution, you attest that:

- you wrote the contribution or have the right to submit it under the project license
- it was not copied, translated, transcribed or mechanically derived from another monitoring/dashboard product
- it does not contain third-party source, UI assets, schemas, templates or documentation without an approved license record
- any new dependency is declared and its license is compatible with the project policy
- test fixtures do not contain customer credentials, personal data or confidential telemetry

See [`docs/NO_FORK_POLICY.md`](docs/NO_FORK_POLICY.md).

## Engineering expectations

- keep performance-sensitive and edge paths in Rust unless an ADR approves otherwise
- preserve tenant context through every storage and messaging boundary
- define delivery, retry and failure semantics explicitly
- add tests for normal, error, restart and boundary behavior
- avoid hidden fallbacks and silent conversion loss
- do not claim performance or scale without a repeatable benchmark artifact
- do not introduce risky automation without approval, verification and rollback design

## Before opening a pull request

Run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cd web && npm install --no-audit --no-fund && npm run build
cd .. && docker compose -f deploy/compose/docker-compose.yml config --quiet
```

The pull request must state:

- problem and scope
- architecture/security effect
- migration or compatibility effect
- tests performed
- rollback path
- provenance of any dependency, protocol definition, fixture or asset

## Contributor License Agreement

ทุก Contribution ที่มีสาระสำคัญต้องลงนาม [`CLA.md`](CLA.md) ผ่าน CLA Assistant ก่อน merge ผู้ลงนามรับรองสิทธิ์ในผลงาน แหล่งที่มาของ third-party material และความถูกต้องของข้อมูลเกี่ยวกับ AI-assisted contribution

License ของ Contribution พิจารณาจาก path ปลายทาง:

- `agents/`, `crates/`, `proto/`, `schemas/`: `Apache-2.0`
- path อื่นทั้งหมดที่ไม่มี `LICENSE` ใกล้กว่า: `AGPL-3.0-only`

ห้ามลบหรือเปลี่ยน SPDX identifier โดยไม่มีการอนุมัติจากผู้ดูแล และห้ามย้ายไฟล์ข้าม license boundary โดยไม่ตรวจ license compatibility
