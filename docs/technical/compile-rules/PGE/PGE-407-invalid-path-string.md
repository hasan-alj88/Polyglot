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

**OS-specific validation:** Exact per-OS character/syntax rules (forbidden chars, max length, reserved names) are deferred to compiler implementation. The language spec covers structure and separator normalization only.

**Cross-platform inference (resolved 2026-03-20):**

The compiler statically infers whether a `=Path"..."` expression is provably cross-platform or potentially single-OS:

- **Provably cross-platform** — contains `{.}`, `{..}`, or interpolates a `$var;path` that has both `.Unix` and `.Windows` defined → no handling needed
- **Potentially single-OS** — contains only literal strings or interpolates variables without dual-OS proof → compiler raises PGE-407, forcing the user to either:
  1. Define explicit dual-OS subfields (`.Unix` + `.Windows`)
  2. Handle with `[!]` + `*Continue >FallBack`

```polyglot
[ ] ✓ provably cross-platform — {.} is dual-OS
[r] $LogDir;path << =Path"{.}/logs"

[ ] ✓ provably cross-platform — $Root has both subfields
[r] $Root;path
   [.] .Unix << "/opt"
   [.] .Windows << "C:\opt"
[r] $AppDir;path << =Path"{$Root}/MyApp"

[ ] ✗ PGE-407 — literal string, no dual-OS proof
[r] $dir;path << =Path"/tmp/MyApp"

[ ] ✓ handled — fallback for single-OS path
[r] $dir;path << =Path"/tmp/MyApp"
   [!] !PathPlatformMismatch
      [*] *Continue >FallBack << $defaultDir
```
