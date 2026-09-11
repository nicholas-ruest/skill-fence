# Dream Machine index — 2026-09-10

The previous report-only output was superseded because it built an internal component in the factory repository rather than a standalone product.

Tonight’s actual experimental product is **SkillFence**: a Rust CLI that creates local, human-reviewable release evidence for AI-agent skills.

- Repository: https://github.com/nicholas-ruest/skill-fence
- Draft implementation PR: https://github.com/nicholas-ruest/skill-fence/pull/1
- Selection research and sources: https://github.com/nicholas-ruest/skill-fence/blob/dream-cycle/2026-09-10-skill-fence/docs/research/2026-09-10-product-selection.md

The product combines NVIDIA SkillEvaluator’s deterministic-first evaluation model, Ruflo’s `SKILL.md` contract, an honest optional RuVector CLI evidence probe, and an honest optional MetaHarness red/blue probe. The code, tests, ADR, DDD, and CI are in the draft PR. The Gist is only the index.
