---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# Collection Operators

<!-- @u:collections -->
<!-- @u:technical/ebnf/12-collections -->
<!-- @u:technical/edge-cases/12-collections -->
Two operator prefixes for collection processing. For the full operator reference and semantics, see [[concepts/collections/INDEX|collections]].

| Prefix | Operation | Example |
|--------|-----------|---------|
| `=ForEach` | Expand (iterate) | `=ForEach.Array` — iterate over collection |
| `*` | Collect (aggregate) | `*Into.Array` — collect results into collection |

These are **operators**, not identifier prefixes. The 5 identifier prefixes (`@`, `#`, `-`, `$`, `!`) remain unchanged.

## Example: Transform an Array

```aljam3
[-] =ForEach.Array
   (=) <Array << $SomeArray
   (=) >item >> $item
   [ ]
   [ ] Here we can do something with the $item
   ...
   [-] *Into.Array
      (*) <item << $item
      (*) >Array >> $NewArray
   [ ] $NewArray can be used one level up in the pipeline
```

## Wait and Collect IO

Inside `(*)` collector blocks, the `<<`/`>>` direction operators distinguish wait inputs from collect outputs:

| Form | Semantics |
|------|-----------|
| `(*) << $var` | **Wait input** — waits for `$var` to be Final. Variable **stays accessible** after. |
| `(*) >> $var` | **Collect output** — in race collectors, losing inputs are **cancelled**; only the `>>` output survives. |

This is the same `<<`/`>>` direction convention used throughout the language:

| Context | `<<` (input / push-left) | `>>` (output / push-right) |
|---------|--------------------------|----------------------------|
| Pipeline IO `(-)` | `<input << $var` — push-left value, waits for Final | `>output >> $result` — push-right, makes Final |
| Expand IO `(=)` | `<Array << $items` — push-left collection in | `>item >> $item` — push-right each item out |
| Collect IO `(*)` | `(*) << $var` — waits for Final, var stays accessible | `(*) >> $out` — receives collected value, inputs cancelled |

See [[concepts/collections/collect#Collect-All & Race Collectors]] for the collectors that use these forms.

> **Disambiguation:** `(*) <<` / `(*) >>` (collector IO) and `(>)` / `(<)` (IO parameter handling) are distinct marker sets. `(*) <<` / `(*) >>` appear inside `(*)` collector blocks for wait/race semantics. `(>)` / `(<)` appear under `(-)` IO lines for parameter handling (e.g., error fallback). See [[syntax/io/io-parameter-handling|IO Parameter Handling]].

## Direct Output Port Writing

Collector outputs can write directly to a pipeline output port using the `>` prefix:

```aljam3
[-] *Agg.Concatenate
   (*) <string << $value
   (*) >result >> >pipelineOutput
```

The target output port reaches **Final** state after the collector writes to it — no other push to that port is allowed. See [[variable-lifecycle#Final]].
