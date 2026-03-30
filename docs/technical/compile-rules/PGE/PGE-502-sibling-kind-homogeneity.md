---
rule: "5.2"
code: PGE-502
name: Sibling Kind Homogeneity
severity: error
---

### Rule 5.2 — Sibling Kind Homogeneity in `{#}`
`PGE-502`

**Statement:** Within a `{#}` data definition, all sibling fields at the same nesting level must be the same kind: either all **enum fields** (no `;type` annotation) or all **value fields** (with `;type` annotation). Mixing enum and value fields at the same level is a compile error. This rule applies per-level only — an enum field may have nested value sub-fields at a deeper level.
**Rationale:** Enum fields and value fields serve different purposes — enums represent discrete variants for branching, while value fields hold typed data. Mixing them at the same level creates ambiguity: is the level a set of variants or a set of data fields? Keeping them homogeneous per level ensures the compiler can treat each level uniformly.
**Detection:** The compiler inspects each nesting level within a `{#}` block and checks whether all siblings have `;type` (value) or none do (enum). A mix triggers PGE-502.

**VALID:**
```polyglot
[ ] ✓ all enum fields at top level
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Warning
   [.] .Info
```

```polyglot
[ ] ✓ all value fields at top level
{#} #UserRecord
   [.] .name#string
   [.] .age#int
   [.] .email#string
```

```polyglot
[ ] ✓ enum at top level with nested value sub-fields — different levels
{#} #Severity
   [.] .Critical
      [.] .code#int <~ 500
      [.] .label#string <~ "CRITICAL"
   [.] .Error
      [.] .code#int <~ 400
      [.] .label#string <~ "ERROR"
   [.] .Info
```

**INVALID:**
```polyglot
[ ] ✗ PGE-502 — mixing enum and value fields at the same level
{#} #Bad
   [.] .Active                         [ ] enum field (no ;type)
   [.] .count#int <~ 0                 [ ] ✗ PGE-502 — value field among enum siblings
```

```polyglot
[ ] ✗ PGE-502 — mixing kinds at same level
{#} #Status
   [.] .Running
   [.] .Stopped
   [.] .uptime#int                     [ ] ✗ PGE-502 — value field among enum siblings
```

### See Also

- [[user/syntax/types/structs|Structs]] — field homogeneity rules reference PGE-502
