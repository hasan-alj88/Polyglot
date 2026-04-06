---
audience: developer
rule: "10.2"
code: PGE12002
name: Duplicate Alias
severity: error
---

### Rule 10.2 — Duplicate Alias
`PGE12002`

**Statement:** Each alias string within a single `%alias` declaration must be unique, AND each fully resolved alias name must be globally unique across all definitions in the compilation unit. Duplicate alias names are a compile error.
**Rationale:** Aliases create alternative lookup paths for definitions. If two definitions resolve to the same alias, the compiler cannot determine which definition the alias refers to. This includes nested paths — `File.Permission.Denied` declared as an alias from two different error branches creates ambiguity. Global uniqueness ensures every alias resolves to exactly one definition.
**Detection:** The compiler collects all alias strings from all `%alias` declarations across the compilation unit, resolves them to fully qualified paths, and rejects any name that appears more than once. Both intra-definition duplicates (same alias listed twice in one `%alias`) and inter-definition duplicates (same alias in different definitions) are errors.

**See also:** PGE01015 (duplicate metadata field — covers fixed fields, not aliases), PGE09011 (duplicate import alias — analogous rule for `[@]` import aliases)

**VALID:**
```polyglot
[ ] ✓ multiple unique aliases on one definition
{#} #SystemConfig
   [%] %alias
      [:] "Config"
      [:] "SysConfig"
   [.] .timeout#int <~ 30
   [.] .retries#int <~ 3
```

```polyglot
[ ] ✓ different definitions with distinct aliases
{#} #UserRecord
   [%] %alias
      [:] "User"
   [.] .name#string

{#} #AdminRecord
   [%] %alias
      [:] "Admin"
   [.] .name#string
```

```polyglot
[ ] ✓ nested key alias with dot separators
{!} !Permission
   [.] .File
      [.] .Denied#Error
         [%] %alias
            [:] "File.Permission.Denied"
            [:] "FileDenied"
```

**INVALID:**
```polyglot
[ ] ✗ PGE12002 — same alias string appears twice in one definition
{#} #Record
   [%] %alias
      [:] "Rec"
      [:] "Rec"                        [ ] ✗ PGE12002 — duplicate within same %alias
   [.] .name#string
```

```polyglot
[ ] ✗ PGE12002 — same alias across different definitions
{#} #UserConfig
   [%] %alias
      [:] "Config"
   [.] .theme#string

{#} #SystemConfig
   [%] %alias
      [:] "Config"                     [ ] ✗ PGE12002 — "Config" already aliases #UserConfig
   [.] .timeout#int
```

```polyglot
[ ] ✗ PGE12002 — nested alias collides with another definition's alias
{!} !Permission
   [.] .File
      [.] .Denied#Error
         [%] %alias
            [:] "FileDenied"

{!} !Error
   [.] .FileDenied#Error
      [%] %alias
         [:] "FileDenied"              [ ] ✗ PGE12002 — "FileDenied" already aliases !Permission.File.Denied
```

### See Also

- [[user/syntax/blocks|Blocks]] — references PGE12002 in metadata alias uniqueness
- [[user/concepts/metadata|Metadata]] — references PGE12002 in `%alias` field description
- [[technical/edge-cases/15-metadata-blocks|EC-15: Metadata Blocks]] — references PGE12002

**Open point:** None.
