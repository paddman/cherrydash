# CherryDash No-Fork and Clean-Room Policy

## Decision

CherryDash will not fork, copy, embed, rebrand or mechanically translate another monitoring, observability, dashboard or automation product.

This rule applies to core services, agents, UI, schemas, expression engines, templates, assets, documentation, tests and deployment artifacts.

## Forbidden

- importing another project's Git history or source tree
- copying or mechanically translating source code between programming languages
- tracing or recreating a proprietary UI screen pixel-for-pixel
- copying internal database schemas, private APIs or expression implementations
- embedding another product's UI through iframe as the CherryDash product experience
- shipping another server binary as an undeclared CherryDash core component
- replacing names/logos while retaining another product's implementation
- executing imported rules, expressions, templates or scripts without conversion and validation
- using customer exports as a shortcut to reconstruct protected implementation details

## Allowed with review

- implementing public, documented protocols and open standards from their normative specifications
- using normal third-party libraries as dependencies under compatible licenses
- allowing external collectors/tools to forward data through a public CherryDash protocol
- writing isolated import/export adapters that convert documents into native CherryDash objects
- comparing product capabilities in planning or marketing material
- studying publicly documented behavior to define user requirements, without copying implementation

An external component may be optional interoperability infrastructure, but CherryDash must remain functional without embedding or disguising that component as native code.

## Native ownership boundary

CherryDash owns and versions its own:

- resource identity model
- telemetry envelope and typed signal schemas
- collection assignment model
- dashboard document model
- query contract
- alert state machine
- incident evidence model
- policy and automation lifecycle
- RBAC and audit semantics
- storage migrations and lifecycle

Compatibility occurs at protocol/document boundaries. Imported data is converted into these native objects and accompanied by a conversion report. Unsupported or lossy fields must be explicit.

## Provenance controls

Every third-party dependency or specification must record:

- canonical source
- version or commit
- license
- purpose
- whether it is linked, executed, bundled or merely interoperated with
- update and removal owner

Before the first release, the repository must contain:

- project license
- dependency license policy
- `THIRD_PARTY_NOTICES`
- SBOM generation
- source/asset provenance review
- contributor attestation process

## Review gate

A pull request is blocked when provenance is unclear. Convenience is not an exception. Software history contains enough accidental license disasters without CherryDash volunteering for another one.
