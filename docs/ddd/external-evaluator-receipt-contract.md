# External evaluator receipt domain contract

## Bounded context

This contract belongs to the **External evidence** context. It converts one explicitly requested external evaluator invocation into one immutable `ExternalEvidence` value inside the `Assessment` aggregate. It collaborates with the operating system through the owned `ExternalProcess` port and cannot alter deterministic findings or exercise human release authority.

The context treats a command declaration, a completed process observation, and a spawn error as different facts. `CommandExternalProcess` is the production adapter. Test doubles implement only the owned port; real executable fixtures contract-test the adapter boundary.

## Ubiquitous language

- **Intended command:** the ordered executable-and-argument vector declared for an adapter.
- **Process observation:** the optional numeric exit code and captured stdout/stderr returned after a process starts and terminates.
- **Spawn error:** an operating-system error returned before a process observation exists.
- **External receipt:** the immutable adapter, command, status, exit, streams, error, and display summary recorded in the evidence pack.
- **Rejection:** a process observation whose exit code is not zero; it is `Failed`, not `Unavailable`.
- **Unavailability:** failure to start the requested executable; it has no exit code or captured streams.

## Invariants

1. One requested adapter invocation yields exactly one external receipt.
2. The receipt command preserves the executable and argument order sent through the owned port.
3. Only an observed exit code of zero maps to `Passed`.
4. A started process without exit code zero maps to `Failed`, preserving both text-decoded streams and any numeric exit code.
5. A spawn error maps to `Unavailable`, preserves the error, and cannot claim an exit code or captured output.
6. `NotRequested` declares the intended command but cannot claim that a process ran.
7. No external receipt can remove a deterministic blocking finding, change the deterministic verdict, or authorize release.

## Collaboration sequence

1. The CLI explicitly requests a RuVector or MetaHarness probe.
2. The adapter-specific function supplies a stable program and ordered arguments to `probe`.
3. `probe` calls `ExternalProcess::run` exactly once.
4. The production adapter starts the program directly and returns a process observation, or returns the operating-system spawn error.
5. `probe` maps that result into one immutable receipt and the CLI replaces the corresponding `NotRequested` declaration in the aggregate.
6. JSON serialization preserves the structured receipt; Markdown renders the concise detail and reiterates the human release boundary.

This slice records evidence about an external process but does not persist a domain-state transition, so it emits no domain event.

## Traceability

- Decision: [`ADR-004`](../adr/ADR-004-external-evaluator-process-receipts.md)
- Receipt value, owned port, production adapter, and collaboration test: [`src/external.rs`](../../src/external.rs)
- Assessment aggregate integration: [`src/lib.rs`](../../src/lib.rs)
- Application wiring: [`src/main.rs`](../../src/main.rs)
- Real executable contract and unavailable fault injection: [`tests/cli_external_receipts.rs`](../../tests/cli_external_receipts.rs)
- Parent context map and release-authority boundary: [`docs/ddd/skill-release.md`](skill-release.md)
