---
audience: developer
rule: "5.1"
code: PGE05001
name: Sibling Separator Homogeneity
severity: error
---

### Rule 5.1 — Sibling Separator Homogeneity
`PGE05001`

**Statement:** All sibling fields at the same nesting level must use the same separator. Mixing `.` (fixed) and `:` (flexible) separators among siblings is a compile error. This applies to `{#}` data definitions, `$` variable field access, and any serialized identifier path. Different nesting levels may use different separators — the rule is per-level only.
**Rationale:** Each separator carries semantic meaning — `.` means a closed, compile-time-known key set; `:` means an open, runtime-extensible key set. Mixing them at the same level creates ambiguity: is the level fixed or flexible? Enforcing homogeneity per level keeps the schema model unambiguous.
**Detection:** The compiler groups sibling fields by their parent path and checks that all siblings share the same separator. If any sibling uses a different separator than its peers, PGE05001 fires.

**VALID:**
```polyglot
[ ] ✓ all siblings use : (flexible)
[r] $user:name << "Alice"
[r] $user:age << 30
[r] $user:email << "alice@example.com"
```

```polyglot
[ ] ✓ all siblings use . (fixed)
{#} #Boolean
   [.] .True
   [.] .False
```

```polyglot
[ ] ✓ different separators at different levels
{#} #Config
   [.] .timeout#int
   [.] .info#serial

[r] $cfg#Config
   [r] $cfg.timeout << 30
   [r] $cfg.info:author << "admin"     [ ] ✓ .info level is fixed, :author level is flexible
   [r] $cfg.info:version << "1.0"
```

**INVALID:**
```polyglot
[ ] ✗ PGE05001 — mixing . and : at the same level
[r] $user.name << "Alice"
[r] $user:age << 30                    [ ] ✗ PGE05001 — : sibling among . siblings
```

```polyglot
[ ] ✗ PGE05001 — mixed separators in {#} definition
{#} #Bad
   [.] .name#string
   [:] :tag#string                     [ ] ✗ PGE05001 — : among . siblings
```

### See Also

- [[user/syntax/types/structs|Structs]] — field homogeneity rules reference PGE05001
- [[user/syntax/types/flexible-fields|Typed Flexible Fields]] — structs with both fixed and flexible levels
