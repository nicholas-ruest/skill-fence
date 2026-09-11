# ADR-001: Local-first evidence gate before optional accelerators

Status: Accepted

## Context

Teams need a usable decision before model credentials, Docker, vector backends, or hosted evaluators are available. Treating missing accelerators as a pass creates false confidence; treating them as a reason to produce nothing makes release review impossible.

## Decision

SkillFence implements deterministic structure and policy checks in Rust first. RuVector and MetaHarness are process adapters that return `Passed`, `Failed`, or `Unavailable` receipts. Their result never changes a deterministic `BLOCKED` finding to a pass. The only positive verdict is `READY_FOR_HUMAN_REVIEW`.

## Consequences

Every run emits useful, reproducible evidence offline. Semantic/live quality is deliberately incomplete without the optional tools; the report says so. No command installs a skill, contacts a model, changes a repository, or releases software.
