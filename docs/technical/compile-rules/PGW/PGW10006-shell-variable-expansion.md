---
audience: developer
rule: "9.7"
code: PGW10006
name: Shell Variable Expansion
severity: warning
---

### Rule 9.7 — Shell Variable Expansion
`PGW10006`

<!-- @u:concepts/permissions/foreign-code -->
<!-- @u:pglib/pipelines/Run/Shell -->

**Statement:** When the compiler's AST analysis of a `-Run.Shell` pipeline detects shell variable expansion (`$VAR` or `${VAR}`) within a command argument that maps to an IO operation (file path, URL, network address), PGW10006 fires. The compiler cannot verify what the expanded value will be at runtime.
**Rationale:** Shell variables are expanded at runtime by the shell interpreter, not at compile time. When a variable appears in an IO context — such as a file path argument to `cat`, `cp`, `curl`, or a redirection target — the compiler cannot determine the actual resource being accessed. The expanded value could fall outside declared `{_}` permission scopes. This warning is specific to shell code; other languages have their own variable resolution handled by `trace_assignment`.
**Detection:** During the shell AST walk (via tree-sitter-bash):
1. Identify command nodes that map to IO operations (file access, network, redirections)
2. For each argument position that corresponds to a resource (file path, URL):
   - Check if the argument contains a `variable_expansion` node (`$VAR` or `${VAR}`)
3. If variable expansion is found in an IO-relevant argument, emit PGW10006
4. Pure variable usage in non-IO context (e.g., `echo $MSG`, arithmetic `$(($X + 1))`) does not trigger this warning

**See also:** PGE10011 (shell without capability — must have `{_}` first), PGW10002 (unverifiable foreign IO — equivalent for non-shell languages), [[permissions/foreign-code#Shell Analysis]]

**VALID (no warning):**
```polyglot
[ ] ✓ no variable expansion in IO context
{_} _ShellGrant
   [.] .intent << #Grant
   [.] .category #System
   [.] .capability #Shell
   [.] .scope "/usr/local/bin/*"

{-} -SafeShell
   (-) _ShellGrant
   (-) ;Bash
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;Bash
   [ ]
   [-] -Run.Shell;Bash
      (-) <code.inline <<
         [C] /usr/local/bin/deploy.sh --env production                 [ ] ✓ literal path — verifiable
         [C] echo "Deployment complete"                                [ ] ✓ echo is non-IO in this context
      (-) >Bind#Code:Shell.Output >> >result
```

**WARNING:**
```polyglot
[ ] ⚠ PGW10006 — variable expansion in file IO context
{_} _ShellGrant
   [.] .intent << #Grant
   [.] .category #System
   [.] .capability #Shell
   [.] .scope "/var/log/*"

{-} -LogShell
   (-) _ShellGrant
   (-) ;Bash
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;Bash
   [ ]
   [-] -Run.Shell;Bash
      (-) <code.inline <<
         [C] cat "$LOG_PATH/output.log"                                [ ] ⚠ PGW10006 — $LOG_PATH in file read context
         [C] cp /var/log/app.log "${BACKUP_DIR}/app.log.bak"           [ ] ⚠ PGW10006 — ${BACKUP_DIR} in file write context
      (-) >Bind#Code:Shell.Output >> >result
```

```polyglot
[ ] ⚠ PGW10006 — variable expansion in network context
{_} _ShellGrant
   [.] .intent << #Grant
   [.] .category #System
   [.] .capability #Shell
   [.] .scope "*"

{-} -CurlShell
   (-) _ShellGrant
   (-) ;Bash
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;Bash
   [ ]
   [-] -Run.Shell;Bash
      (-) <code.inline <<
         [C] curl -s "$API_URL/health"                                 [ ] ⚠ PGW10006 — $API_URL in network context
         [C] echo "Status: $?"                                         [ ] ✓ $? is non-IO context — no warning
      (-) >Bind#Code:Shell.Output >> >result
```

**Open point:** None.
