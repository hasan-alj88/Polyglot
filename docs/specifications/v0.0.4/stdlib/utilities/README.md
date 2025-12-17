---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Utilities (`|U.*`)
summary: API reference: Utilities (`|U.*`)
keywords:
  - stdlib
  - api
  - reference

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: low

# --- Dependency Chain ---
prereqs:
  - type-system
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#stdlib"
  - "#api"
---
# Utilities (`|U.*`)

**Common utility operations for math, strings, dates, and data formats**

---

## 🚀 Inline Calls - The Standard Way

**All utilities are designed for inline calls** using formatted string templates:

```polyglot
// Math - comma-separated arguments
[r] $sum :pg.int << |U.Math.Add"{$x}, {$y}"
[r] $doubled :pg.int << |U.Math.Double"{$value}"

// String - comma-separated arguments
[r] $upper :pg.string << |U.String.Upper"{$text}"
[r] $split :pg.array.pg.string << |U.String.Split"{$text}, {$delimiter}"

// DateTime - empty or comma-separated
[r] $now :pg.string << |DT.Now""
[r] $future :pg.string << |DT.AddDays"{$date}, {7}"
```

**Format Convention:**
- **Empty `""`** - No parameters (e.g., `|DT.Now""`)
- **Comma-separated `"{arg1}, {arg2}, ..."`** - Multiple parameters
- **Variables with format** - `{$value:hex}`, `{$data:json}` (format specifiers)

**Behind the Scenes:**

When you write `|U.Math.Add"{$x}, {$y}"`, here's what actually happens:

**Phase 1: Parallel String Representations**
```polyglot
// Compiler generates (in parallel):
[p] |U.String.Polyglot.Int.Default <variable << $x    // → "5"
[p] |U.String.Polyglot.Int.Default <variable << $y    // → "3"
```

**Phase 2: String Substitution**
```polyglot
// Template: "{$x}, {$y}"
// Result: "5, 3"
```

**Phase 3: Formatter Pipeline Receives String**
```polyglot
// The formatter pipeline is called:
{|} |U.Math.Add.FormattedString.For.Add
[|] <formatted_string :pg.string << %Inline.FormattedString  // ← Contains "5, 3"

   // Parse the formatted string:
   [r] $parts :pg.array.pg.string << |U.String.Split"{%Inline.FormattedString}, {\", \"}"
   [r] $a :pg.int << |U.String.ToInt"{$parts[0]}"
   [r] $b :pg.int << |U.String.ToInt"{$parts[1]}"

   // Wire to main pipeline inputs:
   [|] >a >> <a    // Maps to |U.Math.Add's <a input
   [|] >b >> <b    // Maps to |U.Math.Add's <b input
{x}

// Then main pipeline executes:
{|} |U.Math.Add
[|] <a :pg.int    // ← Receives 5
[|] <b :pg.int    // ← Receives 3
[|] >result :pg.int
   [r] $result :pg.int << ... // a + b
   [|] >result >> $result
{x}
```

**Key Insight:** `%Inline.FormattedString` is a **special compiler-populated variable** that contains the formatted string. It's ONLY available inside formatter pipelines registered via `%Inline` metadata.

