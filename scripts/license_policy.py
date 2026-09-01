#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-only
"""Apply and verify CherryDash path-based licensing metadata."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[1]
AGPL = "AGPL-3.0-only"
APACHE = "Apache-2.0"
APACHE_ROOTS = {"agents", "crates", "proto", "schemas"}
LICENSE_PATHS = {
    Path("LICENSE"): AGPL,
    Path("agents/LICENSE"): APACHE,
    Path("crates/LICENSE"): APACHE,
    Path("proto/LICENSE"): APACHE,
    Path("schemas/LICENSE"): APACHE,
}

SLASH_EXTENSIONS = {
    ".c",
    ".cc",
    ".cpp",
    ".cs",
    ".go",
    ".h",
    ".hpp",
    ".java",
    ".js",
    ".jsx",
    ".kt",
    ".kts",
    ".mjs",
    ".cjs",
    ".proto",
    ".rs",
    ".swift",
    ".ts",
    ".tsx",
}
HASH_EXTENSIONS = {
    ".bash",
    ".cfg",
    ".conf",
    ".env",
    ".ini",
    ".properties",
    ".py",
    ".sh",
    ".toml",
    ".yaml",
    ".yml",
    ".zsh",
}
BLOCK_EXTENSIONS = {".css", ".less", ".scss"}
DASH_EXTENSIONS = {".sql"}
HTML_EXTENSIONS = {".htm", ".html", ".svg", ".xml"}
JSON_EXTENSIONS = {".json"}
SPECIAL_HASH_NAMES = {
    ".editorconfig",
    ".env.example",
    ".gitignore",
    "Dockerfile",
    "Makefile",
}
SKIP_NAMES = {"CLA.md", "LICENSE", "NOTICE"}
SKIP_SUFFIXES = {".license"}
README_MARKER = "## Licensing"
CONTRIBUTING_MARKER = "## Contributor License Agreement"
PR_MARKER = "## Licensing and CLA"


class PolicyError(RuntimeError):
    """Raised when the repository does not satisfy the licensing policy."""


def tracked_paths() -> list[Path]:
    output = subprocess.check_output(
        ["git", "ls-files", "-z"], cwd=ROOT
    ).decode("utf-8")
    return [Path(item) for item in output.split("\0") if item]


def expected_license(path: Path) -> str:
    return APACHE if path.parts and path.parts[0] in APACHE_ROOTS else AGPL


def comment_header(path: Path, license_id: str) -> str | None:
    name = path.name
    suffix = path.suffix.lower()

    if name in SKIP_NAMES or suffix in SKIP_SUFFIXES:
        return None
    if name in SPECIAL_HASH_NAMES or name.endswith(".Dockerfile"):
        return f"# SPDX-License-Identifier: {license_id}"
    if suffix in SLASH_EXTENSIONS:
        return f"// SPDX-License-Identifier: {license_id}"
    if suffix in HASH_EXTENSIONS:
        return f"# SPDX-License-Identifier: {license_id}"
    if suffix in BLOCK_EXTENSIONS:
        return f"/* SPDX-License-Identifier: {license_id} */"
    if suffix in DASH_EXTENSIONS:
        return f"-- SPDX-License-Identifier: {license_id}"
    if suffix in HTML_EXTENSIONS:
        return f"<!-- SPDX-License-Identifier: {license_id} -->"
    return None


def is_json_source(path: Path) -> bool:
    return path.suffix.lower() in JSON_EXTENSIONS and not path.name.endswith(
        ".license"
    )


def insert_or_replace_header(text: str, header: str) -> str:
    lines = text.splitlines(keepends=True)
    header_pattern = re.compile(r"SPDX-License-Identifier:\s*[^\s*<]+")

    for index, line in enumerate(lines[:8]):
        if header_pattern.search(line):
            newline = "\n" if line.endswith("\n") else ""
            if line.lstrip().startswith(("/*", "<!--")):
                replacement = f"{header}{newline}"
            else:
                prefix = line[: len(line) - len(line.lstrip())]
                replacement = f"{prefix}{header}{newline}"
            if lines[index] == replacement:
                return text
            lines[index] = replacement
            return "".join(lines)

    insertion_index = 0
    if lines and lines[0].startswith("#!/"):
        insertion_index = 1
    elif lines and lines[0].lstrip().startswith("<?xml"):
        insertion_index = 1

    lines.insert(insertion_index, f"{header}\n")
    return "".join(lines)


def write_if_changed(path: Path, content: str) -> bool:
    absolute = ROOT / path
    previous = absolute.read_text(encoding="utf-8") if absolute.exists() else None
    if previous == content:
        return False
    absolute.parent.mkdir(parents=True, exist_ok=True)
    absolute.write_text(content, encoding="utf-8")
    return True


def apply_source_headers(paths: Iterable[Path]) -> int:
    changed = 0
    for path in paths:
        header = comment_header(path, expected_license(path))
        if header is None:
            continue
        absolute = ROOT / path
        try:
            text = absolute.read_text(encoding="utf-8")
        except UnicodeDecodeError as error:
            raise PolicyError(f"source file is not UTF-8: {path}") from error
        updated = insert_or_replace_header(text, header)
        changed += int(write_if_changed(path, updated))
    return changed


def sidecar_content(license_id: str) -> str:
    return (
        "SPDX-FileCopyrightText: 2026 CherryDash contributors\n"
        f"SPDX-License-Identifier: {license_id}\n"
    )


def apply_json_sidecars(paths: Iterable[Path]) -> int:
    changed = 0
    for path in paths:
        if not is_json_source(path):
            continue
        sidecar = Path(f"{path}.license")
        changed += int(
            write_if_changed(sidecar, sidecar_content(expected_license(path)))
        )
    return changed


def replace_or_add_package_license(path: Path, license_value: str) -> bool:
    text = (ROOT / path).read_text(encoding="utf-8")
    section_match = re.search(
        r"(?ms)^\[package\]\n(?P<body>.*?)(?=^\[|\Z)", text
    )
    if not section_match:
        raise PolicyError(f"missing [package] section: {path}")

    body = section_match.group("body")
    key = "license.workspace" if license_value == "workspace" else "license"
    value = "true" if license_value == "workspace" else f'"{license_value}"'
    replacement_line = f"{key} = {value}"

    body = re.sub(r"(?m)^license(?:\.workspace)?\s*=.*\n?", "", body)
    body_lines = body.splitlines()
    insert_at = 1 if body_lines else 0
    for index, line in enumerate(body_lines):
        if line.startswith("version") or line.startswith("version.workspace"):
            insert_at = index + 1
            break
    body_lines.insert(insert_at, replacement_line)
    new_body = "\n".join(body_lines).rstrip() + "\n\n"
    updated = text[: section_match.start("body")] + new_body + text[section_match.end("body") :]
    return write_if_changed(path, updated)


def apply_cargo_metadata(paths: Iterable[Path]) -> int:
    changed = 0
    root_path = Path("Cargo.toml")
    root_text = (ROOT / root_path).read_text(encoding="utf-8")
    section_match = re.search(
        r"(?ms)^\[workspace\.package\]\n(?P<body>.*?)(?=^\[|\Z)", root_text
    )
    if not section_match:
        raise PolicyError("missing [workspace.package] section in Cargo.toml")
    body = section_match.group("body")
    body = re.sub(r"(?m)^license\s*=.*\n?", "", body)
    body_lines = body.splitlines()
    insert_at = 1 if body_lines else 0
    for index, line in enumerate(body_lines):
        if line.startswith("version"):
            insert_at = index + 1
            break
    body_lines.insert(insert_at, f'license = "{AGPL}"')
    new_body = "\n".join(body_lines).rstrip() + "\n\n"
    updated = (
        root_text[: section_match.start("body")]
        + new_body
        + root_text[section_match.end("body") :]
    )
    changed += int(write_if_changed(root_path, updated))

    for path in paths:
        if path.name != "Cargo.toml" or path == root_path:
            continue
        if path.parts[0] in {"agents", "crates"}:
            changed += int(replace_or_add_package_license(path, APACHE))
        elif path.parts[0] == "services":
            changed += int(replace_or_add_package_license(path, "workspace"))
    return changed


def apply_package_json() -> int:
    path = Path("web/package.json")
    if not (ROOT / path).exists():
        return 0
    data = json.loads((ROOT / path).read_text(encoding="utf-8"))
    output: dict[str, object] = {}
    inserted = False
    for key, value in data.items():
        output[key] = value
        if key == "version":
            output["license"] = AGPL
            inserted = True
    if not inserted:
        output["license"] = AGPL
    return int(write_if_changed(path, json.dumps(output, indent=2) + "\n"))


def replace_markdown_section(text: str, heading: str, body: str) -> str:
    pattern = re.compile(rf"(?ms)^{re.escape(heading)}\n.*?(?=^##\s|\Z)")
    replacement = f"{heading}\n\n{body.strip()}\n"
    if pattern.search(text):
        return pattern.sub(replacement, text).rstrip() + "\n"
    return text.rstrip() + "\n\n" + replacement


def apply_readme() -> int:
    path = Path("README.md")
    text = (ROOT / path).read_text(encoding="utf-8")
    body = """
