# Pipeline Composition Guide

**Version:** v0.0.4
**Category:** Language Guide
**Audience:** Intermediate to Advanced users
**Prerequisites:** Basic pipeline syntax, understanding of I/O parameters

---

## Overview

Pipeline composition with the `|>` operator allows you to **chain pipelines** without intermediate variables, creating clear data flow with explicit output-to-input wiring.

**Key Benefits:**
- No intermediate variables needed
- Clear, visual data flow
- Explicit parameter wiring
- Reduced state management
- Type-safe connections

---

## Basic Syntax

### The `[|] |>` Composition Marker

```polyglot
[r] |Pipeline1
(|) <input:type << $source

[|] |> |Pipeline2                    // Composition marker
(|) >output1:type >> <input2         // Wire P1 output to P2 input

[|] |>                               // Chain end (no pipeline name)
(|) >final_output:type >> $result    // Capture chain result
```

**Pattern:**
1. Call first pipeline with `[r]` marker
2. Use `[|] |>` to start composition
3. Wire outputs to inputs with `>out >> <in`
4. End chain with `[|] |>` (no pipeline name)
5. Capture result to variable

---

## Simple Example

### Without Composition (Using Variables)

```polyglot
[r] |Data.Validate
(|) <input:pg.string << $raw_data
(|) >validated:pg.string >> $temp1
(|) >is_valid:pg.bool >> $temp2

[r] |Data.Transform
(|) <data:pg.string << $temp1
(|) <should_transform:pg.bool << $temp2
(|) >result:pg.string >> $final_result
```

### With Composition

```polyglot
[r] |Data.Validate
(|) <input:pg.string << $raw_data

[|] |> |Data.Transform
(|) >validated:pg.string >> <data
(|) >is_valid:pg.bool >> <should_transform

[|] |>
(|) >result:pg.string >> $final_result
```

**Benefit:** No `$temp1` or `$temp2` variables needed!

---

## Output-to-Input Wiring

### Explicit Parameter Names

```polyglot
[|] |> |NextPipeline
(|) >previous_output:type >> <next_input
```

**Important:**
- Use **real parameter names** from pipeline definitions
- NOT positional - wiring is by name
- `>output` comes from previous pipeline
- `<input` comes from next pipeline

### Example: LLM Query → File Append

```polyglot
[r] |LLM.Query
(|) <attachments:pg.path << $file
(|) <prompt:pg.string << "Summarize this log file"

[|] |> |File.Append
(|) >result:pg.string >> <content      // |LLM.Query>result → |File.Append<content
(|) <path:pg.path << $output_file      // Additional input from variable

[|] |>
(|) >success:pg.bool >> $write_success
```

**Breakdown:**
- `|LLM.Query` has output `>result:pg.string`
- `|File.Append` has inputs `<content:pg.string` and `<path:pg.path`
- Wire: `>result >> <content`
- Also provide: `<path << $output_file` from variable

---

## Mixing Outputs and Variables

You can mix previous pipeline outputs with variables:

```polyglot
[r] |Step1
(|) <input:pg.string << $data

[|] |> |Step2
(|) >step1_output:pg.string >> <data           // From previous pipeline
(|) <config:pg.path << $config_file            // From variable
(|) <verbose:pg.bool << #True                  // From literal

[|] |>
(|) >result:pg.string >> $final
```

**Pattern:**
- `>output >> <input` - Wire from previous pipeline
- `<input << $var` - Provide from variable
- `<input << #literal` - Provide literal value

---

## Multi-Step Chains

### Chaining More Than Two Pipelines

```polyglot
[r] |Step1
(|) <input:pg.string << $raw

[|] |> |Step2
(|) >data:pg.string >> <input

[|] |> |Step3
(|) >processed:pg.string >> <data

[|] |> |Step4
(|) >result:pg.string >> <final_data

[|] |>
(|) >output:pg.string >> $result
```

**Each `[|] |>` adds another step to the chain.**

---

## Advanced: Multi-Step References

### Later Steps Can Reference Earlier Outputs

```polyglot
[r] |Validate
(|) <raw:pg.string << $data

[|] |> |Transform
(|) >validated_data:pg.string >> <input
(|) >validation_status:pg.bool >> $status     // Save for later

[|] |> |Store
(|) <data:pg.string << >transformed_data      // From Step2
(|) <was_valid:pg.bool << $status             // From Step1 (via variable)
(|) <metadata:pg.serial << >validation_meta   // From Step1

[|] |>
(|) >success:pg.bool >> $stored
```

