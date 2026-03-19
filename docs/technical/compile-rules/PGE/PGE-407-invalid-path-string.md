---
rule: "4.7"
code: PGE-407
name: Invalid Path String
severity: error
---

### Rule 4.7 — Invalid Path String
`PGE-407`

**Statement:** A string literal assigned to a `;path` subfield (`.Unix` or `.Windows`) must be a syntactically valid path for that OS. Invalid characters, malformed syntax (e.g., double separators, illegal characters in filenames), or structurally impossible paths trigger PGE-407. This applies to both explicit subfield assignment and path interpolation contexts.
**Rationale:** Path strings are not arbitrary strings — they must conform to OS filesystem rules. Catching invalid paths at compile time prevents runtime filesystem errors and surfaces typos or copy-paste mistakes early.
**Detection:** When the compiler encounters a string literal being pushed into a `.Unix` or `.Windows` subfield of a `;path` typed variable, it validates the string against the target OS's path rules. For `.Unix`: no null bytes, no double `//` (except root). For `.Windows`: no `<>|"*?` characters in names, valid drive letter or UNC prefix.

**See also:** PGE-401 (type mismatch), PGE-405 (undefined interpolation variable)

**VALID:**
```polyglot
[r] $AppDir;path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"

[ ] ✓ both paths are valid for their respective OS
```

```polyglot
[r] $LogFile;path
   [.] .Unix << "/var/log/app.log"
   [.] .Windows << "C:\ProgramData\App\app.log"

[ ] ✓ valid absolute paths
```

**INVALID:**
```polyglot
[r] $BadPath;path
   [.] .Unix << "/tmp//MyApp"           [ ] ✗ PGE-407 — double separator in Unix path
   [.] .Windows << "C:\MyApp"
```

```polyglot
[r] $BadPath;path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\My<App"          [ ] ✗ PGE-407 — illegal character '<' in Windows path
```

```polyglot
[r] $BadPath;path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "MyApp"              [ ] ✗ PGE-407 — Windows path missing drive letter or UNC prefix
```

**Open point:** Define exact validation rules per OS — current examples show the principle but the full character/syntax rules need formal specification.
