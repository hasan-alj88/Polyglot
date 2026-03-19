---
audience: user
type: spec
updated: 2026-03-15
---

# Variable Lifecycle

<!-- @glossary:Polyglot Code -->
<!-- @identifiers -->
<!-- @pipelines -->
Variables in Polyglot Code ([[glossary#Polyglot Code]]) move through four lifecycle stages. Variables are [[identifiers]] with the `$` prefix. For how lifecycle applies to IO parameters, see [[pipelines#IO as Implicit Triggers]].

## Stages

| Stage | Description | Transitions to |
|-------|-------------|----------------|
| Declared | Variable exists but has no value | Default or Final |
| Default | Assigned via `<~` or `~>` — allows one more reassignment | Final or Released |
| Final | Assigned via `<<` or `>>` — no further assignment allowed | Released |
| Failed | The pipeline responsible for producing this variable failed with an error. The variable will never resolve. Check the source pipeline's error tree for details | — |
| Released | Variable is out of scope and no longer accessible | — |

### Declared

A variable enters the Declared stage when it appears in a block without an assignment operator. It exists but holds no value.

### Default

A variable enters the Default stage when assigned with a default assignment operator (`<~` or `~>`). A default-assigned variable allows **one more** assignment (which promotes it to Final).

### Final

A variable enters the Final stage when assigned with a final assignment operator (`<<` or `>>`). Once final:
- **No more assignments** (pushes) are allowed
- **Reading** (pulling values) is allowed unlimited times, as long as the variable is not released

### Failed

A variable enters the Failed stage when the pipeline responsible for producing its value terminates with an error. A failed variable will never resolve -- it cannot transition to any other stage. Downstream pipelines waiting on a failed variable will not fire. Inspect the source pipeline's error tree (see [[pipelines#Error Trees]]) for details on the failure.

### Released

A variable is released when:
- Its definition scope ends (block indentation returns to parent level)
- It is collected via a `*` collection operator — see [[collections#Collect Operators]]

## Querying Lifecycle State

Variable lifecycle state is queryable at runtime via the `%` metadata accessor:

```polyglot
[?] $myVar%state
   [?] #Ready
      [r] ...
   [?] #Failed
      [r] ...
   [?] *?
      [r] ...
```

`$varName%state` returns a `;live.#VarState` value. The `live` field is always readable and does not follow the standard lifecycle (it is managed by the runtime). The `#VarState` enum maps directly to the stages above: Declared, Default, Final, Failed, Released. See [[metadata]] for the full metadata tree and all `live` fields.

## Assignment Operators

<!-- @operators -->
All assignment operators are directional — the arrow indicates data flow direction between source and destination. See [[operators]] for the full operator table.

| Operator | Type | Direction | Example | Reading |
|----------|------|-----------|---------|---------|
| `<<` | Final (Push) | Right to left | `$x;int << 3` | "Final-push 3 into $x" |
| `>>` | Final (Pull) | Left to right | `>array >> $arr` | "Final-push >array into $arr" |
| `<~` | Default | Right to left | `.field;string <~ "value"` | "Default-assign \"value\" to .field" |
| `~>` | Default | Left to right | `>output;string ~> ""` | "Default-assign >output to empty string" |

## Examples

### Default Assignment — Pipeline IO

```polyglot
...
{=} =Example1
[ ] Daily trigger at 3AM
[t] =DT.Daily"3AM"
[ ] Pipeline IO
[=] <file;path <~ "\tmp\example1.txt"
[=] >output;string ~> ""
...
```

### Default and Final Assignment — Data Fields

```polyglot
...
{#} #CustomDataType
[ ] Data fields with default values
[.] .field1;string <~ "default value"
[.] .field2;int <~ 0
[ ] Data fields with final values
[.] .field3;string << "final value"
[.] .field4;int << 100
...
```
