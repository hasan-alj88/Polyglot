---
audience: developer
rule: "9.8"
code: PGW10007
name: Sandbox-Only Enforcement Active
severity: warning
---

# Rule 9.8 — Sandbox-Only Enforcement Active
`PGW10007`

<!-- @c:technical/spec/job-sandbox#Opaque Code -->
<!-- @c:technical/compiler/compliance-report -->
<!-- @u:concepts/permissions/enforcement#Foreign Code Sandbox -->

**Statement:** When a pipeline includes `[!] _Unsafe.SandboxOnly`, PGW10007 fires as an informational warning. It notes that maximum sandbox enforcement is active and lists any AST-invisible errors (PGE10014) that were suppressed to warnings.
**Rationale:** `_Unsafe.SandboxOnly` activates all isolation layers (Landlock, seccomp, namespaces, cgroups, USER_NOTIF supervisor) at a performance cost. The warning ensures this trade-off is visible in build output and compliance reports. It also surfaces the suppressed PGE10014 findings — the compiler still analyzes whatever it can, but instead of blocking compilation for AST-invisible constructs, it reports them as best-effort findings. This gives developers visibility into what the compiler could not verify.
**Detection:** When the compiler encounters `[!] _Unsafe.SandboxOnly` on a `-Run.*` call:
1. Emit PGW10007 as an informational warning
2. Run AST analysis on available code (even if opaque/incomplete)
3. Collect any PGE10014 violations that would normally be errors
4. Suppress those PGE10014 violations — downgrade from error to warning
5. List suppressed findings in the PGW10007 output and the [[compliance-report|compliance report]]

**See also:** PGE10015 (opaque binary without `_Unsafe.SandboxOnly`), PGE10016 (missing mandatory metadata), PGE10014 (AST-invisible foreign code — suppressed under `_Unsafe.SandboxOnly`), [[spec/job-sandbox#Opaque Code]], [[compiler/compliance-report]]

**WARNING:**
```aljam3
[ ] ⚠ PGW10007 — _Unsafe.SandboxOnly active, maximum sandbox enforcement
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -ProcessData
   [.] %Authors << "jane.doe@company.com"
   [.] %Description << "Legacy Go binary for report generation — no source available"
   [.] %Version << "1.2.0"
   (-) _FileGrant
   (-) ;GoEnv
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;GoEnv
   [ ]
   [-] -Run.Go.CLI;GoEnv "compiled-binary"                               [ ] ⚠ PGW10007 — sandbox-only enforcement active
      [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

```aljam3
[ ] ⚠ PGW10007 with suppressed PGE10014 findings
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -LegacyScript
   [.] %Authors << "security-team@company.com"
   [.] %Description << "Legacy Python script using eval — migrating to safe API"
   [.] %Version << "0.9.0"
   (-) _FileGrant
   (-) ;PythonUV
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;PythonUV
   [ ]
   [-] -Run.Python.Script;PythonUV
      [!] _Unsafe.SandboxOnly                                            [ ] ⚠ PGW10007 — sandbox-only active
      (-) <code.inline <<
         [C] config = eval(open("/data/config.txt").read())               [ ] ⚠ PGE10014 suppressed — eval() found
         [C] result = str(config.get("key", "default"))
      (-) <Bind#Record
      (-) >Bind#Record
         [.] .result#string >> >result
```

**Open point:** None.
