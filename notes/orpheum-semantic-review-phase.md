# Orpheum Semantic Review Phase Gap

## Context

This note captures product feedback from a Codex-driven discovery session where Orpheum successfully produced durable artifacts, explicit workflow structure, and passing structural checks, but the highest-value corrections still emerged during interactive artifact-by-artifact review with the human maintainer.

The feedback is not that Orpheum's current generation or validation model is bad. It is that those mechanisms alone are not sufficient for architecture-heavy discovery and planning work.

## Observation

The session produced a structurally complete discovery package early, and that package could satisfy the current Orpheum framing of artifact completion and checks.

Even so, several materially important assumptions and design choices were still directionally wrong or overfit until the human maintainer reviewed the artifacts section by section.

Examples from the session:

- the package drifted too far toward platform or runner-centric framing
- the intended component model became over-fragmented
- the importance of version-aware APIs at the business-task layer was underrepresented
- the shift from `submission_name` to a richer `submission_period` contract was initially missed
- the maintainer's intended package boundary for a task like `mdcps.msa.32a` was not preserved strongly enough

Those corrections did not emerge from heading completion or structural checks. They emerged from semantic review with the human in the loop.

## Why It Matters

This exposes a product gap in how Orpheum currently signals scenario completion.

Today, Orpheum strongly reinforces:

- durable artifact generation
- explicit workflow phases
- structural validation

But that is not enough when the main failure mode is "the artifacts preserve the wrong architecture" rather than "the artifacts are missing sections."

If Orpheum treats generation plus structural validation as near-sufficient, the system risks:

- declaring discovery or planning complete too early
- preserving polished but directionally wrong architecture
- missing package-boundary drift until downstream implementation pain appears
- letting important human corrections remain conversational instead of becoming durable decisions

## Product Implication

Orpheum should treat human review of generated artifacts as a first-class scenario step, not as an optional courtesy after the real work is done.

For discovery and planning scenarios especially, the system likely needs an explicit semantic-review or artifact-tightening phase before closure.

That phase should be framed as decision-quality review, not as proofreading.

Core questions for that phase:

- Does this artifact reflect the intended end state?
- Did the package boundaries drift?
- Are we accidentally introducing architecture the human did not ask for?
- Are the artifacts preserving decisions, or just sounding complete?
- Did changes in one artifact create contradictions elsewhere?

## Candidate Product Direction

Add an explicit review phase for scenario completion in architecture-heavy or abstraction-heavy work.

Working names to evaluate:

- `artifact-tightening`
- `semantic-review`
- `decision-quality review`
- `human artifact review`

The exact label matters less than the contract:

- each artifact is reviewed for semantic correctness, not just structure
- corrections are captured as durable decisions
- cross-artifact consistency is re-checked after revision
- only then does the scenario move to planning handoff or closure

## Candidate Shapes

Possible implementations to explore:

1. Add a reusable scenario phase for human-reviewed artifact tightening before discovery or planning closeout.
2. Add a closeout checklist requiring artifact-by-artifact semantic review when the scenario introduces new abstractions, architecture boundaries, or significant packaging decisions.
3. Add check or handoff language that distinguishes `structurally complete` from `decision-quality reviewed`.
4. Add a dedicated review artifact that records semantic corrections, changed decisions, and cross-artifact reconciliations before handoff.

## Relationship To Decision Capture

This gap is closely related to the locked-decisions problem, but it is not identical.

- The decision-capture gap is about preserving fixed choices durably.
- This semantic-review gap is about creating the conditions where the right choices are surfaced and corrected before they are preserved.

In the session that prompted this feedback, the human-in-the-loop artifact walk created much of the decision value. That suggests Orpheum should not treat semantic review and decision capture as separate afterthoughts.

## Evaluation Questions

Before choosing a product shape, validate these questions:

- Should semantic review be mandatory for discovery and planning scenarios, or conditional based on architecture intensity?
- What are the best triggers for requiring artifact-by-artifact human review?
- How should Orpheum represent "structurally complete but semantically provisional" state?
- Should the review output live in revised artifacts only, or also in a dedicated reconciliation artifact?
- How should scenario exit conditions reflect decision-quality review rather than only artifact presence and structural checks?

## Proposed Guidance

- Treat human review of generated artifacts as a normal scenario step when architecture, abstraction boundaries, or product-shaping choices are still settling.
- Do not let passing structural checks imply semantic correctness.
- Encourage artifact-by-artifact walkthroughs with the human when the biggest project risk is preserving the wrong architecture rather than missing headings.
- Capture corrections from semantic review as durable decisions, not only as conversational amendments.
- Prefer a scenario contract that distinguishes "artifact exists" from "artifact is directionally trustworthy."

## Status

Candidate.

The feedback is strong and concrete enough to shape scenario design, but the repository still needs to decide whether this becomes a new reusable phase, a stronger closeout checklist, a new artifact, or some combination of those mechanisms.

## Next Step

Use the active planning scenario to decide whether Orpheum should:

1. add a first-class semantic-review phase to discovery and planning scenarios
2. add explicit exit criteria around human-reviewed decision quality
3. add stronger decision-capture support so semantic review outcomes become durable by default
