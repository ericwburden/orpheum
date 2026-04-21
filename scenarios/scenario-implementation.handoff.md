# Scenario Implementation Scenario Handoff

## Purpose

Package the reusable `Scenario Implementation` scenario for downstream adopters, workflow authors, or project leads so they can implement new scenarios without reconstructing the design loop from branch history or informal practice.

## Current Scenario Summary

`Scenario Implementation` is a reusable multi-role scenario that takes a selected scenario idea, drafts the scenario package, verifies that the participant roles can actually support it, runs an explicit skills-review and hardening loop when needed, then produces a reviewed scenario package for downstream adoption.

Primary participating roles:

- `Scenario Designer`
- `Role Builder`
- optional `Security / Compliance Specialist`

## Scenario Package Included

- [scenario-implementation.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.definition.md)
- [scenario-implementation.integration-map.md](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.integration-map.md)
- [scenario-implementation.review.md](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.review.md)

## Current Readiness Posture

`conditional`

This scenario is packaged for downstream adoption and local tailoring, but its final downstream-ready posture still depends on completion of the Scenario Designer quality review across the full package.

It becomes `ready` only when that quality review passes across the completed package and does not identify a remaining defect that still needs upstream remediation before reuse.

Limits:

- it is not a general meta-process for designing any repository change
- it does not replace role-local workflows
- it does not silently create new roles when the actual blocker is a missing role package
- optional security/compliance participation should remain conditional rather than automatic
- the handoff is part of the completed package, but it does not by itself replace the final Scenario Designer quality gate
- repeated hardening is acceptable before that gate, but only while each pass still yields substantive package improvement
- implementation-loop guidance discovered here should stay in this scenario package unless the target scenario's normal operating use genuinely needs the same rule

## Role And Workflow Routing Guidance

Use the scenario in this broad order:

1. Scenario Designer composition to draft the new scenario package
2. Role Builder hardening review of each named participant package and its scenario-relevant workflows, checks, artifacts, and skills
3. Role Builder role-definition or support-system artifact refresh for any participant package that needs those artifacts to satisfy the formal hardening-review inputs honestly
4. Role Builder support-system changes, including skills review and any resulting skill addition or implementation, for each participant package whose support is not yet sufficient
5. Role Builder handoff packaging for any participant package whose conventions or limits should remain visible in the final scenario-readiness story
6. Scenario Designer review of the now-hardened scenario and participant chain
7. optional Security / Compliance Specialist scoping, review, and handoff when the scenario itself introduces trust-boundary-sensitive or approval-sensitive concerns
8. Decide whether the package has converged enough that another hardening pass would be mostly restatement, minor wording cleanup, or cosmetic restructuring rather than a substantive improvement
9. Scenario Designer handoff for downstream adoption and tailoring
10. Scenario Designer quality review of the completed scenario package, including the handoff artifact, before the package is treated as ready for downstream use

Downstream consumers should preserve:

- explicit participant-role fit review before declaring the scenario ready
- explicit skills sufficiency decisions rather than passive skill inventory
- the distinction between scenario composition and role-package hardening
- the rule that existing participant packages should enter through Role Builder hardening review first, with support-system changes applied only when needed
- the rule that role-definition or support-system artifacts may still need to be refreshed when the formal hardening review depends on them
- the rule that participant-fit and hardening judgments should stay package-specific rather than collapsing into one blended review
- the rule that participant-package review or hardening outcomes should remain visible when they materially explain why the final scenario is ready
- the option to preserve participant-package adoption limits or conventions through `role-builder-handoff.md` when those limits materially shape the final scenario-readiness story
- the rule that convergence should be judged before the scenario handoff is treated as the final reviewable package state
- the rule that the scenario handoff artifact must exist before the Scenario Designer quality review can honestly run to completion
- the rule that repeated hardening should stop once additional passes are no longer producing substantive package improvements
- the rule that the package should be promoted from `conditional` to `ready` only after the final Scenario Designer check chain passes with no unresolved blocking defect
- the rule that implementation-loop stop guidance should not be copied into target scenarios by default
- the rule that a genuinely missing role should block or reroute the scenario rather than be hidden inside scenario prose
- the optional nature of the security/compliance branch unless local context requires it

## Entry Conditions For The Next Consumer

Before using this scenario, the next consumer should confirm:

- the target work is actually a reusable scenario worth implementing
- the likely participant role packages are available for review
- the repository is willing to harden participant roles and skills as part of scenario implementation when needed
- the local context does not require broader governance or delivery-management machinery than this scenario is designed to provide

## Active Conditions, Risks, And Watchouts

- avoid treating scenario drafting alone as if it proves participant-role readiness
- avoid treating skills review as a documentation exercise with no sufficiency decision
- avoid treating support-system design as the default first step when the real job is reviewing and hardening an existing participant package
- avoid assuming a hardening review can be honest when the role-definition or support-system artifacts it formally depends on are missing or stale
- avoid using repeated hardening passes to hide the fact that a required role does not yet exist
- avoid collapsing participant-package hardening results into one soft final readiness claim with no preserved evidence trail
- avoid dropping package-level adoption limits or conventions when those are part of why the final scenario is honestly ready
- avoid letting the scenario become recursively process-heavy
- avoid treating every self-application pass as equally necessary once the package has already stopped improving in a meaningful way
- avoid invoking the security/compliance branch without a real trigger

## Follow-Up Owners

- Scenario Designer for future scenario-level hardening
- Role Builder if repeated usage exposes missing participant-role support patterns

## Revisit Triggers

- repeated usage shows inconsistent skills-review outcomes
- repeated usage shows frequent blocks on missing roles
- repeated usage shows that participant hardening needs sharper scenario-specific routing
- a future repository convention adds stronger scenario-implementation artifacts or quality gates

## Recommended Next Consumer

- `Scenario Designer`
  - to run the final quality-review gate across the completed package before treating this scenario as fully ready for downstream reuse, and then to reuse the scenario when implementing other reusable scenarios in this repository
- `Role Builder`
  - when the new scenario exposes participant-role support gaps
- project leads or workflow authors
  - only after the final Scenario Designer quality review has confirmed the package is fully ready for downstream reuse
