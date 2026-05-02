---
audience: automation-builder
type: spec
updated: 2026-04-15
status: draft
---

<!-- @c:concepts/collections/expand -->
<!-- @c:concepts/collections/collect -->
<!-- @u:operators#Collection Operators -->
<!-- @u:technical/ebnf/12-collections -->

## Reassemble Operators (`=*`)

Reassemble operators combine an [[concepts/collections/expand|expander]] (`=`) and a [[concepts/collections/collect|collector]] (`*`) into a single atomic operation. The `=*` prefix reads as "expand, then collect" — fan-out followed by fan-in with no intermediate body logic.

```aljam3
[ ] Sum RAM across all jobs
[-] =*Agg.Sum
   (=) <array << $jobRAMs
   (*) >sum >> $TotalRAM
```

The equivalent expand-collect form requires three blocks:

```aljam3
[-] =ForEach
   (=) <Data << $jobRAMs
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

```aljam3
[-] =*OperatorName
   (=) <inputName << $source       [ ] expander input
   (*) >outputName >> $result      [ ] collector output
```

### Execution Markers

Like expanders and collectors, reassemble operators accept `[-]` (sequential) or `[=]` (parallel) execution markers. The marker controls whether the internal expand step runs items sequentially or in parallel.

### Tree Reorganization (Transposing)

Because the reassembler atomically expands and recollects, it is the native tool for reorganizing DataTrees. By using `=*PermuteLevels` and supplying a `(*) <Permute` input, you can swap the hierarchy of the keys without writing a manual loop. 

For example, transposing a 2D `##Dataframe` (swapping rows and columns):

```aljam3
[ ] Transpose a DataFrame by swapping the Row and Column depths
[-] =*PermuteLevels
   (=) <Data << $users       [ ] since its final the Data type is known
   (*) <Permute << [1, 0]    [ ] Level 1 (Columns) becomes Level 0 (Rows)
   (*) >Data >> >transposedDF
```

This acts as pure structural manipulation; the compiler unrolls this into an exhaustive traversal that collects the leaves back into a new tree using the reversed key paths.

**Compile-Time Safety Note:** 
`=*PermuteLevels` is only valid on **Uniform Trees** (where all branches share the exact same schema structure, like a DataFrame). If `$users` is a non-uniform tree (meaning branches have different schemas), attempting to permute the levels will trigger a strict **Compiler Error**, as the resulting structure cannot be safely inferred.

### Namespaces

Reassemble operators combine the expander and collector namespaces:

- `=*Agg.*` — expand and reduce to scalar ([[jm3lib/reassemblers/Agg/INDEX|reference]])
- `=*Collect` — expand and collect into a different collection type

No `[@]` import needed.

## See Also

- [[concepts/collections/expand|Expand Operators]] — `=` fan-out operators
- [[concepts/collections/collect|Collect Operators]] — `*` fan-in operators
- [[jm3lib/reassemblers/INDEX|Reassemble Operators (jm3lib)]] — full operator reference
- [[concepts/collections/examples|Examples]] — expand/transform/collect patterns
