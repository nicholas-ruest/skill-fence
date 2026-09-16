# Comparison evidence domain contract

## Bounded context

This contract belongs to the **Similarity signal** context. It consumes skill sources from **Skill intake** and contributes immutable `DuplicateSimilarity` values to the `Assessment` aggregate. It does not decide whether two skills are semantically equivalent and it does not grant release authority.

## Ubiquitous language

- **Primary skill:** the skill directory being assessed.
- **Requested comparison:** a skill directory explicitly supplied by one `--compare` argument.
- **Readable source:** a directory whose `SKILL.md` can be loaded completely.
- **Comparison coverage:** the one-to-one relationship between requested comparisons and similarity results.
- **Evidence pack:** the serialized representation of a successfully constructed `Assessment`.

## Aggregate and values

`Assessment` is the aggregate root for one evaluation. `DuplicateSimilarity` is an immutable value containing the requested comparison path and its deterministic Jaccard percentage. `SkillSource` is the owned intake port; `FileSystemSkillSource` is its production adapter.

The comparison list preserves request order so a caller can reconcile each output value with its CLI input. No child value is constructed independently of the aggregate's intake operation.

## Invariants

1. A successful `Assessment` contains exactly one `DuplicateSimilarity` for every requested comparison.
2. The primary skill and all comparisons must be readable before an evidence pack can be serialized.
3. An intake error identifies whether the failed source was the primary skill or a comparison and names the exact `SKILL.md` path.
4. Comparison failure does not become a finding about the primary skill; it aborts aggregate construction.
5. Similarity values remain signals only. They cannot change `BLOCKED` to `READY_FOR_HUMAN_REVIEW` and cannot approve a release.

## Collaboration sequence

1. The CLI passes the primary directory and ordered comparison directories to `assess`.
2. `assess_with_source` requests the primary source through `SkillSource`.
3. It requests each comparison source through the same port and constructs one `DuplicateSimilarity` per successful response.
4. Any port error is contextualized and returned immediately.
5. Only a complete aggregate reaches JSON/Markdown serialization and optional output-file creation.

There is no persisted domain-state transition in this slice, so no domain event is emitted. The evidence pack is an immutable result rather than an event-sourced entity.

## Traceability

- Decision: [`ADR-002`](../adr/ADR-002-fail-closed-comparison-intake.md)
- Aggregate, values, port, and adapter: [`src/lib.rs`](../../src/lib.rs)
- User-boundary acceptance test: [`tests/cli_comparison_boundary.rs`](../../tests/cli_comparison_boundary.rs)
- Collaboration test: `tests::requests_every_comparison_and_propagates_read_failure` in [`src/lib.rs`](../../src/lib.rs)
