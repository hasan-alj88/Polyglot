---
audience: developer
rule: "1.24"
code: PGE01024
name: Incompatible Operation Marker
severity: error
---

### Rule 1.24 — Incompatible Operation Marker
`PGE01024`

**Statement:** Each operation (pipeline, collection, expander, error) declares which block element markers it is compatible with. Using an operation with an incompatible marker is a compile error. For example, `[T]` only accepts operations that declare `[T]` compatibility (trigger pipelines like `=T.*`), `[r]` accepts general pipeline calls, `[p]` accepts parallelizable operations, etc.
**Rationale:** Operations have specific roles — trigger pipelines (`=T.*`) handle event initiation, collection operators (`*Into.*`) aggregate expanded data, expanders (`~ForEach.*`) fan out, etc. Using an operation outside its intended context is a semantic error that the compiler must catch. Each stdlib operation declares its allowed markers as part of its definition.
**Detection:** The compiler checks the operation's declared marker compatibility against the marker it is invoked with. If the operation does not declare the marker as allowed, PGE01024 fires.

**Marker compatibility examples:**

| Operation | Allowed Markers |
|-----------|----------------|
| `=T.Call`, `=T.Daily`, `=T.Webhook` | `[T]` |
| `=File.Text.Read`, `=DB.Query` | `[r]`, `[p]`, `[b]` |
| `=Q.Default`, `=Q.Pause.Hard` | `[Q]` |
| `=W.Polyglot`, `=W.DB.Connection` | `[W]` |
| `*Into.Array`, `*Agg.Sum` | `[r]`, `[p]` (collectors) |
| `~ForEach.Array` | expand context |

**VALID:**
```polyglot
[ ] ✓ trigger pipeline used with [T]
[T] =T.Call

[ ] ✓ general pipeline used with [r]
[r] =File.Text.Read
   [=] <path << "/data/input.txt"
   [=] >content >> $text

[ ] ✓ general pipeline used with [p] (parallel)
[p] =File.Text.Read
   [=] <path << "/data/input.txt"
   [=] >content >> $text
```

**INVALID:**
```polyglot
[ ] ✗ PGE01024 — =File.Text.Read is not a trigger pipeline
[T] =File.Text.Read

[ ] ✗ PGE01024 — =Math.Add is not a trigger pipeline
[T] =Math.Add"3"

[ ] ✗ PGE01024 — =T.Call is a trigger, not a general pipeline
[r] =T.Call
```

**Diagnostic:** "Operation `=Name` is not compatible with `[marker]` — check allowed markers"

**Open point:** Stdlib pipeline files need `[_]`-style marker compatibility declarations added. Tracked as future work.
