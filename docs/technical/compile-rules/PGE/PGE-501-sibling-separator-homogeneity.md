---
rule: "5.1"
code: PGE-501
name: Sibling Separator Homogeneity
severity: error
---

### Rule 5.1 — Sibling Separator Homogeneity
`PGE-501`

**Statement:** All sibling fields at the same nesting level must use the same separator. Mixing `.` (fixed) and `:` (flexible) separators among siblings is a compile error. This applies to `{#}` data definitions, `$` variable field access, and any serialized identifier path. Different nesting levels may use different separators — the rule is per-level only.
**Rationale:** Each separator carries semantic meaning — `.` means a closed, compile-time-known key set; `:` means an open, runtime-extensible key set. Mixing them at the same level creates ambiguity: is the level fixed or flexible? Enforcing homogeneity per level keeps the schema model unambiguous.
**Detection:** The compiler groups sibling fields by their parent path and checks that all siblings share the same separator. If any sibling uses a different separator than its peers, PGE-501 fires.

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
   [.] .timeout;int
   [.] .info;serial

[r] $cfg;Config
   [r] $cfg.timeout << 30
   [r] $cfg.info:author << "admin"     [ ] ✓ .info level is fixed, :author level is flexible
   [r] $cfg.info:version << "1.0"
```

**INVALID:**
```polyglot
[ ] ✗ PGE-501 — mixing . and : at the same level
[r] $user.name << "Alice"
[r] $user:age << 30                    [ ] ✗ PGE-501 — : sibling among . siblings
```

```polyglot
[ ] ✗ PGE-501 — mixed separators in {#} definition
{#} #Bad
   [.] .name;string
   [:] :tag;string                     [ ] ✗ PGE-501 — : among . siblings
```
