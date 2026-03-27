---
rule: "1.9"
code: PGE-109
name: Wrapper IO Mismatch
severity: error
---

### Rule 1.9 — Wrapper IO Mismatch
`PGE-109`

**Statement:** The `[=]` wiring lines under a `[W]` wrapper must match the referenced macro's IO contract:
1. **Completeness** — every required `[{]` input in the macro must have a corresponding `[=] ... <<` wiring line.
2. **Type compatibility** — each wired value must be schema-compatible with the macro's declared `[{]` input or `[}]` output type (per PGE-401/PGE-402 rules).
3. **No extra inputs** — a `[=] ... <<` line that does not match any `[{]` input in the macro is an error.
4. **Output capture** — `[=] ... >>` lines must match `[}]` outputs. Capturing a nonexistent output is an error.

**Rationale:** Macros define a typed IO contract via `[{]` and `[}]`. If the wrapper wiring doesn't satisfy this contract, the macro will receive missing or incompatible data at runtime. Compile-time validation ensures every macro invocation is correctly wired before execution.
**Detection:** The compiler matches each `[=]` wiring line under `[W]` against the target macro's `[{]`/`[}]` declarations. Missing required inputs, extra inputs not in the macro, type mismatches, or nonexistent output captures trigger PGE-109.

**See also:** PGE-108 (wrapper must reference macro), PGE-401 (type mismatch), PGE-402 (schema mismatch)

**VALID:**
```polyglot
[ ] ✓ all macro inputs provided, types match, output captured
{M} =W.DB.Transaction
   [{] $connStr#string
   [{] $timeout#int
   [}] $txHandle#string

   [\]
      [r] =DB.Connect
         [=] <connStr << $connStr
         [=] <timeout << $timeout
         [=] >handle >> $txHandle
   [/]
      [r] =DB.Disconnect
         [=] <handle << $txHandle

{=} =ProcessData
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connStr << $connStr         [ ] ✓ matches [{] $connStr#string
      [=] $timeout << $timeout         [ ] ✓ matches [{] $timeout#int
      [=] $txHandle >> $txHandle       [ ] ✓ matches [}] $txHandle#string
```

**INVALID:**
```polyglot
[ ] ✗ PGE-109 — missing required macro input
{M} =W.DB.Transaction
   [{] $connStr#string
   [{] $timeout#int
   [}] $txHandle#string
   [\]
      [r] =DB.Connect
         [=] <connStr << $connStr
         [=] <timeout << $timeout
         [=] >handle >> $txHandle
   [/]
      [r] =DB.Disconnect
         [=] <handle << $txHandle

{=} =ProcessData
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connStr << $connStr         [ ] ✓ provided
                                       [ ] ✗ PGE-109 — $timeout not wired (required by [{])
      [=] $txHandle >> $txHandle
```

```polyglot
[ ] ✗ PGE-109 — extra input not in macro contract
{M} =W.Simple
   [{] $input#string
   [}] $output#string
   [\]
      [r] =Transform
         [=] <in << $input
         [=] >out >> $output
   [/]
      [r] =DoNothing

{=} =UseSimple
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Simple
      [=] $input << $input
      [=] $extra << $extra             [ ] ✗ PGE-109 — no [{] $extra in macro
      [=] $output >> $output
```

```polyglot
[ ] ✗ PGE-109 — capturing nonexistent output
{M} =W.Simple
   [{] $input#string
   [}] $output#string
   [\]
      [r] =Transform
         [=] <in << $input
         [=] >out >> $output
   [/]
      [r] =DoNothing

{=} =UseSimple
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Simple
      [=] $input << $input
      [=] $output >> $output
      [=] $missing >> $missing         [ ] ✗ PGE-109 — no [}] $missing in macro
```

**Open point:** None.
