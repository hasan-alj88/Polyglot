---
issue: 141
group: 3
group_name: "Naming & Terminology Conflicts"
priority: P3-medium
status: brief-ready
---

# Issue #141: Trigger Monitor role contradicts glossary vs vision/pglib

## Inconsistency
The glossary defines the Trigger Monitor as "Not a scheduler or cron" in its NOT-this column, yet the `=T.Daily` pglib trigger pipeline is explicitly schedule-based ("Fires once per day at the specified time"), and vision.md lists "schedules" as one of the events the Trigger Monitor monitors. The glossary's "Not a scheduler" disclaimer creates a contradiction with the fact that the TM does evaluate schedule conditions (e.g., `=T.Daily"3AM"`) and makes scheduling decisions. Additionally, the glossary says the TM "Evaluates conditions and sends command signals — the decision-maker" while vision.md says it merely "Monitors events... that initiate automated tasks" — a weaker description that omits the decision-making role.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/audit/reference/glossary.md` | Line 21: "Not a scheduler or cron" disclaimer contradicts schedule-based triggers; also defines TM as "the decision-maker" |
| `docs/vision.md` | Line 66: describes TM as monitoring "schedules" but omits decision-making role; weaker than glossary definition |
| `docs/user/pglib/pipelines/T.md` | Line 41: `=T.Daily` is a schedule-based trigger — fires at specified time, which is scheduling behavior |
| `docs/technical/plan/queue-manager/INDEX.md` | Line 12: "The Trigger Monitor is the decision-maker" — stronger wording consistent with glossary |
| `docs/user/concepts/pipelines/io-triggers.md` | Lines 65-79: TM enforces retrigger policy, reads queue config — decision-making beyond simple monitoring |

## Example
**Source A** (`docs/audit/reference/glossary.md`, line ~21):
> Trigger Monitor | Component that monitors events (file changes, schedules, webhooks, resource availability) that initiate automated tasks. Evaluates conditions and sends command signals — the decision-maker | Not a scheduler or cron

**Source B** (`docs/vision.md`, line ~66):
> **Trigger Monitor** — Monitors events (file changes, schedules, HTTP webhooks, resource availability) that initiate automated tasks.

**Source C** (`docs/user/pglib/pipelines/T.md`, lines ~39-41):
> .Daily
>    <InlineStringLiteral#string <~ ""
>    [ ] Fires once per day at the specified time.

## Prior Related Work
- Issue #130 — Closed: fixed QH wording — scoped "never makes decisions" to "trigger conditions or business logic". Narrowed the QH disclaimer but did not revisit the TM's "Not a scheduler" disclaimer in the glossary.

## Recommendation
Clarify the glossary NOT-this column: change "Not a scheduler or cron" to something like "Not a general-purpose scheduler — it monitors trigger conditions (including schedules) and sends commands, but does not own dispatch ordering or job scheduling (that is the Queue Handler's Dispatch Coordinator)". Also align vision.md with the glossary by adding the "evaluates conditions and sends command signals" language. The key distinction is between trigger-condition scheduling (TM) and dispatch scheduling (QH/DC).

## Discussion Prompts
1. Should "Not a scheduler" be removed entirely, or refined to "Not a dispatch scheduler"?
2. Should vision.md's TM description be expanded to match the glossary, or should the glossary be simplified to match vision.md?
3. Is the TM's schedule-trigger capability (`=T.Daily`) a design decision that needs explicit reconciliation, or is the "not a scheduler" warning simply poorly worded?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 141*
