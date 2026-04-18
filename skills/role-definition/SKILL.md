---
name: role-definition
description: Define reusable agent roles from a role idea, job-to-be-done, and supporting references; use when turning a vague role concept, external role pattern, or observed practice into a grounded role definition with explicit purpose, boundaries, outputs, assumptions, and open questions in a local Markdown workflow.
---

# Role Definition

Turn a role idea plus source material into a reusable role-definition brief that another agent or designer can use without reconstructing the intent from chat context.

For this repository's Role Builder workflows, this is the preferred default path for first-phase role design. Use it before deriving workflows, checks, or supporting artifacts.

## Quick start

1) Gather the role idea, job-to-be-done, and source references.
2) Clarify the role's audience, lifecycle position, and boundaries.
3) Separate core responsibilities from out-of-scope work.
4) Identify the role's default outputs and the assumptions shaping the design.
5) Populate the role-definition brief and leave unresolved questions explicit rather than inventing answers.

## Workflow

### 1) Gather inputs
- Start with the role idea, role name if available, and the intended job-to-be-done.
- Pull in local role definitions, notes, comparison material, or workflow docs when they help ground the role.
- Use external references only when they materially improve the role definition.

### 2) Clarify the role shape
- State what job the role is meant to perform and why it should exist.
- Identify who will use the role or depend on its outputs.
- Place the role in the lifecycle: what comes before it, what it hands off to, and where it should stop.

### 3) Separate scope cleanly
- List the responsibilities the role should own directly.
- List what is explicitly out of scope.
- If the role idea is mostly personality language, convert it into responsibilities, boundaries, and outputs before proceeding.

### 4) Identify default outputs
- Describe the role's major artifacts, decisions, or deliverables.
- Keep reusable role outputs separate from project-instance outputs.
- If the role depends on a tool-specific pattern, describe the underlying method separately from the tool assumption.

### 5) Produce the brief
- Populate the local role-definition brief artifact.
- Ensure it includes:
  - role name
  - job-to-be-done
  - intended audience and consumers
  - lifecycle position
  - success criteria
  - primary responsibilities
  - out-of-scope responsibilities
  - default outputs
  - source material and references
  - assumptions
  - open questions

### 6) Finalize for downstream use
- Make sure another designer could derive the role's support package without inventing the role's purpose or boundaries.
- Keep contradictions and unresolved design choices visible instead of smoothing them over.
- Hand the result to the role-support-system design step rather than trying to design the whole package in one pass.

## Notes

- This skill focuses on role definition only. Do not let it drift into full support-system design, implementation planning, or project-instance delivery work.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when local references need synthesis before the role can be defined clearly.
- Use [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md) when external comparison or reference material needs to be summarized first.
- In this repository, the normal output is an instantiated copy of [`artifacts/role-definition-brief.md`](D:/Projects/agoge/artifacts/role-definition-brief.md).
