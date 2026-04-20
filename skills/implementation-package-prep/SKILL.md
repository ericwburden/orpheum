---
name: implementation-package-prep
description: Prepare a downstream-ready Implementation Engineer package from reviewed planning and concrete code changes; use when implementation work needs explicit change inventory, evidence capture, implementation-package self-review, and handoff preparation without drifting into planning, independent code review, or QA authority.
---

# Implementation Package Prep

Turn an implementation slice plus its local validation evidence into a coherent Implementation Engineer package that downstream review and verification roles can use without reconstructing the change from diffs, terminal output, or chat history.

For this repository, this is the preferred direct-support skill for the Implementation Engineer role when the work needs explicit implementation record, evidence capture, implementation-package self-review, and downstream handoff preparation.

## Quick start

1. Read the reviewed implementation handoff and upstream architecture or requirements context.
2. Identify the actual implementation boundary and concrete change footprint.
3. Capture local validation evidence with honest provenance and limits.
4. Prepare the implementation-package self-review without drifting into independent review.
5. Package the results for downstream code review, verification, or release-adjacent use.

## Workflow

### 1) Read the implementation context

- Start with the reviewed implementation handoff.
- Pull in architecture, requirements, specifications, and implementation notes only as needed to clarify the approved slice.
- Keep upstream ambiguity explicit instead of compensating for it silently.

### 2) Build the implementation record

- State what implementation slice was attempted and why.
- Make the change boundary explicit.
- Record planned versus actual scope honestly.
- Build a concrete change inventory that names the files, modules, configs, schemas, prompts, or assets that changed.
- Call out interface, schema, contract, migration, or configuration effects that downstream roles need to notice.

### 3) Capture implementation evidence

- List the local validation activities attempted.
- Record commands, procedures, or manual paths when that detail materially affects reproducibility.
- Distinguish passes, failures, partial results, skipped checks, and inconclusive outcomes.
- Preserve revision and environment provenance.
- Make evidence gaps and confidence limits explicit.

### 4) Prepare the implementation-package self-review

- Judge whether the package is honest, coherent, and downstream-usable.
- Keep this at implementation-package self-review level.
- Do not drift into independent code review findings or final QA verdicts.
- Separate implementation defects, weak evidence, upstream ambiguity, and downstream watchouts.

### 5) Prepare the downstream handoff

- Summarize implemented scope, change footprint, evidence posture, self-review status, known issues, and revalidation triggers.
- Keep the handoff usable by downstream roles without retelling the entire implementation history.
- Preserve limitations instead of smoothing them over.

## Guardrails

- Do not redefine requirements, architecture, or implementation planning.
- Do not present local validation as though it were full downstream verification.
- Do not present implementation-package self-review as though it were independent code review.
- Do not hide scope expansion, skipped checks, weak evidence, or residual risks.

## Supporting skills

- Use [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) when the implementation context or evidence is spread across multiple local sources.
- Use [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md) when the main weakness is the clarity of the downstream implementation handoff.
- Use [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md) when browser-based or web-application evidence needs to be collected or strengthened.
- Use Allium skills only when governing behavioral specifications materially constrain the implementation or drift checking.

## Outputs

This skill should strengthen or help populate:

- [`implementation-record.md`](D:/Projects/orpheum/artifacts/implementation-record.md)
- [`implementation-evidence.md`](D:/Projects/orpheum/artifacts/implementation-evidence.md)
- [`implementation-readiness-review.md`](D:/Projects/orpheum/artifacts/implementation-readiness-review.md)
- [`implementation-package-handoff.md`](D:/Projects/orpheum/artifacts/implementation-package-handoff.md)

## Notes

- This skill is intentionally narrow. It supports Implementation Engineer packaging work after implementation has happened.
- It is not a generic coding skill, not a planning skill, and not a substitute for future Code Reviewer or QA / Verification Lead work.
