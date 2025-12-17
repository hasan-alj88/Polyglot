---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: inline-pipelines
shard: false

# --- Classification ---
type: spec
topic: Inline Pipeline Calls & Formatted String Templates
summary: Complete specification for inline pipeline syntax, formatted string templates, and formatter pipelines
keywords:
  - inline
  - formatted-strings
  - pipelines
  - common-feature

# --- BMAD Agent Routing ---
agents:
  - developer
  - architect
phase: any
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - core-principles
  - pipeline-structure
  - type-system
unlocks:
  - stdlib

# --- Relationships ---
related:
  - operators
  - metadata-system
parent: language-advanced

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#inline"
  - "#common-feature"
  - "#spec"
---
# Inline Pipeline Calls & Formatted String Templates

**The most common feature in Polyglot** - calling pipelines inline with formatted string templates.

**What You'll Learn:**
- How inline pipeline syntax works
- Formatted string template system with `{$var:fmt}`
- Three-phase execution model
- Creating formatter pipelines with `%Inline` metadata
- Type-aware string representations
- Output handling for inline calls

---

## Overview

**Inline syntax** allows calling pipelines within expressions using **formatted string templates**:

```polyglot
[r] $doubled :pg.int << |U.Math.Double"{$value}"
[r] $greeting :pg.string << |Format.Message"{$name}-{$id:hex}"
[r] $result :pg.serial << |Process.Data"{$input}"
```

**This is NOT simple argument passing** - it's a sophisticated system that:
1. Converts variables to strings using **type-specific representation pipelines**
2. Substitutes them into a **formatted string template**
3. Feeds the formatted string to a **formatter pipeline** that parses it
4. Returns the result inline

---

## Quick Example

### Inline Call:
```polyglot
[r] $sum :pg.int << |U.Math.Add"{$x}, {$y}"
```

### What Actually Happens:

**Phase 1: String Representation (Parallel)**
```polyglot
[p] |U.String.Polyglot.Int.Default <variable << $x  >representation >> $x_str
[p] |U.String.Polyglot.Int.Default <variable << $y  >representation >> $y_str
```

**Phase 2: String Substitution**
```polyglot
// Result: "5, 3" (if $x=5, $y=3)
[r] |U.String.Substitute
[|] <template:pg.string << ", "
[|] <positions:pg.array.pg.uint << {0, 2}
[|] <subs:pg.array.pg.string << {$x_str, $y_str}
[|] >formatted_string >> $formatted_string  // "5, 3"
```

**Phase 3: Feed to Formatter**
```polyglot
[r] |U.Math.Add.FormattedString.For.Add
[|] <formatted_string:pg.string << $formatted_string  // "5, 3"
[|] >x:pg.int >> $parsed_x  // 5
[|] >y:pg.int >> $parsed_y  // 3

[r] |U.Math.Add
[|] <x << $parsed_x
[|] <y << $parsed_y
[|] >result >> $sum
```

---

## Formatted String Template Syntax

### Basic Syntax

```polyglot
|Pipeline"{literal-{$var}-more-literal-{$other:format}}"
```

**Components:**
- `{$var}` - Variable substitution with **default format**
- `{$var:format}` - Variable substitution with **specific format**
- `literal` - Literal text preserved as-is
- Separators (`,`, `-`, etc.) - Part of the template

### Examples

```polyglot
// Simple variable substitution
|Process"{$id}"                           // One variable, default format

// Multiple variables with literals
|Format.Message"{$name}-{$id}"            // Two variables, hyphen separator

// Custom format specifiers
|Display.Hex"{$value:hex}"                // Hexadecimal format
|Export.JSON"{$data:json}"                // JSON format

// Complex templates
|Build.URL"user/{$id}/posts/{$post:hex}"  // Mixed literals and variables

// Comma-separated (common pattern)
|U.Math.Add"{$x}, {$y}"                   // Comma separator

// Literal-only (no variables)
|GetConfig"production"                    // Just a literal string
```

---

## Three-Phase Execution Model

### Phase 1: Parallel String Representation

For each `{$var:format}` in the template, the compiler runs a **type-specific representation pipeline**:

**Pipeline Naming Pattern:**
```
|U.String.Polyglot.{Type}.{Format}
```

