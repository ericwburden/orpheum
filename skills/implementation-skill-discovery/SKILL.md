---
name: implementation-skill-discovery
description: Identify when Implementation Engineer work has enough repeated friction to justify a dedicated implementation skill and define what that future skill should contain; use when reviewing recurring implementation-package problems, evidence-capture pain, repeated workaround patterns, or uncertainty about whether to add a repo-native implementation skill.
---

# Implementation Skill Discovery

Turn recurring Implementation Engineer friction into an explicit decision about whether a dedicated implementation skill should be added and, if so, what that skill should contain.

Use this skill to prevent "we can specify it later" from becoming indefinite drift. The output should be concrete enough that a later skill-creation pass can act on it without rediscovering the problem.

## Quick start

1. Gather concrete Implementation Engineer examples, not just opinions.
2. Classify the repeated friction precisely.
3. Decide whether the problem belongs in a new skill, an artifact change, a workflow change, a check change, or a role-boundary clarification.
4. If a skill is warranted, define its narrow job, triggers, inputs, outputs, and likely bundled resources.
5. If a skill is not yet warranted, record the evidence threshold that would justify it later.

## Workflow

### 1) Gather evidence

- Start with actual Implementation Engineer materials when available:
  - implementation records
  - implementation evidence artifacts
  - implementation readiness reviews
  - implementation package handoffs
  - failed or weak Implementation Engineer checks
  - implementation-session notes
  - repeated user requests or remediation patterns
- Prefer multiple concrete examples over a single anecdote.
- Use [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md) if the evidence is spread across many local files.

### 2) Classify the friction

Classify the repeated problem before proposing a skill.

Use categories like:

- implementation execution gap
- evidence capture gap
- implementation-package self-review gap
- downstream handoff gap
- role-boundary confusion
- missing tool guidance
- unsupported external dependency pattern

Do not default to "new skill needed" before the problem is clearly typed.

### 3) Decide whether a skill is the right fix

Ask:

- Is this a repeated procedural gap rather than a one-off mistake?
- Does the same reasoning or structure have to be recreated across multiple implementation passes?
- Would a reusable instruction bundle help more than a role-definition tweak, artifact tweak, workflow tweak, or check tweak?
- Can the future skill stay narrow without absorbing planning, review, or verification responsibilities?

If the answer is no, recommend the smaller package change instead of inventing a new skill.

### 4) Define the future skill candidate

If a dedicated skill is warranted, define:

- proposed skill name
- exact job-to-be-done
- trigger conditions
- what the skill should not own
- required inputs
- expected outputs
- likely bundled resources:
  - scripts
  - references
  - assets
- validation or forward-testing expectations
- dependency relationship to existing skills such as:
  - [`research-documentation`](D:/Projects/orpheum/skills/research-documentation/SKILL.md)
  - [`handoff-packaging`](D:/Projects/orpheum/skills/handoff-packaging/SKILL.md)
  - [`webapp-testing`](D:/Projects/orpheum/skills/webapp-testing/SKILL.md)
  - Allium skills when specification drift or constraint interpretation is involved

Keep the proposed skill as narrow as possible.

### 5) Record the no-skill-yet outcome explicitly

If a skill is not yet justified, do not stop at "not now."

Record:

- why the current package is still sufficient
- what repeated evidence would change the decision
- what candidate skill seams are most plausible
- what artifact, workflow, or check changes should happen first instead

This turns deferral into an explicit threshold rather than an indefinite maybe.

## Output shape

Produce a short decision memo in local Markdown that includes:

- decision summary
- evidence reviewed
- friction classification
- recommendation:
  - add a skill now
  - defer skill, with threshold
  - fix package structure instead
- if adding a skill now:
  - proposed name
  - narrow scope
  - non-goals
  - likely resources
  - follow-on creation steps

## Notes

- This skill is for discovering and specifying future implementation-skill details, not for performing implementation work itself.
- Prefer concrete local evidence over intuition.
- Keep future skill recommendations narrow so they do not collide with Technical Planner, Code Reviewer, QA / Verification Lead, or Release / Handoff Manager responsibilities.
