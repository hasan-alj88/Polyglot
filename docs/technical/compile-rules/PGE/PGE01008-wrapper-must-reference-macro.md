---
rule: "1.8"
code: PGE01008
name: Wrapper Must Reference Wrapper Definition
severity: error
---

### Rule 1.8 — Wrapper Must Reference Wrapper Definition
`PGE01008`

**Statement:** A `[W]` wrapper element must reference a `{W}` wrapper definition. Referencing a `{=}` pipeline, a `{#}` data block, a `{M}` macro, or a nonexistent definition is a compile error.
**Rationale:** Wrappers exist to apply setup/cleanup lifecycle logic around pipeline execution. Only wrapper definitions (`{W}`) provide this lifecycle structure — pipelines have their own trigger/queue/execution lifecycle and cannot be composed as wrappers, and macros (`{M}`) are for compile-time type generation. Catching an invalid reference at compile time prevents runtime confusion about missing setup/cleanup hooks.
**Detection:** The compiler resolves the `[W]` target name against all definitions in scope (including imports). If the target resolves to a non-wrapper definition, or resolves to nothing, PGE01008 fires.

**See also:** PGE01004 (wrapper structural constraints), PGE01009 (wrapper IO mismatch)

**VALID:**
```polyglot
[ ] ✓ wrapper references a {W} wrapper definition
{W} =W.DB.Transaction
   [{] $connStr#string
   [}] $txHandle#string
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
   [W] =W.DB.Transaction               [ ] ✓ references a {W} wrapper
      [=] $connStr << $connStr
      [=] $txHandle >> $txHandle
```

**INVALID:**
```polyglot
[ ] ✗ PGE01008 — wrapper references a {=} pipeline, not a {W} wrapper
{=} =NotAWrapper
   [t] =T.Call
   [Q] =Q.Default
   [r] =DoSomething

{=} =ProcessData
   [t] =T.Call
   [Q] =Q.Default
   [W] =NotAWrapper                    [ ] ✗ PGE01008 — target is a pipeline, not a wrapper
      [=] $input << $input
```

```polyglot
[ ] ✗ PGE01008 — wrapper references a nonexistent definition
{=} =ProcessData
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.DoesNotExist                 [ ] ✗ PGE01008 — no definition found
      [=] $input << $input
```

**Open point:** None.

### See Also

- [[concepts/pipelines/wrappers|Wrappers]] — documents wrapper usage, references PGE01008
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01008
