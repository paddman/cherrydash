## Problem and scope

<!-- What does this change solve? What is intentionally outside scope? -->

## Design

<!-- Describe data flow, state, failure behavior and tenant boundaries. -->

## Clean-room and dependency provenance

- [ ] No source, UI, schema, template, documentation or asset was copied or mechanically translated from another product
- [ ] New dependencies/protocol definitions are listed with origin and license
- [ ] Imported external data remains untrusted and is not executed directly

## Security and privacy

- [ ] Tenant identity and authorization were reviewed
- [ ] Secrets and personal/customer data are not committed or logged
- [ ] Abuse cases, quotas and denial-of-service effects were considered
- [ ] Privileged actions include audit, verification and rollback

## Verification

- [ ] Rust formatting
- [ ] Clippy with warnings denied
- [ ] Rust tests
- [ ] Web production build
- [ ] Compose validation
- [ ] Restart/failure test where state or delivery semantics changed

## Migration and rollback

<!-- State compatibility, rollout order, rollback command/path and data implications. -->
