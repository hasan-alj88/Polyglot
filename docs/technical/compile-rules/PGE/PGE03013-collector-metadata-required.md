---
audience: developer
rule: "3.13"
code: PGE03013
name: Collector Metadata Required
severity: error
---

# Rule 3.13 — Collector Metadata Required
`PGE03013`

<!-- @u:technical/spec/collector-definitions -->

**Statement:** Every `{*}` collector definition must declare `.category` (`#CollectorCategory`), `.scope` (`#CollectorScope`), and `.overflow` (`#OverflowStrategy`) metadata fields via `[%]`. Missing any of these three mandatory fields is a compile error.
**Rationale:** The compiler uses these fields to determine invocation context (PGE03014/PGE03015), overflow behavior (PGE03023), and reconciliation strategy. Without them, the collector cannot be correctly dispatched.
**Detection:** After parsing the `{*}` block, the compiler checks that all three `[%]` fields are present. If any is missing, PGE03013 fires with the missing field name.

**VALID:**
```aljam3
{*} *MyCollector
   [%] .category << #CollectorCategory.Agg
   [%] .scope << #CollectorScope.Expand
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame
   ...
```

**INVALID:**
```aljam3
[ ] ✗ PGE03013 — missing .scope and .overflow
{*} *MyCollector
   [%] .category << #CollectorCategory.Agg
   (*) <Incoming#IncomingDataFrame
   ...
```

## See Also

- [[technical/spec/collector-definitions|Collector Definitions]] — `{*}` block specification
