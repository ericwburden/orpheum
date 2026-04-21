# Scenario Implementation Integration Map

## Purpose

Capture how the `Scenario Implementation` scenario composes Scenario Designer and Role Builder work into one coherent path for producing a reusable scenario package that is both structurally sound and operationally supportable.

## Scenario In Scope

This integration map applies to the reusable `Scenario Implementation` scenario defined in [scenario-implementation.definition.md](C:/Users/ericw/Projects/orpheum/scenarios/scenario-implementation.definition.md).

## Participating Role-Owned Workflows

- [`scenario-designer-composition.md`](C:/Users/ericw/Projects/orpheum/workflows/scenario-designer-composition.md)
- [`scenario-designer-review.md`](C:/Users/ericw/Projects/orpheum/workflows/scenario-designer-review.md)
- [`scenario-designer-quality-review.md`](C:/Users/ericw/Projects/orpheum/workflows/scenario-designer-quality-review.md)
- [`scenario-designer-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/scenario-designer-handoff.md)
- [`role-builder-quality-review.md`](C:/Users/ericw/Projects/orpheum/workflows/role-builder-quality-review.md)
- optional [`role-builder-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/role-builder-handoff.md) when participant-package adoption guidance or preserved package-level limits should remain visible in the final scenario-readiness story
- optional [`role-builder-support-system.md`](C:/Users/ericw/Projects/orpheum/workflows/role-builder-support-system.md) when participant-role hardening requires explicit support-system changes or when the artifacts needed for an honest hardening review are missing or stale
- optional [`role-builder-role-definition.md`](C:/Users/ericw/Projects/orpheum/workflows/role-builder-role-definition.md) when scenario implementation uncovers a genuinely missing role package rather than a hardening need
- optional [`security-compliance-specialist-scoping.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-scoping.md)
- optional [`security-compliance-specialist-review.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-review.md)
- optional [`security-compliance-specialist-handoff.md`](C:/Users/ericw/Projects/orpheum/workflows/security-compliance-specialist-handoff.md)

## Workflow Inputs, Outputs, And Shared Artifacts

- Scenario Designer workflows produce:
  - scenario definition
  - scenario integration map
  - scenario review
  - scenario handoff
  - quality-gated scenario readiness
- Role Builder workflows consume the drafted scenario plus the participant role packages, then produce:
  - explicit participant-role fit decisions for each named participant package
  - any refreshed role-definition or role-support-system artifacts needed to make each hardening review honest
  - durable review or handoff outputs for participant packages when those outputs are needed to preserve the hardening result honestly
  - role-package hardening recommendations per affected participant package
  - updated role workflows, checks, artifacts, and skills when needed
  - explicit role-package readiness judgments for the scenario-relevant participants
- Optional Security / Compliance Specialist workflows consume the reviewed scenario intent and relevant delivery-control context, then produce:
  - security/compliance framing for the scenario itself when trust-boundary-sensitive or approval-sensitive concerns materially constrain the scenario package

Shared artifacts and context that move across the scenario:

- the candidate scenario idea or selection decision
- the participant role definitions, workflows, checks, notes, and supporting skills
- draft scenario-definition and integration-map outputs
- role-fit findings, hardening decisions, and remediation routing
- participant-package review or handoff outputs when those outputs materially constrain the final scenario-readiness judgment
- skills-review findings and any resulting skill implementations
- final scenario-readiness posture and adoption guidance

## Handoff Contracts

- Scenario Designer -> Role Builder
  - Role Builder should receive a concrete drafted scenario package plus the named participant roles so the `Role Builder hardening pass` can assess each participant package from explicit choreography rather than from a vague scenario idea.
- Role Builder -> Scenario Designer
  - Scenario Designer should receive explicit participant-role readiness findings, hardening outcomes, any durable participant-package review or handoff outputs that matter, and skills-review decisions rather than having to infer whether role support is complete.
- Role Builder -> participant role packages
  - When hardening is needed, the work should preserve role ownership and fix the earliest missing workflow, check, artifact, or skill rather than patching the scenario around the defect, and use role-package handoff packaging when package-level adoption limits or conventions should remain visible downstream.
- Optional Security / Compliance Specialist -> Scenario Designer or Role Builder
  - When included, the security/compliance package should preserve approval-sensitive or trust-boundary-sensitive constraints without drifting into live governance or implementation ownership.
- Scenario Designer -> downstream adopters
  - Downstream consumers should receive a reviewed scenario package whose participant roles are supportable for scenario-relevant work rather than merely listed as participants, and the participant-hardening outcomes that materially justify that readiness should remain visible. Final downstream-ready status should not be claimed until the post-handoff Scenario Designer quality review has completed.

## Branching Rules And Decision Logic

