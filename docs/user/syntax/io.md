---
audience: user
type: specification
updated: 2026-03-15
status: draft
---

# IO Parameters

<!-- @operators -->
<!-- @pipelines -->
<!-- @identifiers -->
Input and output parameters bind data into and out of operators. IO labels are [[identifiers#Serialized Identifiers]]. Assignment uses [[operators]] (`<<`, `>>`, `<~`, `~>`). For how IO assignment mode controls pipeline triggering, see [[pipelines#IO as Implicit Triggers]].

## IO Labels

| Prefix | Direction | Example |
|--------|-----------|---------|
| `<` | Input | `<array`, `<InputParameter1` |
| `>` | Output | `>item`, `>OutputParameter1` |

IO labels are serialized identifiers — like all Polyglot identifiers, they follow the `.` (fixed) and `:` (flexible) field separator rules. See [[identifiers#Serialization Rules]].

## IO Line Pattern

```
[operator-ref] <param << source
[operator-ref] >param >> target
```

The statement marker echoes the parent operator's prefix:
- `[=]` — IO line for a pipeline (`=`)
- `[~]` — IO line for a collection-expand operator (`~`)
- `[*]` — IO line for a collection-collect operator (`*`)

**Indentation rule:** IO lines are indented one level under their parent operator to clearly show which IO belongs to which operation. This applies to all IO markers (`[=]`, `[~]`, `[*]`).

## IO Inputs as Variables

IO inputs declared with `[=]` become `$`-prefixed variables in the execution body once filled. There is no need to redeclare them:

```polyglot
[=] <incoming;Alert
[ ] ...execution...
[ ] Use directly as $incoming — it's already Final
[?] $incoming.level >? 5
```

IO inputs with no assignment must be filled externally and are in Final state when the pipeline fires. See [[pipelines#IO as Implicit Triggers]], [[variable-lifecycle]].

## Pipeline Call

<!-- @pipelines:Error Handling -->
Pipeline calls use `[r]` execution with `[=]` IO lines. Error blocks `[!]` scope under the call — see [[pipelines#Error Handling]]. For stdlib pipelines that need no import, see [[packages#Usage]].

```polyglot
[r] =Pipeline.Name
   [=] <InputParameter1 << ...
   [=] >OutputParameter1 >> ...
```

## Chain IO Addressing

<!-- @pipelines:Chain Execution -->
In chain execution (`[r] =A >> =B >> =C`), IO parameters are addressed by step reference — a numeric index (0-based) or pipeline leaf name, followed by `.` and the parameter name. See [[pipelines#Chain Execution]] for full chain semantics.

The direction convention is **caller-perspective**:

| Prefix | Meaning | Example |
|--------|---------|---------|
| `>N.param` | Push into step N's input | `>0.path << $file` |
| `<N.param` | Pull from step N's output | `<1.result >> >output` |
| `>LeafName.param` | Push into step by leaf name | `>Read.path << $file` |
| `<LeafName.param` | Pull from step by leaf name | `<Parse.rows >> >output` |

**Wiring between steps:** Connect one step's output to the next step's input with a single `[=]` line:

```polyglot
[=] <0.outputResult >> <1.inputParam
```

This reads: "from step 0's output, feed step 1's input." Both sides use the caller-perspective `<`/`>` convention.

**Auto-wire:** When adjacent steps have exactly one output and one input of the same type, the `[=]` wire line can be omitted. See [[pipelines#Auto-Wire]].

**Error references** in chains also use step addressing: `!0.ErrorName` or `!LeafName.ErrorName`. See [[pipelines#Error Handling in Chains]].

## Collection Operators

<!-- @collections -->
Two operator prefixes for collection processing. For the full operator reference and semantics, see [[collections]].

| Prefix | Operation | Example |
|--------|-----------|---------|
| `~` | Expand (iterate) | `~ForEach.Array` — iterate over collection |
| `*` | Collect (aggregate) | `*Into.Array` — collect results into collection |

These are **operators**, not identifier prefixes. The 5 identifier prefixes (`@`, `#`, `=`, `$`, `!`) remain unchanged.

### Example: Transform an Array

```polyglot
[r] ~ForEach.Array
   [~] <Array << $SomeArray
   [~] >item >> $item
   [ ]
   [ ] Here we can do something with the $item
   ...
   [r] *Into.Array
      [*] <item << $item
      [*] >Array >> $NewArray
   [ ] $NewArray can be used one level up in the pipeline
```

### Wait and Collect IO

Inside `[*]` collector blocks, the `<<`/`>>` direction operators distinguish wait inputs from collect outputs:

| Form | Semantics |
|------|-----------|
| `[*] << $var` | **Wait input** — waits for `$var` to be Final. Variable **stays accessible** after. |
| `[*] >> $var` | **Collect output** — in race collectors, losing inputs are **cancelled**; only the `>>` output survives. |

This is the same `<<`/`>>` direction convention used throughout the language:

| Context | `<<` (input / pull) | `>>` (output / push) |
|---------|---------------------|----------------------|
| Pipeline IO `[=]` | `<input << $var` — pulls value, waits for Final | `>output >> $result` — pushes, makes Final |
| Expand IO `[~]` | `<Array << $items` — pulls collection in | `>item >> $item` — pushes each item out |
| Collect IO `[*]` | `[*] << $var` — waits for Final, var stays accessible | `[*] >> $out` — receives collected value, inputs cancelled |

See [[collections#Sync & Race Collectors]] for the collectors that use these forms.

### Direct Output Port Writing

Collector outputs can write directly to a pipeline output port using the `>` prefix:

```polyglot
[r] *Agg.Concatenate
   [*] <string << $value
   [*] >result >> >pipelineOutput
```

The target output port reaches **Final** state after the collector writes to it — no other push to that port is allowed. See [[variable-lifecycle#Final]].
