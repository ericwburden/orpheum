# Orpheum Onboarding And Discoverability Improvement Plan

## Context

This note captures a concrete improvement plan for the rough onboarding seams exposed when Orpheum was used from Codex in another project.

To keep the feedback grounded in actual consumer behavior, the current planning pass explicitly dogfooded the install-style workflow:

- installed the CLI locally with `cargo install --path crates\orpheum-cli --force`
- initialized a fresh temp project with the installed `orpheum` binary
- reproduced the no-catalog onboarding path from outside the Orpheum repo
- validated the active-session and check/status behavior with an applied scenario

This is a planning note and backlog-shaping artifact. If the team wants to lock behavior formally, promote the accepted product decisions into Allium next.

## Observation

The main failure mode is not the scenario model itself. It is the gap between "Orpheum is present" and "an agent can confidently use it now."

The dogfood run reproduced five distinct seams:

1. `orpheum init` leaves a local skill behind, but no repo-root artifact that signals what to do next.
2. A project can be Orpheum-capable without being Orpheum-active, but that distinction is not obvious to an agent scanning the repo.
3. In a consumer repo without catalog configuration, `orpheum scenario list --json` and `orpheum doctor --json` both fail with `CATALOG_NOT_FOUND`, which gives accurate diagnosis but weak recovery guidance.
4. After applying a scenario and passing checks, `orpheum status --json` still reports every artifact as `pending`.
5. Boundary and traceability checks with no `applies_to` surface as `skipped`, which reads more like partial failure than "v1 cannot evaluate this honestly."

## Why It Matters

These seams weaken the exact properties Orpheum is trying to provide:

- discoverability for agents entering an unfamiliar repo
- confidence that the machine-readable state is authoritative
- a clear distinction between onboarding, activation, and active work
- fast recovery when a consumer environment is only partially configured

If those seams remain, agents will continue to miss Orpheum when it is available, misread what state the project is in, or distrust `status` after seeing internal contradictions.

## Product Direction

The next improvement pass should optimize for one explicit promise:

> A new agent entering a consumer repo should be able to tell, within one or two file reads or commands, whether the project is Orpheum-capable, whether it is currently Orpheum-active, and exactly what command to run next.

That promise suggests four workstreams rather than one large refactor.

## Workstream 1: Make `init` leave an agent-visible next step

### Problem

`init` currently installs a useful local skill, but it does not create a strong repo-level signal that Orpheum is now relevant.

### Recommended change

Have `orpheum init` create or refresh a small repo-root hint file such as `ORPHEUM.md`.

Preferred contents:

- whether Orpheum is merely initialized or currently active
- whether a local skill was installed
- whether `.orpheum/` exists
- whether catalog discovery is configured
- the exact next commands to try
- the exact fallback commands when catalog discovery is missing

### Why this is the preferred shape

- repo-root placement makes it visible to both humans and agents
- it cleanly distinguishes "capable" from "active"
- it avoids creating `.orpheum/` prematurely
- it gives `init` a visible durable outcome beyond a hidden skill file

### Acceptance criteria

- after `orpheum init`, a fresh consumer repo contains an agent-visible next-step artifact at the repo root
- the artifact explains that `init` does not create an active session
- the artifact includes command examples for both auto-discovered and explicit-catalog usage

## Workstream 2: Make catalog recovery obvious when discovery fails

### Problem

`CATALOG_NOT_FOUND` is correct but not recovery-oriented, and the current `doctor` path still depends on catalog loading.

### Recommended change

Improve the no-catalog experience in two layers:

1. Expand the `CATALOG_NOT_FOUND` message so it prints the next commands to try.
2. Make `doctor` capable of running a preflight mode without a resolved catalog, so it can diagnose discovery state instead of failing at the same boundary.

Preferred recovery hints:

- show `--catalog <path>`
- show `ORPHEUM_CATALOG=<path>`
- mention that discovery works automatically from the Orpheum repo itself
- if likely paths are detectable, print them as suggestions rather than silently probing

### Acceptance criteria

- a consumer repo with no catalog configured gets a recovery-oriented error, not just a terminal code
- `orpheum doctor --json` can report catalog-discovery problems without requiring the catalog to be already resolved
- tests cover the fresh-consumer-repo failure path

