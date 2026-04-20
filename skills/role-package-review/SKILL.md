---
name: role-package-review
description: Review reusable agent role packages for coherence, readiness, and remediation needs; use when assessing a role definition together with its artifacts, workflows, checks, and supporting skills in order to produce an explicit review record, identify structural gaps, and determine whether the package is ready, conditionally ready, or not ready in a local Markdown workflow.
---

# Role Package Review

Review a reusable role package as a package, not as a collection of isolated files. The goal is to produce an explicit, defensible readiness assessment and route remediation to the correct earlier stage.

For this repository's Role Builder workflows, this is the preferred default path for final package review and readiness assessment. Use it after the role definition and support-system design both exist.

In this repository, this skill is the core review method behind the named `Role Builder hardening pass`.

## Quick start

1) Read the role-definition and support-system artifacts together.
2) Check whether the artifacts, workflows, checks, and skill support align to the same role shape.
3) Identify structural gaps, duplication, broken traceability, and role drift.
4) Record findings, remediation decisions, readiness, and remaining risks explicitly.
5) Route remediation to the earliest artifact or design decision that introduced the defect.

## Trigger phrase

Treat the phrase `Role Builder hardening pass` as an explicit request to:

1) review the role package as a chain
2) identify coherence gaps, broken traceability, and role drift
3) record findings and remediation explicitly
4) support the required Role Builder quality-review checks

## Workflow

### 1) Gather the review inputs
- Start with the local role-definition brief and role-support-system artifact.
- Pull in related workflows, checks, supporting skills, and prior notes only as needed to assess coherence honestly.
- Prefer local role-package artifacts over chat memory or implied conventions.

### 2) Review the package as a chain
- Check whether the role's job-to-be-done, outputs, boundaries, workflows, checks, and skills point to the same role shape.
- Look for:
  - missing support elements
  - duplicated or overlapping structure
  - broken traceability
  - role drift
  - hidden conventions
- Treat the package as not ready if key support is implied but not actually present.

### 3) Classify findings
- Record findings by defect type:
  - weak role definition
  - incomplete support system
  - missing or weak checks
  - broken traceability
  - unsupported tool assumptions
  - package overhead or role drift
- Keep findings concrete enough that another designer could act on them.

### 4) Route remediation
- Route each defect to the earliest artifact or design decision that introduced it.
- Do not patch the review artifact to hide an upstream problem.
- If failures trace back to role definition, route remediation there.
- If failures trace back to support-system design, route remediation there instead of declaring the package conditionally acceptable without support.

### 5) Produce the review artifact
- Populate the local role-package-review artifact.
- Ensure it includes:
  - role in scope
  - reviewed inputs
  - review findings
  - remediation decisions
  - readiness assessment
  - remaining risks
  - recommended next steps

### 6) Finalize the readiness judgment
- Mark the package as:
  - **ready** when the support package is coherent and sufficient
  - **conditionally ready** when the package is coherent but intentionally limited and the limits are explicit
  - **not ready** when support gaps, broken traceability, or role drift materially weaken adoption
- Keep the reasoning explicit. Avoid vague approval language.

## Notes

- This skill is for coherence review and readiness assessment, not for redefining the role from scratch or redesigning the entire support system in place.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when findings depend on synthesizing multiple local sources or prior notes before the review can be completed cleanly.
- In this repository, the normal output is an instantiated copy of [`artifacts/role-package-review.md`](D:/Projects/agoge/artifacts/role-package-review.md).
