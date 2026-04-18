# Role Builder Skill Recommendations

## Purpose

Capture the recommended improvements to the `Role Builder` skill set in a durable form so the list can be reviewed, reprioritized, and revisited as the role package matures.

## Current Status

- [`role-definition`](D:/Projects/agoge/skills/role-definition/SKILL.md) is now implemented as the first dedicated `Role Builder` skill.
- [`role-package-review`](D:/Projects/agoge/skills/role-package-review/SKILL.md) is now implemented as the dedicated review and readiness-assessment skill for `Role Builder`.
- [`pattern-adaptation`](D:/Projects/agoge/skills/pattern-adaptation/SKILL.md) is now implemented as the dedicated adaptation skill for translating external or tool-coupled patterns into repo-native methods.
- [`role-support-system-design`](D:/Projects/agoge/skills/role-support-system-design/SKILL.md) is now implemented as the dedicated design skill for deriving a role's operational support package from its role definition.
- Generic synthesis skills have now been repositioned as auxiliary support rather than primary `Role Builder` operating skills.
- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md) is now adapted as the workshop-intake path for role-definition work.
- [`role-handoff-packaging`](D:/Projects/agoge/skills/role-handoff-packaging/SKILL.md) is now implemented as the adoption-facing packaging skill for reviewed role packages.
- The original recommendation list is now fully implemented. Future recommendations should be added based on observed friction in real role-building work.

## Recommendations

### 1. Reposition generic synthesis skills as supporting rather than primary (completed)

Keep these skills in use:

- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md)
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md)

These now act as input-gathering and synthesis helpers rather than the main operational skills for `Role Builder`.

### 2. Adapt `meeting-notes-and-actions` for workshop-led role design (completed)

This is a lower-effort extension that could help when role definition starts from collaborative sessions, interviews, or design workshops.

It would be useful for:

- normalizing role-design workshop notes
- extracting candidate responsibilities and boundaries
- surfacing unresolved questions and decisions

This is now the supported workshop-intake path for `Role Builder` role-definition work.

### 3. Consider `role-handoff-packaging` later (completed)

This is now the dedicated adoption-facing packaging skill. Its purpose is to package:

- the role definition
- the support system
- the review outcome
- adoption guidance and remaining limits

It now completes the end-to-end `Role Builder` lifecycle from role idea through adoption handoff.

## Recommended Order

1. `role-definition` (completed)
2. `role-package-review` (completed)
3. `pattern-adaptation` (completed)
4. `role-support-system-design` (completed)
5. targeted adaptation of `meeting-notes-and-actions` (completed)
6. `role-handoff-packaging` (completed)

## Review Triggers

Revisit this list when:

- `role-definition` has been used enough to expose the next highest-friction gap
- `Role Builder` workflows start depending too heavily on generic synthesis skills again
- a new external or tool-coupled pattern needs adaptation into repo-native form
- another reusable role package is created and reveals support gaps that this list should capture
- adoption handoff starts breaking down for downstream teams or repos and needs stronger packaging guidance