## Workstream 3: Make project state legible as capable vs active

### Problem

Today the main visible distinction is hidden in whether `.orpheum/` exists, which is easy to miss operationally.

### Recommended change

Treat capability and activity as separate first-class states in the product language:

- `initialized`: local skill and onboarding hint exist
- `active`: `.orpheum/` session exists
- `misconfigured`: initialized, but catalog-dependent commands cannot resolve the catalog

Expose those states consistently in:

- the repo-root hint file
- `init --json`
- `doctor --json`

### Acceptance criteria

- docs and command output stop implying that `init` means "ready session"
- machine-readable output can distinguish initialized, active, and misconfigured states
- the agent contract in the installed skill mirrors the same language

## Workstream 4: Restore trust in `status` as the authoritative view

### Problem

The current session state mixes two different stories:

- checks can pass
- artifact status still remains `pending`

That contradiction makes the authoritative state look unreliable.

### Recommended change

Update state refresh logic so artifact status is derived or refreshed from observable evidence, not left permanently at apply-time defaults.

Minimum viable rule:

- if the artifact file exists and its associated checks pass, artifact status should no longer remain `pending`

Preferred rule:

- represent artifact state explicitly with typed values such as `pending`, `present`, `verified`, and `failed`

### Acceptance criteria

- a scenario with copied expected artifacts and passing checks no longer reports all artifacts as `pending`
- `status --json` and `.orpheum/state.json` tell a consistent story after `check run`
- tests cover the before/after state transition

## Workstream 5: Replace misleading `skipped` semantics for v1-unevaluable checks

### Problem

Checks with no `applies_to` currently land in `skipped`, which reads like incomplete coverage rather than deliberate non-evaluability in v1.

### Recommended change

Introduce a clearer surfaced state for checks that v1 cannot evaluate honestly.

Preferred label:

- `not_evaluable_in_v1`

If a new serialized enum value feels too disruptive for the immediate pass, use one of these transitional options:

- keep the underlying enum but change the human-facing message
- add a `reason` or `evaluation_mode` field so consumers can distinguish intentional limitation from ordinary skipping

### Acceptance criteria

- boundary-style checks no longer look like accidental skips
- the CLI explains whether the check was intentionally unevaluated because v1 lacks the necessary runner
- docs and JSON remain explicit about the limitation

## Backlog Order

Recommended implementation order:

1. catalog recovery and non-blocking doctor improvements
2. repo-root onboarding hint from `init`
3. capable-versus-active state language across docs and JSON
4. artifact status reconciliation after checks
5. clearer semantics for currently unevaluable checks
6. host-environment follow-up to ensure local project skills also appear in the advertised skill registry

The first five are Orpheum-owned. The registry item likely requires coordination with the host environment, but it should still be tracked because it directly affected adoption.

## Evidence

Dogfood results from this planning pass:

- In a fresh temp repo, `orpheum init` succeeded, installed `.codex/skills/orpheum/SKILL.md`, and updated `.gitignore`, but left no repo-root next-step artifact.
- In that same repo, `orpheum scenario list --json` failed with `CATALOG_NOT_FOUND: unable to locate the Orpheum catalog at runtime; pass --catalog <path> or set ORPHEUM_CATALOG`.
- `orpheum doctor --json` failed with the same error instead of helping recover from it.
- After applying `project-discovery`, copying the expected artifact templates, and running `orpheum check run --json`, the check summary reported many `passed` results while `orpheum status --json` still reported every artifact as `pending`.
- Boundary-oriented checks with no `applies_to` surfaced as `skipped` with the message `check has no applies_to artifact; skipped by v1 runner`.

## Status

Candidate.

The evidence is strong enough to shape an implementation backlog now, but the exact product contract for the repo-root hint file and the serialized wording for unevaluable checks should still be chosen deliberately before coding.

## Next Step

Turn this note into a small issue set or implementation sequence:

1. add a product issue for catalog-recovery and doctor preflight behavior
2. add a product issue for the repo-root onboarding hint written by `init`
3. add a state-model issue for artifact-status reconciliation after `check run`
4. add a terminology issue for `skipped` versus a clearer v1 limitation state
5. coordinate with the host environment on local-skill registry parity so installed project skills are also advertised to the agent
