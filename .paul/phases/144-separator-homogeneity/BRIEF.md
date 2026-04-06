---
issue: 144
group: 1
group_name: "EBNF / Compiler Rule Gaps"
priority: P3-medium
status: brief-ready
---

# Issue #144: PGE05001 separator homogeneity contradicts flexible-fields documentation

## Inconsistency
PGE05001 states "All sibling fields at the same nesting level must use the same separator" and shows that mixing `.` and `:` at the same level is a compile error. However, the `{#}` data definition grammar (EBNF Section 09) and user docs for structs show types that legitimately have **both** fixed (`.`) and flexible (`:`) fields at different levels within the same definition. A naive reading of PGE05001 could suggest that a single `{#}` definition cannot use both `.` and `:` separators at all, when in fact the rule only applies **per-level among siblings**. The PGE05001 document does include a valid example with different separators at different levels, but the rule statement itself could be clearer about the per-level scope.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/compile-rules/PGE/PGE05001-sibling-separator-homogeneity.md` | Rule statement says "same nesting level" but the title "Sibling Separator Homogeneity" and initial phrasing could be read as applying to the entire definition |
| `docs/user/syntax/types/flexible-fields.md` | Shows structs with both `.` fixed and `:` flexible fields at different levels (e.g., `#Registry` with `.builtins` fixed and `.plugins` using `[:] :*#Handler`) -- no reference to PGE05001 |
| `docs/user/syntax/types/structs.md` | References PGE05001 for separator homogeneity but only at the struct-level rules section |
| `docs/user/syntax/identifiers.md` | Serialization rule 1 says "all siblings at the same level must use the same separator" -- correct but terse |

## Example
**Source A** (`docs/technical/compile-rules/PGE/PGE05001-sibling-separator-homogeneity.md`, line ~12):
> **Statement:** All sibling fields at the same nesting level must use the same separator. Mixing `.` (fixed) and `:` (flexible) separators among siblings is a compile error. This applies to `{#}` data definitions, `$` variable field access, and any serialized identifier path. Different nesting levels may use different separators -- the rule is per-level only.

**Source B** (same file, line ~31-41, valid example):
> ```polyglot
> [ ] valid -- different separators at different levels
> {#} #Config
>    [.] .timeout#int
>    [.] .info#serial
>
> [r] $cfg#Config
>    [r] $cfg.timeout << 30
>    [r] $cfg.info:author << "admin"     [ ] valid -- .info level is fixed, :author level is flexible
>    [r] $cfg.info:version << "1.0"
> ```

**Source C** (`docs/user/syntax/types/flexible-fields.md`, line ~16-22):
> ```polyglot
> {#} #Registry
>    [.] .builtins
>       [.] .http#Handler
>       [.] .grpc#Handler
>    [.] .plugins
>       [:] :*#Handler
> ```

The rule statement in PGE05001 is actually correct ("per-level only") but could create confusion because the flexible-fields doc never cross-references PGE05001, and a reader encountering both docs might think fixed+flexible in the same definition violates the rule.

## Prior Related Work
- Issue #86 -- Audited fixed vs flexible field usage across `%_`, `%!`, `%@` branches
- Issue #88 -- Introduced schema properties; structural constraint system for types
- Issues #99-#106 -- EBNF edge cases batch; PGE05001 may have been refined during this work

## Recommendation
This is primarily a cross-referencing and clarification issue, not a real contradiction. The fix involves: (1) Add a cross-reference from the flexible-fields doc to PGE05001, noting that the per-level homogeneity rule applies even when a struct has both `.` and `:` levels. (2) Consider adding a brief note to PGE05001 emphasizing that a `{#}` definition with fixed fields at level 1 and flexible fields at level 2 is perfectly valid -- the rule never applies across levels. (3) Add "Different nesting levels may use different separators" to the identifiers.md serialization rules where it is currently absent.

## Discussion Prompts
1. Is the PGE05001 statement sufficiently clear with its "per-level only" qualifier, or does it need rephrasing?
2. Should the flexible-fields doc explicitly reference PGE05001 as a validation rule?
3. Are there any edge cases where level boundaries are ambiguous (e.g., a branch field typed as `#serial` that opens a flexible scope)?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 144*
