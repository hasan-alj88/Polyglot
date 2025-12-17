<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

# Pipeline Composition & Chaining Examples

**Operator:** `|>` (chain/compose operator)
**Date:** 2025-12-16 (Updated with correct v0.0.4 syntax)

---

## Critical Syntax Rules

**Pipeline chaining uses the following structure:**

1. `[r]` marker starts the chain (sequential execution)
2. `|Pipeline1 |> |Pipeline2 |> |Pipeline3` declares the pipeline chain
3. `[|]` markers for ALL IO wiring lines
4. `<param:datatype` = input parameter (prefix `<`)
5. `>param:datatype` = output parameter (prefix `>`)
6. `<<` = PULL from source (assignment direction ←)
7. `>>` = PUSH into destination (assignment direction →)
8. `$var` = variable prefix
9. Chain ends with `|>` alone (no following pipeline)
10. Final outputs use `>>` to push to variables

---

## Case 1: Single Input → Single Output (Simple Chain)

**Traditional (without chaining):**
```polyglot
[r] |FetchData
[|] <url:pg.url << $endpoint
[|] >data:pg.serial >> $raw

[r] |ParseData
[|] <input:pg.serial << $raw
[|] >parsed:pg.serial >> $result

[r] |ValidateData
[|] <input:pg.serial << $result
[|] >validated:pg.serial >> $final
```

**With Composition:**
```polyglot
[r] |FetchData |> |ParseData |> |ValidateData
[|] <url:pg.url << $endpoint
[|] >data:pg.serial >> <input
[|] >parsed:pg.serial >> <input
[|] |>
[|] >validated:pg.serial >> $final
```

**Rule:** Single output flows directly into single input via `>>` operator.

---

## Case 2: Multi-Output → Multi-Input (Full Connection)

**Scenario:** Each pipeline outputs multiple values, next pipeline uses ALL of them.

### Example: Data Processing Pipeline

**Traditional:**
```polyglot
[r] |FetchUserData
[|] <user_id:pg.string << $userId
[|] >user:pg.serial >> $userData
[|] >metadata:pg.serial >> $userMeta
[|] >timestamp:pg.dt >> $fetchTime

[r] |EnrichData
[|] <user:pg.serial << $userData
[|] <metadata:pg.serial << $userMeta
[|] <timestamp:pg.dt << $fetchTime
[|] >enriched_user:pg.serial >> $enrichedData
[|] >enrichment_log:pg.serial >> $enrichLog

[r] |ValidateAndStore
[|] <user:pg.serial << $enrichedData
[|] <log:pg.serial << $enrichLog
[|] >stored:pg.bool >> $success
[|] >errors:pg.array{!} >> $validationErrors
```

**With Composition:**
```polyglot
[r] |FetchUserData |> |EnrichData |> |ValidateAndStore
[|] <user_id:pg.string << $userId
[|] >user:pg.serial >> <user
[|] >metadata:pg.serial >> <metadata
[|] >timestamp:pg.dt >> <timestamp
[|] >enriched_user:pg.serial >> <user
[|] >enrichment_log:pg.serial >> <log
[|] |>
[|] >stored:pg.bool >> $success
[|] >errors:pg.array{!} >> $validationErrors
```

**Semantics:**
- All outputs from previous pipeline flow to next pipeline's inputs
- Outputs must match inputs (by name/position and type)
- Compiler verifies the chain is valid
- Final `|>` indicates end of chain, outputs go to variables

---

## Case 3: Partial Output Usage (Named Routing)

**Scenario:** Only SOME outputs needed by next pipeline.

**Traditional:**
```polyglot
[r] |ParseRequest
[|] <raw:pg.string << $request
[|] >headers:pg.serial >> $reqHeaders
[|] >body:pg.serial >> $reqBody
[|] >metadata:pg.serial >> $reqMeta

// Only use body and headers, ignore metadata
[r] |ValidateRequest
[|] <headers:pg.serial << $reqHeaders
[|] <body:pg.serial << $reqBody
[|] >valid:pg.bool >> $isValid
[|] >errors:pg.array{!} >> $validationErrors
```

