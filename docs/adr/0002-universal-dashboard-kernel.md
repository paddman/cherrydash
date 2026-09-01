# ADR-0002: Universal dashboard kernel and domain packs

- Status: Accepted for architecture foundation
- Date: 2026-09-01

## Context

CherryDash ต้องรองรับ dashboard สำหรับหลายอุตสาหกรรมและหลายรูปแบบ เช่น infrastructure, security, telecom, manufacturing, energy, healthcare, finance, logistics, government, scientific visualization และวงการที่ยังไม่ได้กำหนดล่วงหน้า

การเพิ่มเงื่อนไขเฉพาะวงการลงใน Core จะทำให้ data model, UI, query และ release lifecycle ผูกกันแน่น ขยายยาก และทำให้วงการใหม่ต้องรอการแก้ผลิตภัณฑ์หลัก

## Decision

1. ใช้ dashboard definition แบบ versioned และ declarative
2. Query Gateway แปลงข้อมูลทุกแหล่งเป็น universal data frame พร้อม semantic metadata
3. Panel อ้าง renderer ด้วย stable versioned identifier ไม่อ้าง React component หรือ database โดยตรง
4. ใช้ renderer registry และ capability negotiation ตาม data shape
5. Layout แยกตาม desktop, tablet, mobile, wallboard, kiosk, embed และ print
6. ความหมายเฉพาะวงการอยู่ใน signed declarative Domain Pack
7. Renderer/adapter เฉพาะทางเป็น extension แยกพร้อม manifest, permission, sandbox, provenance และ rollback
8. Secret อยู่ใน server-side secret reference เท่านั้น ห้ามฝังใน dashboard definition
9. Dashboard, alert และ report ใช้ query/transform semantics เดียวกัน
10. วงการใหม่ต้องเพิ่มได้โดยไม่ fork และไม่แก้ Core

## Consequences

### Positive

- Core เป็นกลางต่ออุตสาหกรรม
- Renderer และ data source พัฒนาแยกกันได้
- Dashboard-as-code, revision, migration และ GitOps ทำได้สม่ำเสมอ
- รองรับ output และอุปกรณ์หลายรูปแบบจาก definition เดียว
- Security และ resource governance บังคับที่ query/extension boundary
- Domain Pack สามารถมี lifecycle และ conformance test ของตัวเอง

### Negative

- Universal frame และ semantic layer ต้องออกแบบอย่างระมัดระวัง
- Renderer SDK และ sandbox เป็นงานระบบขนาดใหญ่
- Domain Pack ต้องมี governance, provenance และ compatibility test
- บาง visualization เฉพาะทางอาจต้องสร้าง extension ใหม่
- คำว่า universal ไม่ลดภาระการทำ template และ validation รายวงการ

## Rejected alternatives

- **Hard-code dashboard ต่อวงการใน Core:** ทำ demo ได้เร็วแต่ขยายและทดสอบไม่ได้ในระยะยาว
- **One renderer accepts arbitrary JSON:** ยืดหยุ่นบนกระดาษแต่ไม่มี type safety, accessibility หรือ performance contract
- **Allow arbitrary JavaScript plugins in the main UI:** เพิ่มความสามารถเร็วแต่สร้างความเสี่ยง supply-chain, tenant isolation และ stability
- **Separate dashboard product per industry:** ทำให้ object model, RBAC, alert และ release lifecycle แตกออกหลายชุด
- **Copy dashboard implementation from another product:** ขัดกับ clean-room/no-fork policy และทำให้สถาปัตยกรรมถูกกำหนดโดยระบบภายนอก
