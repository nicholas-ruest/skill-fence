# Dream Machine research log

This Gist is the public, append-only research notebook for the Product Foundry. The product itself always lives in its own repository; this log records the evidence that led to its selection.

## 2026-09-10 — 08:30 Enterprise Open-Source Radar

### Ranked input

1. **NVIDIA SkillEvaluator** — [official documentation](https://docs.nvidia.com/skills/skillevaluator) describes a three-tier agent-skill evaluation pipeline: offline deterministic checks, semantic overlap detection, and optional live agent evaluation. Its deterministic-first gate is useful without requiring model credentials.
2. **Google Data Agent Kit** — [source repository](https://github.com/GoogleCloudPlatform/data-agent-kit) packages skills, MCP configurations, extensions, and agent evaluation/monitoring integration points for enterprise data-agent development.

### Ruvnet composition signal

Ruflo has a portable `SKILL.md` convention. RuVector documents local embedding/vector-store surfaces, and MetaHarness documents local red/blue evaluation. The viable enterprise opportunity is not another agent framework: it is a small, transparent release gate for teams adopting skills.

### Bounded hypothesis

A local Rust CLI that validates a skill’s structure and policy boundaries before requesting optional semantic or live evaluation can produce useful review evidence even when model credentials or accelerators are unavailable.

## 2026-09-10 — 12:30 Frontier Research Scout

### Primary research signal

**Agent Lightning v1.0** — [paper](https://arxiv.org/abs/2608.17528) develops a harnessed-agent learning workflow that separates agent execution from training/instrumentation. The relevant product insight is the separation, not a claim that this product trains an agent: evidence collection can be added without changing or executing the target skill.

### Constraints

Live evaluation and reinforcement learning need comparable tasks, a model runtime, credentials, and a sandbox. Those prerequisites were not established for this cycle, so no performance-lift claim is made.

### Bounded hypothesis

Separate a skill’s intake/evidence boundary from any external evaluator. Make every unavailable evaluator an explicit receipt rather than turning it into either a pass or a reason to emit no result.

## 2026-09-10 — 21:30 Product Decision

### Candidates

| Candidate | Decision |
|---|---|
| **SkillFence** — local skill-release evidence CLI | **Selected**: bounded, user-facing, implementable in Rust, and composes NVIDIA SkillEvaluator’s model with Ruflo, RuVector, and MetaHarness surfaces. |
| Autonomous skill optimizer | Rejected: cannot truthfully make optimization claims without live, comparable evaluations and model/runtime prerequisites. |
| Dream Machine semantic-memory component | Rejected: an internal factory component, not a standalone product. |

### What was built

**[SkillFence](https://github.com/nicholas-ruest/skill-fence)** is a Rust CLI that reads a `SKILL.md` directory and produces a SHA-256-addressed JSON/Markdown evidence pack. It checks required frontmatter, known prompt-injection markers, destructive-command markers, an explicit human-boundary signal, and deterministic duplicate-content similarity. It has explicit optional adapters for a real RuVector CLI probe and a real MetaHarness mock red/blue probe; unavailable tools are recorded honestly and never grant release authority.

- Draft implementation: [SkillFence PR #1](https://github.com/nicholas-ruest/skill-fence/pull/1)
- Product selection research: [repository brief](https://github.com/nicholas-ruest/skill-fence/blob/dream-cycle/2026-09-10-skill-fence/docs/research/2026-09-10-product-selection.md)
- Checks: GitHub Actions passed `cargo fmt --check`, `cargo test`, Clippy with warnings denied, and the example CLI run.

### Correction

The first attempted nightly output mistakenly placed an internal counterfactual-memory component in the Dream Machine repository. That draft was closed and superseded. Dream Machine is now explicitly configured as the research-and-selection control plane; each selected product is created in its own repository.
