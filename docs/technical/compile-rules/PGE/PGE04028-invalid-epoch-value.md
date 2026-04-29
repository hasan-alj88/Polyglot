---
audience: developer
rule: "4.28"
code: PGE04028
name: Invalid Epoch Value
severity: error
---

# Rule 4.28 — Invalid Epoch Value
`PGE04028`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** A literal value assigned to `#DateTime.Instant.epoch` or passed as `<epoch#int` to `-DT.From.Epoch` must be within the representable epoch range. An epoch value outside the supported range triggers PGE04028. This only applies to literal values — runtime-computed values are validated at runtime.
**Rationale:** Epoch seconds have a finite representable range determined by the target platform's integer width. Detecting out-of-range literals at compile time prevents integer overflow or undefined behavior at runtime.
**Detection:** When the compiler encounters a literal integer assigned to `.Instant.epoch` or `<epoch`, it checks the value falls within the platform's representable range. The exact bounds are implementation-defined but must at minimum cover years 0001–9999 CE.

**See also:** PGE04001 (type mismatch), PGE04027 (missing required DateTime subfield)

**VALID:**
```aljam3
[-] $event#dt
   [.] .Instant
      [.] .epoch << 1711929600

[ ] ✓ valid epoch (2024-04-01 00:00:00 UTC)
```

**INVALID:**
```aljam3
[-] $event#dt
   [.] .Instant
      [.] .epoch << 999999999999999999

[ ] ✗ PGE04028 — epoch value outside representable range
```

## See Also

- [[pglib/types/datetime|DateTime Types]] — `#DateTime.Instant` definition
- [[pglib/pipelines/DT/From.Epoch|-DT.From.Epoch]] — epoch-to-DateTime pipeline
