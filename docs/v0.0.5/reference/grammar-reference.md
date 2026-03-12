# Grammar Reference - v0.0.5

**Version:** 0.0.5
**Status:** Official Language Reference
**Last Updated:** 2026-01-04
**Formal Grammar:** [grammar-v0.0.5.ebnf](./grammar-v0.0.5.ebnf)

---

## Overview

This document provides a human-readable explanation of Polyglot v0.0.5 syntax. For the formal EBNF grammar specification, see [grammar-v0.0.5.ebnf](./grammar-v0.0.5.ebnf).

---

## Program Structure

### File Organization

Every Polyglot file follows this structure:

```
1. Package block (required, once)
2. Enum definitions (optional, multiple)
3. Pipeline definitions (optional, multiple)
```

### Package Block

```polyglot
{@} @Local:Example.Project:1.0.0
{x}
```

**Components:**
- `{@}` - Package block start
- `@` - Package marker
- Scope: `Local`, `Shared`, or `Public`
- Name: `Example.Project` (namespace with `::`  or `.`)
- Version: `1.0.0` (semantic versioning)
- `{x}` - Block end

---

## Enum Definitions

### Basic Enum

```polyglot
{#} #MyEnum
[.] .Value1
[.] .Value2
{x}
```

### Enum with Alias

```polyglot
{#} #Config.Database
[A] #DBConfig
[.] .host:string
[.] .port:uint
{x}
```

### Enum Implementing Schema

```polyglot
{#} #MyApp.Database-DB-Settings
[A] #AppDB
[s] << |TOML.Load"\\FileDir\\config.toml"
   [.] .host:string << .db.host
   [.] .port:uint << .db.port
   [.] .username:string << .db.user
   [.] .password:string << .db.pass
   [.] .database:string << .db.name
[s][!] !*
{x}
```

**Key Elements:**
- `{#}` - Enum block start
- Enum name with optional schema: `#Name-Schema`
- `[A]` - Alias declaration
- `[s]` - Serial load block
- `[.]` - Field definition
- `[s][!]` - Serial error handler (scope-wide)
- `{x}` - Block end

---

## Pipeline Definitions

### Basic Structure

```polyglot
{|} |PipelineName
[%] %Doc << "Description"

[t] |T.Cli"command"           %% Optional trigger

[<] <input:type               %% Optional inputs
[>] >output:type              %% Optional outputs

[w] |W.Wrapper                %% Optional wrappers
 |  >env >> $env

[r] $var << expression        %% Execution statements

{x}
```

### Required Order

```
1. {|} Pipeline signature
2. [%] Metadata (optional)
3. [t] Triggers (optional)
4. [<] Inputs (optional)
5. [>] Outputs (optional)
6. [w] Wrappers (optional, MUST come before execution)
7. Execution markers: [r] [p] [f] [b] [*]
8. {x} Block end
```

**CRITICAL:** ALL `[w]` wrappers MUST come BEFORE ANY execution markers!

---

## Markers Reference

### Block Markers

| Marker | Name | Purpose |
|--------|------|---------|
| `{@}...{x}` | Package block | Package definition |
| `{#}...{x}` | Enum block | Enum definition |
| `{|}...{x}` | Pipeline block | Pipeline definition |

### Enum Markers

| Marker | Name | Purpose |
|--------|------|---------|
| `[A]` | Alias | Enum alias name |
| `[s]` | Serial load | Load data from file |
| `[.]` | Field | Field definition |
| `[s][!]` | Serial error | Error handler for serial loads |

### Pipeline Structure Markers

| Marker | Name | Purpose |
|--------|------|---------|
| `[%]` | Metadata | Pipeline metadata |
| `[t]` | Trigger | Activation trigger |
| `[<]` | Input | Input parameter |
| `[>]` | Output | Output parameter |
| `[w]` | Wrapper | Resource wrapper |
| `[Q]` | Query | Query parameter (optional) |

### Execution Markers

| Marker | Name | Purpose |
|--------|------|---------|
| `[r]` | Run | Execute statement/pipeline |
| `[p]` | Parallel | Parallel loop |
| `[f]` | Fork | Conditional branch (exhaustive) |
| `[b]` | Branch | Conditional branch (non-exhaustive) |
| `[*]` | Pack | Pack/aggregate operation |
| `[c]` | Code | Code block line |
| `[+]` | Continue | Line continuation (legacy) |

### Error Handling

| Marker | Name | Purpose |
|--------|------|---------|
| `[!]` | Error block | Handle specific error |
| `[!] !*` | Catch-all | Handle remaining errors |

