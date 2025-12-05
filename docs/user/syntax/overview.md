# Polyglot Syntax Overview

**Version:** 0.0.2
**Status:** Complete Reference
**Last Updated:** 2025-12-02

---

## Table of Contents

1. [Introduction](#introduction
2. [Core Principles](#core-principles
3. [Block Marker System](#block-marker-system
4. [No Keywords Philosophy](#no-keywords-philosophy
5. [Formatting Rules](#formatting-rules
6. [File Structure](#file-structure
7. [Quick Reference](#quick-reference

---

## Introduction

Polyglot is a **pipeline orchestration language** designed for **async-first**, **multi-language execution**. Unlike traditional programming languages that use keywords and indentation, Polyglot uses **explicit 3-character block markers** to define structure and execution semantics.

### Key Features

- **No keywords** - All identifiers use operator prefixes (`.` `#` `|` `!`
- **Block markers** - Structure defined by `[x]` markers, not indentation
- **Async-centric** - Variables transition through states (Declared → Pending → Ready
- **Type-safe** - Strong typing with namespace-based type system (`:pg.`, `rs\`, `py\`, etc.
- **Multi-language** - Native FFI support for Python, Rust, Node, Go

---

## Core Principles

### 1. Everything is a Pipeline

Polyglot code consists of **pipelines** that orchestrate execution across multiple languages and services.

```polyglot
[|] |ProcessData
[i] .input_file:pg.path
[t] |T.Call
[o] .result:pg.string

[W] |W.RT.Python3.14

[r] |RT.Python.Run.File
[<] <file:pg.path << \\FileDir\\python\\process.py
[<] <input:pg.serial << {.input_file
[>] >output:pg.serial >> .result

[o] .result:pg.string
[X]
```

### 2. Explicit Structure via Block Markers

**No indentation** is used. Instead, `[x]` block markers define:
- **What** happens (structure
- **When** it happens (execution semantics
- **How** it happens (sequential, parallel, conditional

```polyglot
[r]  Sequential execution
[p]  Parallel execution
[?]  Conditional branch
[~]  Nesting scope
```

### 3. Async State System

Variables transition through states automatically:

```
Declared → Pending → Ready (success
                  └→ Faulted (error
```

**No explicit await** - the language handles async automatically when you PULL from variables.

### 4. Type Safety with Namespaces

All types use backslash separator: `:pg.string`, `rs\HashMap`, `py\dict`

```polyglot
.user:pg.string              // Polyglot string
.data: rs\Vec                 // Rust vector
.config: py\dict              // Python dictionary
```

---

## Block Marker System

### What Are Block Markers?

Block markers are **3-character sequences** that start every line:

```
[x]  where x is a single character
```

### Zero Indentation Rule

**CRITICAL:** Polyglot uses **NO indentation**. All lines start at column 0.

```polyglot
[|] |Pipeline
[i] .input:pg.string
[r] .x:pg.int << 1
[?] .x >? 0
[~][r] .y:pg.int << 2      // [~] indicates nesting, NOT indentation
[~]
[X]
```

### Block Marker Categories

#### Structure Markers
- `[@]` Package declaration
- `[|]` Pipeline definition
- `[#]` Enumeration definition
- `[!]` Error definition
- `[M]` Macro definition
- `[X]` Block terminator (closes all blocks

#### Execution Markers
- `[r]` Sequential execution OR variable declaration
- `[p]` Parallel execution
- `[b]` Background execution
- `[s]` Serial execution (streaming
- `[Y]` Join point (synchronize parallel tasks

#### Data Flow Markers
- `[i]` Input declaration
- `[o]` Output declaration
- `[<]` Input binding (push into operation
- `[>]` Output binding (pull from operation

#### Control Flow Markers
- `[?]` Conditional check
- `[t]` Trigger declaration
- `[Q]` Queue control
- `[W]` Wrapper (runtime environment

#### Nesting and Scope
- `[~]` Nesting indicator (marks nested scope
- `[{]` Macro scope in
- `[]` Macro scope out
- `[\]` Setup block
- `[/]` Cleanup block

#### Boolean Logic
- `[&]` AND operator
- `[+]` OR operator
- `[-]` NOT operator
- `[^]` XOR operator
- `[.]` Grouping

#### Special Markers
- `[*]` Line continuation
- `[A]` Alias definition

### Context-Dependent Markers

Some markers have different meanings based on context:

#### `[r]` - Sequential Execution OR Variable Declaration

**At pipeline scope:**
```polyglot
[r] .variable:pg.int << 42        // Variable declaration
```

**In execution flow:**
```polyglot
[r] |Pipeline                      // Sequential pipeline call
[<] .input << .data
[>] .output >> .result
```

#### `[<]` - Input Binding (Requires Parent

**MUST have parent block** - cannot stand alone:

```polyglot
// ✓ CORRECT: [<] within pipeline call
[r] |ProcessData
[<] <input:pg.string << .data     // Has parent: [r] |ProcessData

// ✗ WRONG: [<] at pipeline scope
[<] <variable:pg.string << "value"  // ERROR: No parent!
// Should be: [r] .variable:pg.string << "value"
```

---

## No Keywords Philosophy

### ALL Identifiers Need Prefixes

Polyglot has **zero keywords**. Every identifier uses an operator prefix:

| Prefix | Usage | Example |
|--------|-------|---------|
| `.` | Variables | `.user`, `.file_path`, `.data.value` |
| `#` | Enumerations | `#UserProfile`, `#PgVar.States.Ready` |
| `&#124;` | Pipelines | `&#124;ProcessData`, `&#124;T.Cron`, `&#124;W.Python` |
| `!` | Errors | `!NetworkTimeout`, `!ValidationError` |

### Why No Keywords?

1. **Clarity** - No ambiguity about identifier type
2. **Serialization** - Maps cleanly to hierarchical data
3. **Cross-language** - Unambiguous FFI boundaries
4. **String interpolation** - `"Hello {.name"` is unambiguous

### Common Mistakes

```polyglot
// ✗ WRONG: Missing prefixes
[|] FetchUser              // Missing |
[t] T.Call                 // Missing |
[r] user:pg.string        // Missing .

// ✓ CORRECT: All prefixes present
[|] FetchUser
[t] |T.Call
[r] .user:pg.string
```

---

## Formatting Rules (PFG v1.0

### Vertical Spacing

#### 3 Blank Lines Between File-Scope Definitions

**REQUIRED:** Three blank lines before each:
- `[@]` Package declaration
- `[|]` Pipeline definition
- `[#]` Enumeration definition
- `[!]` Error definition
- `[M]` Macro definition

```polyglot
[@] @Local::MyProject:1.0.0.0
[X]

[#] #Config
[<] <timeout:pg.int << 30
[X]

[|] |ProcessData
[i] .input:pg.string
[t] |T.Call
[o] !No.Output
[X]
```

#### 1 Blank Line Before Branch Points

**RECOMMENDED:** Single blank line before:
- `[p]` Parallel blocks
- `[?]` Conditional blocks
- `[!]` Error handlers

```polyglot
[r] .x:pg.int << 1
[r] .y:pg.int << 2

[?] .x >? .y
[~][o] .x:pg.int
[~]
```

### Horizontal Spacing

```polyglot
.variable:pg.string << "value"    // ✓ Spaces around operators
.variable::pg.string<<"value"       // ✗ No spaces
```

### Case Sensitivity

**CRITICAL:** Polyglot is **fully case-sensitive**:

```polyglot
.user   ≠  .User   ≠  .USER        // Different variables
#Config ≠  #config                 // Different enumerations
|Pipeline ≠ |pipeline              // Different pipelines
```

### Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Variables | `.snake_case` | `.user_name`, `.file_path` |
| Pipelines | `&#124;CamelCase` | `&#124;ProcessData`, `&#124;ValidateInput` |
| Enumerations | `#CamelCase` | `#UserProfile`, `#Config.Database` |
| Errors | `!CamelCase` | `!NetworkTimeout`, `!ValidationError` |

---

## File Structure

### File Extension

All Polyglot files use `.pg` extension:
```
ProcessData.pg
UserService.pg
Config.pg
```

### Package Declaration

**Every file starts with package declaration:**

```polyglot
[@] @Local::MyProject:1.0.0.0
[X]
```

### Mandatory Pipeline Sections

Every `[|]` pipeline **MUST include** (in order:

1. **Inputs** `[i]` - At least one (or `[i] !No.Input`
2. **Triggers** `[t]` - At least one (or pipeline will NEVER run!
3. **Queue Config** `[Q]` - Optional
4. **Wrapper** `[W]` - Optional (only when using external runtimes
5. **Execution Blocks** - Sequential, parallel, conditional logic
6. **Outputs** `[o]` - At least one (or `[o] !No.Output`

### Minimal Valid Pipeline

```polyglot
[|] |MinimalPipeline
[i] !No.Input                      // No inputs
[t] |T.Call                        // Manual trigger
[o] !No.Output                     // No output
[X]
```

### Complete Pipeline Example

```polyglot
[|] |ProcessData
[i] .input_file:pg.path           // Required input
[i] .timeout:pg.int <~ 30         // Optional with default
[t] |T.Call                        // Manual trigger
[o] .result:pg.string             // Output

[Q]                                // Queue control
[<] <priority:pg.int << 5

[W] |W.RT.Python3.14               // Python wrapper

[r] |RT.Python.Run.File
[<] <file:pg.path << \\FileDir\\python\\process.py
[<] <input:pg.serial << {.input_file, .timeout
[>] >output:pg.serial >> .result

[o] .result:pg.string
[X]
```

---

## Quick Reference

### Operator Prefixes

```polyglot
.variable                          // Variables
#Enumeration                       // Enumerations
|Pipeline                          // Pipelines
!Error                             // Errors
~Operator                          // Unpack operators (~ForEach
~Y.Join                            // Join operators (~Y.IntoArray
```

### Type Separator

```polyglot
:pg.string                          // ✓ Backslash separator
pg/string                          // ✗ Forward slash
pg:string                          // ✗ Colon
```

### Push and Pull Operators

```polyglot
<< value                           // PUSH (immediate
<~ value                           // PUSH default (can override
>> .variable                       // PULL (auto-await
```

### Comparison Operators

```polyglot
=?                                 // Equal to
=!?                                // Not equal to
>?                                 // Greater than
<?                                 // Less than
=>?                                // Greater than or equal
=<?                                // Less than or equal
```

### Pattern Matching

```polyglot
*?                                 // Wildcard match (catch-all
re?                                // Regex match
```

### String Interpolation

```polyglot
"Hello {.name"                    // Variable interpolation
"{.count:Hex"                     // With format specifier
"Path: {.file.path"               // Nested variable
```

### Collection Syntax

```polyglot
<< {1, 2, 3                       // ✓ Use braces
<< [1, 2, 3]                       // ✗ NO brackets!
<< {                              // Empty collection
```

### Reserved Namespaces

```polyglot
.*.pgvar.*                         // Variable metadata (ALWAYS Final
#PgVar.States.*                    // State enumeration
#Boolean.True / #Boolean.False     // Boolean values
#None                              // Unit type
```

---

## Common Patterns

### Variable Declaration

```polyglot
[r] .user:pg.string               // Declared state
[r] .count:pg.int << 42           // Final state (Immediate
[r] .timeout:pg.int <~ 30         // DefaultReady (can push one more time
```

### Conditional Logic

```polyglot
[?] .x >? 10
[~][r] .result:pg.string << "high"
[~]

[?] *?                             // REQUIRED catch-all
[~][r] .result:pg.string << "low"
[~]
```

### Parallel Execution

```polyglot
[p] ~ForEach                       // Unpack operator
[<] .items                         // Input collection
[>] .item                          // Individual item
[~][r] |ProcessItem                // Process each in parallel
[~][<] .data << .item
[~][>] .result >> .processed
[~]
[~][Y] ~Y.IntoArray                // Join results
[~][<] .processed
[~][>] .all_results
[~]
```
 or 
```Polyglot
[p] |Task1
[~][r] |SubTask1
...

[p] |Task2
...
```
### Error Handling

```polyglot
[r] |MightFail
[<] .input << .data
[>] .output >> .result
[~]
[~][!] !NetworkTimeout             // Catch specific error
[~][<] <error: !NetworkTimeout << .timeout_error
[~][r] |HandleTimeout              // Recovery logic
[~]
```

---

## Next Steps

- [Block Markers Reference](block-markers.md - Complete list of all 23+ markers
- [Operators Guide](operators.md - All operators with examples
- [Type System](type-system.md - Type declarations and foreign types
- [Enumerations](enumerations.md - Regular and reserved enumerations
- [Error Handling](error-handling.md - Error types and recovery patterns
- [Comments](comments.md - Single-line and multi-line comments

---

## See Also

- [Polyglot Formatting Guidelines (PFG v1.0](/docs/technical/polyglot-formatting-guidelines-v1.0.md
- [Grammar Specification (EBNF](/docs/ai-context/grammar.ebnf
- [Constraints Reference](/docs/ai-context/constraints.yaml
- [Annotated Examples](/docs/ai-context/examples-annotated.pg

---

**Next:** [Block Markers →](block-markers.md
