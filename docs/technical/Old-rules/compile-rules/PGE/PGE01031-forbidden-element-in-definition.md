---
audience: developer
rule: "1.31"
code: PGE01031
name: Forbidden Element in Definition
severity: error
---

# Rule 1.31 — Forbidden Element in Definition
`PGE01031`

<!-- @c:brainstorming:marker-declarations §4 Rule C -->

**Statement:** Each definition type restricts which block elements may appear inside it. Using a forbidden element is a compile error. This is a single error code with variable messages — the definition type and forbidden element identify the specific violation.
**Rationale:** Definition types have distinct roles: wrappers manage lifecycle, queue operations manage scheduling, data types define structure. Allowing arbitrary elements would blur these roles and create semantic confusion. The compiler enforces structural correctness per type.
**Detection:** The compiler checks each block element against the definition type's allowed-element set. If the element is not in the set, PGE01031 fires.

**Valid elements per definition type:**

| Definition | Required | Allowed | Forbidden |
|---|---|---|---|
| `{-}[exe]` | `[T]`, `[Q]`, `[W]` | `(-)` IO, `[-]`/`[=]`/`[b]` body, `[%]`, `[?]`, `[!]`, `(<)`/`(>)` | — |
| `{T}` | `>IsTriggered#bool` | `(-)` IO, `[%]`, `[T]` (composed AND), `[Q]`, `[W]`, `[-]`/`[=]`/`[b]` body, `[?]`, `[!]`, `(<)`/`(>)` | — |
| `{W}` | `[\]`/`[/]` (unless base) | `(-)` IO, `[\]`/`[/]` body, `[%]`, `[W]` (composed), `[-]` in setup/cleanup | `[T]`, `[Q]` |
| `{Q} -Q.*` | — | `(-)` IO, `[%]` | `[T]`, `[W]`, `[-]`/`[=]`/`[b]` body |
| `{Q} #Name` | — | `[.]`/`[:]` fields, `[#]` | All pipeline elements |
| `{#}` | — | `[.]`/`[:]` fields, `[#]`, `[%]` | All pipeline elements |
| `{!}` | — | `[.]`/`[:]` fields | All pipeline elements |

**Note:** `{T}` triggers may have execution body, `[Q]`, and `[W]`. These are optional (not required like in `{-}[exe]`). The only structural requirement unique to `{T}` is `>IsTriggered#bool` (see PGE01032).

**VALID:**
```aljam3
[ ] ✓ — derived trigger with full execution body
{T} -T.Complex.SystemReady
   [Q] -Q.Default
   [W] -W.DB.Connection
      (-) $connectionString << "postgres://..."
      (-) $dbConn >> $dbConn
   (-) <config#string
   (-) >IsTriggered#bool
   (-) >systemState#serial
   [ ]
   [-] -DB.Query
      (-) <conn << $dbConn
      (-) <sql << "SELECT ready FROM system"
      (-) >rows >> $rows
   [?] $rows<0.ready
      [?] ?= "true"
         [-] >IsTriggered << #True
      [?] ?*
         [-] >IsTriggered << #False

[ ] ✓ — simple trigger, no body needed
{T} -T.Simple
   (-) >IsTriggered#bool
```

**INVALID:**
```aljam3
[ ] ✗ PGE01031 — {W} cannot have [T]
{W} -W.Bad.WithTrigger
   [T] -T.Call
   (-) <input;string

[ ] ✗ PGE01031 — {W} cannot have [Q]
{W} -W.Bad.WithQueue
   [Q] -Q.Default
   (-) <input;string

[ ] ✗ PGE01031 — {Q} cannot have [T]
{Q} -Q.Bad.WithTrigger
   [T] -T.Call

[ ] ✗ PGE01031 — {Q} cannot have [W]
{Q} -Q.Bad.WithWrapper
   [W] -W.Aljam3

[ ] ✗ PGE01031 — {Q} cannot have execution body
{Q} -Q.Bad.WithBody
   (-) <threshold#float
   [ ]
   [-] -SomeWork
```

**Diagnostic:** "`{X}` definition `-Name` cannot contain `[Y]` — [element] is forbidden in [type] definitions"

## See Also

- [[PGE01029-invalid-marker-for-definition-type|PGE01029]] — invalid markers on definition line (complementary rule)
- [[PGE01032-missing-trigger-boolean-output|PGE01032]] — `{T}` required output constraint
- [[PGE01004-macro-structural-constraints|PGE01004]] — wrapper structural constraints
- [[marker-declarations|Marker Declarations Brainstorming]] — §4 Rule C, valid/invalid examples