---

## Operators

### Assignment Operators

| Operator | Direction | State | Example |
|----------|-----------|-------|---------|
| `<<` | Pull ← | Final | `$x << 5` |
| `<~` | Pull ← | Default | `$x <~ 5` |
| `>>` | Push → | Final | `5 >> $x` |
| `~>` | Push → | Default | `5 ~> $x` |

**Arrow shows data flow direction!**

### Comparison Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `?=` | Equal | `$x ?= 5` |
| `?>` | Greater than | `$x ?> 5` |
| `?<` | Less than | `$x ?< 5` |
| `?>=` | Greater or equal | `$x ?>= 5` |
| `?<=` | Less or equal | `$x ?<= 5` |
| `?!=` | Not equal | `$x ?!= 5` |

### Loop Operators

| Operator | Purpose | Example |
|----------|---------|---------|
| `>>` | Feed to unpack | `$array >> ~ForEach.Array` |
| `~` | Unpack prefix | `~ForEach.Array` |
| `*` | Pack prefix | `*Into.Array` |

### Enum Operators

| Operator | Type | Example |
|----------|------|---------|
| `-` | Reserved enum | `-DT-Now` |
| `#` | User enum | `#MyEnum.Value` |

### Field Operators

| Operator | Purpose | Example |
|----------|---------|---------|
| `.` | Field access | `$config.host` |
| `.` | Field definition | `.field_name:type` |

---

## Types

### Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| `:string` | Text | `"hello"` |
| `:uint` | Unsigned integer | `42` |
| `:int` | Signed integer | `-10` |
| `:float` | Floating point | `3.14` |
| `:bool` | Boolean | `-True` / `-False` |
| `:dt` | DateTime | `|DT.Now""` |
| `:path` | File/folder path | `\\FileDir\\file.txt` |
| `:serial` | Serial structure | `{:}` |
| `:error` | Error value | `!ErrorType` |

### Collection Types

| Type | Description | Example |
|------|-------------|---------|
| `:array.T` | Ordered list | `( 1, 2, 3 )` |
| `:set.T` | Unique values | `{ "a", "b" }` |

**Note:** Replace `T` with element type (e.g., `:array.uint`)

### Native Types

For runtime kwargs (language-specific):

**Python:**
- `:py.str`, `:py.int`, `:py.float`, `:py.bool`

**Rust:**
- `:rust.String`, `:rust.i32`, `:rust.u32`, `:rust.f64`, `:rust.bool`

**JavaScript:**
- `:js.string`, `:js.number`, `:js.boolean`

### Enum Types

| Type | Prefix | Example |
|------|--------|---------|
| User enum | `#` | `#MyEnum.Value` |
| Reserved enum | `-` | `-Boolean-True` |

### Schema Types

Reserved schemas (type-safe configuration):

- `-DB-Settings` - Database configuration
- `-RT-Environment-Python` - Python runtime
- `-RT-Environment-Rust` - Rust runtime
- `-RT-Environment-JavaScript` - JavaScript runtime

---

## Literals

### String Literals

```polyglot
"simple string"
"string with {$variable} interpolation"
```

### Number Literals

```polyglot
42          %% Unsigned integer
-10         %% Signed integer
3.14        %% Float
```

### Path Literals

```polyglot
\\FileDir\\file.txt           %% File
\\FileDir\\logs\              %% Folder (trailing backslash)
\\NoPath\                     %% No path value
\\WorkDir\                    %% Working directory
```

### Collection Literals

```polyglot
%% Arrays
( )                           %% Empty
( 1, 2, 3 )                   %% Integers
( "a", "b", "c" )             %% Strings

%% Sets
{ }                           %% Empty
{ "unique", "values" }        %% Strings

%% Serials
{:}                           %% Empty
{ .field: "value" }           %% Inline (simple)
{
[+]  .field1: "value1",
[+]  .field2: "value2"
[+] }                         %% Multi-line
```

### Boolean Literals

```polyglot
-True
-False
-Boolean-True
-Boolean-False
```

**Note:** `:bool` type is equivalent to `-Boolean`

---

## Comments

### Single-Line

```polyglot
%% This is a single-line comment
```

### Multi-Line

```polyglot
%{
This is a
multi-line comment
}%
```