**Note:** Step 3 uses both Step 2's output AND Step 1's output (via saved variable).

---

## Pipeline Composition in Loops

### Combining with Unpack/Pack Operators

```polyglot
[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file

   [r] |File.Load
   (|) <path:pg.path << $file

   [|] |> |Data.Process
   (|) >content:pg.string >> <input

   [|] |> |Data.Validate
   (|) >processed:pg.string >> <data

   [|] |>
   (|) >is_valid:pg.bool >> $valid

[*] *Into.Array
(*) <item << $valid

// $valid is now array of validation results
```

**Benefit:** Clean data flow through multiple steps per iteration.

---

## Error Handling in Chains

### Error Blocks on Chain Output

```polyglot
[r] |RiskyOperation
(|) <input:pg.string << $data

[|] |> |FollowUp
(|) >result:pg.string >> <data

[|] |>
(|) >success:pg.bool >> $status
   [!] $success_bool:pg.bool << !
      [?] !NoError ? #True
      [?] !* ? #False

// Handle $success_bool
[f] $success_bool =? #False
   [r] >error << !OperationFailed
{x}
```

**Pattern:** Apply error block to final chain output.

---

## Complete Example: Log Processing

### Scenario: Load → Process → Summarize → Store

```polyglot
{@} @Local:Examples.LogPipeline:0.0.0.1
{x}



{|} |ProcessLog
[%] %Doc << "Load log, process errors, summarize, and store"

[|] <log_path:pg.path
[|] >summary_path:pg.path
[|] >error <~ !NoError

[r] |File.Load
(|) <path:pg.path << $log_path

[|] |> |Log.ExtractErrors
(|) >content:pg.string >> <log_text

[|] |> |LLM.Summarize
(|) >errors:pg.array.pg.string >> <items
(|) <prompt:pg.string << "Summarize these errors concisely"

[|] |> |File.Write
(|) >summary:pg.string >> <content
(|) <path:pg.path << $summary_path

[|] |>
(|) >success:pg.bool >> $write_ok
   [!] $result:pg.bool << !
      [?] !NoError ? #True
      [?] !* ? #False

[f] $result =? #True
   [r] >error << !NoError
{x}

[f] $result =? #False
   [r] >error << !ProcessingFailed
{x}
{x}
```

**Usage:**
```bash
polyglot run processlog --log_path=/var/logs/app.log --summary_path=/reports/summary.txt
```

**Data Flow:**
1. Load log file → content string
2. Extract errors from content → error array
3. Summarize errors → summary string
4. Write summary to file → success bool
5. Check success → set error output

---

## Documentation Notation

### Reading Pipeline I/O References

**Format:** `|PipelineName>output` or `|PipelineName<input`

**Examples:**
- `|LLM.Query>result` = "The >result output parameter of |LLM.Query pipeline"
- `|Data.Validate<input` = "The <input parameter of |Data.Validate pipeline"

**This notation helps document which parameter from which pipeline.**

---

## Common Patterns

### Pattern 1: Validate-Transform-Store

```polyglot
[r] |Validate
(|) <data:pg.string << $input

[|] |> |Transform
(|) >validated:pg.string >> <input
(|) >is_valid:pg.bool >> $valid

[|] |> |Store
(|) >result:pg.string >> <data
(|) <valid:pg.bool << $valid

[|] |>
(|) >success:pg.bool >> $stored
```

### Pattern 2: Load-Process-Save

```polyglot
[r] |Load
(|) <path:pg.path << $file

[|] |> |Process
(|) >data:pg.serial >> <input

[|] |> |Save
(|) >processed:pg.serial >> <data
(|) <output_path:pg.path << $output

[|] |>
(|) >success:pg.bool >> $saved
```

### Pattern 3: Multi-Source Merge

```polyglot
[r] |Source1
(|) <input:pg.path << $file1

[|] |> |Merge
(|) >data1:pg.serial >> <source_a
(|) <source_b:pg.serial << $data2      // From different source

[|] |>
(|) >merged:pg.serial >> $result
```

---

## Comparison: Variables vs Composition

### Traditional Approach (Variables)

```polyglot
[r] |Step1
(|) <input << $data
(|) >output >> $temp1

[r] |Step2
(|) <input << $temp1
(|) >output >> $temp2

[r] |Step3
(|) <input << $temp2
(|) >output >> $result
```

**Issues:**
- Many intermediate variables (`$temp1`, `$temp2`)
- Harder to track data flow
- More state to manage

### Pipeline Composition

