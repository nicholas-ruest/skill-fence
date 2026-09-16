# ADR-001: Local-first evidence gate before optional accelerators

Status: Accepted

## Context

Teams need a usable decision before model credentials, Docker, vector backends, or hosted evaluators are available. Treating missing accelerators as a pass creates false confidence; treating them as a reason to produce nothing makes release review impossible.

## Decision

SkillFence implements deterministic structure and policy checks in Rust first. RuVector and MetaHarness are process adapters that return `Passed`, `Failed`, or `Unavailable` receipts. Their result never changes a deterministic `BLOCKED` finding to a pass. The only positive verdict is `READY_FOR_HUMAN_REVIEW`.

## Alternatives Considered

- **Require hosted or model-backed evaluation for every run.** Rejected because credentials, network access, and evaluator runtimes are not universal prerequisites for inspecting a local skill.
- **Treat an unavailable optional evaluator as successful.** Rejected because absence is not evidence and would grant false confidence.
- **Automatically approve release after deterministic checks pass.** Rejected because deterministic rules are bounded signals; a human retains release authority.

## Consequences

Every run emits useful, reproducible evidence offline. Semantic/live quality is deliberately incomplete without the optional tools; the report says so. No command installs a skill, contacts a model, changes a repository, or releases software.

## Verification

- [`src/lib.rs`](../../src/lib.rs) contains the deterministic assessment; its unit tests cover bounded and blocked skills.
- [`src/external.rs`](../../src/external.rs) contains the external-receipt values, owned process port, and production adapter.
- [`tests/cli_comparison_boundary.rs`](../../tests/cli_comparison_boundary.rs) exercises a real CLI failure without allowing an incomplete evidence pack.
- [`tests/cli_external_receipts.rs`](../../tests/cli_external_receipts.rs) exercises successful, rejected, and unavailable external processes through the real adapter.
- `cargo test`
- `cargo run -- --skill examples/release-ready-skill --out evidence.json`

## Implementation and Domain Links

- Deterministic aggregate implementation: [`src/lib.rs`](../../src/lib.rs)
- External adapter boundary: [`src/external.rs`](../../src/external.rs)
- CLI application boundary: [`src/main.rs`](../../src/main.rs)
- Domain context map: [`docs/ddd/skill-release.md`](../ddd/skill-release.md)
