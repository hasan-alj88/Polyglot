---
audience: developer
rule: "1.25"
code: PGE01025
name: Discard in Wrapper IO
severity: error
---

### Rule 1.25 — Discard in Wrapper IO
`PGE01025`

**Statement:** Wrapper IO wiring lines (`(-)` under `[W]`) must wire to named `$` variables that map to the wrapper's `[{]` inputs and `[}]` outputs. Using `$*` (discard) as the target defeats the purpose of wiring and is a compile error. Additionally, wrapper IO must reference actual IO parameters — the wired variables must correspond to the wrapper macro's declared `[{]`/`[}]` parameters.
**Rationale:** Wrapper IO exists to pass data between the pipeline scope and the wrapper's setup/cleanup scope. Discarding a wired value means the wrapper input is never provided or the wrapper output is lost. If a wrapper output is intentionally unused, the pipeline should not wire it at all rather than explicitly discarding.
**Detection:** The compiler checks that each `(-)` line under `[W]` uses a named `$` variable (not `$*`) and that the variable corresponds to a declared wrapper IO parameter.

**See also:** PGE01009 (wrapper IO mismatch)

**VALID:**
```polyglot
[ ] ✓ named variables wired to wrapper IO
[W] -W.DB.Connection
   (-) $connectionString << $connStr
   (-) $dbConn >> $dbConn
```

**INVALID:**
```polyglot
[ ] ✗ PGE01025 — $* discards the wrapper input
[W] -W.DB.Connection
   (-) $* << $connectionString
```

```polyglot
[ ] ✗ PGE01025 — $* discards the wrapper output
[W] -W.DB.Connection
   (-) $connectionString << $connStr
   (-) $* >> $dbConn
```

**Diagnostic:** "Discard `$*` in wrapper IO wiring — wrapper requires named variables for IO"