```polyglot
[r] |Step1
(|) <input << $data

[|] |> |Step2
(|) >output >> <input

[|] |> |Step3
(|) >output >> <input

[|] |>
(|) >output >> $result
```

**Benefits:**
- No intermediate variables
- Clear data flow visualization
- Less state management
- Type-safe connections

---

## Best Practices

### ✅ 1. Use Descriptive Output Names

```polyglot
// ✅ GOOD: Clear what's being wired
[|] |> |Process
(|) >validated_data:pg.string >> <input_text
(|) >validation_status:pg.bool >> <is_valid

// ❌ AVOID: Generic names
[|] |> |Process
(|) >out1:pg.string >> <in1
(|) >out2:pg.bool >> <in2
```

### ✅ 2. Keep Chains Focused

```polyglot
// ✅ GOOD: 3-4 steps, clear purpose
Load → Validate → Transform → Store

// ❌ AVOID: Too many steps
Load → Clean → Validate → Transform1 → Transform2 → Merge → Filter → Sort → Dedupe → Store
```

**Consider breaking long chains into separate pipelines.**

### ✅ 3. Document Complex Chains

```polyglot
[r] |ExtractData
(|) <source:pg.path << $file

[|] |> |TransformFormat
// Converts CSV to JSON structure
(|) >raw_data:pg.string >> <csv_content

[|] |> |ValidateSchema
// Checks against schema definition
(|) >json_data:pg.serial >> <data

[|] |>
(|) >validated:pg.serial >> $result
```

### ✅ 4. Handle Errors Explicitly

```polyglot
[|] |>
(|) >result:type >> $value
   [!] $success:pg.bool << !
      [?] !NoError ? #True
      [?] !* ? #False

[f] $success =? #False
   [r] >error << !ChainFailed
{x}
```

---

## Troubleshooting

### Issue 1: Type Mismatch

**Error:** Output type doesn't match input type

```polyglot
// ❌ WRONG: Types don't match
[|] |> |Next
(|) >result:pg.string >> <input:pg.int  // Error!
```

**Solution:** Ensure types match or add conversion step:

```polyglot
// ✅ RIGHT: Add conversion
[|] |> |StringToInt
(|) >result:pg.string >> <text

[|] |> |Next
(|) >number:pg.int >> <input
```

### Issue 2: Missing Required Input

**Error:** Pipeline requires input that isn't provided

```polyglot
// ❌ WRONG: |Process needs <config but it's not provided
[|] |> |Process
(|) >data:pg.string >> <input
// Missing: <config:pg.path
```

**Solution:** Provide all required inputs:

```polyglot
// ✅ RIGHT: All inputs provided
[|] |> |Process
(|) >data:pg.string >> <input
(|) <config:pg.path << $config_file
```

### Issue 3: Unused Outputs

**Warning:** Previous pipeline has outputs that aren't wired

```polyglot
[r] |Generate
(|) <input << $data
// Outputs: >result:pg.string, >metadata:pg.serial, >status:pg.bool

[|] |> |Use
(|) >result:pg.string >> <data
// ⚠️ metadata and status not used
```

**Solution:** Either use them or acknowledge they're intentionally unused:

```polyglot
[|] |> |Use
(|) >result:pg.string >> <data
(|) >metadata:pg.serial >> $meta_saved
// status intentionally unused - just checking result
```

---

## Quick Reference

```
┌─────────────────────────────────────────────┐
│ PIPELINE COMPOSITION                        │
├─────────────────────────────────────────────┤
│                                             │
│  BASIC PATTERN                              │
│  [r] |Pipeline1                             │
│  (|) <input << $source                      │
│                                             │
│  [|] |> |Pipeline2                          │
│  (|) >output1 >> <input2                    │
│                                             │
│  [|] |>                                     │
│  (|) >final_output >> $result               │
│                                             │
│  WIRING SYNTAX                              │
│  >prev_output >> <next_input  Wire outputs │
│  <input << $variable          From var     │
│  <input << #literal           From literal │
│                                             │
│  CHAIN END                                  │
│  [|] |>  (no pipeline name)                 │
│  (|) >output >> $variable                   │
│                                             │
└─────────────────────────────────────────────┘
```

---

## See Also

- [Operators Reference](../syntax/operators.md) - Output operators `>>`, `~>`
- [Error Handling](../error-handling/basics.md) - Error blocks in chains
- [Loop System](../control-flow/loops.md) - Composition in loops
- [Standard Library](../../stdlib/index.md) - Pipeline I/O definitions

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-27
**Confidence:** ✅ Verified - All patterns human-validated