**Examples:**
```polyglot
{$name}         → |U.String.Polyglot.String.Default
{$count}        → |U.String.Polyglot.Int.Default
{$value:hex}    → |U.String.Polyglot.Int.Hex
{$data:json}    → |U.String.Polyglot.Serial.Json
{$timestamp:iso8601} → |U.String.Polyglot.DateTime.ISO8601
```

**Pipeline Signature:**
```polyglot
{|} |U.String.Polyglot.{Type}.{Format}
[|] <variable:{type}              // Variable to represent
[|] >representation:pg.string     // String representation

[t] |T.Call
[W] |W.Polyglot.Scope
   // Convert variable to string representation
   [|] >representation << $string_value
{x}
```

**Execution:**
- All representations run **in parallel** (`[p]` marker)
- If only one variable, compiler may optimize to sequential
- **Compile error** if representation pipeline doesn't exist

**Custom Formats:**
Users can define custom formats by creating representation pipelines:

```polyglot
{|} |U.String.Polyglot.Int.Binary
[|] <variable:pg.int
[|] >representation:pg.string

[t] |T.Call
[W] |W.Polyglot.Scope
   // Convert to binary string: 5 → "101"
   [|] >representation << $binary_string
{x}
```

Usage: `|Process"{$value:binary}"`

---

### Phase 2: String Substitution

**Collect** all string representations and **substitute** into template:

```polyglot
[v] *Collect.All              // Join parallel results
[*] $var1_representation
[*] $var2_representation
[*] ...

[r] |U.String.Substitute
[|] <template:pg.string << "literal-parts"        // Template with positions for substitutions
[|] <positions:pg.array.pg.uint << {0, 5, 10}     // Where to substitute
[|] <subs:pg.array.pg.string << {...}             // What to substitute
[|] >formatted_string >> $formatted_string
```

**Example:**

Template: `"{$name}-{$id:hex}"`
- Variables: `$name = "Alice"`, `$id = 255`
- Representations: `"Alice"`, `"FF"`
- Template parts: `"-"` (the literal separator)
- Positions: `{0, 6}` (before hyphen, after hyphen)
- Result: `"Alice-FF"`

**Special Cases:**

**Literal-only (no variables):**
```polyglot
|GetConfig"production"
// Phase 1 skipped entirely
// Phase 2: formatted_string = "production"
```

**No literals (variables only):**
```polyglot
|Process"{$value}"
// Template is empty
// Result is just the representation
```

---

### Phase 3: Feed to Formatter Pipeline

The **formatted string** (result of Phase 2) is passed to a **formatter pipeline** that:
1. **Parses** the string
2. **Extracts/computes** the main pipeline's input parameters
3. **Outputs** the parsed values

**Formatter Pipeline Pattern:**
```polyglot
{|} |MainPipeline.FormattedString.For.MainPipeline
[|] <formatted_string:pg.string       // POST-SUBSTITUTION string
[|] >param1:type                      // Extracted parameter 1
[|] >param2:type                      // Extracted parameter 2

[t] |T.Call
[W] |W.Polyglot.Scope

   // Parse the formatted string
   // Example: "5, 3" → split by comma → parse ints

   [|] >param1 << $parsed_value1
   [|] >param2 << $parsed_value2
{x}
```

**Formatter receives POST-SUBSTITUTION string:**
- Template: `"{$x}, {$y}"` with `$x=5`, `$y=3`
- Formatter receives: `"5, 3"` (NOT `"{$x}, {$y}"`)

**Formatter can be complex:**
```polyglot
// Example: Formatter that fetches data by ID
{|} |User.FormattedString.For.GetUser
[|] <formatted_string:pg.string       // "7FF0"
[|] >user_data:pg.serial

[t] |T.Call
[W] |W.Polyglot.Scope

   // Parse hex ID and fetch user from database
   [r] $user_id :pg.int << |Parse.Hex"{$formatted_string}"

   [r] $user :pg.serial << |Database.Users.FindById
   [|] <id << $user_id
   [|] >user >> $user

   [|] >user_data << $user
{x}
```

---

## Making Pipelines Inline-Callable

To make a pipeline callable inline, use **`%Inline` metadata**.

### Basic Pattern

