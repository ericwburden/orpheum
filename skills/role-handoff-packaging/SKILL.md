---
name: role-handoff-packaging
description: Package a reviewed reusable agent role for downstream adoption; use when a role definition, support system, and review outcome already exist and need to be turned into a clear adoption-facing handoff with summary, consumers, limitations, preserved conventions, and next steps in a local Markdown workflow.
---

# Role Handoff Packaging

Turn a reviewed role package into an adoption-facing handoff that another repo, team, or downstream designer can use without reconstructing the package from earlier artifacts or chat context.

For this repository's Role Builder workflows, this is the preferred default path for the final adoption-packaging stage after quality review has passed.

## Quick start

1) Start with the role definition, support system, and reviewed package outcome.
2) Confirm the package is actually ready or conditionally ready for adoption.
3) Summarize what the role does, what the support package includes, and what conventions adopters need to preserve.
4) State limitations, intended consumers, and next adoption steps explicitly.
5) Populate the role-package-handoff artifact without inventing missing support or overstating readiness.

## Workflow

### 1) Gather the reviewed package
- Start with the local role-definition brief, role-support-system artifact, and role-package-review artifact.
- Use the review artifact as the readiness source of truth.
- Pull in supporting notes only when they materially affect adoption guidance.

### 2) Confirm the package is handoff-ready
- Verify the package is marked ready or conditionally ready with explicit limits.
- If the review says not ready, route back to remediation instead of producing a polished handoff for an unready package.
- Preserve known limits instead of smoothing them over.

### 3) Identify what adopters need to know
- Summarize:
  - the role's job-to-be-done
  - the core artifact, workflow, check, and skill support
  - who should adopt the role
  - what tooling or local-first conventions matter
  - what the package does not include
- Keep adoption guidance focused on operational use, not on retelling every design decision.

### 4) Produce the handoff artifact
- Populate the local role-package-handoff artifact.
- Ensure it includes:
  - role in scope
  - package summary
  - reviewed inputs
  - review outcome
  - adoption guidance
  - recommended consumers
  - current limitations
  - required conventions and preferences
  - next adoption steps

### 5) Finalize without overstating readiness
- Keep the handoff honest about remaining risks and limitations.
- Avoid inventing missing workflows, checks, or support elements just to make the package look complete.
- Make it easy for a downstream adopter to know whether to use the role now, adapt it first, or wait for more refinement.

## Notes

- This skill is for adoption packaging, not for defining the role from scratch or re-reviewing the full package.
- Use [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md) if readiness is still ambiguous.
- Use [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md) when the handoff still depends on synthesizing multiple local notes before it can be packaged cleanly.
- In this repository, the normal output is an instantiated copy of [`artifacts/role-package-handoff.md`](D:/Projects/agoge/artifacts/role-package-handoff.md).