**With Named Composition:**
```polyglot
[r] |ParseRequest |> |ValidateRequest
[|] <raw:pg.string << $request
[|] >headers:pg.serial >> <headers
[|] >body:pg.serial >> <body
[|] >metadata:pg.serial >> $_    // Discard unused output
[|] |>
[|] >valid:pg.bool >> $isValid
[|] >errors:pg.array{!} >> $validationErrors
```

**Note:** `$_` is a discard variable for unused outputs.

---

## Case 4: Branching - One Output Feeds Multiple Pipelines

**Scenario:** Fork the data flow (parallel execution).

**Traditional:**
```polyglot
[r] |FetchData
[|] <url:pg.url << $endpoint
[|] >data:pg.serial >> $rawData

// Branch 1: Validate
[p] |ValidateData
[|] <input:pg.serial << $rawData
[|] >valid:pg.bool >> $isValid

// Branch 2: Transform
[p] |TransformData
[|] <input:pg.serial << $rawData
[|] >transformed:pg.serial >> $transformed

// Join
[v] ~V.JoinAll
[|] <append << $isValid
[|] <append << $transformed
[|] >array >> $results
```

**With Composition + Fork:**
```polyglot
[r] |FetchData
[|] <url:pg.url << $endpoint
[|] >data:pg.serial >> $rawData

// Parallel branches
[p] |ValidateData
[|] <input:pg.serial << $rawData
[|] >valid:pg.bool >> $isValid

[p] |TransformData
[|] <input:pg.serial << $rawData
[|] >transformed:pg.serial >> $transformed

// Join
[v] ~V.JoinAll
[|] <append << $isValid
[|] <append << $transformed
[|] >array >> $results
```

**Note:** Branching with `[p]` parallelmarkers is separate from chaining. Use join operations to collect parallel results.

---

## Case 5: Complex Multi-IO Chain (Real Example)

**Scenario:** Multi-stage ETL pipeline with explicit IO wiring.

**With Composition:**
```polyglot
{|} |Pipeline.ETLWorkflow
[<] .source_url:pg.url
[<] .schema:pg.serial
[>] .success:pg.bool
[>] .allErrors:pg.array{!}

[t] |T.Call
[W] |W.Polyglot.Scope

// Main ETL chain
[r] |Extract |> |Transform |> |Load
[|] <url:pg.url << $source_url
[|] >raw_data:pg.serial >> <data
[|] >row_count:pg.int >> $extractCount
[|] >errors:pg.array{!} >> $extractErrors
[|] <schema:pg.serial << $schema
[|] >transformed_data:pg.serial >> <data
[|] >transform_stats:pg.serial >> <stats
[|] >errors:pg.array{!} >> $transformErrors
[|] |>
[|] >loaded:pg.bool >> $success
[|] >load_errors:pg.array{!} >> $loadErrors

// Collect all errors
[r] |CollectErrors
[|] <extract_errors:pg.array{!} << $extractErrors
[|] <transform_errors:pg.array{!} << $transformErrors
[|] <load_errors:pg.array{!} << $loadErrors
[|] >all_errors:pg.array{!} >> $allErrors

{x}
```

---

## Case 6: Explicit Pipeline Output References

**You can explicitly reference pipeline outputs for IDE autocomplete:**

```polyglot
[r] |Pipeline1 |> |Pipeline2 |> |Pipeline3
[|] <p1in1:pg.string << $var
[|] >p1out1:pg.int >> <p2in2
[|] >p1out2:pg.string >> <p2in1
[|] >p1out3:pg.float >> <p2in3
[|] |Pipeline2>p2out1 >> <p3in3   // Explicit pipeline reference (optional)
[|] >p2out2:pg.bool >> <p3in4
[|] >p2out3:pg.serial >> <p3in2
[|] >p2out4:pg.array >> <p3in1
[|] |>
[|] >p3out1:pg.string >> $result1
[|] >p3out2:pg.int >> $result2
```

