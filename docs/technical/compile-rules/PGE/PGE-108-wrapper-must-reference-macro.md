---
rule: "1.8"
code: PGE-108
name: Wrapper Must Reference Macro
severity: error
---

### Rule 1.8 — Wrapper Must Reference Macro
`PGE-108`

**Statement:** A `[W]` wrapper element must reference a `{M}` macro definition. Referencing a `{=}` pipeline, a `{#}` data block, or a nonexistent definition is a compile error.
**Rationale:** Wrappers exist to apply setup/cleanup lifecycle logic around pipeline execution. Only macros (`{M}`) provide this lifecycle structure — pipelines have their own trigger/queue/execution lifecycle and cannot be composed as wrappers. Catching an invalid reference at compile time prevents runtime confusion about missing setup/cleanup hooks.
**Detection:** The compiler resolves the `[W]` target name against all definitions in scope (including imports). If the target resolves to a non-macro definition, or resolves to nothing, PGE-108 fires.

**See also:** PGE-104 (macro structural constraints), PGE-109 (wrapper IO mismatch)

**VALID:**
```polyglot
[ ] ✓ wrapper references a {M} macro
{M} =W.DB.Transaction
   [{] $connStr;string
   [}] $txHandle;string
   [\]
      [r] =DB.Connect
         [=] <connStr << $connStr
         [=] >handle >> $txHandle
   [/]
      [r] =DB.Disconnect
         [=] <handle << $txHandle

{=} =ProcessData
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction               [ ] ✓ references a {M} macro
      [=] $connStr << $connStr
      [=] $txHandle >> $txHandle
```

**INVALID:**
```polyglot
[ ] ✗ PGE-108 — wrapper references a {=} pipeline, not a {M} macro
{=} =NotAMacro
   [t] =T.Call
   [Q] =Q.Default
   [r] =DoSomething

{=} =ProcessData
   [t] =T.Call
   [Q] =Q.Default
   [W] =NotAMacro                      [ ] ✗ PGE-108 — target is a pipeline, not a macro
      [=] $input << $input
```

```polyglot
[ ] ✗ PGE-108 — wrapper references a nonexistent definition
{=} =ProcessData
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.DoesNotExist                 [ ] ✗ PGE-108 — no definition found
      [=] $input << $input
```

**Open point:** None.