- If the scenario idea is not actually reusable, stop and route back to scenario selection rather than forcing one-off work into a scenario package.
- If participant role packages appear sufficient, Role Builder review may be light, but it should not be silently omitted.
- Review and hardening decisions should be made per named participant package rather than as one undifferentiated omnibus judgment.
- If a participant role lacks the role-definition or support-system artifacts needed for an honest hardening review, instantiate or refresh them before that package's hardening judgment is treated as conclusive.
- If a participant role lacks scenario-critical workflows, checks, artifacts, or skills, run the hardening loop for that package before the scenario is treated as ready, using support-system changes where the hardening itself or the review prerequisites actually require them.
- If participant-package hardening introduces conventions, limits, or adoption guidance that materially justify the final scenario readiness, preserve them through `role-builder-handoff.md` rather than compressing them into scenario prose only.
- If a gap is really a missing role rather than a support-system defect, route it into explicit role-definition work or block the scenario rather than hiding the gap inside scenario prose.
- If trust-boundary-sensitive, compliance-sensitive, or approval-sensitive conditions materially shape the scenario package, invoke the Security / Compliance Specialist branch before the scenario is treated as settled.
- If a new hardening pass no longer produces a concrete structural fix, stronger evidence trail, clearer routing rule, or more honest readiness posture, stop iterating and treat the package as converged enough for the final Scenario Designer quality review.
- If the hardening loop yields a meta-level convergence or stop rule, keep that rule in the `Scenario Implementation` package unless the target scenario's own normal operating use genuinely needs the same rule.

## Parallelism And Synchronization Points

- Scenario Designer composition and early Role Builder context gathering may overlap once the candidate scenario and likely participants are known.
- Security/compliance scoping may overlap with scenario review when the control-sensitive concerns are already identifiable, but the scenario must reconverge before readiness is declared.
- The scenario must reconverge at:
  - a drafted scenario package before participant-role fit review becomes conclusive
  - a completed participant-role hardening and skills-review pass before final scenario readiness is declared
  - a convergence judgment that repeated hardening is no longer surfacing substantive package improvements
  - a completed scenario handoff before the final scenario-quality pass is treated as conclusive
  - a completed scenario-quality pass before downstream adoption
  - an explicit readiness transition from `conditional` to `ready` only if the full Scenario Designer check chain passes without unresolved blocking defects

## Shared Context, State, And Dependency Assumptions

- The scenario assumes Scenario Designer remains the owner of scenario composition and readiness packaging.
- The scenario assumes Role Builder remains the owner of participant-role hardening and skill-support judgments.
- The scenario assumes participant-role support should be fixed at the role-package layer rather than smoothed over in scenario text.
- The scenario assumes skills review is part of scenario implementation when the scenario's success depends on participant operational support, not a separate optional cleanup phase.
- The scenario assumes the default Role Builder entry point for existing participant packages is a hardening or quality-review pass, not a fresh role-definition exercise.
- The scenario assumes Role Builder hardening review may still require refreshed role-definition or support-system artifacts when those are the formal inputs needed to make the review defensible.
- The scenario assumes Role Builder review and hardening execute per participant package even when the scenario uses several participants.
- The scenario assumes participant-package handoff packaging is optional, but appropriate when downstream scenario adoption depends on preserving package-level conventions or limits explicitly.

## Failure Handling And Escalation Routing

- If the scenario idea is weak, route back to scenario recommendation or selection.
- If the drafted scenario exposes broken role boundaries or missing support, route back to Role Builder rather than softening the issue in scenario review.
- If hardening work reveals a genuinely missing role, route to explicit role-definition work or block the scenario rather than declaring partial readiness.
- If skills review reveals a scenario-critical method that is unsupported, route to skill addition or implementation before downstream adoption.
- If security/compliance constraints are unresolved and materially affect the scenario package, route back to Security / Compliance Specialist before the scenario is handed downstream.

## Coordination Risks And Watchouts

- Scenario Designer may overcompensate for weak participant-role support by writing too much scenario prose unless the Role Builder boundary stays explicit.
- Role Builder may over-harden participant roles for scenario-specific edge cases unless the scenario stays focused on reusable support gaps.
- Role Builder work can become vague if several participant packages are discussed together without making package-specific fit and hardening judgments explicit.
- Skills review can become performative if it only inventories existing skills without deciding whether they are sufficient for the scenario-relevant work.
- The scenario can become recursively heavy if it starts treating every implementation pass as a full meta-framework instead of a lightweight package-design loop.
- The scenario can also fail to converge if repeated hardening is allowed to continue without a clear threshold for what counts as a substantive improvement.
- The scenario can misroute Role Builder work if participant-fit review is treated like fresh role-package creation instead of a hardening pass over existing packages.
- The scenario can also under-specify Role Builder review if it forgets that the hardening pass may depend on refreshed role-definition or support-system artifacts for existing packages.
- The scenario can lose important hardening evidence if package-level limits or conventions are not preserved through role-package handoff when they materially shape the final readiness claim.
- Optional security/compliance participation can become ceremonial if trigger conditions are not kept explicit.

## Recommended Next Step

Run the final `Scenario Designer` quality review across the completed Scenario Implementation package, using the review and handoff artifacts as the readiness basis, then update the package posture to `ready` only if the full Scenario Designer check chain passes.