**See:**
- [Inline Pipelines Complete Specification](../../language/advanced/inline-pipelines.md) - Full three-phase execution model
- [Pipeline Structure - %Inline Metadata](../../language/control-flow/pipeline-structure.md#7-inline-metadata---making-pipelines-inline-callable) - How to register formatter pipelines

---

## Complete Utilities Tree

```
|U.* (Utilities)
│
├── |U.Math.* (10 pipelines)
│   ├── |U.Math.Add
│   ├── |U.Math.Subtract
│   ├── |U.Math.Multiply
│   ├── |U.Math.Divide
│   ├── |U.Math.Modulo
│   ├── |U.Math.Double
│   ├── |U.Math.Round
│   ├── |U.Math.Floor
│   ├── |U.Math.Ceiling
│   └── |U.Math.Abs
│
├── |U.String.* (8 pipelines)
│   ├── |U.String.Concat
│   ├── |U.String.Split
│   ├── |U.String.Upper
│   ├── |U.String.Lower
│   ├── |U.String.Trim
│   ├── |U.String.Length
│   ├── |U.String.Substring
│   └── |U.String.Replace
│
├── |U.DateTime.* (Alias: |DT.*) (12 pipelines)
│   ├── |DT.Now
│   ├── |DT.Parse
│   ├── |DT.Format
│   ├── |DT.AddDays
│   ├── |DT.AddHours
│   ├── |DT.AddMinutes
│   ├── |DT.Diff
│   ├── |DT.Year
│   ├── |DT.Month
│   ├── |DT.Day
│   ├── |DT.Hour
│   ├── |DT.Minute
│   └── |DT.Second
│
└── |U.Data.* (9 pipelines)
    ├── |YAML.Load
    ├── |YAML.Parse
    ├── |YAML.Dump
    ├── |JSON.Load
    ├── |JSON.Parse
    ├── |JSON.Dump
    ├── |TOML.Load
    ├── |TOML.Parse
    └── |XML.Parse
```

**Total: ~39 utility pipelines**

---

## Math Utilities

**Package:** `|U.Math.*`
**Purpose:** Arithmetic and mathematical operations

**See:** [Math Package Documentation](./math/README.md)

### Quick Reference

| Pipeline | Purpose | Example |
|----------|---------|---------|
| `\|U.Math.Add` | Add two numbers | `\|U.Math.Add"{5, 3}" → 8` |
| `\|U.Math.Subtract` | Subtract numbers | `\|U.Math.Subtract"{10, 3}" → 7` |
| `\|U.Math.Multiply` | Multiply numbers | `\|U.Math.Multiply"{4, 5}" → 20` |
| `\|U.Math.Divide` | Divide numbers | `\|U.Math.Divide"{20, 4}" → 5` |
| `\|U.Math.Modulo` | Remainder of division | `\|U.Math.Modulo"{10, 3}" → 1` |
| `\|U.Math.Double` | Multiply by 2 | `\|U.Math.Double"{5}" → 10` |
| `\|U.Math.Round` | Round to nearest int | `\|U.Math.Round"{5.7}" → 6` |
| `\|U.Math.Floor` | Round down | `\|U.Math.Floor"{5.7}" → 5` |
| `\|U.Math.Ceiling` | Round up | `\|U.Math.Ceiling"{5.2}" → 6` |
| `\|U.Math.Abs` | Absolute value | `\|U.Math.Abs"{-5}" → 5` |

### Common Patterns

```polyglot
// Basic arithmetic
[r] $total :pg.float << \|U.Math.Add"{$price, $tax}"
[r] $discounted :pg.float << \|U.Math.Multiply"{$price, 0.9}"

// In loop
[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [r] $doubled :pg.float << \|U.Math.Double"{$num}"
   [v] *Into.Array
   [*] <item << $doubled
   [*] >array >> $results
```

---

## String Utilities

**Package:** `|U.String.*`
**Purpose:** String manipulation and processing

**See:** [String Package Documentation](./string/README.md)

### Quick Reference

| Pipeline | Purpose | Example |
|----------|---------|---------|
| `\|U.String.Concat` | Concatenate strings | `\|U.String.Concat"{\"Hello\", \" \", \"World\"}"` |
| `\|U.String.Split` | Split by delimiter | `\|U.String.Split"{\"a,b,c\", \",\"}" → ["a","b","c"]` |
| `\|U.String.Upper` | Convert to uppercase | `\|U.String.Upper"{\"hello\"}" → "HELLO"` |
| `\|U.String.Lower` | Convert to lowercase | `\|U.String.Lower"{\"HELLO\"}" → "hello"` |
| `\|U.String.Trim` | Remove whitespace | `\|U.String.Trim"{\" hello \"}" → "hello"` |
| `\|U.String.Length` | Get string length | `\|U.String.Length"{\"hello\"}" → 5` |
| `\|U.String.Substring` | Extract substring | `\|U.String.Substring"{\"hello\", 1, 3}" → "ell"` |
| `\|U.String.Replace` | Replace occurrences | `\|U.String.Replace"{\"hello\", \"l\", \"L\"}" → "heLLo"` |

### Common Patterns

```polyglot
// Text processing
[r] $cleaned :pg.string << \|U.String.Trim"{$input}"
[r] $normalized :pg.string << \|U.String.Lower"{$cleaned}"

// Pipeline composition
[r] \|U.String.Trim \|> \|U.String.Lower              // Chain Trim → Lower
[|] <input:pg.string << $raw_input                   // Input to Trim
[|] >trimmed:pg.string >> <input                     // Trim output → Lower input
[|] \|> \|U.String.Replace                            // Chain Lower → Replace
[|] >lowered:pg.string >> <input                     // Lower output → Replace input
[|] <old:pg.string << " "                            // Replace parameter: old
[|] <new:pg.string << "_"                            // Replace parameter: new
[|] \|>                                               // End chain
[|] >result:pg.string >> $result                     // Capture Replace output
```

---

## DateTime Utilities

**Package:** `|U.DateTime.*` (Alias: `|DT.*`)
**Purpose:** Date and time operations

**See:** [DateTime Package Documentation](./datetime/README.md)

### Quick Reference

| Pipeline | Purpose | Example |
|----------|---------|---------|
| `\|DT.Now` | Current timestamp | `\|DT.Now"" → "2025-12-15T10:30:00Z"` |
| `\|DT.Parse` | Parse date string | `\|DT.Parse"{\"2025-12-15\"}"` |
| `\|DT.Format` | Format timestamp | `\|DT.Format"{$ts, \"YYYY-MM-DD\"}"` |
| `\|DT.AddDays` | Add days to date | `\|DT.AddDays"{$date, 7}"` |
| `\|DT.AddHours` | Add hours to date | `\|DT.AddHours"{$date, 2}"` |
| `\|DT.Diff` | Difference between dates | `\|DT.Diff"{$end, $start}"` |
| `\|DT.Year` | Extract year | `\|DT.Year"{$date}" → 2025` |
| `\|DT.Month` | Extract month | `\|DT.Month"{$date}" → 12` |
| `\|DT.Day` | Extract day | `\|DT.Day"{$date}" → 15` |

### Common Patterns

```polyglot
// Current timestamp
[r] $timestamp :pg.string << \|DT.Now""
[r] $formatted :pg.string << \|DT.Format"{$timestamp, \"YYYY-MM-DD HH:mm:ss\"}"

// Date arithmetic
[r] $tomorrow :pg.string << \|DT.AddDays"{$today, 1}"
[r] $next_week :pg.string << \|DT.AddDays"{$today, 7}"
```

---

## Data Utilities

**Package:** `|U.Data.*`
**Purpose:** Data format loading and parsing (YAML, JSON, TOML, XML)

**See:** [Data Package Documentation](./data/README.md)

### Quick Reference

| Pipeline | Purpose | Example |
|----------|---------|---------|
| `\|YAML.Load` | Load YAML file | Used with `[s]` serial load block |
| `\|YAML.Parse` | Parse YAML string | `\|YAML.Parse"{$yaml_string}"` |
| `\|YAML.Dump` | Convert to YAML | `\|YAML.Dump"{$data}"` |
| `\|JSON.Load` | Load JSON file | Used with `[s]` serial load block |
| `\|JSON.Parse` | Parse JSON string | `\|JSON.Parse"{$json_string}"` |
| `\|JSON.Dump` | Convert to JSON | `\|JSON.Dump"{$data}"` |
| `\|TOML.Load` | Load TOML file | Used with `[s]` serial load block |
| `\|TOML.Parse` | Parse TOML string | `\|TOML.Parse"{$toml_string}"` |
| `\|XML.Parse` | Parse XML string | `\|XML.Parse"{$xml_string}"` |

### Common Patterns

```polyglot
// Load configuration file
[s] \|YAML.Load
[s] <file << "/config.yaml"
[s] >content >> $config :pg.serial
   [.] << *
[s][!] *! >> $error :!

[r] $db_host :pg.string << $config.database.host

// Parse string
[r] $parsed :pg.serial << \|JSON.Parse"{$json_string}"
[r] $user_name :pg.string << $parsed.user.name

// Convert to JSON
[r] $json_output :pg.string << \|JSON.Dump"{$data}"
```

---

## Cross-Package Examples

### Example 1: Data Processing Pipeline

```polyglot
{|} \|ProcessUserData
[|] <user_id :pg.string
[|] >report :pg.string

[t] \|T.Call
[W] \|W.Polyglot.Scope

   // Load user data
   [s] \|YAML.Load
   [s] <file << \|U.String.Concat"{"/users/", $user_id, ".yaml"}"
   [s] >content >> $user :pg.serial
      [.] << *

   // Process name
   [r] $full_name :pg.string << \|U.String.Concat"{$user.first_name, \" \", $user.last_name}"
   [r] $display_name :pg.string << \|U.String.Upper"{$full_name}"

   // Calculate age
   [r] $now :pg.string << \|DT.Now""
   [r] $age_days :pg.int << \|DT.Diff"{$now, $user.birth_date}"
   [r] $age_years :pg.int << \|U.Math.Divide"{$age_days, 365}"

   // Build report
   [r] $report_text :pg.string << \|U.String.Concat"{
      \"User: \", $display_name, \"\\n\",
      \"Age: \", $age_years, \" years\"
   }"

   [|] >report << $report_text

{x}
```

### Example 2: Batch Processing

```polyglot
{|} \|ProcessFiles
[|] <file_paths :pg.array.pg.string
[|] >results :pg.array.pg.serial

[t] \|T.Call
[W] \|W.Polyglot.Scope

   [p] ~ForEach.Array
   [~] <array << $file_paths
   [~] >item >> $file_path

      // Load file
      [s] \|YAML.Load
      [s] <file << $file_path
      [s] >content >> $data :pg.serial
         [.] << *
      [s][!] *! >> $load_error :!

      // Process if loaded successfully
      [y] $load_error.state !=? :pg.state.faulted
         [r] $upper_name :pg.string << \|U.String.Upper"{$data.name}"
         [r] $timestamp :pg.string << \|DT.Now""

      [v] *Into.Array
      [*] <item << $data
      [*] >array >> $all_data

   [|] >results << $all_data

{x}
```

---

## Related Documentation

- [Standard Library Overview](../README.md) - Complete package tree
- [Loop System](../../language/advanced/loop-system.md) - Using utilities in loops
- [Inline Pipelines](../../features/string-handling/inline-pipelines.md) - Inline syntax
- [Serial Load Block](../../language/advanced/serial-load-block.md) - Loading data files

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../README.md)
