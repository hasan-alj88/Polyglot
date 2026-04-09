---
audience: developer
rule: "9.14"
code: PGE09013
name: Circular Pipeline Call
severity: error
---

### Rule 9.14 — Circular Pipeline Call
`PGE09013`

**Statement:** Pipelines within the same package must not call each other in a cycle — directly or transitively. Self-calls and mutual call loops are compile errors.
**Rationale:** Polyglot has no recursion mechanism — no base case construct, no call stack, and no way to terminate a recursive cycle. A circular call graph would execute forever. Cross-package call cycles are already caught by PGE09002 (circular package dependency); this rule covers intra-package cycles.
**Detection:** The compiler builds a directed call graph: each `{-}` pipeline in the package is a node, each `[-]`/`[=]`/`[b]` reference to another same-package pipeline is an edge. A topological sort is attempted — if it fails, a cycle exists. The diagnostic reports the full cycle path (e.g., `-A → -B → -C → -A`). Cross-package calls are excluded (covered by PGE09002).

**See also:** PGE09002 (circular package dependency — cross-package import cycles)

**VALID:**
```polyglot
[ ] ✓ linear call chain — no cycle
{-} -Ingest
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >result#string
   [-] -Transform
      (-) <data << "raw"
      (-) >clean >> $cleaned
   [-] -Store
      (-) <item << $cleaned
      (-) >ok >> >result

{-} -Transform
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >clean#string
   $clean <~ $data
   >> >clean

{-} -Store
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <item#string
   (-) >ok#string
   $ok <~ "stored"
   >> >ok
```

**INVALID:**
```polyglot
[ ] ✗ PGE09013 — self-call
{-} -Recurse
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >result#string
   [-] -Recurse                         [ ] ✗ PGE09013 — calls itself
      (-) <data << $data
      (-) >result >> >result
```

```polyglot
[ ] ✗ PGE09013 — direct mutual recursion
{-} -Ping
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <msg#string
   (-) >out#string
   [-] -Pong                            [ ] ✗ PGE09013 — Ping→Pong→Ping
      (-) <msg << $msg
      (-) >out >> >out

{-} -Pong
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <msg#string
   (-) >out#string
   [-] -Ping
      (-) <msg << $msg
      (-) >out >> >out
```

```polyglot
[ ] ✗ PGE09013 — transitive cycle (A→B→C→A)
{-} -StepA
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <in#string
   (-) >out#string
   [-] -StepB
      (-) <in << $in
      (-) >out >> >out

{-} -StepB
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <in#string
   (-) >out#string
   [-] -StepC
      (-) <in << $in
      (-) >out >> >out

{-} -StepC
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <in#string
   (-) >out#string
   [-] -StepA                           [ ] ✗ PGE09013 — StepA→StepB→StepC→StepA
      (-) <in << $in
      (-) >out >> >out
```

**Diagnostic:** "Circular pipeline call detected: `-A → -B → -C → -A` — Polyglot does not support recursion"

### See Also

- [[user/syntax/packages|Packages]] — references PGE09013 in dependency rules

**Open point:** None.