**Note:** Distinct from embedded code comments (Python #, JavaScript //, etc.)

---

## Expressions

### Variable References

```polyglot
$variable
$config.host
$current.index
```

### Pipeline Calls

```polyglot
|DT.Now""
|U.Math.Add
|RT.Python.Code
```

### Enum Values

```polyglot
%% User enums
#MyEnum.Value
#Config.Database.Production

%% Reserved enums
-DT-Now
-Boolean-True
-Email-Status-Success
```

### Field Access

```polyglot
$config.database.host
$current.value
$item.index
```

---

## I/O Syntax

### Pipeline I/O

```polyglot
[r] |Pipeline
 |  <input:type << $value
 |  <env.vars.name:string << "value"
 |  >output:type >> $result
```

**Key:**
- ` | ` - Space-wrapped pipe for I/O
- `<` - Input parameter
- `>` - Output parameter
- `<<` / `>>` - Assignment operators

### Trigger I/O

```polyglot
[t] |T.Folder.NewFiles
 |  <folder:path << $watchFolder
 |  >files:array.path >> <newFiles
```

**Wiring:** `>trigger_output >> <pipeline_input`

### Wrapper I/O

```polyglot
[w] |W.RT.Python
 |  <version:string << "3.11"
 |  >environment-RTenv-python >> $pyEnv
```

### Loop I/O

```polyglot
%% Unpack
[r] $items >> ~ForEach.Array

%% Pack
[*] *Into.Array
 *  <item.field:type << $value
 *  >array >> $results
```

**Note:** Loop parameters use ` * ` not ` | `

---

## Control Flow

### Fork (Exhaustive)

```polyglot
[f] $value ?= 10
   [r] $status << "ten"

[f] $value ?> 10
   [r] $status << "greater"

[f] *?
   [r] $status << "other"  %% Required for exhaustiveness
```

### Branch (Non-Exhaustive)

```polyglot
[b] $count ?> 0
   [r] |ProcessItems
```

### Loops

**Sequential:**
```polyglot
[r] $items >> ~ForEach.Array
   [r] $result << |Process"{$current}"
```

**Parallel:**
```polyglot
[p] $urls >> ~ForEach.Array
   [r] |HTTP.Get"{$current}"
```

---

## Error Handling

### Error Blocks

```polyglot
[r] |RiskyOperation
 |  <input:string << $data
   [!] !SpecificError
      [r] $status << "failed"

   [!] !*
      [r] $status << "success"  %% Catch-all required
```

### Error Patterns

```polyglot
!ErrorType              %% Specific error
!Category.SubError      %% Hierarchical error
!*                      %% Wildcard (any error or success)
```

---

## Code Blocks

### Recommended: [c] Marker

```polyglot
<code:string << |Python""
[c] import os
[c] def hello():
[c]     print("Hello World")
[c] hello()
```

### Legacy: [+] Marker

```polyglot
<code:string << |Python""
[+] +"import os"
[+] +"def hello():"
[+] -"    print('Hello World')"
[+] +"hello()"
```

### Variable Interpolation

```polyglot
[c] log_file = "{$logPath}"
[c] print(f"Logging to {log_file}")
```

---

## Syntax Rules

### Rule 1: One Expression Per Line

```polyglot
%% Correct
[r] $a << 1
[r] $b << 2

%% Wrong
[r] $a << 1; [r] $b << 2
```

**Pattern:** `<indent(s)><Marker><One-expression>`

### Rule 2: Field Naming

```polyglot
%% Compound fields: underscores
[.] .total_customers:uint
[.] .created_at:dt

%% Reserved enum references: dashes
[.] .connection-DB-Connection
[.] .status-Email-Status
```

### Rule 3: Wrapper Ordering

```polyglot
%% Correct
[w] |W.RT.Python >> $env
[r] |RT.Python.Code

%% Wrong
[r] $value << 42
[w] |W.RT.Python  %% ERROR: wrapper after execution!
```

**ALL `[w]` MUST precede ALL execution markers!**

### Rule 4: Newline Conventions

```
- 3 blank lines before {} blocks (except {@} at file start)
- 1 blank line before [] marker with pipeline call
- No blank lines between consecutive I/O parameters
```

### Rule 5: Type Annotations

Always explicit and clear:

```polyglot
%% Good
[r] $items:array.uint << ( 1, 2, 3 )
[r] $config:serial << {:}

%% Bad
[r] $items:array << ( 1, 2, 3 )  %% Missing element type
```

### Rule 6: Error Exhaustiveness

```polyglot
%% Correct - exhaustive
[!] !SpecificError
   [r] $result << "error"
[!] !*
   [r] $result << "success"

%% Wrong - not exhaustive
[!] !SpecificError
   [r] $result << "error"
%% COMPILE ERROR: missing catch-all
```

### Rule 7: Variable Lifecycle

```polyglot
%% Correct - default then final
[r] $status:string <~ "pending"   %% Default
[f] $condition
   [r] $status << "complete"      %% Final override

%% Wrong - trying to reassign final
[r] $status:string << "pending"   %% Final
[r] $status << "complete"         %% ERROR: already final!
```

**Maximum 2 pushes:** one default `<~`, one final `<<`

### Rule 8: Reserved vs User Enums

```polyglot
%% Reserved enums (stdlib)
-DT-Now
-Boolean-True
-Email-Status-Success

%% User enums (your code)
#MyEnum.Value
#Config.Database.Production
```

**Visual distinction prevents conflicts!**

---

## Common Patterns

### Pattern: Default with Override

```polyglot
[r] $timeout:uint <~ 30  %% Default value

[f] $config.timeout ?> 0
   [r] $timeout << $config.timeout  %% Override
```

### Pattern: Collection Building

```polyglot
[r] $items >> ~ForEach.Array
   [f] $current.active ?= -True
      [*] *Into.Array
       *  <item.id:uint << $current.id
       *  <item.name:string << $current.name
       *  >array >> $activeItems
```

### Pattern: Aggregation

```polyglot
[r] $values >> ~ForEach.Array
   [*] *Aggregate.Sum
    *  <value:uint << $current
    *  >sum:uint >> $total

[r] $total:uint <~ 0  %% Default before loop
```

### Pattern: Error Accumulation

```polyglot
[r] $operations >> ~ForEach.Array
   [r] |RiskyOp
    |  <input << $current
      [!] !RiskyOp.Error
         [*] *Into.Array
          *  <item.error:error << !RiskyOp.Error
          *  >array >> $errors
      [!] !*
         [r] |U.Do.Nothing
```

---

## See Also

- [Formal EBNF Grammar](./grammar-v0.0.5.ebnf) - Machine-readable specification
- [Loop System Guide](../language/loop-system.md) - Iteration patterns
- [Variable Lifecycle](../language/variable-lifecycle.md) - Variable states
- [Error Handling](../language/error-handling.md) - Error patterns
- [What's New in v0.0.5](../whats-new-v0.0.5.md) - Feature overview

---

## Quick Reference Tables

### Markers Summary

| Marker | Category | Purpose |
|--------|----------|---------|
| `{@}` | Block | Package definition |
| `{#}` | Block | Enum definition |
| `{|}` | Block | Pipeline definition |
| `[A]` | Enum | Alias |
| `[s]` | Enum | Serial load |
| `[.]` | Field | Field definition |
| `[%]` | Pipeline | Metadata |
| `[t]` | Pipeline | Trigger |
| `[<]` | Pipeline | Input |
| `[>]` | Pipeline | Output |
| `[w]` | Pipeline | Wrapper |
| `[r]` | Execution | Run/sequential |
| `[p]` | Execution | Parallel |
| `[f]` | Execution | Fork (exhaustive) |
| `[b]` | Execution | Branch |
| `[*]` | Execution | Pack |
| `[c]` | Code | Code line |
| `[!]` | Error | Error handler |

### Operators Summary

| Operator | Type | Meaning |
|----------|------|---------|
| `<<` | Assignment | Pull final |
| `<~` | Assignment | Pull default |
| `>>` | Assignment/Loop | Push final / Feed |
| `~>` | Assignment | Push default |
| `?=` | Comparison | Equal |
| `?>` | Comparison | Greater |
| `?<` | Comparison | Less |
| `~` | Loop | Unpack |
| `*` | Loop | Pack |
| `-` | Enum | Reserved |
| `#` | Enum | User |
| `.` | Field | Access |
| `!` | Error | Error type |

### Types Summary

| Type | Category | Example |
|------|----------|---------|
| `:string` | Primitive | `"text"` |
| `:uint` | Primitive | `42` |
| `:int` | Primitive | `-10` |
| `:float` | Primitive | `3.14` |
| `:bool` | Primitive | `-True` |
| `:dt` | Primitive | `|DT.Now""` |
| `:path` | Primitive | `\\Path\` |
| `:serial` | Primitive | `{:}` |
| `:error` | Primitive | `!Error` |
| `:array.T` | Collection | `( )` |
| `:set.T` | Collection | `{ }` |
| `:py.*` | Native | Python |
| `:rust.*` | Native | Rust |
| `:js.*` | Native | JavaScript |

---

**Status:** ✅ Official Reference
**Version:** 0.0.5
**Last Updated:** 2026-01-04
