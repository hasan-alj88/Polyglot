---
issue: 159
group: 4
group_name: "Cross-Reference / Routing Errors"
priority: P3-medium
status: brief-ready
---

# Issue #159: Audience routing sends Architects to syntax docs, Designers to runtime docs

## Inconsistency
The technical documentation index (`docs/technical/INDEX.md`) declares `audience: [architect, designer]`, routing both audiences to the same content pool -- EBNF grammar, compile rules, edge cases, and metadata tree specs. However, the audience rules explicitly forbid cross-contamination: Architects must NEVER "mix language syntax design concerns into architecture docs" (that is Designer scope), and Designers must NEVER "include runtime implementation details (Queue Manager, Dispatcher)" (that is Architect scope). The Master Index (`docs/user/SPEC-INDEX.md`) further compounds this by directing all "Contributors" to `docs/technical/INDEX.md` without audience filtering.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/INDEX.md` | `audience: [architect, designer]` in frontmatter routes both audiences to the same content -- EBNF grammar (Designer scope) and runtime specs (Architect scope) are co-listed without separation |
| `docs/user/SPEC-INDEX.md` | "For Contributors" section sends all contributors to `docs/technical/INDEX.md` without audience filtering |
| `docs/audit/audiences/architect.md` | NEVER rule forbids Architects from syntax design content, but INDEX routes them there |
| `docs/audit/audiences/designer.md` | NEVER rule forbids Designers from runtime implementation content, but INDEX routes them there |

## Example
**Source A** (`docs/technical/INDEX.md`, line ~3):
> audience: [architect, designer]

**Source B** (`docs/audit/audiences/architect.md`, line ~39):
> Mix language syntax design concerns into architecture docs (that's [[audiences/designer]] scope)

**Source C** (`docs/audit/audiences/designer.md`, line ~36):
> Include runtime implementation details (Queue Manager, Dispatcher -- that's [[audiences/architect]] scope)

**Source D** (`docs/user/SPEC-INDEX.md`, lines ~68-70):
> ## For Contributors
> See [[technical/INDEX|docs/technical/INDEX.md]] for internal specifications (EBNF grammar, edge cases, compiler rules, metadata tree spec).

## Prior Related Work
- Issue #138 -- Added `audience` frontmatter to 218 files; established `technical/ -> developer` convention but did not resolve the multi-audience INDEX routing conflict
- Issue #120 -- IO perspective terminology fix; established that audience-specific scoping rules must be enforced
- IC-002 -- Resolved audience mismatch where `metadata.md` had `audience: developer` in a user-facing path; same class of routing error

## Recommendation
Split `docs/technical/INDEX.md` into audience-specific entry points or add audience-scoped sections within the existing INDEX. EBNF grammar, compile rules, and edge cases should route to Designers; runtime architecture specs (queue, trigger monitor, dispatcher) should route to Architects. The "For Contributors" cross-reference in SPEC-INDEX should direct each audience to their scoped subsection rather than the undifferentiated technical INDEX.

## Discussion Prompts
1. Should `docs/technical/INDEX.md` be split into separate `INDEX-designer.md` and `INDEX-architect.md`, or should it use audience-scoped sections within one file?
2. Does the current `audience: [architect, designer]` convention on technical files need a general policy update, given that most technical content is audience-specific?
3. Is this a documentation restructure or can it be resolved with routing annotations alone?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 159*
