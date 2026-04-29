---
audience: developer
rule: "1.63"
code: PGE01063
name: Invalid Pipeline Marker Order
severity: error
---

# Rule 1.63 — Invalid Pipeline Marker Order
`PGE01063`

**Statement:** Pipeline configuration markers must strictly follow this structural order: `[T]` (Trigger), then `[Q]` (Queue), then `[W]` (Wrapper). Execution blocks (like `[-]`, `[?]`) must come strictly after this setup sequence.
**Rationale:** Polyglot reads top-down, precisely matching its execution lifecycle. Enforcing this order eliminates cognitive load—developers always know exactly where to look for trigger details, concurrency configuration, and telemetry boundaries, in that exact sequence.
**Detection:** The compiler maintains a state machine during block parsing (0=Start, 1=Seen T, 2=Seen Q, 3=Seen W). If a marker is encountered out of order (e.g., `[Q]` before `[T]`, or an execution step before `[W]`), an error is raised.

**VALID:**
```polyglot
[ ] ✓ Strict T, Q, W, Execution order
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >run
```

**INVALID:**
```polyglot
[ ] ✗ PGE01063 — Out of order markers
{-} -ProcessData
   [Q] -Q.Default                          [ ] ✗ PGE01063 — Q must come after T
   [T] -T.Manual
   [W] -W.Polyglot
   [-] >run
```

```polyglot
[ ] ✗ PGE01063 — Execution before W
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [-] >run                                [ ] ✗ PGE01063 — Execution must be after W
   [W] -W.Polyglot
```

**Diagnostic:** "Invalid Pipeline Marker Order. `[Q]` must appear after Trigger `[T]` but before Wrapper `[W]`... (or Execution actions must appear after the setup markers...)"
