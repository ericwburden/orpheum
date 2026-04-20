# QA / Verification Lead Skill Sourcing

## Purpose

Capture which local skills already support the [`QA / Verification Lead`](D:/Projects/agoge/roles/qa-verification-lead.md) role, which gaps are intentionally being handled inside the first package shape, and which future additions are most likely if real verification work exposes friction.

## Local Skill Support

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing test-session notes, walkthrough notes, review workshops, and verification working sessions before the verification artifacts are drafted.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md)
  - Useful when verification direction depends on external standards, compliance expectations, platform guidance, or comparison research that should be sourced explicitly.

### Useful With Existing Local-Markdown Positioning

- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md)
  - Useful for synthesizing local requirements, architecture, planning, implementation, evidence, and defect context before verification work begins.
- [`requirements-verification`](D:/Projects/agoge/skills/requirements-verification/SKILL.md)
  - Useful as the core local-Markdown verification skill because it already translates upstream evidence into requirement-grounded, testable, verifiable structure and helps separate verified support from assumptions and open questions.
- [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md)
  - Useful for packaging reviewed verification outputs into a downstream-ready handoff without inventing a new verification-handoff skill too early.

### Allium-Aware Support

- [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md)
  - Useful when mature behavioral specifications should inform verification expectations or test coverage direction.
- [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md)
  - Useful when verification confidence depends on checking whether implementation behavior has drifted from a governing specification.

## New Repo-Native Skills Added

None in the first QA / Verification Lead package.

That is an intentional design choice rather than a missing implementation.

The current package is using the existing local-Markdown verification, synthesis, and packaging skills first, with role-specific behavior embedded in:

- the [`QA / Verification Lead`](D:/Projects/agoge/roles/qa-verification-lead.md) role definition
- the QA / Verification Lead artifact set in [`artifacts/`](D:/Projects/agoge/artifacts)
- the QA / Verification Lead workflow set in [`workflows/`](D:/Projects/agoge/workflows)
- the QA / Verification Lead check set in [`checks/`](D:/Projects/agoge/checks)

## Why Existing Skills Were Enough For V1

### `requirements-verification` already covers the main evidence-grounding method

The QA / Verification Lead role needed a repeatable way to turn reviewed requirements, architecture, planning, and implementation context into:

- a verification posture
- explicit coverage expectations
- evidence-based confidence judgments
- gap identification
- remediation routing

`requirements-verification` already provides most of the underlying method for separating verified support from assumptions, non-support, and open questions in local Markdown form.

The role package therefore adapts that skill into a tighter role-specific operating model instead of adding a new verification skill before real usage proves the gap.

### Verification review can be expressed through artifacts and checks first

The main new requirement was not "run tests" in the abstract. It was:

- define what evidence should count
- connect evidence back to requirements, architecture, and planning
- review evidence explicitly
- preserve role boundaries
- package the outcome for downstream consumers

Those concerns are being handled in v1 through:

- [`verification-matrix.md`](D:/Projects/agoge/artifacts/verification-matrix.md)
- [`evidence-review.md`](D:/Projects/agoge/artifacts/evidence-review.md)
- [`verification-traceability.check.md`](D:/Projects/agoge/checks/verification-traceability.check.md)
- [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md)

That keeps the package lean while still making the role operational.

### A dedicated verification-handoff skill would be premature duplication

The existing [`handoff-packaging`](D:/Projects/agoge/skills/handoff-packaging/SKILL.md) skill already expresses the right downstream packaging posture:

- summarize what is being handed off
- preserve traceability and evidence context
- separate supported conclusions from open questions
- avoid drifting into implementation or release ownership

The QA / Verification Lead handoff needed different content, not a fundamentally different packaging method.

## Role-Specific Design Decisions

### Keep QA / Verification Lead separate from Implementation Engineer

The package intentionally keeps evidence judgment separate from implementation ownership.

The verification role is allowed to:

- define verification scope
- map coverage
- judge evidence strength
- route remediation

The verification role is not supposed to:

- write production fixes by default
- own test automation authoring as its primary identity
- silently correct weak implementation while calling it verification

This boundary is enforced in the role definition and in [`qa-verification-boundary.check.md`](D:/Projects/agoge/checks/qa-verification-boundary.check.md).

### Keep QA / Verification Lead separate from Release / Handoff Manager

The package intentionally stops short of deployment choreography, production go/no-go operations, and routine release coordination.

The goal is evidence-based readiness framing, not generic release management.

This is why the package emphasizes:

- verification strategy
- coverage and evidence mapping
- review and readiness
- downstream verification handoff

instead of deployment steps, rollback drills, or operational ownership.

### Keep the first package artifact-first

The package was derived from artifact responsibilities first:

- verification strategy
- verification matrix
- evidence review
- verification handoff

The workflows and checks then follow that artifact chain.

This matches the broader repo pattern established by Business Analyst, Solution Architect, and Technical Planner and keeps the role easier to validate and hand off.

## Allium Relationship

The QA / Verification Lead role should treat Allium as an upstream behavioral anchor, not as the format for verification artifacts.

That means:

- consume existing Allium when it materially constrains verification expectations
- use [`propagate`](C:/Users/ericw/.codex/skills/allium/skills/propagate/SKILL.md) when mature specifications should shape verification direction
- use [`weed`](C:/Users/ericw/.codex/skills/allium/skills/weed/SKILL.md) when verification depends on spec-to-implementation alignment
- route unstable behavior back upstream rather than inventing it in verification artifacts

The verification artifacts themselves remain Markdown-first in this repository.

## Implementation Decision

For this repository, the right move is:

- keep the QA / Verification Lead role repo-neutral
- reuse the existing requirement-verification, synthesis, and handoff skills before introducing new dedicated verification skills
- embed evidence review, traceability, and boundary discipline directly in the artifacts, workflows, and checks
- keep the role focused on confidence and readiness framing rather than test execution ownership or release operations

## Follow-Up Considerations

If future QA / Verification Lead work becomes repetitive or exposes consistent friction, the next likely additions would be:

- a dedicated evidence-review skill for stronger readiness judgments and remediation routing
- a dedicated verification-matrix or coverage-mapping skill if repeated projects need richer structured support than the current artifacts provide
- a stronger bridge to implementation or release roles once those packages exist and real downstream consumers sharpen the handoff contract

For now, those concerns are handled adequately by the current artifact chain and existing skills.
