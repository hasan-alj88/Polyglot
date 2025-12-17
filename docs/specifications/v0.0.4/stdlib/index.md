---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: index
shard: false

# --- Classification ---
type: api
topic: Polyglot Standard Library v0.0.4
summary: API reference: Polyglot Standard Library v0.0
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
# Polyglot Standard Library v0.0.4

**Complete reference for all built-in pipelines and operators**

---

## 🚀 Inline Pipeline Calls - Most Common Feature

**All standard library utilities support inline calls** using formatted string templates:

```polyglot
// Math utilities
[r] $sum :pg.int << |U.Math.Add"{$x}, {$y}"
[r] $doubled :pg.int << |U.Math.Double"{$value}"

// String utilities
[r] $upper :pg.string << |U.String.Upper"{$text}"
[r] $replaced :pg.string << |U.String.Replace"{$str}, {$old}, {$new}"

// DateTime utilities
[r] $now :pg.string << |DT.Now""
[r] $formatted :pg.string << |DT.Format"{$timestamp}, {$pattern}"
```

**Key Points:**
- `"{template}"` is a **formatted string**, not simple arguments
- `{$var}` uses default format, `{$var:hex}` uses specific format
- Behind the scenes: **three-phase execution** (representation → substitution → parsing)
- All utilities use **comma-separated format**: `"{arg1}, {arg2}, ..."`

**See:** [Inline Pipelines Complete Specification](../language/advanced/inline-pipelines.md)

---

## Quick Navigation

