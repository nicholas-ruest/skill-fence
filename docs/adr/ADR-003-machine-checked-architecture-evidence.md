# ADR-003: Machine-check architecture evidence and completion separately

Status: Accepted

## Context

SkillFence's Product Foundry completion contract requires substantive ADRs and DDD artifacts that trace to implemented code and exercised tests. Counts alone can be padded, prose review can miss broken links, and a document can claim verification that no longer exists. At the same time, the active draft must retain passing CI while it advances through multiple bounded slices; enforcing the final 24-ADR and 12-DDD count floors in ordinary CI would make every intermediate head indistinguishable from a code regression.

## Decision

The repository owns a Rust documentation validator behind a `DocumentationSource` port. Its default schema mode discovers every Markdown file under `docs/adr` and `docs/ddd`, then fails closed on missing required sections, non-Accepted ADRs, broken local links, or absent decision/domain/implementation/test traceability. CI runs this mode on every pull-request head.

An explicit `--completion` mode applies the same schema checks and additionally enforces the Product Foundry floors of 24 Accepted ADRs and 12 DDD artifacts. It reports exact current counts when either floor is unmet. Completion mode is evidence for the product status; it is not used to disguise an otherwise verified intermediate slice as a CI failure.

## Alternatives Considered

- **Validate document counts only.** Rejected because filenames do not prove that a decision is substantive or implemented.
- **Rely on manual architecture review.** Rejected because section omissions and broken relative links are deterministic defects that should be caught on every change.
- **Run final count floors in ordinary CI.** Rejected because the planned multi-slice build would remain red for known scope rather than for a regression at the exact head.
- **Use a third-party Markdown linter.** Rejected for this bounded slice because generic syntax checks do not understand SkillFence's ADR/DDD traceability contract.

## Consequences

- Every existing and future architecture artifact is checked through the same discover-and-validate path.
- The schema can evolve only with code and tests, making architecture evidence part of the maintained product surface.
- Default CI proves artifact quality but not product completion; release evidence must also show the explicit completion-mode result.
- The validator checks structural substance and traceability edges, while human review remains responsible for the quality of the actual decision and model.

## Verification

- [`tests/architecture_docs.rs`](../../tests/architecture_docs.rs) invokes the compiled validator against the real repository and verifies both schema success and exact fail-closed completion counts.
- [`src/documentation.rs`](../../src/documentation.rs) contains the owned-source collaboration test and deterministic validation rules.
- `cargo test --test architecture_docs`
- `cargo test requests_every_architecture_document_through_the_owned_source`
- `cargo run --bin skill-fence-docs -- --root .`
- `cargo run --bin skill-fence-docs -- --root . --completion`

## Implementation and Domain Links

- Validator policy and filesystem adapter: [`src/documentation.rs`](../../src/documentation.rs)
- CLI boundary: [`src/bin/skill-fence-docs.rs`](../../src/bin/skill-fence-docs.rs)
- CI wiring: [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)
- Domain contract: [`docs/ddd/architecture-evidence-contract.md`](../ddd/architecture-evidence-contract.md)
