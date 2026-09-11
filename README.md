# SkillFence

SkillFence is a local-first Rust CLI for deciding whether an AI-agent skill is ready for **human review**. It turns a directory containing `SKILL.md` into an evidence pack: deterministic structure and policy findings, duplicate-content signals, a content hash, and optional receipts from real RuVector and MetaHarness probes.

It does not install, execute, or promote skills. A passing result means only `READY_FOR_HUMAN_REVIEW`.

## Why this exists

NVIDIA's [SkillEvaluator](https://docs.nvidia.com/skills/skillevaluator) establishes a useful shape: deterministic validation before semantic and live evaluation. SkillFence is the local Rust release gate that makes that shape practical for a team’s existing skill directories:

- **NVIDIA SkillEvaluator:** tiered, evidence-backed evaluation inspiration.
- **Ruflo:** consumes its portable `SKILL.md` convention as the product input.
- **RuVector:** an optional, real CLI probe for semantic acceleration; absence is recorded honestly.
- **MetaHarness:** an optional, real `redblue --mock-judge` adversarial probe.

## Run it

```bash
cargo run -- --skill examples/release-ready-skill --compare examples/unsafe-skill --out evidence.json
```

To explicitly collect installed-tool receipts:

```bash
cargo run -- --skill examples/release-ready-skill --ruvector-probe --redblue-probe
```

The deterministic gate is fully local. External probes never upgrade a blocked skill to releaseable and their missing executables are emitted as `Unavailable`, not silently ignored.

## Current MVP checks

- `SKILL.md` and required `name` / `description` frontmatter.
- Known prompt-injection and destructive-command markers.
- Explicit human-approval boundary signal.
- Deterministic token-shingle duplicate similarity against supplied skills.
- SHA-256 evidence identity and JSON/Markdown evidence output.

See [the architecture decision](docs/adr/ADR-001-local-first-evidence-gate.md) and [bounded contexts](docs/ddd/skill-release.md).
