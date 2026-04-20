# Implementation Engineer Skill Sourcing

## Purpose

Capture which local skills already support the [`Implementation Engineer`](D:/Projects/orpheum/roles/implementation-engineer.md) role, which gaps are intentionally being handled inside the first package shape, and which external patterns informed the first-pass support decisions.

## External Pattern Summary

The first-pass Implementation Engineer package is shaped by three recurring external patterns:

- implementation roles work downstream of explicit requirement, architecture, and planning artifacts rather than inventing product or design direction in code
- coding, review, and testing remain distinct responsibilities even when one system orchestrates all three
- implementation quality depends on durable implementation context and evidence, not only on the raw code diff

These patterns are visible in:

- MetaGPT's software-company framing, which places engineers downstream of product, architecture, and project-management structure and gives the engineer role a focused coding goal rather than end-to-end SDLC ownership
- ChatDev's explicit separation of design, coding, review, and testing phases, with programmer, reviewer, and tester treated as distinct roles
- GitHub Spec Kit's spec-driven-development framing, which treats specifications and implementation plans as the governing artifacts that code should serve rather than replace

## Local Skill Support

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/orpheum/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing implementation working-session notes, defect triage notes, and build-or-debug notes before implementation artifacts are drafted.
- [`content-research-writer`](D:/Projects/orpheum/skills/content-research-writer/SKILL.md)
  - Useful when implementation depends on external standards, platform guidance, integration references, or migration patterns that should be sourced explicitly.
- [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md)
  - Useful when the implemented slice includes browser-based or web-application behavior and the engineer needs stronger local evidence than a code-only summary can provide.

### Useful With Existing Local-Markdown Positioning

- [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md)
  - Useful as the core local-Markdown synthesis skill because implementation work often depends on combining planning, architecture, requirements, local code context, and validation notes into one durable record.
- [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md)
  - Useful for packaging reviewed implementation outputs into a downstream-ready handoff without inventing a new implementation-handoff skill too early.

### Allium-Aware Support

- [`allium`](C:/Users/ericw/.codex/skills/allium/SKILL.md)
  - Useful when an existing behavioral specification materially constrains how the implementation should behave.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md)
  - Useful when implementation confidence depends on checking whether the code may have drifted from a governing specification.
- [`tend`](C:/Users/ericw/.codex/skills/allium/skills/tend/SKILL.md)
  - Useful only when implementation uncovers an upstream behavioral gap that needs clarification rather than silent invention.

## New Repo-Native Skills Added

A narrow direct-support implementation skill is now part of the Implementation Engineer package:

- [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md)
  - This is the dedicated repo-native skill for preparing the implementation record, implementation evidence, implementation-package self-review, and downstream implementation handoff.
  - It closes the main v1 support gap without collapsing into generic coding, planning, independent review, or QA authority.

There is still no dedicated generic implementation-execution coding skill.

That remains an intentional design choice rather than a missing implementation.

The repository also includes [`implementation-skill-discovery`](D:/Projects/orpheum/skills/implementation-skill-discovery/SKILL.md) as a dedicated meta-skill for deciding when recurring Implementation Engineer friction has become strong enough to justify an additional narrower repo-native implementation skill and for specifying what that future skill should contain.

The current package is using the new dedicated implementation-packaging skill plus the existing local-Markdown synthesis, testing, and packaging skills, with role-specific behavior embedded in:

- the [`Implementation Engineer`](D:/Projects/orpheum/roles/implementation-engineer.md) role definition
- the Implementation Engineer artifact set in [`artifacts/`](D:/Projects/orpheum/artifacts)
- the Implementation Engineer workflow set in [`workflows/`](D:/Projects/orpheum/workflows)
- the Implementation Engineer check set in [`checks/`](D:/Projects/orpheum/checks)

The new `implementation-skill-discovery` skill exists specifically to prevent indefinite deferral of future implementation-skill definition.

## Why Existing Skills Were Enough For V1

### The missing problem was package discipline, not "how to code"

This repository already operates inside a coding-capable agent environment.

The main missing need for an Implementation Engineer role was not a generic code-generation skill. It was a repeatable way to:

- keep implementation inside reviewed scope
- preserve traceability back to requirements, architecture, and planning
- capture local validation evidence honestly
- hand implementation downstream without forcing review and QA roles to reconstruct context from diffs

The package therefore treats coding ability as an assumed baseline capability of the agent environment and spends its reusable structure on scope discipline, evidence capture, and downstream packaging.

Those concerns are now handled through a combination of:

- the dedicated [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md) skill
- the role definition, implementation artifacts, workflows, and checks
- supporting synthesis, testing, and handoff skills where they are still the better abstraction

### Implementation evidence can be expressed through artifacts and checks first

The major new requirement was not merely "write code." It was:

