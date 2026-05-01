---
audience: developer
rule: "1.29"
code: PGE01029
name: Invalid Marker for Definition Type
severity: error
---

# Rule 1.29 — Invalid Marker for Definition Type
`PGE01029`

<!-- @c:brainstorming:marker-declarations §4 Rule A -->

**Statement:** Only `{-}` accepts an explicit marker declaration, and only `[exe]`. Sugar types (`{T}`, `{W}`, `{Q}`) already have implicit markers (`[T]`, `[W]`, `[Q]`) and cannot take additional markers. Data types (`{#}`, `{!}`) cannot have markers at all.
**Rationale:** Each definition type has a fixed role in the type hierarchy. `{T}` is syntactic sugar for `{-}[T]` — attaching `[exe]` or `[W]` to it would create a contradictory declaration. Data types (`{#}` and its subtype `{!}`) are not callable and have no execution semantics, so markers are meaningless.
**Detection:** The compiler checks the definition block type against any `marker_decl` on the definition line. If the type does not permit markers, or the marker is not `[exe]`, PGE01029 fires.

**Valid markers per definition type:**

| Definition | Valid Markers | Notes |
|---|---|---|
| `{-}` | `[exe]` only (default if omitted) | `[exe]` = `[rpb]` — the execution group |
| `{T}` | none (implicit `[T]`) | Already IS `{-}[T]` — adding markers is invalid |
| `{W}` | none (implicit `[W]`) | Already IS `{-}[W]` |
| `{Q} -Q.*` | none (implicit `[Q]`) | Already IS `{-}[Q]` |
| `{Q} #Name` | none — data type | Kind of `{#}`, not callable |
| `{#}` | none — data type | Not callable, no markers |
| `{!}` | none — subtype of `{#}` | Inherits "no markers" |

**Note:** `{-}` without an explicit marker is treated as `{-}[exe]`. No diagnostic is emitted (decided in #108).

**VALID:**
```aljam3
[ ] ✓ — {-} takes [exe]
{-}[exe] -Good.Pipeline

[ ] ✓ — {T} with no extra marker
{T} -T.Custom

[ ] ✓ — {-} without marker defaults to {-}[exe]
{-} -Implicit.Exe
```

**INVALID:**
```aljam3
[ ] ✗ PGE01029 — {#} cannot have markers
{#}[exe] #BadData

[ ] ✗ PGE01029 — {T} already implies [T], cannot add [exe]
{T}[exe] -T.Bad

[ ] ✗ PGE01029 — {W} already implies [W], cannot add [T]
{W}[T] -W.Bad

[ ] ✗ PGE01029 — {!} inherits {#} — no markers allowed
{!}[exe] !Bad
```

**Diagnostic:** "Definition `{X}` does not accept marker `[Y]` — [reason per type]"

## See Also

- [[marker-declarations|Marker Declarations Brainstorming]] — §4 Rule A, valid/invalid examples
- [[09-definition-blocks|EBNF §9]] — `marker_decl` grammar and implicit marker rules
- [[concepts/pipelines/INDEX|Pipeline Subtypes]] — documents `{T}` = `{-}[T]`, `{W}` = `{-}[W]`, `{Q}` = `{-}[Q]`
