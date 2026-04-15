---
audience: automation-builder
type: spec
updated: 2026-04-15
status: draft
---

<!-- @c:concepts/collections/expand -->
<!-- @c:concepts/collections/collect -->

## Reassemble Operators (`=*`)

Reassemble operators combine an [[concepts/collections/expand|expander]] (`=`) and a [[concepts/collections/collect|collector]] (`*`) into a single atomic operation. The `=*` prefix reads as "expand, then collect" — fan-out followed by fan-in with no intermediate body logic.

```polyglot
[ ] Sum RAM across all jobs
[-] =*Agg.Sum
   (=) <array << $jobRAMs
   (*) >sum >> $TotalRAM
```

The equivalent expand-collect form requires three blocks:

```polyglot
[-] =ForEach.Array
   (=) <Array << $jobRAMs
   (=) >item >> $ram

   [-] *Agg.Sum
      (*) <number << $ram
      (*) >sum >> $TotalRAM
```

The `=*` form eliminates the intermediate variable and the nested collector. The compiler expands `=*` into the equivalent `=` + `*` pair — no new runtime instruction is created.

### When to Use `=*`

Use `=*` when the operation is a direct collection-to-scalar or collection-to-collection transformation with **no intermediate logic**. If you need conditionals, pipeline calls, or error handling between expand and collect, use the full `=` ... `*` form.

| Use `=*` when | Use `=` ... `*` when |
|---|---|
| Summing, counting, averaging a field | Transforming each item with a pipeline call |
| Collecting items into a different collection type | Filtering items with conditionals |
| No per-item logic needed | Error handling on per-item processing |

### IO Pattern

Reassemble operators use both `(=)` (expander input) and `(*)` (collector output) IO markers. The expander feeds directly into the collector — no `(=) >item` output is declared.

```polyglot
[-] =*OperatorName
   (=) <inputName << $source       [ ] expander input
   (*) >outputName >> $result      [ ] collector output
```

### Execution Markers

Like expanders and collectors, reassemble operators accept `[-]` (sequential) or `[=]` (parallel) execution markers. The marker controls whether the internal expand step runs items sequentially or in parallel.

### Namespaces

Reassemble operators combine the expander and collector namespaces:

- `=*Agg.*` — expand and reduce to scalar ([[pglib/reassemblers/Agg/INDEX|reference]])
- `=*Into.*` — expand and collect into a different collection type ([[pglib/reassemblers/Into/INDEX|reference]])

No `[@]` import needed.

## See Also

- [[concepts/collections/expand|Expand Operators]] — `=` fan-out operators
- [[concepts/collections/collect|Collect Operators]] — `*` fan-in operators
- [[pglib/reassemblers/INDEX|Reassemble Operators (pglib)]] — full operator reference
- [[concepts/collections/examples|Examples]] — expand/transform/collect patterns
