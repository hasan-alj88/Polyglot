---
audience: developer
rule: "9.29"
code: PGE10015
name: Opaque Binary Without Sandbox Acknowledgment
severity: error
---

# Rule 9.29 — Opaque Binary Without Sandbox Acknowledgment
`PGE10015`

<!-- @c:technical/spec/job-sandbox#Opaque Code -->
<!-- @u:concepts/permissions/enforcement#Foreign Code Sandbox -->

**Statement:** A `-Run.*.CLI` pipeline or any pipeline where the compiler cannot perform full AST analysis on the foreign code (compiled binary, no tree-sitter support) must include `[!] _Unsafe.SandboxOnly`. Without this acknowledgment, the pipeline is a compile error.
**Rationale:** When the compiler cannot parse foreign code, it cannot verify permission compliance through AST analysis. The developer must explicitly acknowledge that the OS-level sandbox is the primary enforcement mechanism by adding `[!] _Unsafe.SandboxOnly`. This shifts liability clearly: the developer accepts that sandbox containment, not compile-time verification, is the line of defense. The acknowledgment also triggers maximum sandbox enforcement (all isolation layers active) and mandatory metadata requirements (see PGE10016).
**Detection:** During the foreign code analysis phase, the compiler checks whether the target code is analyzable:
1. If the `-Run.*` mode is `.CLI` — binary is always opaque
2. If the language has no tree-sitter parser in the [[compiler/parsers|parser registry]] — code is opaque
3. If opaque and no `[!] _Unsafe.SandboxOnly` is present on the `-Run.*` call — emit PGE10015

**See also:** PGE10016 (missing mandatory metadata — fires when `_Unsafe.SandboxOnly` is present but metadata is incomplete), PGW10007 (informational warning when `_Unsafe.SandboxOnly` is active), PGE10014 (AST-invisible constructs — suppressed to warning under `_Unsafe.SandboxOnly`), [[spec/job-sandbox#Opaque Code]]

**VALID:**
```aljam3
[ ] ✓ _Unsafe.SandboxOnly acknowledges opaque binary
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
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;GoEnv
   [ ]
   [-] -Run.Go.CLI;GoEnv "compiled-binary"
      [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

```aljam3
[ ] ✓ source code available — switch to .Function, no _Unsafe.SandboxOnly needed
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -ProcessDataSafe
   (-) _FileGrant
   (-) ;GoEnv
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;GoEnv
   [ ]
   [-] -Run.Go.Function;GoEnv
      (-) <code.file << "/src/process.go"
      (-) <func << "ProcessData"
      (-) <args#Record
         [.] .input#string << "/data/in.csv"
      (-) >output#Record
         [.] .result#string >> >result
```

**INVALID:**
```aljam3
[ ] ✗ PGE10015 — opaque binary without _Unsafe.SandboxOnly
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -ProcessData
   (-) _FileGrant
   (-) ;GoEnv
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;GoEnv
   [ ]
   [-] -Run.Go.CLI;GoEnv "compiled-binary"                               [ ] ✗ PGE10015 — no [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

```aljam3
[ ] ✗ PGE10015 — unsupported language has no tree-sitter parser
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -CustomLangScript
   (-) _FileGrant
   (-) ;CustomEnv
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Env;CustomEnv
   [ ]
   [-] -Run.Custom.Script;CustomEnv                                      [ ] ✗ PGE10015 — no parser for "Custom" language
      (-) <code.file << "/src/process.custom"
      (-) >output#string >> >result
```

**Open point:** None.
