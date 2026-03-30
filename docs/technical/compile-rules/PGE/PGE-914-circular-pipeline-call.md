---
rule: "9.14"
code: PGE-914
name: Circular Pipeline Call
severity: error
---

### Rule 9.14 — Circular Pipeline Call
`PGE-914`

**Statement:** Pipelines within the same package must not call each other in a cycle — directly or transitively. Self-calls and mutual call loops are compile errors.
**Rationale:** Polyglot has no recursion mechanism — no base case construct, no call stack, and no way to terminate a recursive cycle. A circular call graph would execute forever. Cross-package call cycles are already caught by PGE-902 (circular package dependency); this rule covers intra-package cycles.
**Detection:** The compiler builds a directed call graph: each `{=}` pipeline in the package is a node, each `[r]`/`[p]`/`[b]` reference to another same-package pipeline is an edge. A topological sort is attempted — if it fails, a cycle exists. The diagnostic reports the full cycle path (e.g., `=A → =B → =C → =A`). Cross-package calls are excluded (covered by PGE-902).

**See also:** PGE-902 (circular package dependency — cross-package import cycles)

**VALID:**
```polyglot
[ ] ✓ linear call chain — no cycle
{=} =Ingest
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >result#string
   [r] =Transform
      [=] <data << "raw"
      [=] >clean >> $cleaned
   [r] =Store
      [=] <item << $cleaned
      [=] >ok >> >result

{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >clean#string
   $clean <~ $data
   >> >clean

{=} =Store
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <item#string
   [=] >ok#string
   $ok <~ "stored"
   >> >ok
```

**INVALID:**
```polyglot
[ ] ✗ PGE-914 — self-call
{=} =Recurse
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >result#string
   [r] =Recurse                         [ ] ✗ PGE-914 — calls itself
      [=] <data << $data
      [=] >result >> >result
```

```polyglot
[ ] ✗ PGE-914 — direct mutual recursion
{=} =Ping
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <msg#string
   [=] >out#string
   [r] =Pong                            [ ] ✗ PGE-914 — Ping→Pong→Ping
      [=] <msg << $msg
      [=] >out >> >out

{=} =Pong
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <msg#string
   [=] >out#string
   [r] =Ping
      [=] <msg << $msg
      [=] >out >> >out
```

```polyglot
[ ] ✗ PGE-914 — transitive cycle (A→B→C→A)
{=} =StepA
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <in#string
   [=] >out#string
   [r] =StepB
      [=] <in << $in
      [=] >out >> >out

{=} =StepB
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <in#string
   [=] >out#string
   [r] =StepC
      [=] <in << $in
      [=] >out >> >out

{=} =StepC
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <in#string
   [=] >out#string
   [r] =StepA                           [ ] ✗ PGE-914 — StepA→StepB→StepC→StepA
      [=] <in << $in
      [=] >out >> >out
```

**Diagnostic:** "Circular pipeline call detected: `=A → =B → =C → =A` — Polyglot does not support recursion"

### See Also

- [[user/syntax/packages|Packages]] — references PGE-914 in dependency rules

**Open point:** None.