- [Utilities (`\|U.*`)](#utilities-u) - Math, String, DateTime, Data operations
- [Wrappers (`\|W.*`)](#wrappers-w) - Runtime integration and scope management
- [Unpack Operators (`~*`)](#unpack-operators-) - Loop expansion operations
- [Pack Operators (`**`)](#pack-operators--1) - Loop aggregation operations
- [Triggers (`\|T.*`)](#triggers-t) - Pipeline trigger types
- [Queues (`\|Q.*`)](#queues-q) - Queue control types

---

## Complete Package Tree

```
Polyglot Standard Library v0.0.4
│
├── |U.* (Utilities)
│   ├── |U.Math.*
│   │   ├── |U.Math.Add
│   │   ├── |U.Math.Subtract
│   │   ├── |U.Math.Multiply
│   │   ├── |U.Math.Divide
│   │   ├── |U.Math.Modulo
│   │   ├── |U.Math.Double
│   │   ├── |U.Math.Round
│   │   ├── |U.Math.Floor
│   │   ├── |U.Math.Ceiling
│   │   └── |U.Math.Abs
│   │
│   ├── |U.String.*
│   │   ├── |U.String.Concat
│   │   ├── |U.String.Split
│   │   ├── |U.String.Upper
│   │   ├── |U.String.Lower
│   │   ├── |U.String.Trim
│   │   ├── |U.String.Length
│   │   ├── |U.String.Substring
│   │   └── |U.String.Replace
│   │
│   ├── |U.DateTime.* (Alias: |DT.*)
│   │   ├── |DT.Now
│   │   ├── |DT.Parse
│   │   ├── |DT.Format
│   │   ├── |DT.AddDays
│   │   ├── |DT.AddHours
│   │   ├── |DT.Diff
│   │   ├── |DT.Year
│   │   ├── |DT.Month
│   │   ├── |DT.Day
│   │   ├── |DT.Hour
│   │   ├── |DT.Minute
│   │   └── |DT.Second
│   │
│   └── |U.Data.*
│       ├── |YAML.Load
│       ├── |YAML.Parse
│       ├── |YAML.Dump
│       ├── |JSON.Load
│       ├── |JSON.Parse
│       ├── |JSON.Dump
│       ├── |TOML.Load
│       ├── |TOML.Parse
│       └── |XML.Parse
│
├── |W.* (Wrappers)
│   ├── |W.Polyglot.Scope
│   ├── |W.RT.Python3.12
│   ├── |W.RT.Rust
│   ├── |W.RT.JavaScript
│   ├── |W.RT.Go
│   ├── |W.RT.Java
│   ├── |W.DB.Transaction
│   └── |W.HTTP.Client
│
├── |T.* (Triggers)
│   ├── |T.Call
│   ├── |T.Schedule
│   ├── |T.Event
│   └── |T.Watch
│
├── |Q.* (Queues)
│   ├── |Q.Serial
│   ├── |Q.Parallel
│   ├── |Q.Pool
│   └── |Q.Debounce
│
├── ~* (Unpack Operators)
│   ├── ~ForEach.Array
│   ├── ~ForEach.Range
│   ├── ~ForEach.Set
│   ├── ~ForEach.Map
│   ├── ~Enumerate
│   ├── ~Zip
│   ├── ~ZipWith
│   ├── ~Filter
│   ├── ~Take
│   ├── ~Drop
│   ├── ~TakeWhile
│   ├── ~DropWhile
│   ├── ~Partition
│   ├── ~Chunk
│   ├── ~Flatten
│   ├── ~Window
│   ├── ~Cycle
│   ├── ~Repeat
│   └── ~Range
│
└── ** (Pack Operators)
    ├── *Into.Array
    ├── *Into.Set
    ├── *Into.Map
    ├── *Join.All
    ├── *Join.First
    ├── *Join.Last
    ├── *Join.Any
    ├── *Reduce
    ├── *Fold
    ├── *Sum
    ├── *Product
    ├── *Min
    ├── *Max
    ├── *Count
    ├── *Average
    ├── *Concat
    ├── *Group
    └── *Distinct
```

---

## Utilities (`|U.*`)

**Purpose:** Common utility operations for math, strings, dates, and data formats

### Math (`|U.Math.*`)

Basic arithmetic and mathematical operations.

**See:** [Math Utilities](./utilities/math/README.md)

**Examples:**
```polyglot
[r] $sum :pg.float << \|U.Math.Add"{$a, $b}"
[r] $doubled :pg.float << \|U.Math.Double"{$value}"
[r] $rounded :pg.float << \|U.Math.Round"{$price}"
```

### String (`|U.String.*`)

String manipulation and processing.

**See:** [String Utilities](./utilities/string/README.md)

**Examples:**
```polyglot
[r] $upper :pg.string << \|U.String.Upper"{$name}"
[r] $parts :pg.array.pg.string << \|U.String.Split"{$text, \",\"}"
[r] $trimmed :pg.string << \|U.String.Trim"{$input}"
```

### DateTime (`|U.DateTime.*` / `|DT.*`)

Date and time operations.

**See:** [DateTime Utilities](./utilities/datetime/README.md)

**Examples:**
```polyglot
[r] $now :pg.string << \|DT.Now""
[r] $formatted :pg.string << \|DT.Format"{$timestamp, \"YYYY-MM-DD\"}"
[r] $tomorrow :pg.string << \|DT.AddDays"{$now, 1}"
```

### Data (`|U.Data.*`)

Data format loading and parsing (YAML, JSON, TOML, XML).

**See:** [Data Utilities](./utilities/data/README.md)

**Examples:**
```polyglot
[s] \|YAML.Load <file << "/config.yaml" >content >> $config [.] << *
[r] $json :pg.string << \|JSON.Dump"{$data}"
```

---

## Wrappers (`|W.*`)

**Purpose:** Runtime integration and execution environment management

### Polyglot Wrapper

**`|W.Polyglot.Scope`** - Pure Polyglot execution environment

**See:** [Polyglot Wrapper](./wrappers/polyglot/scope.md)

**Example:**
```polyglot
{|} \|MyPipeline
[t] \|T.Call
[W] \|W.Polyglot.Scope           // Pure Polyglot, no external runtime
{x}
```

### Runtime Wrappers

Integration with external programming language runtimes.

**See:** [Runtime Wrappers](./wrappers/runtime/README.md)

**Available Runtimes:**
- `|W.RT.Python3.12` - Python 3.12 runtime
- `|W.RT.Rust` - Rust runtime
- `|W.RT.JavaScript` - JavaScript/Node runtime
- `|W.RT.Go` - Go runtime
- `|W.RT.Java` - Java runtime

**Example:**
```polyglot
{|} \|AnalyzeData
[t] \|T.Call
[W] \|W.RT.Python3.12            // Python environment with NumPy, etc.

   [r] $analysis << \|Python.NumPy.Analyze <data << $dataset

{x}
```

### Specialized Wrappers

- `|W.DB.Transaction` - Database transaction wrapper
- `|W.HTTP.Client` - HTTP client wrapper

**See:** [Wrappers Documentation](./wrappers/README.md)

---

## Triggers (`|T.*`)

**Purpose:** Define when and how pipelines execute

**See:** [Pipeline Structure](../language/control-flow/pipeline-structure.md#trigger-t---required)

| Trigger | Purpose | Example |
|---------|---------|---------|
| `\|T.Call` | Execute on explicit call | Most common trigger |
| `\|T.Schedule` | Execute on schedule | Cron-like scheduling |
| `\|T.Event` | Execute on event | Event-driven pipelines |
| `\|T.Watch` | Execute on file/resource change | File watchers |

**Example:**
```polyglot
{|} \|ProcessOrder
[t] \|T.Call                      // Execute when called
[W] \|W.Polyglot.Scope
{x}
```

---

## Queues (`|Q.*`)

**Purpose:** Control pipeline instance concurrency

**See:** [Pipeline Structure](../language/control-flow/pipeline-structure.md#queue-q---optional)

| Queue | Behavior | Use Case |
|-------|----------|----------|
| `\|Q.Serial` | One instance at a time (FIFO) | Sequential operations |
| `\|Q.Parallel` | Unlimited concurrent instances | Independent operations |
| `\|Q.Pool` | Limited concurrent instances | Resource throttling |
| `\|Q.Debounce` | Ignore rapid calls | User input handling |

**Example:**
```polyglot
{|} \|ProcessPayment
[t] \|T.Call
[Q] \|Q.Serial                    // One payment at a time
[W] \|W.Polyglot.Scope
{x}
```

---

## Unpack Operators (`~*`)

**Purpose:** Expand data from main scope to iteration scope in loops

**Marker:** `[~]` used with unpack operators

**See:** [Loop System](../language/advanced/loop-system.md)

### Core Unpack Operators

| Operator | Purpose | Example |
|----------|---------|---------|
| `~ForEach.Array` | Iterate over array elements | Most common loop |
| `~orEach.Range` | Iterate over numeric range | `for i in 0..10` |
| `~orEach.Set` | Iterate over set elements | Unique items |
| `~orEach.Map` | Iterate over key-value pairs | Dictionary iteration |
| `~numerate` | Add index to iteration | `(index, value)` pairs |
| `~ip` | Combine multiple arrays | Parallel iteration |

**Example:**
```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $processed << \|Transform <input << $element

   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $results
```

**See:** [Unpack Operators Documentation](./unpack-operators/README.md)

---

## Pack Operators (`**`)

**Purpose:** Aggregate data from iteration scope back to main scope

**Marker:** `[*]` used with pack operators
**Join Marker:** `[v]` precedes pack operation

**See:** [Loop System](../language/advanced/loop-system.md)

### Core Pack Operators

| Operator | Purpose | Example |
|----------|---------|---------|
| `*nto.Array` | Collect into array | Build result list |
| `*nto.Set` | Collect into set | Unique results |
| `*nto.Map` | Collect into map | Key-value results |
| `*oin.All` | Wait for all iterations | Synchronization |
| `*oin.First` | Take first result | Early termination |
| `*um` | Sum numeric values | Total calculation |
| `*ount` | Count iterations | Length/size |
| `*ax` / `*in` | Find maximum/minimum | Extrema |

**Example:**
```polyglot
[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num

   [r] $squared :pg.int << \|U.Math.Multiply"{$num, $num}"

   [v] *Sum                       // Sum all squared values
   [*] <item << $squared
   [*] >result >> $sum_of_squares
```

**See:** [Pack Operators Documentation](./pack-operators/README.md)

---

## Usage Patterns

### Pattern 1: Simple Utility Call

```polyglot
[r] $uppercase :pg.string << \|U.String.Upper"{$name}"
[r] $total :pg.float << \|U.Math.Add"{$price, $tax}"
```

### Pattern 2: Loop with Transform

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $element

   [r] $transformed << \|ProcessItem <input << $element

   [v] *Into.Array
   [*] <item << $transformed
   [*] >array >> $results
```

### Pattern 3: Data Loading

```polyglot
[s] \|YAML.Load
[s] <file << "/config.yaml"
[s] >content >> $config :pg.serial
   [.] << *
[s][!] *! >> $error :!

[r] $db_host :pg.string << $config.database.host
```

### Pattern 4: Runtime Integration

```polyglot
{|} \|AnalyzeWithPython
[t] \|T.Call
[W] \|W.RT.Python3.12

   [r] $result << \|Python.ML.Predict <model << $model <data << $input

{x}
```

---

## Inline Pipeline Syntax

**Many standard library pipelines support inline syntax:**

```polyglot
[r] $now :pg.string << \|DT.Now""
[r] $doubled :pg.float << \|U.Math.Double"{$value}"
[r] $upper :pg.string << \|U.String.Upper"{$text}"
```

**Pattern:** `|PipelineName"{args}"` or `|PipelineName""`

**See:** [Inline Pipelines](../features/string-handling/inline-pipelines.md)

---

## Package Organization

```
standard-library/
├── README.md (this file)
│
├── utilities/
│   ├── README.md
│   ├── math/
│   │   ├── README.md
│   │   ├── add.md
│   │   ├── subtract.md
│   │   └── ...
│   ├── string/
│   │   ├── README.md
│   │   └── ...
│   ├── datetime/
│   │   ├── README.md
│   │   └── ...
│   └── data/
│       ├── README.md
│       └── ...
│
├── wrappers/
│   ├── README.md
│   ├── polyglot/
│   │   └── scope.md
│   └── runtime/
│       ├── README.md
│       └── ...
│
├── unpack-operators/
│   ├── README.md
│   ├── foreach-array.md
│   └── ...
│
└── pack-operators/
    ├── README.md
    ├── into-array.md
    └── ...
```

---

## Documentation Format

**Each pipeline/operator document includes:**

1. **Signature** - Complete pipeline definition with I/O
2. **Parameters** - Input/output with types
3. **Description** - What it does
4. **Examples** - Basic usage + in-pipeline usage
5. **See Also** - Cross-references

**Example template:**

```markdown
# |U.Math.Double

**Category:** Utilities > Math
**Purpose:** Doubles a numeric value
**Since:** v0.0.1

## Signature
{|} |U.Math.Double
[|] <value :pg.float
[|] >result :pg.float
{x}

## Parameters
**Inputs:**
- `<value` :pg.float - The number to double

**Outputs:**
- `>result` :pg.float - The doubled value

## Description
Multiplies the input value by 2.

## Examples

### Basic Usage
[r] $doubled :pg.float << |U.Math.Double"{5.0}"
// $doubled = 10.0

### In Pipeline
[r] |ProcessNumbers
[|] <numbers :pg.array.pg.float

[p] ~ForEach.Array
[~] <array << $numbers
[~] >item >> $num
   [r] $doubled :pg.float << |U.Math.Double"{$num}"
   [v] *Into.Array
   [*] <item << $doubled
   [*] >array >> $results
{x}

## See Also
- [Math Package](../README.md)
- [|U.Math.Add](add.md)
```

---

## Related Documentation

- [Loop System](../language/advanced/loop-system.md) - Unpack/pack operators in detail
- [Pipeline Structure](../language/control-flow/pipeline-structure.md) - Triggers and wrappers
- [Inline Pipelines](../features/string-handling/inline-pipelines.md) - Inline syntax

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../README.md)