```polyglot
{|} |MyPipeline
[%] %Inline
   [%] |MyPipeline.FormattedString.For.MyPipeline          // Formatter pipeline
   [|] <formatted_string:pg.string << %Inline.FormattedString  // Compiler-populated
   [|] >param1 >> <input1                                  // Wire formatter outputs
   [|] >param2 >> <input2                                  // to main inputs

[|] <input1:pg.int
[|] <input2:pg.string
[|] >result:pg.string

[t] |T.Call
[W] |W.Polyglot.Scope

   // Pipeline logic
   [|] >result << $computed_result
{x}
```

**Key Components:**

1. **`%Inline` metadata block** - Marks pipeline as inline-callable
2. **Formatter pipeline reference** - `[%] |FormatterPipeline`
3. **`%Inline.FormattedString`** - Special variable containing formatted string (compiler-populated)
4. **Output wiring** - `>formatter_output >> <main_input` connects formatter to main pipeline

---

### Complete Example: Math.Add

**Main Pipeline:**
```polyglot
{|} |U.Math.Add
[%] %Doc << "Add two numbers"

[%] %Inline
   [%] |U.Math.Add.FormattedString.For.Add
   [|] <formatted_string:pg.string << %Inline.FormattedString
   [|] >x:pg.int >> <x
   [|] >y:pg.int >> <y

[|] <x:pg.int
[|] <y:pg.int
[|] >result:pg.int

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $sum :pg.int << |Internal.Add <x << $x <y << $y
   [|] >result << $sum
{x}
```

**Formatter Pipeline:**
```polyglot
{|} |U.Math.Add.FormattedString.For.Add
[|] <formatted_string:pg.string       // Receives "5, 3"
[|] >x:pg.int
[|] >y:pg.int

[t] |T.Call
[W] |W.Polyglot.Scope

   // Split by comma and parse
   [r] $parts :pg.array.pg.string << |U.String.Split
   [|] <input << $formatted_string
   [|] <delimiter << ", "
   [|] >parts >> $parts

   [r] $x :pg.int << |Parse.Int"{$parts[0]}"
   [r] $y :pg.int << |Parse.Int"{$parts[1]}"

   [|] >x << $x
   [|] >y << $y
{x}
```

**Usage:**
```polyglot
[r] $sum :pg.int << |U.Math.Add"{$a}, {$b}"
// Equivalent to:
// 1. Convert $a, $b to strings (parallel)
// 2. Substitute into template: "5, 3"
// 3. Formatter parses: x=5, y=3
// 4. Main pipeline computes: result=8
// 5. Result captured into $sum
```

---

## Output Handling

### Single Output (Implied)

If pipeline has **one output parameter**, it's automatically the inline result:

```polyglot
{|} |Process
[|] <input:pg.string
[|] >result:pg.string         // ✅ Implied inline output

[t] |T.Call
[W] |W.Polyglot.Scope
   [|] >result << $value
{x}

// Usage:
[r] $output :pg.string << |Process"{$data}"  // Captures >result
```

---

### Multiple Outputs (Mark with %Inline.Output)

If pipeline has **multiple outputs**, use `%Inline.Output` to mark which one is returned inline:

```polyglot
{|} |Process
[|] <input:pg.string
[|] >result:pg.string
   [%] %Inline.Output << #True        // ✅ This is the inline result
[|] >debug_info:pg.string            // Ignored in inline calls

[t] |T.Call
[W] |W.Polyglot.Scope
   [|] >result << $main_result
   [|] >debug_info << $debug_data
{x}

// Usage:
[r] $output :pg.string << |Process"{$data}"  // Captures >result only
```

