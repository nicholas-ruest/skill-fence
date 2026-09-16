# ADR-004: Record external evaluator process receipts through an owned port

Status: Accepted

## Context

SkillFence can explicitly invoke RuVector and MetaHarness as optional external evaluators. The original adapter reduced a successful process to trimmed stdout, a rejected process to trimmed stderr, and a spawn failure to an error string. That summary did not preserve the exact command, exit code, or both output streams, so a downstream reviewer could not reliably distinguish what was requested, what actually ran, and why it received a particular status.

The Product Foundry gate also requires contract evidence for mocked external boundaries and a real production-component journey. Tests must control the process boundary without mocking third-party CLI internals, while acceptance tests must exercise real executable fixtures through the operating system.

## Decision

SkillFence owns an internal `ExternalProcess` port whose operation accepts a program and ordered argument slice and returns a `ProcessObservation`. `CommandExternalProcess` is the production adapter and invokes the executable directly with `std::process::Command`; it does not interpolate arguments through a shell.

Every `ExternalEvidence` value records the adapter, intended command vector, status, optional exit code, complete text-decoded stdout and stderr, optional spawn error, and a concise display detail. Status has exact process semantics:

- `Passed`: the process started and returned exit code zero.
- `Failed`: the process started but did not return exit code zero. A signal-terminated process may have no numeric exit code.
- `Unavailable`: the process could not be started, so it has no exit code or captured output and carries the spawn error.
- `NotRequested`: the intended command is declared, but there is no outcome, captured output, or error.

The full receipt fields are authoritative; `detail` remains a human-readable summary for the existing Markdown report. External status never changes deterministic findings or grants release authority.

## Alternatives Considered

- **Keep only one detail string.** Rejected because it collapses distinct process facts and prevents reliable reconciliation with the requested command.
- **Run fixture behavior through an in-memory mock only.** Rejected because a mock cannot prove executable discovery, argv delivery, exit-code handling, or real stdout/stderr capture.
- **Invoke a shell command string and record it.** Rejected because shell interpolation changes the security boundary and makes argv ambiguous.
- **Treat an unavailable executable as a failed evaluator result.** Rejected because “did not run” and “ran and rejected” are materially different evidence states.

## Consequences

- JSON evidence consumers receive an additive, structured process receipt for every external adapter state.
- Success, rejection, and unavailability can be audited without parsing display prose.
- Unit tests substitute only the owned process port; integration tests use executable fixtures and the real production adapter.
- Captured streams are decoded with UTF-8 loss replacement because the evidence format is JSON text. Binary-perfect receipts would require a future encoding and schema decision.
- Receipts attest only to local process invocation and output. They do not establish evaluator correctness, semantic quality, or release approval.

## Verification

- [`tests/cli_external_receipts.rs`](../../tests/cli_external_receipts.rs) places real RuVector and MetaHarness fixture executables on a scoped `PATH`, verifies exit-zero and exit-23 receipts, and removes the executable to fault-inject unavailability.
- [`src/external.rs`](../../src/external.rs) contains `requests_ruvector_through_the_owned_process_port`, which verifies the exact collaboration and returned observation mapping.
- `cargo test --test cli_external_receipts`
- `cargo test requests_ruvector_through_the_owned_process_port`
- `cargo test --locked`

## Implementation and Domain Links

- Receipt value, owned port, production adapter, and mapping policy: [`src/external.rs`](../../src/external.rs)
- Assessment aggregate integration: [`src/lib.rs`](../../src/lib.rs)
- CLI application boundary: [`src/main.rs`](../../src/main.rs)
- Domain contract: [`docs/ddd/external-evaluator-receipt-contract.md`](../ddd/external-evaluator-receipt-contract.md)
