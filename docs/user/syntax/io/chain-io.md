---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# Chain IO Addressing

<!-- @u:pipelines:Chain Execution -->
<!-- @u:technical/ebnf/10-execution#Chain -->
In chain execution (`[-] -A->-B->-C`), IO parameters are addressed by step reference — a numeric index (0-based) or pipeline leaf name, followed by `.` and the parameter name. See [[concepts/pipelines/chains#Chain Execution]] for full chain semantics.

`<` and `>` always describe the port from the pipeline's own viewpoint — `<` marks the pipeline's input, `>` marks its output — whether in a definition, a call site, or a chain step reference.

The direction convention is **pipeline-perspective**:

| Prefix | Meaning | Example |
|--------|---------|---------|
| `>N.param` | Push into step N's input | `>0.path << $file` |
| `<N.param` | Pull from step N's output | `<1.result >> >output` |
| `>LeafName.param` | Push into step by leaf name | `>Read.path << $file` |
| `<LeafName.param` | Pull from step by leaf name | `<Parse.rows >> >output` |

**Wiring between steps:** Connect one step's output to the next step's input with a single `(-)` line:

```polyglot
(-) <0.outputResult >> <1.inputParam
```

This reads: "from step 0's output, feed step 1's input." Both sides use the pipeline-perspective `<`/`>` convention.

**Auto-wire:** When adjacent steps have exactly one output and one input of the same type, the `(-)` wire line can be omitted. See [[concepts/pipelines/chains#Auto-Wire]].

**Error references** in chains also use step addressing: `!0.ErrorName` or `!LeafName.ErrorName`. See [[concepts/pipelines/chains#Error Handling in Chains]].
