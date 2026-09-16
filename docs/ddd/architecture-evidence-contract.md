# Architecture evidence domain contract

## Bounded context

This contract belongs to the **Architecture evidence** context. It inventories SkillFence's decision and domain artifacts, verifies their structural and referential integrity, and reports whether the separate Product Foundry count floors are met. It does not invent decisions, assess prose quality, or convert an incomplete product into a completed one.

The context consumes repository documents through the owned `DocumentationSource` port. `FileSystemDocumentationSource` is the production adapter, `DocumentationReport` is the immutable result value, and the `skill-fence-docs` binary is the application boundary used by contributors and CI.

## Ubiquitous language

- **Architecture artifact:** one discovered Markdown file under `docs/adr` or `docs/ddd`.
- **Schema gate:** validation of required sections, Accepted ADR status, live local links, and implementation/test/decision/domain traceability.
- **Completion gate:** the schema gate plus the minimum counts of 24 Accepted ADRs and 12 DDD artifacts.
- **Traceability edge:** a Markdown link from an artifact to an existing decision, domain artifact, source file, or test.
- **Documentation report:** the exact Accepted-ADR and DDD-artifact counts returned after successful validation.

## Invariants

1. Every discovered ADR and DDD Markdown file is read through the owned source port exactly once per validation.
2. Schema success requires every ADR to be Accepted and every artifact to contain its kind-specific required sections.
3. A traceability edge must resolve to an existing local target; a broken target blocks validation.
4. ADRs link to implementation, tests, and a DDD artifact; DDD artifacts link to an ADR, implementation, and tests.
5. Completion mode cannot succeed below either count floor and must report the observed numerator and required denominator.
6. Schema mode does not claim completion; it only proves the quality contract for the artifacts currently present.

## Traceability

- Decision: [`ADR-003`](../adr/ADR-003-machine-checked-architecture-evidence.md)
- Policy, report value, port, and adapter: [`src/documentation.rs`](../../src/documentation.rs)
- CLI application boundary: [`src/bin/skill-fence-docs.rs`](../../src/bin/skill-fence-docs.rs)
- Outside-in acceptance and completion fault injection: [`tests/architecture_docs.rs`](../../tests/architecture_docs.rs)
- CI integration: [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)
