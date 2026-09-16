# ADR-002: Fail closed when requested comparison evidence is unreadable

Status: Accepted

## Context

SkillFence accepts zero or more `--compare` directories and reports duplicate-content signals against them. The first implementation ignored a comparison whenever its `SKILL.md` could not be read. That made a successful assessment ambiguous: a reviewer could not distinguish “all requested comparisons were checked” from “some requested comparisons disappeared.” This violates the evidence gate's requirement that absence be explicit and prevents a caller from relying on the comparison set as declared input.

## Decision

Assessment construction must read the primary skill and every requested comparison through the owned `SkillSource` port. The filesystem adapter reads `<directory>/SKILL.md`. If any requested source is unreadable, construction returns an error that names the exact path, the CLI exits unsuccessfully before serialization, and no new evidence pack is written. A successful `Assessment` therefore proves that every requested comparison contributed exactly one similarity result.

The port remains internal because filesystem replacement is a testing and architecture seam, not part of the public crate API. Collaboration tests substitute the owned port; end-to-end tests invoke the compiled binary and the real filesystem.

## Alternatives Considered

- **Silently omit unreadable comparisons.** Rejected because it produces apparently complete evidence from incomplete inputs.
- **Emit a warning finding and continue.** Rejected for now because a comparison finding is metadata about assessment construction, not a finding about the primary skill, and downstream consumers could still overlook it.
- **Represent every comparison as a success/error receipt inside `Assessment`.** Viable for a future partial-evidence mode, but it requires a versioned evidence-schema decision and explicit consumer handling.

## Consequences

- Comparison coverage is deterministic and auditable for successful assessments.
- One unreadable requested comparison blocks the entire run; callers must repair or remove that explicit input before obtaining evidence.
- Filesystem errors gain path and role context while retaining their original `ErrorKind`.
- The intake policy can be tested interaction-first without mocking Rust filesystem internals.

## Verification

- [`tests/cli_comparison_boundary.rs`](../../tests/cli_comparison_boundary.rs) starts with the user-visible behavior: failure, path-bearing stderr, and no output pack.
- [`src/lib.rs`](../../src/lib.rs) contains the `SkillSource` collaboration test that verifies the primary and requested comparison are both read and the comparison failure propagates.
- `cargo test --test cli_comparison_boundary`
- `cargo test requests_every_comparison_and_propagates_read_failure`
- `cargo test`

## Implementation and Domain Links

- Implementation: [`src/lib.rs`](../../src/lib.rs)
- CLI boundary: [`src/main.rs`](../../src/main.rs)
- Domain contract: [`docs/ddd/comparison-evidence-contract.md`](../ddd/comparison-evidence-contract.md)
