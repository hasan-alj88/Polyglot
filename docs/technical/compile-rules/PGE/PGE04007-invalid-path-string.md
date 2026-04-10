---
audience: developer
rule: "4.7"
code: PGE04007
name: Invalid Path String
severity: error
---

### Rule 4.7 — Invalid Path String
`PGE04007`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/errors -->
<!-- @u:syntax/operators -->

**Statement:** A string literal assigned to a `#path` subfield (`.Unix` or `.Windows`) must be a syntactically valid path for that OS. Invalid characters, malformed syntax (e.g., double separators, illegal characters in filenames), or structurally impossible paths trigger PGE04007. This applies to both explicit subfield assignment and path interpolation contexts.
**Rationale:** Path strings are not arbitrary strings — they must conform to OS filesystem rules. Catching invalid paths at compile time prevents runtime filesystem errors and surfaces typos or copy-paste mistakes early.
**Detection:** When the compiler encounters a string literal being pushed into a `.Unix` or `.Windows` subfield of a `#path` typed variable, it validates the string against the target OS's path rules. For `.Unix`: no null bytes, no double `//` (except root). For `.Windows`: no `<>|"*?` characters in names, valid drive letter or UNC prefix.

**See also:** PGE04001 (type mismatch), PGE04005 (undefined interpolation variable)

**VALID:**
```polyglot
[-] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"

[ ] ✓ both paths are valid for their respective OS
```

```polyglot
[-] $LogFile#path
   [.] .Unix << "/var/log/app.log"
   [.] .Windows << "C:\ProgramData\App\app.log"

[ ] ✓ valid absolute paths
```

**INVALID:**
```polyglot
[-] $BadPath#path
   [.] .Unix << "/tmp//MyApp"           [ ] ✗ PGE04007 — double separator in Unix path
   [.] .Windows << "C:\MyApp"
```

```polyglot
[-] $BadPath#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\My<App"          [ ] ✗ PGE04007 — illegal character '<' in Windows path
```

```polyglot
[-] $BadPath#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "MyApp"              [ ] ✗ PGE04007 — Windows path missing drive letter or UNC prefix
```

**OS-specific validation:** Exact per-OS character/syntax rules (forbidden chars, max length, reserved names) are deferred to compiler implementation. The language spec covers structure and separator normalization only.

**Cross-platform inference (resolved 2026-03-20):**

The compiler statically infers whether a `-Path"..."` expression is provably cross-platform or potentially single-OS:

- **Provably cross-platform** — contains `{.}`, `{..}`, or interpolates a `$var#path` that has both `.Unix` and `.Windows` defined → no handling needed
- **Potentially single-OS** — contains only literal strings or interpolates variables without dual-OS proof → compiler raises PGE04007, forcing the user to either:
  1. Define explicit dual-OS subfields (`.Unix` + `.Windows`)
  2. Handle with `[!]` block or `<!` fallback

```polyglot
[ ] ✓ provably cross-platform — {.} is dual-OS
[-] $LogDir#path << -Path"{.}/logs"

[ ] ✓ provably cross-platform — $Root has both subfields
[-] $Root#path
   [.] .Unix << "/opt"
   [.] .Windows << "C:\opt"
[-] $AppDir#path << -Path"{$Root}/MyApp"

[ ] ✗ PGE04007 — literal string, no dual-OS proof
[-] $dir#path << -Path"/tmp/MyApp"

[ ] ✓ handled — <! fallback for single-OS path
[-] $dir#path << -Path"/tmp/MyApp"
   (>) <! $defaultDir                 [ ] catch-all fallback
   [!] !PathPlatformMismatch
      [-] >dir << $defaultDir
```

### See Also

- [[syntax/types/strings|String Types]] — path validation error references
- [[pglib/types/structs|Struct Types]] — `#path` invalid path string examples
