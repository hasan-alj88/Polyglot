---
rule: "9.12"
code: PGE-912
name: Circular Pipeline Call
severity: error
---

### Rule 9.12 — Circular Pipeline Call
`PGE-912`

**Statement:** Pipelines within the same package must not call each other in a cycle — directly or transitively. Self-calls and mutual call loops are compile errors.
**Rationale:** Polyglot has no recursion mechanism — no base case construct, no call stack, and no way to terminate a recursive cycle. A circular call graph would execute forever. Cross-package call cycles are already caught by PGE-902 (circular package dependency); this rule covers intra-package cycles.
**Detection:** The compiler builds a directed call graph from all `[r]`/`[p]`/`[b]` pipeline references within a package and rejects any cycle.

**See also:** PGE-902 (circular package dependency — cross-package import cycles)

**VALID:**
```polyglot
[ ] ✓ linear call chain — no cycle
{=} =Ingest
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >result;string
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
   [=] <data;string
   [=] >clean;string
   $clean <~ $data
   >> >clean

{=} =Store
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <item;string
   [=] >ok;string
   $ok <~ "stored"
   >> >ok
```

**INVALID:**
```polyglot
[ ] ✗ PGE-912 — direct mutual recursion
{=} =Ping
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <msg;string
   [=] >out;string
   [r] =Pong                            [ ] ✗ PGE-912 — Ping→Pong→Ping
      [=] <msg << $msg
      [=] >out >> >out

{=} =Pong
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <msg;string
   [=] >out;string
   [r] =Ping
      [=] <msg << $msg
      [=] >out >> >out
```

```polyglot
[ ] ✗ PGE-912 — self-call
{=} =Recurse
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >result;string
   [r] =Recurse                         [ ] ✗ PGE-912 — calls itself
      [=] <data << $data
      [=] >result >> >result
```

**Open point:** None.
