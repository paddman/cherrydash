# CLA Assistant Operations

CherryDash requires contributors to accept [`CLA.md`](../CLA.md) before a non-trivial pull request is merged.

## Repository-side configuration

The repository contains:

- `CLA.md`, the authoritative agreement text;
- a CLA checkbox in the pull-request template;
- licensing and provenance requirements in `CONTRIBUTING.md`; and
- the `License policy` workflow for path licenses and SPDX metadata.

## Hosted CLA Assistant setup

The hosted CLA Assistant is a GitHub App. Activating it requires account-level authorization and cannot be completed by committing a YAML file to this repository.

The repository owner must:

1. Review `CLA.md` with qualified counsel and confirm the Project Steward identity.
2. Create a GitHub Gist containing an exact copy of `CLA.md`.
3. Install the [CLA Assistant GitHub App](https://github.com/apps/cla-assistant) for `paddman/cherrydash`.
4. Sign in to [CLA Assistant](https://cla-assistant.io/) and link `paddman/cherrydash` to the CLA Gist.
5. Open a test pull request from an account that has not signed the CLA and verify that the app comments and reports a failing check.
6. Add the resulting CLA status context to the required checks for `main` branch protection.
7. Re-run the test after signing and verify that the status becomes successful.

## Change control

- `CLA.md` is versioned in Git and must be changed through a reviewed pull request.
- The Gist must exactly match the accepted repository version.
- When the agreement changes materially, publish a new CLA version and require contributors to accept it again.
- Export and retain signature records according to the project's retention and privacy policy.
- Do not place personal signature data in the public repository.

## Bot and automation accounts

Bots must be explicitly allowlisted only when their generated contributions are covered by the account owner's agreement or another documented legal basis. A name ending in `[bot]` is not, by itself, a license grant. Humanity has automated many things; legal authority remains stubbornly manual.

## Branch protection target

After the first successful test exposes the exact check name, configure `main` to require at least:

- the existing CherryDash CI checks;
- `License policy`; and
- the CLA Assistant status check.

Do not guess the CLA status-context name in repository configuration. Use the exact context emitted by the installed GitHub App.