CherryDash ใช้ **path-based licensing** โดยไฟล์ `LICENSE` ที่ใกล้ไฟล์งานที่สุดเป็นตัวกำหนดสิทธิ์ของไฟล์นั้น

| ขอบเขต | SPDX identifier | License file |
|---|---|---|
| Root และทุก path ที่ไม่มี `LICENSE` เฉพาะทาง เช่น `services/`, `web/`, `deploy/`, `docs/`, `examples/` | `AGPL-3.0-only` | [`LICENSE`](LICENSE) |
| `agents/` | `Apache-2.0` | [`agents/LICENSE`](agents/LICENSE) |
| `crates/` | `Apache-2.0` | [`crates/LICENSE`](crates/LICENSE) |
| `proto/` | `Apache-2.0` | [`proto/LICENSE`](proto/LICENSE) |
| `schemas/` | `Apache-2.0` | [`schemas/LICENSE`](schemas/LICENSE) |

ไฟล์ source และ configuration ที่รองรับ comment มี SPDX header ตาม license ของ path นั้น ส่วน JSON ซึ่งไม่รองรับ comment ใช้ไฟล์ sidecar `.license` เพื่อไม่ทำลาย syntax หรือ schema validation

การส่ง Contribution เข้ามาในโครงการต้องยอมรับ [`CLA.md`](CLA.md) และผ่านสถานะของ CLA Assistant ก่อน merge ดูขั้นตอนสำหรับผู้ดูแลที่ [`docs/CLA_ASSISTANT.md`](docs/CLA_ASSISTANT.md)
"""
    updated = replace_markdown_section(text, README_MARKER, body)
    return int(write_if_changed(path, updated))


def append_markdown_section(path: Path, marker: str, body: str) -> bool:
    text = (ROOT / path).read_text(encoding="utf-8")
    if marker in text:
        updated = replace_markdown_section(text, marker, body)
    else:
        updated = text.rstrip() + f"\n\n{marker}\n\n{body.strip()}\n"
    return write_if_changed(path, updated)


def apply_contribution_docs() -> int:
    changed = 0
    changed += int(
        append_markdown_section(
            Path("CONTRIBUTING.md"),
            CONTRIBUTING_MARKER,
            """
