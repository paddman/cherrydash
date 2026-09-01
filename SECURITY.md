# Security Policy

CherryDash is currently **pre-alpha** and must not be exposed to untrusted networks or used as a production security boundary.

## Reporting a vulnerability

Report vulnerabilities privately through GitHub Security Advisories for this repository. Do not open a public issue containing credentials, exploit details, tenant data, or a working proof of concept.

Include, where available:

- affected commit or version
- component and deployment mode
- reproduction steps
- expected and observed behavior
- impact and tenant-boundary implications
- suggested mitigation

## Current security status

The development stack contains bootstrap controls only. In particular:

- environment-based ingest credentials are for development and transition work
- production edge identity requires enrollment, short-lived credentials and mTLS
- PostgreSQL row-level security is not implemented yet
- the central WAL is not segmented and replay is not implemented yet
- the web/API authorization model is not implemented yet
- infrastructure containers are not a production deployment profile

A successful CI run does not mean the release is production-safe. Production eligibility is governed by [`docs/PRODUCTION_READINESS.md`](docs/PRODUCTION_READINESS.md).

## Security principles

1. Tenant identity comes from authenticated credentials, never a caller-selected tenant field.
2. No secret is stored in dashboard, monitor or template definitions.
3. Every privileged action has authorization, policy, audit evidence and a rollback path.
4. AI may propose an action but cannot bypass approval or policy.
5. Imported documents are treated as untrusted data and are never executed directly.
6. Dependencies, images and release artifacts require provenance and integrity checks.
