---
audience: developer
rule: "1.9"
code: PGE01009
name: Wrapper IO Mismatch
severity: error
---

### Rule 1.9 — Wrapper IO Mismatch
`PGE01009`

**Statement:** The `[=]` wiring lines under a `[W]` wrapper must match the referenced wrapper's IO contract:
1. **Completeness** — every required `[{]` input in the wrapper must have a corresponding `[=] ... <<` wiring line.
2. **Type compatibility** — each wired value must be schema-compatible with the wrapper's declared `[{]` input or `[}]` output type (per PGE04001/PGE04002 rules).
3. **No extra inputs** — a `[=] ... <<` line that does not match any `[{]` input in the wrapper is an error.
4. **Output capture** — `[=] ... >>` lines must match `[}]` outputs. Capturing a nonexistent output is an error.

**Rationale:** Wrappers define a typed IO contract via `[{]` and `[}]`. If the wrapper wiring doesn't satisfy this contract, the wrapper will receive missing or incompatible data at runtime. Compile-time validation ensures every wrapper invocation is correctly wired before execution.
**Detection:** The compiler matches each `[=]` wiring line under `[W]` against the target wrapper's `[{]`/`[}]` declarations. Missing required inputs, extra inputs not in the wrapper, type mismatches, or nonexistent output captures trigger PGE01009.

**See also:** PGE01008 (wrapper must reference wrapper definition), PGE04001 (type mismatch), PGE04002 (schema mismatch)

**VALID:**
```polyglot
[ ] ✓ all wrapper inputs provided, types match, output captured
{W} =W.DB.Transaction
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
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connStr << $connStr         [ ] ✓ matches [{] $connStr#string
      [=] $timeout << $timeout         [ ] ✓ matches [{] $timeout#int
      [=] $txHandle >> $txHandle       [ ] ✓ matches [}] $txHandle#string
```

**INVALID:**
```polyglot
[ ] ✗ PGE01009 — missing required wrapper input
{W} =W.DB.Transaction
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
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connStr << $connStr         [ ] ✓ provided
                                       [ ] ✗ PGE01009 — $timeout not wired (required by [{])
      [=] $txHandle >> $txHandle
```

```polyglot
[ ] ✗ PGE01009 — extra input not in wrapper contract
{W} =W.Simple
   [{] $input#string
   [}] $output#string
   [\]
      [r] =Transform
         [=] <in << $input
         [=] >out >> $output
   [/]
      [r] =DoNothing

{=} =UseSimple
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Simple
      [=] $input << $input
      [=] $extra << $extra             [ ] ✗ PGE01009 — no [{] $extra in wrapper
      [=] $output >> $output
```

```polyglot
[ ] ✗ PGE01009 — capturing nonexistent output
{W} =W.Simple
   [{] $input#string
   [}] $output#string
   [\]
      [r] =Transform
         [=] <in << $input
         [=] >out >> $output
   [/]
      [r] =DoNothing

{=} =UseSimple
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Simple
      [=] $input << $input
      [=] $output >> $output
      [=] $missing >> $missing         [ ] ✗ PGE01009 — no [}] $missing in wrapper
```

**Open point:** None.

### See Also

- [[concepts/pipelines/wrappers|Wrappers]] — documents wrapper IO wiring contract, references PGE01009
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01009
