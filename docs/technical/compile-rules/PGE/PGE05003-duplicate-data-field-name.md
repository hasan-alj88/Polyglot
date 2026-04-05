---
audience: developer
rule: "4.13"
code: PGE05003
name: Duplicate Data Field Name
severity: error
---

### Rule 4.13 — Duplicate Data Field Name
`PGE05003`

**Statement:** All sibling fields at the same nesting level must have unique names. Two fields with the same name at the same level — whether in a `{#}` data definition or in `$` variable field assignment — are a compile error. Fields at different levels are unaffected (e.g., `.config.name` and `.user.name` are distinct paths).
**Rationale:** Duplicate field names at the same level create ambiguous access — `$record.name` cannot resolve to two distinct fields. Catching this at compile time prevents silent data loss where one field silently shadows the other.
**Detection:** The compiler collects all field names at each nesting level (grouped by parent path) and rejects any name that appears more than once within a level. This applies to `{#}` definitions, `$` variable field pushes, and any context where sibling fields are declared or assigned.

**See also:** PGE05001 (sibling separator homogeneity — same-level separator check), PGE05002 (sibling kind homogeneity — same-level kind check), PGE01011 (duplicate IO parameter name — analogous rule for pipeline IO)

**VALID:**
```polyglot
[ ] ✓ unique field names at each level
{#} #UserRecord
   [.] .name#string
   [.] .age#int
   [.] .email#string
```

```polyglot
[ ] ✓ same name at different levels is fine
{#} #Config
   [.] .db
      [.] .name#string
   [.] .cache
      [.] .name#string           [ ] ✓ .db.name and .cache.name are distinct paths
```

```polyglot
[ ] ✓ unique field pushes on a struct variable
[r] $user#UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30
   [r] $user.email << "alice@example.com"
```

**INVALID:**
```polyglot
[ ] ✗ PGE05003 — duplicate field name in {#} definition
{#} #Broken
   [.] .name#string
   [.] .name#int                             [ ] ✗ PGE05003 — .name declared twice
   [.] .age#int
```

```polyglot
[ ] ✗ PGE05003 — duplicate field push on struct variable
[r] $user#UserRecord
   [r] $user.name << "Alice"
   [r] $user.name << "Bob"                   [ ] ✗ PGE05003 — .name pushed twice
   [r] $user.age << 30
```

```polyglot
[ ] ✗ PGE05003 — duplicate flexible field name
{#} #Registry
   [:] :http#Handler
   [:] :http#Plugin                          [ ] ✗ PGE05003 — :http declared twice
```

**Open point:** None.
