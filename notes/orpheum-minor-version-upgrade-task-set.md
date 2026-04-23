# Orpheum Minor-Version Upgrade Task Set

## Purpose

Turn the recent product-feedback notes into a concrete, reviewable implementation sequence for the next minor Orpheum release.

This task set is intentionally product-shaped rather than codebase-shaped. The goal is to make the next minor version materially better for consumer-project adoption, decision preservation, and human-in-the-loop scenario quality.

## Planning Basis

This task set consolidates these planning inputs:

- [orpheum-onboarding-discoverability-plan.md](C:/Users/ericw/Projects/orpheum/notes/orpheum-onboarding-discoverability-plan.md)
- [orpheum-decision-capture-gap.md](C:/Users/ericw/Projects/orpheum/notes/orpheum-decision-capture-gap.md)
- [orpheum-semantic-review-phase.md](C:/Users/ericw/Projects/orpheum/notes/orpheum-semantic-review-phase.md)

## Release Goal

The next minor version should make Orpheum:

- easier for agents to discover and activate correctly in consumer repos
- more trustworthy as an authoritative status surface
- better at preserving locked decisions and architecture constraints
- stronger at treating human semantic review as part of scenario completion rather than an optional extra

## Recommended Delivery Order

1. onboarding and recovery hardening
2. authoritative status and check-semantics hardening
3. locked-decision capture contract
4. semantic-review phase design
5. cross-artifact decision-quality closeout

This order improves baseline usability first, then addresses the higher-level scenario and artifact-model gaps.

## Task 1: Add an agent-visible onboarding hint and recovery path

### Problem

`orpheum init` currently makes a project Orpheum-capable, but it does not leave a strong repo-level signal about what to do next or how to recover when catalog discovery is missing.

### Scope

- make `init` create or refresh a repo-root onboarding artifact such as `ORPHEUM.md`
- expand `CATALOG_NOT_FOUND` recovery guidance
- let `doctor` report discovery problems without requiring full catalog resolution first

### Expected outcome

A fresh agent entering a consumer repo can tell whether the repo is initialized, active, or misconfigured, and can see the exact next commands to try.

### Acceptance criteria

- `orpheum init` leaves an agent-visible next-step artifact
- `CATALOG_NOT_FOUND` prints concrete recovery commands
- `doctor --json` can diagnose discovery state without already having a resolved catalog

## Task 2: Make `status` internally trustworthy after checks run

### Problem

The current state model can report passed checks while leaving artifact state at `pending`, which makes `status` feel inconsistent.

### Scope

- reconcile artifact status after `check run`
- distinguish genuine pending state from present or verified state
- clarify the surfaced meaning of currently unevaluable checks

### Expected outcome

`status --json` tells one consistent story about artifact progress, check progress, and known limitations.

### Acceptance criteria

- passing checks do not leave all related artifacts at `pending`
- artifact and check states remain compatible after state refresh
- unevaluable checks do not look like accidental or low-information skips

## Task 3: Add a compact locked-decision capture pattern

### Problem

Orpheum artifacts can preserve decisions, but only if authors deliberately overload nearby sections. That is too fragile for maintainer-oriented discovery and planning.

### Scope

- choose whether the product shape is a new artifact, a stronger `architecture-decisions` contract, or both
- define a compact built-in pattern such as `Decisions Made` or `Locked Constraints`
- ensure the pattern distinguishes fixed choices from open questions

### Expected outcome

Important architecture constraints and locked choices become hard to miss and do not stay trapped in chat context.

### Acceptance criteria

- at least one core artifact path has an explicit compact decision-capture pattern
- the pattern explains what is fixed, why it was fixed, and what downstream roles must preserve
- the repository no longer relies on `Verified Requirements` or `Assumptions And Open Questions` as the default escape hatch for locked decisions

## Task 4: Add a first-class semantic-review phase for architecture-heavy work

### Problem

Generation plus structural checks can produce polished but directionally wrong discovery or planning packages when human semantic review has not yet happened.

### Scope

- design a reusable scenario phase such as `semantic-review` or `artifact-tightening`
- define when this phase is required or strongly recommended
- encode the core review questions around end-state correctness, package-boundary drift, unwanted architecture invention, and decision preservation

### Expected outcome

Human artifact-by-artifact review becomes part of the scenario contract when the main risk is preserving the wrong architecture rather than missing headings.

### Acceptance criteria

- discovery and planning scenarios gain an explicit semantic-review or equivalent closeout step
- the phase is framed as decision-quality review, not proofreading
- the review loop includes cross-artifact consistency follow-up after revisions

## Task 5: Strengthen scenario closeout around decision quality

### Problem

Current scenario completion can feel too close to "artifacts exist and checks pass" instead of "the package is directionally trustworthy."

### Scope

- add a stronger distinction between structural completeness and decision-quality review
- update scenario exit conditions, handoff language, or review artifacts to reflect that distinction
- make semantic-review outcomes become durable decisions rather than conversational leftovers

### Expected outcome

Scenario closure becomes harder to claim unless the package is both structurally complete and semantically reviewed to the degree the work requires.

### Acceptance criteria

- at least one scenario path distinguishes `structurally complete` from `decision-quality reviewed`
- scenario handoff or review artifacts reflect semantic corrections explicitly
- revised artifacts preserve the decisions surfaced during human review

## Suggested Implementation Shape

To keep the minor version bounded, prefer this rollout shape:

1. CLI and state-model changes first
2. artifact-contract changes second
3. scenario and closeout model changes third

This reduces the risk of mixing low-level CLI stabilization with higher-level scenario redesign in the same implementation slice.

## Suggested GitHub Issue Breakdown

If you turn this into issues, a clean first pass would be:

1. add repo-root onboarding hint and recovery-oriented doctor behavior
2. reconcile artifact state after checks and clarify unevaluable-check semantics
3. add compact locked-decision capture to architecture-oriented artifacts
4. design semantic-review phase for discovery and planning scenarios
5. update scenario closeout to distinguish structural completeness from decision-quality review

## Recommended Next Step

Use the active `scenario-implementation` session to decide which of these tasks require:

- new artifacts
- check changes
- scenario-definition changes
- role-package support changes

Then turn this task set into the first concrete implementation slice for the minor release.
