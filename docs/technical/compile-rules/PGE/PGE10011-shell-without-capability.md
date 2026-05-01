---
audience: developer
rule: "9.25"
code: PGE10011
name: Shell Without Capability
severity: error
---

# Rule 9.25 — Shell Without Capability
`PGE10011`

<!-- @u:concepts/permissions/foreign-code -->
<!-- @u:concepts/permissions/enforcement -->
<!-- @u:aj3lib/pipelines/Run/Shell -->

**Statement:** A `{-}` pipeline that uses `-Run.Shell` must declare a `(-) _PermName` reference to a `{_}` permission object with `.category #System` and `.capability #Shell`. If no such permission is declared, PGE10011 fires.
**Rationale:** `-Run.Shell` executes arbitrary shell commands on the host system. Without an explicit `{_}` grant for `#System.#Shell`, the pipeline could execute any system command — file deletion, process spawning, network access — without the compiler or auditor seeing the declared intent. This rule ensures that every shell-executing pipeline has an explicit, auditable permission grant.
**Detection:** When the compiler encounters a `{-}` pipeline definition containing a `-Run.Shell` call:
1. Collect all `(-) _PermName` IO declarations on the pipeline
2. Resolve each `_PermName` to its `{_}` definition
3. Check that at least one `{_}` has `.category #System` and `.capability #Shell`
4. If none match, PGE10011 fires on the `-Run.Shell` call line

**See also:** PGE10004 (undeclared permission — general case), PGW10006 (shell variable expansion warning), [[permissions/foreign-code#Shell Analysis]]

**VALID:**
```aljam3
[ ] ✓ -Run.Shell with matching {_} #System.#Shell grant
{_} _DeployShell
   [.] .intent << #Grant
   [.] .category #System
   [.] .capability #Shell
   [.] .scope "/usr/local/bin/*"

{-} -DeployApp
   (-) _DeployShell
   (-) ;Bash
   [T] -T.Git.Push
   [Q] -Q.Default
   [W] -W.Env;Bash
   [ ]
   [-] -Run.Shell;Bash
      (-) <code.inline <<
         [C] /usr/local/bin/deploy.sh --env production
      (-) >Bind#Code:Shell.Output >> >result
```

**INVALID:**
```aljam3
[ ] ✗ PGE10011 — -Run.Shell with no {_} #System.#Shell permission
{_} _FileOnly
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/app.log"

{-} -BadDeploy
   (-) _FileOnly                                                    [ ] ✗ wrong category — #File, not #System
   (-) ;Bash
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;Bash
   [ ]
   [-] -Run.Shell;Bash                                              [ ] ✗ PGE10011 — no #System.#Shell permission
      (-) <code.inline <<
         [C] rm -rf /tmp/cache
      (-) >Bind#Code:Shell.Output >> >result
```

```aljam3
[ ] ✗ PGE10011 — -Run.Shell with no permission declarations at all
{-} -NakedShell
   (-) ;Bash
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;Bash
   [ ]
   [-] -Run.Shell;Bash                                              [ ] ✗ PGE10011 — no (-) _PermName declared
      (-) <code.inline <<
         [C] echo "hello"
      (-) >Bind#Code:Shell.Output >> >result
```

**Open point:** None.
