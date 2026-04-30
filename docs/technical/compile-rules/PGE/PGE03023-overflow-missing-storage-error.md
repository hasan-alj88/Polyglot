---
audience: developer
rule: "3.23"
code: PGE03023
name: Overflow Strategy Missing Storage Error
severity: error
---

# Rule 3.23 — Overflow Strategy Missing Storage Error
`PGE03023`

<!-- @u:technical/spec/collector-definitions -->
<!-- @u:technical/plan/queue-manager/overflow -->

**Statement:** A `{*}` collector with `.overflow` set to any value other than `#OverflowStrategy.InMemoryOnly` must declare `(*) !Storage.Space` in its IO section. Missing this error declaration is a compile error.
**Rationale:** Collectors that spill to disk (Append, Merge, Custom) can exhaust storage. The `!Storage.Space` error declaration ensures the caller can handle this failure. InMemoryOnly collectors don't touch disk and don't need it.
**Detection:** The compiler checks `.overflow` metadata. If not `.InMemoryOnly`, it scans the `(*)` IO for `!Storage.Space`. If absent, PGE03023 fires.

**VALID:**
```aljam3
{*} *Into.Text.Append
   [%] .overflow << #OverflowStrategy.Append
   (*) !Storage.Space                     [ ] ✓ declared
   ...
```

**INVALID:**
```aljam3
[ ] ✗ PGE03023 — Append overflow but no !Storage.Space
{*} *Into.Text.Append
   [%] .overflow << #OverflowStrategy.Append
   (*) <Incoming#IncomingDataFrame
   (*) >text#RawString
   ...
```

## See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — Ground Rule 9
- [[aj3lib/types/OverflowStrategy|#OverflowStrategy]] — overflow variants
