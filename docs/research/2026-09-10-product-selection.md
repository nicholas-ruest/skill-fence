# 2026-09-10 product selection: SkillFence

## Original context

Enterprise teams are accumulating agent skills faster than they can safely inspect them. A team needs a small, local tool that answers a practical question before adding a skill to a production agent: **what deterministic evidence supports sending this skill to a human reviewer?**

The product must compose the day’s enterprise open-source signal, a frontier research mechanism, and multiple Ruflo/Ruvnet capabilities. It must be a new product, not another Dream Machine subsystem.

## Inputs and evidence

| Ingredient | Evidence | Product use |
|---|---|---|
| NVIDIA SkillEvaluator | [official documentation](https://docs.nvidia.com/skills/skillevaluator) describes deterministic quality gates, semantic deduplication, and live evaluation for agent skills. | Use a deterministic-first release-gate shape; do not claim SkillEvaluator itself is embedded. |
| Agent Lightning v1.0 | [paper](https://arxiv.org/abs/2608.17528) argues for decoupling an agent’s execution from learning/instrumentation. | Keep SkillFence as a non-invasive assessor that reads a skill directory and records evidence rather than executing or changing the skill. |
| Ruflo | Its local source describes `SKILL.md` bundles and a plugin ecosystem. | `SKILL.md` is the explicit intake contract. |
| RuVector | Ruflo’s local RuVector plugin documents `embed text`, vector storage, and RVF surfaces. | An explicit CLI adapter requests a real embedding probe and records `Passed`, `Failed`, or `Unavailable`. |
| MetaHarness | Its local CLI documentation provides a mock red/blue evaluation command. | An explicit adapter can collect a mock adversarial-evaluation receipt. |

## Candidate decision

| Candidate | User problem | Why it was not/was selected |
|---|---|---|
| **SkillFence** — local skill release evidence CLI | Whether a skill is safe and reviewable before adoption | **Selected.** Bounded Rust MVP, direct enterprise user, validates offline, and composes all four inputs without fabricating availability. |
| Autonomous skill optimizer | Improve agent performance through live trial and reinforcement learning | Rejected for this cycle: requires model credentials, an agent runtime, and comparable live outcomes before it can make a trustworthy optimization claim. |
| Another semantic memory backend | Improve memory recall in an existing harness | Rejected: this is an internal capability, not a standalone product, and repeats the mistake of targeting Dream Machine. |

## Frozen product contract

Input: a directory containing `SKILL.md`, plus zero or more peer skill directories.

Output: a JSON and Markdown evidence pack with stable content identity, deterministic policy findings, duplicate-content signals, and optional external-tool receipts.

Invariants:

- SkillFence never installs, runs, deploys, merges, or promotes a skill.
- A deterministic block remains blocked regardless of optional-tool results.
- A missing RuVector or MetaHarness executable is recorded, not translated into a positive claim.
- The strongest result is `READY_FOR_HUMAN_REVIEW`; a person owns release authority.

## Evaluation evidence

The Rust MVP passed `cargo fmt --check`, `cargo test` (three unit tests), and `cargo clippy --all-targets -- -D warnings`. The example skill produces `READY_FOR_HUMAN_REVIEW`; the unsafe fixture blocks on prompt-injection and destructive-command policy markers.

`npx ruvector@0.2.25 embed text …` was tried during product discovery and failed because the package’s ONNX WASM assets were not bundled in this environment. This is why the product’s RuVector adapter exposes a receipt rather than pretending to supply semantic evaluation.

## Decision

Create **SkillFence** as `nicholas-ruest/skill-fence`. Publish the MVP as a draft pull request for human review. Keep the Gist, if any, as a one-screen index to the actual repository and PR.
