# Operators

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Polyglot uses a minimal set of operators with clear, unambiguous semantics. Each operator has a specific purpose and cannot be combined with others.

**Design Philosophy:**
- Minimal operator set
- Clear semantic meaning
- No operator overloading
- No operator combination
- Visual clarity

---

## Table of Contents

1. [Operator Overview](#operator-overview)
2. [Pipeline Operator `|`](#pipeline-operator-)
3. [Unpack Operator `~`](#unpack-operator-)
4. [Package Operator `@`](#package-operator-)
5. [Enumeration Operator `#`](#enumeration-operator-)
6. [Error Type Operator `!`](#error-type-operator-)
7. [Assignment Operators](#assignment-operators)
8. [Operator Precedence](#operator-precedence)
9. [Common Mistakes](#common-mistakes)
10. [Operator Summary](#operator-summary)

---

## Operator Overview

### Complete Operator List

| Operator | Name | Purpose | Example |
|----------|------|---------|---------|
| `|` | Pipeline | Call pipeline | `|ProcessData` |
| `~` | Unpack | Expand/iterate collection | `~Array.ForEach` |
| `@` | Package | Access from package | `@pkg|Pipeline` |
| `#` | Enumeration | Mark enumeration | `#MyEnum` |
| `!` | Error | Mark error type | `!CustomError` |
| `<<` | Push | Assign INTO variable | `.x << value` |
| `>>` | Pull | Extract FROM source | `.x >> output` |

---

### No Operator Overloading

**Important:** Operators have fixed meanings and cannot be overloaded or combined.

```polyglot
// ✓ VALID - Single operators
|Pipeline
~Array
#Enumeration

// ✗ INVALID - Cannot combine
|~Something
#!Something
@|#Something
```

---

## Pipeline Operator `|`

### Purpose

The `|` operator **calls pipelines** - the fundamental units of execution defined with `[|]...[X]`.

---

### Syntax

```polyglot
|PipelineName
```

**Rules:**
- Always required when calling a pipeline
- Must be followed immediately by pipeline name (no space)
- Cannot be combined with other operators

---

### Examples

**Call user-defined pipeline:**
```polyglot
[r] |ProcessData
[r] |ValidateInput
[r] |TransformOutput
```

**Call standard library pipeline:**
```polyglot
[r] |U.String.Format
[r] |U.Array.Length
[r] |Q.Pause
```

**Call trigger pipeline:**
```polyglot
[t] |T.Daily
[t] |T.Every.Minute
[t] |T.File.Modified
```

**Call wrapper pipeline:**
```polyglot
[w] |W.Python3.11
[w] |W.Node20
[w] |W.Rust
```

---

### What is NOT a Pipeline Call

**Unpack operations use `~` not `|`:**
```polyglot
// ✓ CORRECT - Unpack with ~
[r] ~Array.ForEach
[r] ~myCollection

// ✗ WRONG - These are NOT pipeline calls
[r] |Array.ForEach  // Wrong operator
```

---

### Pipeline Call with Parameters

```polyglot
[r] |ProcessData
[<] .input: pg\string << "value"
[<] .max_size: pg\int << 1024
[>] .result: pg\string >> output
```

---

## Unpack Operator `~`

### Purpose

The `~` operator **unpacks and expands** collections (arrays, sets, enumerations) for iteration or enumeration.

---

### Syntax

```polyglot
~collection_or_operation
```

---

### Unpacking Arrays

```polyglot
[r] .items: pg\array{pg\string} << array{"a", "b", "c"}

// Unpack and iterate
[r] ~.items
[~][r] |ProcessItem
[~][<] .item: pg\string << .items.item
```

---

### Unpacking with Standard Library

```polyglot
// Array.ForEach - unpack operation
[r] ~Array.ForEach
[~][r] |ProcessElement
[~][<] .element: pg\string << .current_element
```

---

### Unpacking Enumerations

```polyglot
[#] Config
[<] .field1: pg\string << "value1"
[<] .field2: pg\int << 42
[X]

// Unpack enumeration fields
[r] ~Config
[~][r] |ProcessField
[~][<] .field_name: pg\string << .field_key
[~][<] .field_value: pg\string << .field_value
```

---

### Unpack vs Pipeline

**Critical Distinction:**

```polyglot
// Pipeline call - executes a pipeline
[r] |SomePipeline

// Unpack operation - expands collection
[r] ~SomeCollection
```

**Not Interchangeable:**
```polyglot
// ✓ CORRECT
[r] ~Array.ForEach  // Unpack operation

// ✗ WRONG
[r] |Array.ForEach  // Not a pipeline call
```

---

## Package Operator `@`

### Purpose

The `@` operator accesses pipelines and enumerations from **external packages** in the three-tier registry system.

---

### Syntax

**For pipelines:**
```polyglot
@packageName|PipelineName
```

**For enumerations:**
```polyglot
@packageName#EnumerationName
```

---

### Three-Tier Registry System

**1. Local Registry** (`Local.*`)
```polyglot
// Local development packages
[r] @Local.MyProject|ProcessData
[r] @Local.TestUtils|ValidateInput
```

**2. Community Registry** (`Community.*`)
```polyglot
// Open-source community packages
[r] @Community.username|UtilityPipeline
[r] @Community.hasan|DataTransform
```

**3. Company Registry** (`Company.*`)
```polyglot
// Enterprise/private packages
[r] @Company.acme|InternalAPI
[r] @Company.acme.team|SharedUtils
```

---

### Package + Pipeline

```polyglot
// Call pipeline from package
[r] @Community.datatools|Transform
[<] .input: pg\string << "data"
[>] .result: pg\string >> output

// Call with full namespace
[r] @Company.acme.analytics|CalculateMetrics
[<] .data: pg\array{pg\int} << numbers
[>] .result: pg\float >> metrics
```

---

### Package + Enumeration

```polyglot
// Access enumeration from package
[i] .config: @Local.MyApp#Configuration
[r] .setting: pg\string << @Local.MyApp#Configuration.field

// Use in type declaration
[i] .error_codes: @Company.acme#ErrorCodes
```

---

### Package Resolution

**Resolution order:**
1. Local registry (localhost/LAN)
2. Community registry (public)
3. Company registry (private/authenticated)

**DNS evaluation for Local:**
- `Local.hostname` resolves via DNS
- Falls back to localhost if DNS fails

---

## Enumeration Operator `#`

### Purpose

The `#` operator **marks enumeration types** for definition and reference.

---

### Syntax

**Definition:**
```polyglot
[#] EnumerationName
[X]
```

**Reference:**
```polyglot
#EnumerationName
#EnumerationName.field
```

---

### Defining Enumerations

```polyglot
[#] Configuration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[X]
```

---

### Referencing Enumerations

```polyglot
// Use entire enumeration
[r] .config: #Configuration << #Configuration

// Access specific field
[r] .host: pg\string << #Configuration.host
[r] .port: pg\int << #Configuration.port
```

---

### Type Declarations

```polyglot
// Input parameter of enumeration type
[i] .settings: #Configuration

// Variable of enumeration type
[r] .app_config: #Configuration << #Configuration
```

---

### Reserved Enumerations

```polyglot
// System-defined enumerations
#Status.Success
#Status.Failed
#None

// Extendable reserved enumerations
#Path.Identifiers.*
#Queues.*
#DT.Business.Week.*
```

---

### Enumeration from Package

```polyglot
// Access enumeration from external package
@packageName#EnumerationName

// Example
[i] .config: @Local.MyApp#Configuration
[r] .setting: pg\string << @Local.MyApp#Configuration.field
```

---

## Error Type Operator `!`

### Purpose

The `!` operator **marks error types** - special enumerations with three reserved fields.

---

### Syntax

**Definition:**
```polyglot
[!] !ErrorName
[X]
```

**Catching:**
```polyglot
[!] !ErrorType
```

**Reference:**
```polyglot
!ErrorType
```

---

### Defining Errors

```polyglot
[!] !MyApp.CustomError
[<] .message: pg\string << "Error occurred"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]
```

---

### Catching Errors

```polyglot
[r] |MightFail

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg

[r] |HandleError
[<] .msg: pg\string << err_msg
```

---

### Error Type References

```polyglot
// Type declaration
[i] .error: !ErrorType

// Error field access
[r] .msg: pg\string << !SomeError.message
```

---

## Assignment Operators

### Two Assignment Operators

Polyglot has two distinct assignment operators with opposite data flow directions:

| Operator | Name | Direction | Use Case |
|----------|------|-----------|----------|
| `<<` | Push | INTO variable | Inputs, assignments |
| `>>` | Pull | FROM source | Outputs, extraction |

---

### Push INTO: `<<`

**Semantic Meaning:** Push data INTO the variable (left side)

**Visual Mnemonic:** Arrows point left → data flows INTO variable

---

#### Push Literal Values

```polyglot
[r] .x: pg\int << 5
[r] .name: pg\string << "Alice"
[r] .flag: pg\bool << True
```

---

#### Push Variables

```polyglot
[r] .source: pg\string << "original"
[r] .destination: pg\string << .source
```

---

#### Push INTO Pipeline Parameters

```polyglot
[r] |ProcessData
[<] .input: pg\string << input_var
[<] .max_size: pg\int << 1024
```

---

#### Push INTO Enumeration Fields

```polyglot
[#] Configuration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[X]
```

---

#### Push INTO Error Fields

```polyglot
[!] !CustomError
[<] .message: pg\string << "Default message"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]
```

---

### Pull FROM: `>>`

**Semantic Meaning:** Pull data FROM the source (right side)

**Visual Mnemonic:** Arrows point right → data flows FROM source

---

#### Pull FROM Pipeline Outputs

```polyglot
[r] |ProcessData
[>] .result: pg\string >> output_var
[>] .status: pg\int >> status_code
```

---

#### Pull FROM Error Fields

```polyglot
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[>] .code: pg\int >> err_code
```

---

#### Pull FROM Parallel Blocks

```polyglot
[p] |ProcessPartA
[<] .data: pg\string << input
[>] .output >> result1

[p] |ProcessPartB
[<] .data: pg\string << input
[>] .output >> result2
```

---

#### Pull in Join Blocks

```polyglot
[Y] |Y.Join
[>] result1
[>] result2
```

---

### Direction Summary

**Data Flow Direction:**

```polyglot
// << pushes INTO (left direction)
.variable << source_value
    ↑
    Data flows INTO variable

// >> pulls FROM (right direction)
source_value >> .variable
                    ↑
                    Data flows FROM source
```

---

### Context Determines Usage

**Input Context → Use `<<`:**
```polyglot
[<] .input: pg\string << "value"  // Pushing INTO input
[r] .x: pg\int << 42               // Pushing INTO variable
```

**Output Context → Use `>>`:**
```polyglot
[>] .output: pg\string >> result   // Pulling FROM output
[>] .message: pg\string >> err_msg // Pulling FROM error field
```

---

### Cannot Mix Directions

```polyglot
// ✓ CORRECT - Consistent direction
[<] .input: pg\string << source_var

// ✗ ERROR - Wrong direction for input context
[<] .input: pg\string >> source_var

// ✓ CORRECT - Consistent direction
[>] .output: pg\string >> result_var

// ✗ ERROR - Wrong direction for output context
[>] .output: pg\string << result_var
```

---

## Operator Precedence

### No Complex Expressions

Polyglot does not support complex expressions with multiple operators in a single statement.

**Not Supported:**
```polyglot
// ✗ NOT SUPPORTED - Complex expression
[r] .result: pg\int << .a + .b * .c

// ✗ NOT SUPPORTED - Chained operations
[r] .value: pg\string << |Pipeline1 | Pipeline2
```

---

### Sequential Operations

Break complex operations into sequential steps:

```polyglot
// ✓ CORRECT - Sequential operations
[r] |Multiply
[<] .left: pg\int << .b
[<] .right: pg\int << .c
[>] .result: pg\int >> temp1

[r] |Add
[<] .left: pg\int << .a
[<] .right: pg\int << temp1
[>] .result: pg\int >> final_result
```

---

### One Operator Per Statement

Each line should use only one operator:

```polyglot
// ✓ CORRECT - One operator per line
[r] |ProcessData
[<] .input: pg\string << .source
[>] .output: pg\string >> .destination

// ✗ WRONG - Multiple operations in one line
[r] |ProcessData[<] .input: pg\string << .source
```

---

## Common Mistakes

### Mistake 1: Using `|` for Unpack

```polyglot
// ✗ WRONG
[r] |Array.ForEach

// ✓ CORRECT
[r] ~Array.ForEach
```

**Remember:** `|` is for pipelines, `~` is for unpacking.

---

### Mistake 2: Using `~` for Pipeline

```polyglot
// ✗ WRONG
[r] ~ProcessData

// ✓ CORRECT
[r] |ProcessData
```

**Remember:** `~` is for unpacking, `|` is for pipelines.

---

### Mistake 3: Combining Operators

```polyglot
// ✗ WRONG - Cannot combine
[r] |~Something
[r] @|Pipeline
[r] #!Error

// ✓ CORRECT - Use separately
[r] |Pipeline
[r] ~Array
[r] @pkg|Pipeline
```

---

### Mistake 4: Wrong Assignment Direction

```polyglot
// ✗ WRONG - Input needs <<
[<] .input: pg\string >> value

// ✓ CORRECT
[<] .input: pg\string << value

// ✗ WRONG - Output needs >>
[>] .output: pg\string << result

// ✓ CORRECT
[>] .output: pg\string >> result
```

---

### Mistake 5: Spaces in Operator

```polyglot
// ✗ WRONG - Space after operator
[r] | ProcessData
[r] ~ Array

// ✓ CORRECT - No space
[r] |ProcessData
[r] ~Array
```

---

### Mistake 6: Using `#` for Errors

```polyglot
// ✗ WRONG - Errors use !
[!] #Errors.SomeError

// ✓ CORRECT
[!] !MyApp.SomeError
```

---

### Mistake 7: Wrong Package Syntax

```polyglot
// ✗ WRONG - Missing | or #
[r] @packageName.Pipeline

// ✓ CORRECT - Use | for pipelines
[r] @packageName|Pipeline

// ✓ CORRECT - Use # for enumerations
[i] .config: @packageName#Enumeration
```

---

## Operator Summary

### Quick Reference

**Pipeline Operations:**
```polyglot
|PipelineName              // Call pipeline
|T.Daily                   // Call trigger
|Q.Pause                   // Call queue control
|W.Python3.11              // Call wrapper
```

**Unpack Operations:**
```polyglot
~Array.ForEach             // Unpack array iteration
~myCollection              // Unpack collection
~Enumeration               // Unpack enumeration
```

**Package Access:**
```polyglot
@pkg|Pipeline              // Pipeline from package
@pkg#Enumeration           // Enumeration from package
@Local.proj|Util           // Local package
@Community.user|Tool       // Community package
@Company.org|Internal      // Company package
```

**Type Markers:**
```polyglot
#EnumerationName           // Mark enumeration
!ErrorType                 // Mark error type
```

**Assignment:**
```polyglot
.var << value              // Push INTO variable
.field >> output           // Pull FROM source
```

---

### Operator Characteristics

| Operator | Combinable | Spaces Allowed | Context |
|----------|-----------|----------------|---------|
| `|` | No | No | Pipeline calls |
| `~` | No | No | Unpack operations |
| `@` | Yes (with `|` or `#`) | No | Package access |
| `#` | Yes (with `@` for packages) | No | Enumerations |
| `!` | No | No | Errors |
| `<<` | No | Yes | Inputs/assignments |
| `>>` | No | Yes | Outputs/extraction |

---

### Memory Aids

**Pipeline = Pipe → `|`**
```polyglot
|Pipeline  // Pipe symbol for pipeline
```

**Unpack = Unwrap → `~`**
```polyglot
~Array  // Tilde for unwrapping/expanding
```

**Package = At → `@`**
```polyglot
@package|Pipeline  // At symbol for package location
```

**Enumeration = Hash/Number → `#`**
```polyglot
#Enum  // Hash for named constants
```

**Error = Bang/Alert → `!`**
```polyglot
!Error  // Bang for error attention
```

**Assignment Direction:**
```polyglot
<< // Arrows point left = push INTO
>> // Arrows point right = pull FROM
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](01-syntax-complete.md) - All operators overview
- [Block Markers](06-block-markers.md) - Block elements that use operators
- [Type System](02-type-system.md) - Operator usage with types

### Advanced Features
- [Parallel Execution](08-parallel-execution.md) - `>>` in parallel blocks
- [Expansion Operator](09-expansion-operator.md) - `~` usage details

### Examples
- [Hello World](../examples/hello-world.md) - Basic operator usage
- [Complete Workflows](../examples/complete-workflows.md) - Complex operator patterns

### Planning
- [Decision Log](../decision-log.md) - Operator decisions (#4, #6, #13)

---

**End of Operators Reference**