---
audience: developer
rule: "4.27"
code: PGE04027
name: Missing Required DateTime Subfield
severity: error
---

### Rule 4.27 — Missing Required DateTime Subfield
`PGE04027`

**Statement:** When constructing a `#DateTime` via direct field assignment (not via `-DT.From.*` pipelines), the `.Instant.epoch` subfield is mandatory. A `#DateTime` without `.Instant.epoch` is incomplete and triggers PGE04027. Other subfields (`.Civil`, `.Calendar`, etc.) are optional — the runtime derives them from epoch when accessed.
**Rationale:** `.Instant.epoch` is the single source of truth in `#DateTime`. All calendar projections, civil representations, and time unit conversions are computed from the epoch value. Without it, no conversions are possible.
**Detection:** When the compiler encounters a `#dt` variable being constructed via direct field assignment, it checks that `.Instant.epoch` is assigned before the variable reaches Final state. Missing `.Instant.epoch` at the point of finalization triggers PGE04027.

**See also:** PGE04002 (schema mismatch)

**VALID:**
```polyglot
[-] $event#dt
   [.] .Instant
      [.] .epoch << 1711929600
      [.] .nano << 0
      [.] .precision << #Precision.Second

[ ] ✓ .Instant.epoch present
```

**INVALID:**
```polyglot
[-] $event#dt
   [.] .Civil
      [.] .date
         [.] .year << 2026
         [.] .month << 4
         [.] .day << 1

[ ] ✗ PGE04027 — .Instant.epoch is missing; use -DT.From.Parts instead
```

### See Also

- [[pglib/types/datetime|DateTime Types]] — `#DateTime.Instant.epoch` definition
- [[pglib/pipelines/DT/From.Parts|-DT.From.Parts]] — human-readable DateTime construction
