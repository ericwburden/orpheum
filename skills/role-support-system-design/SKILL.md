---
name: role-support-system-design
description: Design operational support systems for reusable agent roles; use when a role definition exists and needs to be turned into an explicit local support package of artifacts, workflows, checks, skill support, tooling preferences, and lifecycle handoffs in a local Markdown workflow.
---

# Role Support System Design

Turn a stable role definition into the minimum support system needed for the role to work consistently. The goal is to make the role operational without inventing unnecessary framework overhead.

For this repository's Role Builder workflows, this is the preferred default path for second-phase role design after the role-definition brief exists.

## Quick start

1) Read the role-definition brief and identify the role's default outputs, boundaries, and lifecycle position.
2) Derive the minimum artifact set the role needs.
3) Derive the workflows, checks, and skill support needed to produce and validate those outputs.
4) Make tooling preferences and adaptation decisions explicit.
5) Populate the role-support-system artifact and leave known gaps visible instead of compensating with chat-only guidance.

## Workflow

### 1) Read the role definition as the source of truth
- Start with the local role-definition brief.
- Extract the role's job-to-be-done, lifecycle position, boundaries, and default outputs.
- Do not invent support structure that the role definition does not justify.

### 2) Derive the minimum artifact set
- Identify the reusable artifacts the role must produce by default.
- Keep the set as small as possible while still making the role operational.
- Distinguish reusable role artifacts from project-instance outputs.

### 3) Derive the workflow set
- Determine what workflows are needed to:
  - create the role's outputs
  - validate them
  - hand them downstream
- Avoid adding separate workflows for concerns that should stay inside a larger workflow unless the split materially improves repeatability.

### 4) Derive the check set
- Identify the primary artifact-level checks needed for definition-of-done quality.
- Identify cross-cutting checks needed for traceability, boundaries, or other systemic risks.
- Treat checks as part of the role contract, not as optional afterthoughts.

### 5) Map skill support and tooling preferences
- Classify skills as:
  - direct support
  - local adaptations
  - adjacent or downstream only
- If a candidate support pattern is tool-coupled, use [`pattern-adaptation`](D:/Projects/agoge/skills/pattern-adaptation/SKILL.md) to decide what should be preserved, stripped, adapted, or rejected.
- Make local-first or Markdown-first tooling preferences explicit when they materially affect the role.

### 6) Produce the support-system artifact
- Populate the local role-support-system artifact.
- Ensure it includes:
  - artifact set
  - workflow set
  - check set
  - skill support map
  - tooling preferences
  - lifecycle and handoffs
  - gaps and risks
  - open questions

### 7) Finalize with restraint
- Keep the support system lean enough that another designer could follow it without constant interpretation.
- Leave known gaps explicit rather than compensating with vague "future work" assumptions.
- Hand the result to the role-package review stage instead of trying to approve the package from inside the design step.

## Notes

- This skill is for support-system design, not for defining the role from scratch or for final readiness review.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when multiple local precedents or role-package examples need synthesis before design decisions can be made cleanly.
- Use [`pattern-adaptation`](D:/Projects/agoge/skills/pattern-adaptation/SKILL.md) when a candidate support pattern is tool-coupled and needs explicit local adaptation.
- In this repository, the normal output is an instantiated copy of [`artifacts/role-support-system.md`](D:/Projects/agoge/artifacts/role-support-system.md).
