---
audience: developer
rule: "4.8"
code: PGW04001
name: Single-Platform Path
severity: warning
---

### Rule 4.8w — Single-Platform Path
`PGW04001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** A `#path` variable that assigns only one OS subfield (`.Unix` or `.Windows`) emits PGW04001 as a portability warning — even when the assigned subfield matches the current OS. The code compiles but will not work if deployed to the other platform. Suppress with `[ ] Ignore PGW04001`.
**Rationale:** Single-platform paths are a portability risk. The warning alerts developers that their code is OS-specific, giving them the chance to add the missing subfield or explicitly acknowledge the limitation.
**Detection:** The compiler counts explicit `.Unix` and `.Windows` subfield assignments on a `#path` variable. If exactly one is present, PGW04001 fires — regardless of whether it matches the current OS.

**See also:** PGE04008 (error when current OS subfield is missing), PGE04007 (invalid path string)

**WARNING:**
```polyglot
[ ] compiling on Unix:
[-] $AppDir#path
   [.] .Unix << "/tmp/MyApp"             [ ] ⚠ PGW04001 — only .Unix assigned, not portable
```

```polyglot
[ ] ✓ suppressed — developer acknowledges single-platform
[ ] Ignore PGW04001
[-] $AppDir#path
   [.] .Unix << "/tmp/MyApp"             [ ] no warning — suppressed
```

```polyglot
[ ] ✓ no warning — both platforms assigned
[-] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

```polyglot
[ ] ✓ no warning — -Path"..." with cross-platform interpolation
[-] $AppDir#path << -Path"{.}/MyApp"
```