- record what scope was actually implemented
- preserve changed-area and interface visibility
- separate evidence from optimism
- review implementation readiness explicitly
- package the implementation for downstream review and verification

Those concerns are being handled in v1 through:

- [`implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
- [`implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
- [`implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md)
- [`implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md)
- [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md)
- [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md)

That keeps the package lean while still making the role operational.

### A dedicated implementation-handoff skill would be premature duplication

The existing [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) skill already expresses the right downstream packaging posture:

- summarize what is being handed off
- preserve traceability and evidence context
- separate ready content from unresolved issues
- avoid drifting into downstream review or release ownership

The Implementation Engineer handoff needed different content, not a fundamentally different packaging method.

## Role-Specific Design Decisions

### Keep Implementation Engineer separate from Technical Planner

The package intentionally keeps execution planning and code implementation separate.

The Implementation Engineer is allowed to:

- implement the approved slice
- record deviations discovered during coding
- surface blockers, defects, and evidence limits
- package the implementation for downstream roles

The Implementation Engineer is not supposed to:

- replace the reviewed plan with a new one
- silently change slice boundaries or dependency strategy
- absorb backlog or delivery-administration ownership

This boundary is enforced in the role definition and in [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md).

### Keep Implementation Engineer separate from QA / Verification Lead and future Code Reviewer work

The package intentionally keeps implementation ownership separate from formal quality judgment.

The Implementation Engineer is allowed to:

- collect local validation evidence
- record known failures and skipped checks
- self-assess whether the implementation package is ready to move downstream

The Implementation Engineer is not supposed to:

- act as the final verification authority
- replace an independent code review function
- present local evidence as though it were full downstream confidence

This boundary is enforced in the role definition and in [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md).

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- implementation record
- implementation evidence
- implementation readiness review
- implementation package handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by Business Analyst, Solution Architect, Technical Planner, and QA / Verification Lead and keeps the role easier to validate and hand off.

## Allium Relationship

The Implementation Engineer role should treat Allium as an upstream behavioral constraint, not as the format for implementation artifacts.

That means:

- consume existing Allium when it materially constrains implementation behavior
- use [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when implementation evidence depends on checking for spec-to-code drift
- route unstable behavior back upstream rather than inventing it in code or in implementation artifacts

The implementation artifacts themselves remain Markdown-first in this repository.

## External Sources

- [MetaGPT README](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/README.md)
- [MetaGPT engineer role](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/metagpt/roles/engineer.py)
- [ChatDev ACL paper](https://aclanthology.org/2024.acl-long.810.pdf)
- [ChatDev README](https://raw.githubusercontent.com/OpenBMB/ChatDev/main/README.md)
- [GitHub Spec Kit: spec-driven.md](https://github.com/github/spec-kit/blob/main/spec-driven.md)
- [Diving Into Spec-Driven Development With GitHub Spec Kit](https://developer.microsoft.com/blog/spec-driven-development-spec-kit)
- [AutoGen agents documentation](https://microsoft.github.io/autogen/stable/user-guide/agentchat-user-guide/tutorial/agents.html)
- [AutoGen Magentic-One overview](https://microsoft.github.io/autogen/stable/user-guide/agentchat-user-guide/magentic-one.html)

## Implementation Decision

For this repository, the right move is:

- keep the Implementation Engineer role repo-neutral
- use [`implementation-package-prep`](D:/Projects/orpheum/skills/implementation-package-prep/SKILL.md) as the direct-support skill for implementation packaging work
- reuse the existing synthesis, testing, and handoff skills where they remain the best abstraction
- embed scope discipline, evidence capture, traceability, and boundary control directly in the artifacts, workflows, and checks
- use [`implementation-skill-discovery`](D:/Projects/orpheum/skills/implementation-skill-discovery/SKILL.md) whenever recurring implementation friction needs to be converted into an explicit skill-now versus defer-with-threshold decision
- keep the role focused on disciplined implementation rather than absorbing planning, review, or release responsibilities

## Follow-Up Considerations

If future Implementation Engineer work becomes repetitive or exposes consistent friction, the next likely additions would be:

- a dedicated implementation-review or self-check skill for stronger readiness findings
- a dedicated implementation-evidence skill if repeated projects need richer evidence capture than the current artifacts provide
- a stronger bridge to future Code Reviewer and Release / Handoff Manager roles once those packages exist and real downstream consumers sharpen the contract

The standard trigger for evaluating those additions should be repeated failure or remediation patterns observed during [`implementation-engineer-quality-review.md`](D:/Projects/orpheum/workflows/implementation-engineer-quality-review.md), using [`implementation-skill-discovery`](D:/Projects/orpheum/skills/implementation-skill-discovery/SKILL.md) to turn that repetition into an explicit skill-now versus defer-with-threshold decision.

For now, those concerns are handled adequately by the current artifact chain and existing skills.
