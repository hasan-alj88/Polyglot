---
audience: developer
rule: "4.8"
code: PGE04008
name: Missing Path Platform Subfield
severity: error
---

### Rule 4.8 — Missing Path Platform Subfield
`PGE04008`

**Statement:** A `#path` variable that uses explicit subfield assignment (`.Unix`, `.Windows`) must include the subfield for the current compilation OS. If the current OS subfield is absent, the compiler raises PGE04008. This applies only to explicit subfield assignment — `=Path"..."` inline calls handle platform resolution differently (see [[pglib/pipelines/Path|=Path]]).
**Rationale:** A path that cannot resolve on the current OS is unusable. Catching this at compile time prevents runtime failures when the code attempts to use a path with no value for the host platform.
**Detection:** The compiler checks each `#path` variable with explicit `.Unix` or `.Windows` subfield assignment. If the subfield for the current compilation target is missing, PGE04008 fires.

**See also:** PGW04001 (single-platform path warning), PGE04007 (invalid path string), PGE04001 (type mismatch)

**VALID:**
```polyglot
[r] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"

[ ] ✓ both OS subfields assigned — works everywhere
```

```polyglot
[r] $LogDir#path << =Path"{.}/logs"

[ ] ✓ =Path"..." resolves separators per OS — no explicit subfields needed
```

**INVALID:**
```polyglot
[ ] compiling on Unix:
[r] $AppDir#path
   [.] .Windows << "C:\MyApp"            [ ] ✗ PGE04008 — .Unix missing, current OS is Unix
```

```polyglot
[ ] compiling on Windows:
[r] $AppDir#path
   [.] .Unix << "/tmp/MyApp"             [ ] ✗ PGE04008 — .Windows missing, current OS is Windows
```

### See Also

- [[syntax/types/strings|String Types]] — missing path platform subfield references
- [[pglib/types/structs|Struct Types]] — `#path` platform subfield examples
- [[technical/edge-cases/04-type-system|EC-4.6: Single-platform path]] — references PGE04008
