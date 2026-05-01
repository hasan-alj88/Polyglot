---
audience: developer
rule: "9.30"
code: PGE10016
name: Missing Mandatory Metadata for Sandbox-Only
severity: error
---

# Rule 9.30 — Missing Mandatory Metadata for Sandbox-Only
`PGE10016`

<!-- @c:technical/spec/job-sandbox#Opaque Code -->
<!-- @u:concepts/permissions/enforcement#Foreign Code Sandbox -->

**Statement:** A pipeline using `[!] _Unsafe.SandboxOnly` must declare `%Authors`, `%Description`, and `%Version` metadata fields. Missing any of these is a compile error.
**Rationale:** When a pipeline acknowledges opaque execution via `_Unsafe.SandboxOnly`, the sandbox becomes the primary enforcement mechanism instead of compile-time AST verification. The mandatory metadata creates an audit trail: who approved this opaque code (`%Authors`), what it does and why it needs opaque execution (`%Description`), and which version is running (`%Version`). Without this accountability, opaque code could be introduced without any traceable approval chain.
**Detection:** When the compiler encounters `[!] _Unsafe.SandboxOnly` on a `-Run.*` call:
1. Check the enclosing pipeline definition for `%Authors` metadata field
2. Check for `%Description` metadata field
3. Check for `%Version` metadata field
4. If any of the three is missing — emit PGE10016, listing which fields are absent

**See also:** PGE10015 (opaque binary without `_Unsafe.SandboxOnly` — fires before this rule), PGW10007 (informational warning when `_Unsafe.SandboxOnly` is active), [[spec/job-sandbox#Opaque Code]]

**VALID:**
```aljam3
[ ] ✓ all three metadata fields present
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
   [-] -Run.Go.CLI;GoEnv "compiled-binary"
      [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

**INVALID:**
```aljam3
[ ] ✗ PGE10016 — missing %Authors
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -ProcessData
   [.] %Description << "Legacy Go binary for report generation"
   [.] %Version << "1.2.0"
   (-) _FileGrant                                                        [ ] ✗ PGE10016 — missing %Authors
   (-) ;GoEnv
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;GoEnv
   [ ]
   [-] -Run.Go.CLI;GoEnv "compiled-binary"
      [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

```aljam3
[ ] ✗ PGE10016 — missing all three metadata fields
{_} _FileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/data/*"
   [.] .path "/data/*"

{-} -ProcessData
   (-) _FileGrant                                                        [ ] ✗ PGE10016 — missing %Authors, %Description, %Version
   (-) ;GoEnv
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Env;GoEnv
   [ ]
   [-] -Run.Go.CLI;GoEnv "compiled-binary"
      [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

**Open point:** None.
