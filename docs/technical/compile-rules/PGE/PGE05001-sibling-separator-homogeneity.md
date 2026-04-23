---
audience: developer
rule: "5.1"
code: PGE05001
name: Sibling Separator Homogeneity
severity: error
---

# Rule 5.1 — Sibling Separator Homogeneity
`PGE05001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** All sibling fields at the same nesting level must use the same separator. Mixing `.` (fixed) and `:` (flexible) separators among siblings is a compile error. This applies to `{#}` data definitions, `$` variable field access, and any serialized identifier path. Different nesting levels may use different separators — the rule is per-level only.
**Rationale:** Each separator carries semantic meaning — `.` means a closed, compile-time-known key set; `:` means an open, runtime-extensible key set. Mixing them at the same level creates ambiguity: is the level fixed or flexible? Enforcing homogeneity per level keeps the schema model unambiguous.
**Detection:** The compiler groups sibling fields by their parent path and checks that all siblings share the same separator. If any sibling uses a different separator than its peers, PGE05001 fires.

**Scope:** PGE05001 applies per sibling level in definitions and field access — not per navigation path. A single `field_path` like `$config.db:host` that crosses from a fixed level (`.db`) to a flexible level (`:host`) does **not** trigger PGE05001, because the `.` and `:` operate at different tree levels. See EC-3.8.

**VALID:**
```polyglot
[ ] ✓ all siblings use : (flexible)
[-] $user:name << "Alice"
[-] $user:age << 30
[-] $user:email << "alice@example.com"
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

[-] $cfg#Config
   [-] $cfg.timeout << 30
   [-] $cfg.info:author << "admin"     [ ] ✓ .info level is fixed, :author level is flexible
   [-] $cfg.info:version << "1.0"
```

**INVALID:**
```polyglot
[ ] ✗ PGE05001 — mixing . and : at the same level
[-] $user.name << "Alice"
[-] $user:age << 30                    [ ] ✗ PGE05001 — : sibling among . siblings
```

```polyglot
[ ] ✗ PGE05001 — mixed separators in {#} definition
{#} #Bad
   [.] .name#string
   [:] :tag#string                     [ ] ✗ PGE05001 — : among . siblings
```

## See Also

- [[user/syntax/types/structs|Structs]] — field homogeneity rules reference PGE05001
- [[user/syntax/types/flexible-fields|Typed Flexible Fields]] — structs with both fixed and flexible levels
