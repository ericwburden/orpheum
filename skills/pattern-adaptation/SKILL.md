---
name: pattern-adaptation
description: Adapt external or tool-coupled workflow patterns into reusable repo-native methods; use when evaluating a Notion-based, platform-specific, or otherwise transport-coupled pattern to determine what should be preserved, stripped away, adapted into local Markdown or local-first practice, or rejected as unnecessary overhead in a role-support workflow.
---

# Pattern Adaptation

Turn a promising external pattern into a reusable local method without importing irrelevant tooling assumptions. The goal is to preserve the useful practice while making the operating model explicit for this repository.

For this repository's Role Builder workflows, this is the preferred default path for evaluating tool-coupled patterns and documenting adaptation decisions before they are baked into a role support system.

## Quick start

1) Read the external or tool-coupled pattern closely enough to understand its actual method.
2) Separate the underlying practice from the transport, platform, or product-specific implementation.
3) Decide what to preserve, what to strip, what to adapt, and what to reject.
4) Record explicit adaptation decisions and their rationale.
5) Feed those decisions into the local role support system rather than copying the source pattern wholesale.

## Workflow

### 1) Gather the pattern inputs
- Start with the external workflow, skill, role, or artifact pattern being considered.
- Pull in the local role definition or support-system context that the pattern is meant to serve.
- Read just enough surrounding material to understand the pattern's method and intended use.

### 2) Identify the underlying method
- State what job the pattern is actually helping accomplish.
- Separate the method from the platform-specific transport or storage assumptions.
- Look for reusable ingredients such as:
  - sequencing logic
  - decision points
  - quality gates
  - artifact shapes
  - collaboration or handoff moves

### 3) Classify the pattern elements
- **Preserve** when the element is core to the method and still valuable locally.
- **Strip** when the element is only transport, platform, or product glue.
- **Adapt** when the method is useful but needs a local-first or Markdown-first form.
- **Reject** when the element adds overhead without improving repeatability, quality, or clarity.

### 4) Make the adaptation decisions explicit
- Record:
  - what is being preserved
  - what is being stripped
  - what the local adaptation should look like
  - what is intentionally not being imported
- Explain the rationale in terms of repo goals: repeatability, clarity, local-first operation, and low-overhead coherence.

### 5) Feed the result into the support system
- Use the adaptation decisions to refine the role's skill map, workflows, checks, or tooling preferences.
- Keep the local operating model explicit so future agents do not have to rediscover why the adaptation exists.
- If the pattern is still too broad, preserve only the reusable subset and leave the rest out.

## Notes

- This skill is for adaptation decisions, not for full role definition or full support-system design by itself.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the source pattern spans multiple local documents or when comparison context needs synthesis first.
- Use [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) when the external pattern needs summarization before adaptation decisions can be made cleanly.
- In this repository, the normal destination for adaptation outcomes is the instantiated [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md) artifact and its linked workflows.