ทุก Contribution ที่มีสาระสำคัญต้องลงนาม [`CLA.md`](CLA.md) ผ่าน CLA Assistant ก่อน merge ผู้ลงนามรับรองสิทธิ์ในผลงาน แหล่งที่มาของ third-party material และความถูกต้องของข้อมูลเกี่ยวกับ AI-assisted contribution

License ของ Contribution พิจารณาจาก path ปลายทาง:

- `agents/`, `crates/`, `proto/`, `schemas/`: `Apache-2.0`
- path อื่นทั้งหมดที่ไม่มี `LICENSE` ใกล้กว่า: `AGPL-3.0-only`

ห้ามลบหรือเปลี่ยน SPDX identifier โดยไม่มีการอนุมัติจากผู้ดูแล และห้ามย้ายไฟล์ข้าม license boundary โดยไม่ตรวจ license compatibility
""",
        )
    )
    changed += int(
        append_markdown_section(
            Path(".github/PULL_REQUEST_TEMPLATE.md"),
            PR_MARKER,
            """
- [ ] ฉันได้อ่านและลงนาม `CLA.md` เมื่อ CLA Assistant ร้องขอ
- [ ] ไฟล์ใหม่และไฟล์ที่ย้ายมี SPDX identifier ตรงกับ license boundary
- [ ] ไม่มี source, asset หรือ schema จากบุคคลที่สามที่ขาด provenance/license record
""",
        )
    )
    return changed


def apply_policy() -> int:
    paths = tracked_paths()
    changed = 0
    changed += apply_source_headers(paths)
    changed += apply_json_sidecars(paths)
    changed += apply_cargo_metadata(paths)
    changed += apply_package_json()
    changed += apply_readme()
    changed += apply_contribution_docs()
    return changed


def verify_license_files() -> list[str]:
    errors: list[str] = []
    apache_content: str | None = None
    for path, license_id in LICENSE_PATHS.items():
        absolute = ROOT / path
        if not absolute.is_file():
            errors.append(f"missing license file: {path}")
            continue
        text = absolute.read_text(encoding="utf-8")
        if license_id == AGPL:
            if "GNU AFFERO GENERAL PUBLIC LICENSE" not in text or "Version 3" not in text:
                errors.append(f"{path} is not the AGPL v3 license text")
        else:
            if "Apache License" not in text or "Version 2.0" not in text:
                errors.append(f"{path} is not the Apache 2.0 license text")
            if apache_content is None:
                apache_content = text
            elif text != apache_content:
                errors.append(f"{path} differs from the other Apache license copies")
    return errors


def verify_headers(paths: Iterable[Path]) -> list[str]:
    errors: list[str] = []
    for path in paths:
        header = comment_header(path, expected_license(path))
        if header is None:
            continue
        text = (ROOT / path).read_text(encoding="utf-8")
        first_lines = "".join(text.splitlines(keepends=True)[:8])
        if header not in first_lines:
            errors.append(f"missing or incorrect SPDX header: {path} (expected {header})")
    return errors


def verify_json_sidecars(paths: Iterable[Path]) -> list[str]:
    errors: list[str] = []
    for path in paths:
        if not is_json_source(path):
            continue
        sidecar = ROOT / f"{path}.license"
        expected = sidecar_content(expected_license(path))
        if not sidecar.is_file():
            errors.append(f"missing SPDX sidecar for JSON: {path}.license")
        elif sidecar.read_text(encoding="utf-8") != expected:
            errors.append(f"incorrect SPDX sidecar: {path}.license")
    return errors


def verify_metadata(paths: Iterable[Path]) -> list[str]:
    errors: list[str] = []
    root_cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    if f'license = "{AGPL}"' not in root_cargo:
        errors.append("Cargo workspace license is not AGPL-3.0-only")

    for path in paths:
        if path.name != "Cargo.toml" or path == Path("Cargo.toml"):
            continue
        text = (ROOT / path).read_text(encoding="utf-8")
        if path.parts[0] in {"agents", "crates"}:
            expected = f'license = "{APACHE}"'
        elif path.parts[0] == "services":
            expected = "license.workspace = true"
        else:
            continue
        if expected not in text:
            errors.append(f"incorrect Cargo license metadata: {path}")

    package_json = json.loads((ROOT / "web/package.json").read_text(encoding="utf-8"))
    if package_json.get("license") != AGPL:
        errors.append("web/package.json license is not AGPL-3.0-only")
    return errors


def verify_documents() -> list[str]:
    errors: list[str] = []
    required = [Path("CLA.md"), Path("docs/CLA_ASSISTANT.md")]
    for path in required:
        if not (ROOT / path).is_file():
            errors.append(f"missing required document: {path}")
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    for marker in (README_MARKER, AGPL, APACHE, "CLA.md"):
        if marker not in readme:
            errors.append(f"README licensing section is missing {marker!r}")
    return errors


def verify_policy() -> list[str]:
    paths = tracked_paths()
    errors: list[str] = []
    errors.extend(verify_license_files())
    errors.extend(verify_headers(paths))
    errors.extend(verify_json_sidecars(paths))
    errors.extend(verify_metadata(paths))
    errors.extend(verify_documents())
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true", help="apply the policy")
    mode.add_argument("--check", action="store_true", help="verify the policy")
    args = parser.parse_args()

    if args.write:
        changed = apply_policy()
        print(f"licensing policy applied; {changed} files changed")

    errors = verify_policy()
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    print("licensing policy verified")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
