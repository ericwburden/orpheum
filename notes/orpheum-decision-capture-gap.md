# Orpheum Decision Capture Gap

## Context

This note captures product feedback from Codex usage in a consumer project after Orpheum had already been recognized and used.

The feedback is not primarily about onboarding. It is about what Orpheum preserves once scenario-driven work is underway.

## Observation

The current scenario artifacts provide strong structure, but they do not push hard enough on preserving fine-grained architectural decisions and locked constraints by default.

In practice, that means the conversation often remains the real home of important decisions unless the agent deliberately restates them into the artifacts.

The current templates can preserve those decisions, but only if the author consciously overloads nearby sections such as:

- `Verified Requirements`
- `Assumptions And Open Questions`
- general architecture prose

That works as an escape hatch, but not as a reliable default pattern.

The gap is especially visible for maintainer-oriented handoff:

- requirements capture what the system should do
- product direction captures why and what matters now
- architecture artifacts describe the proposed shape
- but there is not yet a strong default pattern for "locked decisions" or "architecture constraints chosen during discovery"

Those decisions are real project control points, but today they can end up smeared across chat context, architecture prose, and handoff language instead of being captured as first-class durable decisions.

## Why It Matters

If Orpheum leaves decision capture implicit, the system risks:

- losing important rationale across long-running agent threads
- forcing downstream maintainers to reconstruct why a constraint exists
- making handoffs look complete while still depending on hidden conversation state
- weakening traceability between upstream choices and downstream implementation constraints

This is especially costly in maintainer-oriented workflows, where the most important thing is often not just the target shape, but which choices are now considered intentionally fixed.

## Candidate Product Direction

Orpheum likely needs a stronger reusable pattern for decision capture that sits adjacent to, but is not identical with:

- requirements specification
- product direction
- solution architecture
- architecture decisions as currently framed

The missing pattern appears to be something closer to a durable "locked decisions and constraints" layer.

For maintainer-oriented discovery and planning, that pattern should probably optimize for compactness, not just completeness. The need is not only "capture every rationale detail," but "make the fixed choices impossible to miss."

## Candidate Shapes

Possible implementations to explore:

1. Strengthen the existing architecture artifacts so they require an explicit compact `Decisions Made` or `Locked Constraints` section with downstream implications.
2. Introduce a dedicated artifact such as `decision-register`, `locked-decisions`, or `architecture-constraints`.
3. Add a scenario-level handoff rule that requires decision capture whenever discovery or planning hardens a constraint that downstream work must not silently revisit.

## Compact Pattern Requirements

If Orpheum adds a built-in decision-capture pattern, it should likely:

- surface the locked choice in a compact, scannable form
- distinguish fixed constraints from open questions
- state why the choice was locked
- state what downstream roles should treat as non-negotiable
- avoid forcing maintainers to reconstruct the decision from surrounding prose

That suggests a small explicit section may be more useful than relying only on longer narrative artifacts.

## Evaluation Questions

Before choosing the shape, validate these questions:

- Is the current `architecture-decisions` artifact conceptually close enough, but under-specified?
- Are discovery-stage constraints different enough from architecture-stage decisions that they need a separate artifact?
- Should locked decisions be scenario-level output, role-level output, or check-enforced sections embedded in existing artifacts?
- Which downstream roles most need this preserved explicitly: planners, implementers, reviewers, or future maintainers?

## Proposed Guidance

- Treat decision capture as a first-class product concern rather than as optional prose quality.
- Prefer durable capture of locked decisions whenever downstream work is expected to preserve a chosen constraint or tradeoff.
- Avoid assuming that a strong conversation thread is an acceptable substitute for durable decision artifacts.
- Prefer an explicit compact pattern such as `Decisions Made` or `Locked Constraints` over relying on overloaded neighboring sections.

## Status

Candidate.

This feedback is strong enough to shape planning, but it still needs a deliberate design choice about whether the right answer is a new artifact, a stronger existing artifact contract, or new checks that force explicit capture.

## Next Step

Use the active planning scenario to decide whether this becomes:

1. a new artifact pattern for locked decisions or architecture constraints
2. an upgrade to the current architecture artifact/check contract
3. both, if the repository needs one lightweight section requirement now and a fuller reusable artifact later