**Note:** `|Pipeline2>p2out1` explicitly shows which pipeline the output comes from. This is optional but helps with IDE autocomplete.

---

## Composition Rules

### Rule 1: Output Must Connect to Input or Variable

```polyglot
// Valid: Output connects to next input
[r] |Step1 |> |Step2
[|] <in1:pg.string << $input
[|] >out1:pg.int >> <in2
[|] >out2:pg.string >> <in3
[|] |>
[|] >out3:pg.float >> $result
```

### Rule 2: Types Must Match

```polyglot
// Invalid: Type mismatch
[r] |GetString |> |ParseInt
[|] >str:pg.string >> <num:pg.int   // ❌ Type error
```

### Rule 3: Chain Ends with `|>` Alone

```polyglot
[r] |Pipeline1 |> |Pipeline2 |> |Pipeline3
[|] <input:pg.string << $data
[|] >out1:pg.int >> <in2
[|] >out2:pg.float >> <in3
[|] |>                              // End of chain
[|] >out3:pg.string >> $result     // Outputs to variables
```

### Rule 4: Use `[|]` for ALL Wiring Lines

```polyglot
[r] |Step1 |> |Step2
[|] <input:pg.string << $value      // [|] required
[|] >output:pg.int >> <next_input   // [|] required
[|] |>                               // [|] required
[|] >final:pg.string >> $result     // [|] required
```

---

## Implementation Notes

**Parser Requirements:**
- Track output signature of each pipeline
- Verify input/output compatibility in chain
- Validate type matching across `>>` operators
- Ensure all outputs are wired (to next input or to variable)

**Syntax Structure:**
- `[r]` starts sequential pipeline chain
- `|Pipeline1 |> |Pipeline2` declares chain
- `[|]` marks all IO wiring lines
- `<<` pulls from source variable
- `>>` pushes to destination (input or variable)
- `|>` alone ends the chain

---

## Common Patterns

### Pattern 1: Linear Transformation Chain

```polyglot
[r] |Trim |> |Lower |> |Validate
[|] <input:pg.string << $raw_input
[|] >trimmed:pg.string >> <input
[|] >lowered:pg.string >> <input
[|] |>
[|] >validated:pg.string >> $clean_email
```

### Pattern 2: Multi-Input Chain

```polyglot
[r] |Fetch |> |Combine |> |Store
[|] <user_id:pg.string << $userId
[|] <config:pg.serial << $configData
[|] >user:pg.serial >> <left
[|] <right:pg.serial << $configData
[|] >combined:pg.serial >> <data
[|] |>
[|] >success:pg.bool >> $stored
```

### Pattern 3: With Error Collection

```polyglot
[r] |Extract |> |Transform |> |Load
[|] <source:pg.url << $url
[|] >data:pg.serial >> <input
[|] >errors:pg.array{!} >> $extractErrors
[|] >output:pg.serial >> <input
[|] >errors:pg.array{!} >> $transformErrors
[|] |>
[|] >success:pg.bool >> $result
[|] >errors:pg.array{!} >> $loadErrors
```

---

## Summary

### Correct Pipeline Chaining Syntax

```polyglot
[r] |Pipeline1 |> |Pipeline2 |> |Pipeline3
[|] <p1input:datatype << $variable
[|] >p1output:datatype >> <p2input
[|] >p2output:datatype >> <p3input
[|] |>
[|] >p3output:datatype >> $result
```

### Key Points

- `[r]` = sequential execution marker ("run")
- `[|]` = pipeline marker (must be at start of ALL IO wiring lines)
- `<param` = input parameter prefix
- `>param` = output parameter prefix
- `<<` = PULL from source
- `>>` = PUSH into destination
- `$var` = variable prefix
- `|> |Pipeline` = chain to next pipeline
- `|>` alone = end of chain, outputs go to variables

---

**Last Updated:** 2025-12-16
**Part of:** [v0.0.4 Specification](../../README.md)
