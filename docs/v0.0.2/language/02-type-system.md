# Type System

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Polyglot features a strong, static type system with compile-time type checking. The type system is designed to prevent common errors while providing flexibility for complex workflows.

**Key Principles:**
- Static typing with compile-time checks
- Explicit type declarations
- Immutable by default, mutable when needed
- Type safety across runtime boundaries

---

## Table of Contents

1. [Type Separator Syntax](#type-separator-syntax)
2. [Primitive Types](#primitive-types)
3. [Numeric Types](#numeric-types)
4. [String Type](#string-type)
5. [Boolean Type](#boolean-type)
6. [Path Type](#path-type)
7. [DateTime Type](#datetime-type)
8. [Collection Types](#collection-types)
9. [Serial Type](#serial-type)
10. [Mutable Types](#mutable-types)
11. [Enumeration Types](#enumeration-types)
12. [Error Types](#error-types)
13. [Type Comparison](#type-comparison)
14. [Literal Syntax Sugar](#literal-syntax-sugar)
15. [Type Safety Rules](#type-safety-rules)

---

## Type Separator Syntax

### Backslash is the Type Separator

**Critical Rule:** The type separator is ALWAYS `\` (backslash), NEVER `/` (forward slash).

```polyglot
// ✓ CORRECT
pg\int
pg\string
pg\path

// ✗ WRONG - DO NOT USE
pg/int
pg/string
pg/path
```

---

### Type Declaration Format

```polyglot
language\type
```

**Components:**
- `language` - Type namespace (e.g., `pg` for Polyglot built-in)
- `\` - Type separator (backslash)
- `type` - Type name (e.g., `int`, `string`, `bool`)

**Examples:**
```polyglot
pg\int       // Polyglot integer
pg\string    // Polyglot string
pg\bool      // Polyglot boolean
```

---

### Mutable Type Format

```polyglot
language.mutable\type
```

**Components:**
- `language` - Type namespace
- `.mutable` - Mutable modifier
- `\` - Type separator
- `type` - Type name

**Examples:**
```polyglot
pg.mutable\int
pg.mutable\string
pg.mutable\bool
```

---

## Primitive Types

### Overview

Primitive types are the fundamental building blocks of Polyglot's type system.

| Type | Description | Size/Range | Default Value |
|------|-------------|------------|---------------|
| `pg\int` | Signed integer | 64-bit | 0 |
| `pg\uint` | Unsigned integer | 64-bit | 0 |
| `pg\float` | Floating point | 64-bit | 0.0 |
| `pg\string` | UTF-8 string | Variable | "" (empty) |
| `pg\bool` | Boolean | 1-bit | False |
| `pg\path` | File system path | Variable | *(special)* |
| `pg\dt` | DateTime | Variable | *(special)* |

---

## Numeric Types

### Integer Type: `pg\int`

**Description:** 64-bit signed integer

**Range:** -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

**Usage:**
```polyglot
// Declaration with value
[r] .count: pg\int << 42
[r] .temperature: pg\int << -15

// Input declaration
[i] .max_retries: pg\int

// Default input
[i] Default .timeout_seconds: pg\int << 30
```

**Operations:**
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: NOT SUPPORTED - use range notation with `?>` or named operations like `|Q.DispatchIf.Number.GreaterThan`
- Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`

---

### Unsigned Integer Type: `pg\uint`

**Description:** 64-bit unsigned integer (non-negative only)

**Range:** 0 to 18,446,744,073,709,551,615

**Usage:**
```polyglot
// File sizes, memory amounts
[r] .file_size: pg\uint << 1024
[r] .memory_mb: pg\uint << 2048

// Array indices (always non-negative)
[r] .index: pg\uint << 0

// Counts (cannot be negative)
[r] .user_count: pg\uint << 100
```

**When to Use:**
- Memory sizes (bytes, MB, GB)
- Array indices
- Counts that cannot be negative
- Network ports

---

### Float Type: `pg\float`

**Description:** 64-bit floating-point number (IEEE 754 double precision)

**Range:** Approximately ±1.7 × 10^308

**Usage:**
```polyglot
// Decimal numbers
[r] .price: pg\float << 19.99
[r] .percentage: pg\float << 0.75
[r] .pi: pg\float << 3.14159265359

// Scientific notation
[r] .speed_of_light: pg\float << 2.998e8
```

**Precision Note:**
- Floating-point arithmetic is approximate
- Use integers for exact calculations (e.g., money: store cents as `pg\int`)

---

## String Type

### String Type: `pg\string`

**Description:** UTF-8 encoded string, immutable by default

**Usage:**
```polyglot
// String literals
[r] .name: pg\string << "Alice"
[r] .message: pg\string << "Hello, World!"

// Empty string
[r] .empty: pg\string << ""

// Multi-line strings (if supported)
[r] .description: pg\string << "This is a
multi-line
string"
```

---

### String Interpolation

Polyglot supports string interpolation with `{var:format}` syntax, which is **syntax sugar** that compiles to pipeline operations.

```polyglot
// Basic interpolation
[r] .greeting: pg\string << "Hello, {.name}!"

// With formatting
[r] .formatted: pg\string << "Count: {.count:03d}"
[r] .money: pg\string << "Price: ${.price:.2f}"
```

**Note:** String interpolation is processed by the `pg\serial` type's string handling capabilities and compiles to underlying pipeline operations.

---

### String Operations

Strings support various operations through standard library utilities:

```polyglot
// Concatenation
[r] |U.String.Concat
[<] .left: pg\string << "Hello"
[<] .right: pg\string << "World"
[>] .result: pg\string >> combined

// Formatting
[r] |U.String.Format
[<] .template: pg\string << "Count: {}"
[<] .value: pg\int << 42
[>] .result: pg\string >> formatted

// Splitting
[r] |U.String.Split
[<] .input: pg\string << "a,b,c"
[<] .delimiter: pg\string << ","
[>] .result: pg\array{pg\string} >> parts
```

---

## Boolean Type

### Boolean Type: `pg\bool`

**Description:** Boolean type with two values: `True` and `False`

**Values:**
- `True` - Boolean true (capitalized)
- `False` - Boolean false (capitalized)

**Usage:**
```polyglot
// Boolean literals
[r] .is_valid: pg\bool << True
[r] .is_enabled: pg\bool << False

// Comparison results
[r] .result: pg\bool << (.count > 10)

// Input with default
[i] Default .debug_mode: pg\bool << False
```

---

### Boolean Operations

```polyglot
// Logical AND
[r] .both_true: pg\bool << (.a && .b)

// Logical OR
[r] .either_true: pg\bool << (.a || .b)

// Logical NOT
[r] .negated: pg\bool << (!.is_valid)

// Comparison using switch/match pattern
[?] .x ?> .y
[~][r] .equal: pg\bool << True
[~][~][r] .equal: pg\bool << False
```

---

## Path Type

### Path Type: `pg\path`

**Description:** Special type for file system paths with cross-platform support

**Reserved Fields:**
- `.unix: pg\path` - Unix/Linux/macOS path
- `.windows: pg\path` - Windows path

**Usage:**
```polyglot
// Simple path
[r] .file: pg\path << "data.csv"

// Path with identifier
[r] .config: pg\path << \\DataDir\\config.json

// Input path
[i] .input_file: pg\path

// Path with both OS variants
[#] Path.Identifiers.MyApp.LogDir
[<] .unix: pg\path << \\UnixRoot\\var\log\myapp\
[<] .windows: pg\path << \\C\\Logs\MyApp\
[X]

[r] .log_file: pg\path << \\LogDir\\app.log
```

---

### Path Identifiers

Path identifiers enable cross-platform paths using the reserved enumeration `#Path.Identifiers.*`:

```polyglot
// Define custom path identifier
[#] Path.Identifiers.MyApp.DataDirectory
[A] DataDir
[<] .unix: pg\path << \\UnixRoot\\opt\myapp\data\
[<] .windows: pg\path << \\C\\ProgramData\MyApp\Data\
[X]

// Use path identifier
[r] .data_file: pg\path << \\DataDir\\records.csv
```

**Special Path Value:**
- `\\NoPath\\` - Indicates path doesn't exist on this OS

```polyglot
[#] Path.Identifiers.WindowsOnly.Registry
[<] .unix: pg\path << \\NoPath\\
[<] .windows: pg\path << \\HKEY_LOCAL_MACHINE\\Software\
[X]
```

---

### Path Operations

```polyglot
// Join paths
[r] |U.Path.Join
[<] .base: pg\path << \\DataDir\\
[<] .relative: pg\path << "subfolder/file.txt"
[>] .result: pg\path >> full_path

// Check existence
[r] |U.Path.Exists
[<] .path: pg\path << .file
[>] .exists: pg\bool >> file_exists

// Get parent directory
[r] |U.Path.Parent
[<] .path: pg\path << .file
[>] .parent: pg\path >> directory
```

---

## DateTime Type

### DateTime Type: `pg\dt`

**Description:** DateTime type with calendar support and timezone awareness

**Supported Calendars:**
- Gregorian (default)
- Hijri (Islamic)
- Chinese
- Hebrew
- Persian

**Usage:**
```polyglot
// DateTime literals (syntax sugar)
[r] .now: pg\dt << DT"2024-01-15 14:30:00"
[r] .date_only: pg\dt << DT"2024-01-15"
[r] .time_only: pg\dt << DT"14:30:"

// Relative time
[r] .past: pg\dt << DT.Ago"2 days"
[r] .future: pg\dt << DT.From.Now"1 hour"

// Calendar-specific
[r] .hijri_date: pg\dt << DT.Hijri"1445-07-15"
[r] .chinese_date: pg\dt << DT.Chinese"2024-01-15"
```

---

### DateTime Literals are Syntax Sugar

**Important:** `DT"..."` literals are syntax sugar that compile to underlying pipeline operations.

```polyglot
// This literal syntax...
[r] .time: pg\dt << DT"12:30:"

// ...compiles to pipeline operations equivalent to:
[r] |pg.DateTime.Parse
[<] .format: pg\string << "12:30:"
[>] .result: pg\dt >> .time
```

**See Also:** [DateTime System](07-datetime-system.md) for complete DT reference

---

## Collection Types

### Array Type: `pg\array{T}`

**Description:** Ordered collection of elements of the same type

**Syntax:**
```polyglot
pg\array{element_type}
```

**Usage:**
```polyglot
// Array of strings
[r] .names: pg\array{pg\string} << array{
[^]  "Alice",
[^]  "Bob",
[^]  "Charlie"
[^]}

// Array of integers
[r] .numbers: pg\array{pg\int} << array{
[^]  1, 2, 3, 4, 5
[^]}

// Empty array
[r] .empty: pg\array{pg\string} << array{}

// Input array
[i] .items: pg\array{pg\path}
```

---

### Array Operations

```polyglot
// Access element by index
[r] .first: pg\string << .names[0]

// Get array length
[r] |U.Array.Length
[<] .arr: pg\array{pg\string} << .names
[>] .length: pg\uint >> count

// Append element
[r] |U.Array.Append
[<] .arr: pg\array{pg\int} << .numbers
[<] .element: pg\int << 6
[>] .result: pg\array{pg\int} >> updated

// Iterate (using unpack operator)
[r] ~.names
[~][r] |ProcessItem
[~][<] .item: pg\string << .names.item
```

---

### Set Type: `pg\set{T}`

**Description:** Unordered collection of unique elements

**Syntax:**
```polyglot
pg\set{element_type}
```

**Usage:**
```polyglot
// Set of integers
[r] .unique_ids: pg\set{pg\int} << set{1, 2, 3, 4, 5}

// Set of strings
[r] .tags: pg\set{pg\string} << set{
[^]  "important",
[^]  "urgent",
[^]  "review"
[^]}

// Empty set
[r] .empty: pg\set{pg\int} << set{}
```

---

### Set Operations

```polyglot
// Check membership
[r] |U.Set.Contains
[<] .set: pg\set{pg\int} << .unique_ids
[<] .element: pg\int << 42
[>] .result: pg\bool >> is_member

// Add element
[r] |U.Set.Add
[<] .set: pg\set{pg\string} << .tags
[<] .element: pg\string << "new-tag"
[>] .result: pg\set{pg\string} >> updated

// Union
[r] |U.Set.Union
[<] .left: pg\set{pg\int} << .set1
[<] .right: pg\set{pg\int} << .set2
[>] .result: pg\set{pg\int} >> combined
```

---

## Serial Type

### Serial Type: `pg\serial`

**Description:** Serializable data structure with mutable schema (keys can change at runtime)

**Key Characteristics:**
- Dynamic key-value pairs
- Mutable schema (unlike enumerations)
- Used for JSON-like data
- Supports nesting

**Usage:**
```polyglot
// Simple serial
[r] .config: pg\serial << serial{
[^]  "host": "localhost",
[^]  "port": 8080,
[^]  "debug": True
[^]}

// Nested serial
[r] .settings: pg\serial << serial{
[^]  "database": serial{
[^]    "host": "db.example.com",
[^]    "port": 5432
[^]  },
[^]  "cache": serial{
[^]    "enabled": True,
[^]    "ttl": 3600
[^]  }
[^]}

// Empty serial
[r] .data: pg\serial << serial{}
```

---

### Serial vs Enumeration

| Feature | `pg\serial` | Enumeration |
|---------|-------------|-------------|
| Schema | Mutable (runtime) | Immutable (compile-time) |
| Type safety | Weaker | Stronger |
| Use case | JSON/dynamic data | Configuration/constants |
| Definition | Inline literals | `[#]...[X]` blocks |
| Fields known | Runtime | Compile-time |

---

### Serial Operations

```polyglot
// Access field
[r] .host: pg\string << .config["host"]

// Add field (mutable)
[r] .config["new_key"] << "new_value"

// Serialize to JSON
[r] |U.Serial.ToJSON
[<] .data: pg\serial << .config
[>] .json: pg\string >> json_string

// Parse from JSON
[r] |U.Serial.FromJSON
[<] .json: pg\string << json_string
[>] .data: pg\serial >> parsed
```

---

### String Interpolation Processing

The `{}` processing in string interpolation is handled by `pg\serial` capabilities:

```polyglot
// String interpolation with formatting
[r] .message: pg\string << "Count: {.count:03d}, Price: ${.price:.2f}"

// Internally processed by pg\serial's string handling
// Compiles to pipeline operations
```

**Note:** This is syntax sugar - the actual string building happens through serial type's capabilities compiled to pipeline operations.

---

## Mutable Types

### Mutable Type Modifier

**Default Behavior:** All types are immutable by default

**Mutable Modifier:** Add `.mutable` between language and type

**Syntax:**
```polyglot
language.mutable\type
```

---

### When to Use Mutable Types

Use mutable types when:
- Counters that increment/decrement
- Accumulators in loops
- State that changes over time
- Performance-critical updates

**Example:**
```polyglot
// Immutable counter (creates new value each time)
[r] .count: pg\int << 0
[r] .count: pg\int << .count + 1  // New binding

// Mutable counter (updates in place)
[r] .counter: pg.mutable\int << 0
[r] .counter << .counter + 1  // In-place update
```

---

### Mutable Type Examples

```polyglot
// Mutable integer
[r] .accumulator: pg.mutable\int << 0

// Mutable string (for building)
[r] .builder: pg.mutable\string << ""

// Mutable boolean (for flags)
[r] .flag: pg.mutable\bool << False

// Loop with mutable counter
[r] .i: pg.mutable\int << 0
[r] ~Array.ForEach
[~][r] .i << .i + 1
[~][r] |ProcessItem
[~][<] .index: pg\int << .i
```

---

### Immutable vs Mutable

| Aspect | Immutable (default) | Mutable |
|--------|---------------------|---------|
| Syntax | `pg\int` | `pg.mutable\int` |
| Update | Creates new value | Updates in place |
| Safety | Thread-safe | Requires care |
| Performance | May copy | Direct update |
| Use case | Most variables | Counters, accumulators |

---

## Enumeration Types

### Enumeration Type: `#EnumName`

**Description:** Immutable data structure with fixed schema defined at compile-time

**Definition:**
```polyglot
[#] MyEnumeration
[<] .field1: pg\string << "value1"
[<] .field2: pg\int << 42
[X]
```

**Usage:**
```polyglot
// Reference enumeration
[r] .config: #MyEnumeration << #MyEnumeration

// Access field
[r] .value: pg\string << #MyEnumeration.field1

// Input enumeration
[i] .settings: #Configuration
```

---

### Regular vs Reserved Enumerations

**Regular Enumerations:**
- Defined by users
- Full control over schema
- Not extendable

**Reserved Enumerations:**
- System-defined by Polyglot
- Fixed schema provided by language
- Extendable with `.* ` suffix
- Examples: `#Path.Identifiers.*`, `#Queues.*`, `#DT.Business.Week.*`

---

### Enumeration vs Serial Comparison

| Feature | Enumeration (`#Enum`) | Serial (`pg\serial`) |
|---------|----------------------|---------------------|
| Schema | Immutable (compile-time) | Mutable (runtime) |
| Definition | `[#]...[X]` blocks | Inline literals |
| Type safety | Strong | Weaker |
| Fields | Known at compile-time | Can change at runtime |
| Use case | Configuration, constants | JSON data, dynamic structures |
| Access | `#Enum.field` | `.serial["key"]` |

**See Also:** [Enumerations](03-enumerations.md) for complete enumeration reference

---

## Error Types

### Error Type: `!ErrorName`

**Description:** Special enumeration with three reserved fields for error handling

**Reserved Fields:**
- `.message: pg\string` - Error message
- `.code: pg\int` - Error code
- `.trace: pg\string` - Stack trace

**Definition:**
```polyglot
[!] !MyApp.CustomError
[<] .message: pg\string << "Custom error occurred"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[<] .context: pg\string << ""  // Additional custom field
[X]
```

**Usage:**
```polyglot
// Catch error
[r] |MightFail
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[>] .code: pg\int >> err_code

// Handle error
[r] |U.Log.Error
[<] .msg: pg\string << err_msg
```

**See Also:** [Error Handling](04-error-handling.md) for complete error reference

---

## Type Comparison

### Overview of All Types

| Category | Types | Mutability | Schema |
|----------|-------|------------|--------|
| **Primitives** | `pg\int`, `pg\uint`, `pg\float`, `pg\string`, `pg\bool` | Immutable default | Fixed |
| **Special** | `pg\path`, `pg\dt` | Immutable | Fixed with reserved fields |
| **Collections** | `pg\array{T}`, `pg\set{T}` | Immutable default | Fixed (element type) |
| **Dynamic** | `pg\serial` | Mutable schema | Runtime mutable |
| **Enumerations** | `#EnumName` | Immutable | Compile-time fixed |
| **Errors** | `!ErrorName` | Immutable | Fixed (3 reserved + custom) |

---

### When to Use Each Type

**Use `pg\int`, `pg\float`, `pg\string`, `pg\bool`:**
- Simple values
- Calculations and comparisons
- Most common use cases

**Use `pg\path`:**
- File system paths
- Cross-platform path handling
- When using path identifiers

**Use `pg\dt`:**
- Dates and times
- Scheduling and timestamps
- Calendar operations

**Use `pg\array{T}` or `pg\set{T}`:**
- Collections of same-type elements
- When element type is known
- When order matters (array) or uniqueness matters (set)

**Use `pg\serial`:**
- JSON-like data
- Dynamic schemas
- When keys are not known at compile-time
- External API responses

**Use Enumerations `#EnumName`:**
- Configuration with fixed schema
- Constants and predefined values
- When type safety is critical
- When schema is known at compile-time

**Use Errors `!ErrorName`:**
- Error handling
- When you need structured error information
- When extending standard errors

---

## Literal Syntax Sugar

### Overview

Many Polyglot literals are **syntax sugar** that compile to underlying pipeline operations.

---

### String Literals

```polyglot
// String literal
[r] .name: pg\string << "Alice"

// Compiles to string construction pipeline
```

---

### Array and Set Literals

```polyglot
// Array literal
[r] .items: pg\array{pg\int} << array{1, 2, 3}

// Compiles to array construction pipeline

// Set literal
[r] .unique: pg\set{pg\string} << set{"a", "b", "c"}

// Compiles to set construction pipeline
```

---

### Serial Literals

```polyglot
// Serial literal
[r] .config: pg\serial << serial{
[^]  "host": "localhost",
[^]  "port": 8080
[^]}

// Compiles to serial construction pipeline
```

---

### DateTime Literals

```polyglot
// DateTime literal
[r] .now: pg\dt << DT"2024-01-15 14:30:00"

// Compiles to DateTime parsing pipeline
[r] |pg.DateTime.Parse
[<] .input: pg\string << "2024-01-15 14:30:00"
[>] .result: pg\dt >> .now
```

---

### String Interpolation

```polyglot
// String interpolation
[r] .msg: pg\string << "Count: {.count}"

// Compiles to string formatting pipeline
[r] |pg.String.Format
[<] .template: pg\string << "Count: {}"
[<] .values: pg\array{pg\serial} << array{.count}
[>] .result: pg\string >> .msg
```

**Note:** These literals provide convenient syntax while maintaining the pipeline-based execution model underneath.

---

## Type Safety Rules

### Compile-Time Type Checking

Polyglot performs strict type checking at compile-time:

**Type Mismatch Errors:**
```polyglot
// ✗ ERROR - Type mismatch
[r] .count: pg\int << "not a number"

// ✗ ERROR - Cannot assign string to int
[i] .number: pg\int
[r] .number: pg\int << "42"
```

**Correct Usage:**
```polyglot
// ✓ CORRECT - Matching types
[r] .count: pg\int << 42
[r] .name: pg\string << "Alice"
```

---

### Type Conversion

Explicit type conversion required:

```polyglot
// Convert string to int
[r] |U.String.ToInt
[<] .input: pg\string << "42"
[>] .result: pg\int >> number

// Convert int to string
[r] |U.Int.ToString
[<] .input: pg\int << 42
[>] .result: pg\string >> text
```

---

### Collection Type Safety

Collections enforce element type:

```polyglot
// ✓ CORRECT - All elements same type
[r] .numbers: pg\array{pg\int} << array{1, 2, 3}

// ✗ ERROR - Mixed types not allowed
[r] .mixed: pg\array{pg\int} << array{1, "two", 3}

// ✓ CORRECT - Use serial for mixed types
[r] .mixed: pg\serial << serial{
[^]  "number": 1,
[^]  "text": "two",
[^]  "another": 3
[^]}
```

---

### Enumeration Type Safety

Enumerations provide strong type safety:

```polyglot
// Define enumeration
[#] Status
[<] .success: pg\string << "SUCCESS"
[<] .failed: pg\string << "FAILED"
[X]

// ✓ CORRECT - Using defined enumeration
[r] .result: #Status << #Status.success

// ✗ ERROR - Cannot use undefined enum
[r] .result: #Status << #UndefinedEnum.value

// ✗ ERROR - Cannot use undefined field
[r] .result: #Status << #Status.undefined_field
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](01-syntax-complete.md) - All syntax elements
- [Enumerations](03-enumerations.md) - Enumeration system details
- [Error Handling](04-error-handling.md) - Error type details
- [DateTime System](07-datetime-system.md) - DateTime type details

### Standard Library
- [Overview](../standard-library/00-overview.md) - Type conversion utilities

### Examples
- [Data Processing](../examples/data-processing.md) - Type usage examples

### Planning
- [Decision Log](../decision-log.md) - Type system decisions (#1, #2, #8, #10)

---

**End of Type System Reference**