**Alias:** `#True` is shorthand for `#;Boolean;True` (see [Reserved Boolean Aliases](../types/enums-serial.md#reserved-boolean-aliases))

---

### Multiple Outputs (No Marker)

If **multiple outputs** and **no `%Inline.Output` marker**, result is **`:pg.serial`** with all outputs as key-value pairs:

```polyglot
{|} |ProcessBoth
[|] <input:pg.string
[|] >result:pg.string
[|] >status:pg.int
// No %Inline.Output marker

[t] |T.Call
[W] |W.Polyglot.Scope
   [|] >result << $value
   [|] >status << $code
{x}

// Usage:
[r] $output :pg.serial << |ProcessBoth"{$data}"
// $output = {
//   "result": "some value",
//   "status": 200
// }

[r] $result_value :pg.string << $output.result
[r] $status_code :pg.int << $output.status
```

---

## Multiple Format Styles

A pipeline can support **multiple inline format styles** using **conditionals**:

```polyglot
{|} |FlexiblePipeline
[%] %Inline
   // Determine format style based on pattern
   [y] %Inline.FormattedString re? "^\\d+,\\d+$"        // CSV format
      [%] |FlexiblePipeline.FormattedString.CSV
      [|] <formatted_string:pg.string << %Inline.FormattedString
      [|] >x:pg.int >> <x
      [|] >y:pg.int >> <y

   [y] %Inline.FormattedString re? "^\\{.*\\}$"        // JSON format
      [%] |FlexiblePipeline.FormattedString.JSON
      [|] <formatted_string:pg.string << %Inline.FormattedString
      [|] >x:pg.int >> <x
      [|] >y:pg.int >> <y

[|] <x:pg.int
[|] <y:pg.int
[|] >result:pg.string

[t] |T.Call
[W] |W.Polyglot.Scope
   [|] >result << $computed
{x}
```

**Usage:**
```polyglot
[r] $result1 << |FlexiblePipeline"{$a}, {$b}"        // Uses CSV formatter
[r] $result2 << |FlexiblePipeline"{{x: {$a}, y: {$b}}}"  // Uses JSON formatter
```

---

## Error Handling

Formatter pipelines use **standard error handling** with `[z][!]`:

```polyglot
{|} |SafeParse.FormattedString.For.SafeParse
[|] <formatted_string:pg.string
[|] >value:pg.int

[t] |T.Call
[W] |W.Polyglot.Scope

   [z] $parsed :pg.int << |Parse.Int"{$formatted_string}"
   [z][!] !Parse.InvalidFormat ? "Invalid number format"
      [|] >value << 0                    // Default value on error
   [z][!] *! ? "Unknown parsing error"
      [|] >value << 0

   [|] >value << $parsed
{x}
```

**Usage:**
```polyglot
[r] $value :pg.int << |SafeParse"{$input}"
// If $input = "abc" → value = 0 (error handled in formatter)
```

---

## Type Validation

The compiler performs **compile-time type validation**:

```polyglot
{|} |Process
[%] %Inline
   [%] |Process.FormattedString.For.Process
   [|] <formatted_string:pg.string << %Inline.FormattedString
   [|] >param:pg.int >> <input          // ✅ Formatter output type
                                         //    must match main input type

[|] <input:pg.int                       // ✅ Must match formatter output
[|] >result:pg.string

[t] |T.Call
[W] |W.Polyglot.Scope
   [|] >result << $value
{x}
```

**Compile Error Example:**
```polyglot
[|] >param:pg.string >> <input          // ❌ Type mismatch!
[|] <input:pg.int                       // ❌ string → int
```

---

## Performance Considerations

### Parallel Optimization

**Multiple variables:** Representations run in **parallel**
```polyglot
|Process"{$var1}, {$var2}, {$var3}"
// [p] |U.String.Polyglot.* runs 3 times in parallel
```

**Single variable:** Compiler may **optimize to sequential**
```polyglot
|Process"{$var}"
// Compiler may skip parallel overhead
```

### Literal-Only Optimization

**No variables:** Phase 1 skipped entirely
```polyglot
|GetConfig"production"
// No representation pipelines run
// Direct to formatter with literal string
```

---

## Common Patterns

### Pattern 1: Comma-Separated Arguments

**Most common pattern** for utility functions:

```polyglot
// Math utilities
[r] $sum << |U.Math.Add"{$x}, {$y}"
[r] $product << |U.Math.Multiply"{$a}, {$b}"

// String utilities
[r] $replaced << |U.String.Replace"{$text}, {$old}, {$new}"
```

**Formatter parses by comma:**
```polyglot
[r] $parts << |U.String.Split <input << $formatted_string <delimiter << ", "
```

---

### Pattern 2: Path-Style Templates

For hierarchical data:

```polyglot
[r] $user << |API.Get"users/{$id}/profile"
[r] $post << |API.Get"users/{$user_id}/posts/{$post_id:hex}"
```

---

### Pattern 3: Format Specifiers

Type-specific formatting:

```polyglot
[r] $hex_str << |Display.Hex"{$value:hex}"           // FF
[r] $json_str << |Export.JSON"{$data:json}"          // {"key":"value"}
[r] $iso_date << |Format.Date"{$timestamp:iso8601}"  // 2025-12-16T10:30:00Z
```

---

### Pattern 4: Single Parameter

Simple pass-through:

```polyglot
[r] $doubled << |U.Math.Double"{$value}"
[r] $upper << |U.String.Upper"{$text}"
```

---

### Pattern 5: Complex Parsing

Formatter does more than simple parsing:

```polyglot
// Formatter fetches user by hex ID
[r] $user :pg.serial << |User.GetByHexId"{$id:hex}"

// Formatter pipeline:
// 1. Parse hex: "7FF0" → 32752
// 2. Query database
// 3. Return user data
```

---

## Standard Format Specifiers

*Note: Format specifiers are on the backlog and will be added incrementally.*

**Planned formats:**
- `:hex` - Hexadecimal representation
- `:bin` - Binary representation
- `:oct` - Octal representation
- `:json` - JSON representation
- `:yaml` - YAML representation
- `:iso8601` - ISO 8601 date format
- `:rfc3339` - RFC 3339 date format
- `:base64` - Base64 encoding
- `:url` - URL encoding

---

## Comparison: Inline vs Explicit I/O

### Inline (Formatted String)

```polyglot
[r] $sum :pg.int << |U.Math.Add"{$x}, {$y}"
```

**Advantages:**
- ✅ Concise, single line
- ✅ Most common usage pattern
- ✅ Perfect for utility functions
- ✅ Natural for simple operations

**Disadvantages:**
- ❌ Formatter required
- ❌ String parsing overhead
- ❌ Less explicit parameter mapping

---

### Explicit I/O

```polyglot
[r] |U.Math.Add
[|] <x << $x
[|] <y << $y
[|] >result >> $sum
```

**Advantages:**
- ✅ Explicit parameter names
- ✅ No formatter needed
- ✅ No string parsing
- ✅ Better for complex I/O

**Disadvantages:**
- ❌ Verbose (4 lines vs 1)
- ❌ Less natural for simple calls

---

## When to Use Each Style

### Use Inline When:
- ✅ Calling standard library utilities
- ✅ Simple operations with 1-3 parameters
- ✅ Quick, readable code needed
- ✅ Parameters can be naturally formatted as strings

### Use Explicit I/O When:
- ✅ Pipeline has many parameters (5+)
- ✅ Complex structured data inputs
- ✅ Chaining pipelines with `|>`
- ✅ Parameter names add clarity
- ✅ No formatter exists and creating one is overkill

---

## Summary

### Key Points

1. **Inline syntax = Formatted string templates**, not argument passing
2. **Three-phase execution:** Representation → Substitution → Parsing
3. **Type-aware string representations** via `|U.String.Polyglot.{Type}.{Format}`
4. **Formatter pipelines** parse strings and output main pipeline inputs
5. **`%Inline` metadata** registers formatter and wires outputs
6. **Most common feature** - all standard library utilities support it

### Requirements for Inline-Callable Pipelines

| Component | Requirement |
|-----------|-------------|
| **Trigger** | `[t] \|T.Call` |
| **Wrapper** | Any `[W] \|W.*` |
| **Formatter** | Define with `%Inline` metadata |
| **Representation Pipelines** | Must exist for all types/formats used |
| **Type Validation** | Formatter outputs must match main inputs |

---

## Related Documentation

- [Core Principles](../../getting-started/core-principles.md) - One line = one expression
- [Pipeline Structure](../control-flow/pipeline-structure.md) - Pipeline anatomy
- [Prefix System](../syntax/prefix-system.md) - `|` pipeline prefix
- [Metadata System](./metadata-system.md) - `%Inline` metadata
- [Standard Library](../../stdlib/index.md) - All utilities support inline calls
- [Operators Reference](../syntax/operators.md) - Pipeline composition

---

**Last Updated:** 2025-12-16
**Part of:** [v0.0.4 Specification](../../README.md)
