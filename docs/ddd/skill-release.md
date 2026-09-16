# Skill release bounded contexts

## Bounded context

SkillFence separates skill intake, deterministic gating, similarity signalling, external evidence collection, and human release authority. The contexts collaborate to build one immutable `Assessment`; none can install or execute a skill, and only a human can make the release decision.

This artifact is the context map. Detailed invariants live in the [comparison evidence contract](comparison-evidence-contract.md), [external evaluator receipt contract](external-evaluator-receipt-contract.md), and [architecture evidence contract](architecture-evidence-contract.md).

| Context | Owns | Does not own |
|---|---|---|
| Skill intake | Reading a skill directory and stable content identity | Installing or executing a skill |
| Deterministic gate | Structural and policy findings | Model-based judgment or promotion |
| Similarity signal | Stable local fingerprint comparison | Claiming semantic equivalence |
| External evidence | Running explicitly requested RuVector/MetaHarness probes and recording receipts | Masking unavailable tools or granting release authority |
| Human release | Reviewable verdict and evidence pack | Automatic deployment, merge, or publication |
| Architecture evidence | Validating ADR/DDD schema, links, traceability, and explicit completion counts | Inventing decisions or judging prose quality |

## Ubiquitous language

- **Assessment:** aggregate containing the content identity, deterministic findings, similarity values, and external receipts for one primary skill.
- **Deterministic verdict:** `BLOCKED` or `READY_FOR_HUMAN_REVIEW`, derived only from local findings.
- **External receipt:** an explicit `Passed`, `Failed`, `Unavailable`, or `NotRequested` observation from an optional adapter.
- **Human release:** the approval boundary outside SkillFence; no positive tool result crosses it automatically.

## Invariants

1. A blocking deterministic finding always yields `BLOCKED`.
2. External receipts cannot upgrade a deterministic verdict or grant release authority.
3. A successful assessment identifies the exact primary content by SHA-256.
4. Intake, assessment, and optional adapter execution do not install or execute the inspected skill.
5. Every explicitly requested comparison is present in the assessment or the assessment fails before serialization.
6. Every requested external evaluator produces exactly one receipt that distinguishes a completed process from a process that could not start.

## Traceability

- Local-first decision: [`ADR-001`](../adr/ADR-001-local-first-evidence-gate.md)
- Fail-closed comparison decision: [`ADR-002`](../adr/ADR-002-fail-closed-comparison-intake.md)
- Machine-checked architecture decision: [`ADR-003`](../adr/ADR-003-machine-checked-architecture-evidence.md)
- External process receipt decision: [`ADR-004`](../adr/ADR-004-external-evaluator-process-receipts.md)
- Assessment aggregate and deterministic values: [`src/lib.rs`](../../src/lib.rs)
- External receipt values and process adapter: [`src/external.rs`](../../src/external.rs)
- Application boundary: [`src/main.rs`](../../src/main.rs)
- Real CLI boundary test: [`tests/cli_comparison_boundary.rs`](../../tests/cli_comparison_boundary.rs)
- Real external executable fixtures: [`tests/cli_external_receipts.rs`](../../tests/cli_external_receipts.rs)
