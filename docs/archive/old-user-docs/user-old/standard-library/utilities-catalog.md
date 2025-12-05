# Utilities Catalog (|U.*)

**Version:** 0.0.2
**Status:** Catalog Only - APIs To Be Determined
**Block Marker:** None (utilities are regular pipelines)

## Overview

The `|U.*` namespace provides essential utility operations for common programming tasks. This document catalogs the available utility categories and their intended purposes. Detailed APIs will be specified in future documentation versions.

### Philosophy

- **Batteries Included:** Common operations should be available without external dependencies
- **Type-Safe:** All utilities respect Polyglot's type system
- **Composable:** Utilities work seamlessly in pipeline compositions
- **Predictable:** Consistent naming and behavior patterns across utilities

### Documentation Status

**Current Version (0.0.2):** This is a catalog-only reference. Each utility category is listed with its intended purpose and scope. Specific function signatures, parameters, and detailed examples will be provided in future documentation versions.

**Future Versions:** Complete API specifications, parameter details, error handling, and comprehensive examples.

## Utility Categories

### |U.String.*

**Purpose:** String manipulation, processing, and formatting operations

**Intended Scope:**
- Case conversion (uppercase, lowercase, title case)
- Trimming and padding
- Substring extraction
- String splitting and joining
- Pattern matching and replacement
- String validation (empty, whitespace, format checks)
- Encoding/decoding (URL encoding, HTML escaping, etc.)
- String comparison and searching
- **Type-specific formatting (for string literal interpolation)**

**Example Operations (APIs TBD):**
- `|U.String.ToUpper`
- `|U.String.ToLower`
- `|U.String.Trim`
- `|U.String.Split`
- `|U.String.Join`
- `|U.String.Replace`
- `|U.String.Contains`
- `|U.String.StartsWith`
- `|U.String.EndsWith`

#### Format Identifier Pipelines

**CRITICAL:** String literals with interpolation (e.g., `"{.var:FormatId}"`) invoke formatting pipelines following the pattern:

```
|U.String.{language}.{type}.{format_identifier}
```

These pipelines are called automatically during string literal processing when format identifiers are used.

**Pipeline Signature Requirements:**
- **Input:** `.formatted_argument_string: pg\string` (MANDATORY name)
- **Trigger:** `|T.String.Call` (MANDATORY)
- **Output:** Single output - can be ANY type (not limited to strings!)

**Processing Flow:**
1. Extract `{.var:FormatId}` from string
2. Infer type of `.var` (e.g., `pg\int`)
3. Construct pipeline call: `|U.String.Polyglot.Int.FormatId`
4. Call pipeline with variable value
5. Substitute result back into string

##### Integer Format Identifiers

**Namespace:** `|U.String.Polyglot.Int.*`

| Format ID | Description | Example Input | Example Output |
|-----------|-------------|---------------|----------------|
| `Hex` | Hexadecimal representation | `17` | `"11"` |
| `Binary` | Binary representation | `5` | `"101"` |
| `Octal` | Octal representation | `8` | `"10"` |
| `Decimal` | With thousands separators | `1000` | `"1,000"` |
| `Ordinal` | Ordinal number | `1` | `"1st"` |
| `Roman` | Roman numerals | `42` | `"XLII"` |
| `Padded` | Zero-padded (default width 8) | `42` | `"00000042"` |

**Usage Example:**
```polyglot
[r] .count: pg\int << 255
[r] .msg: pg\string << "Value in hex: {.count:Hex}"
// Result: "Value in hex: FF"
```

##### Float Format Identifiers

**Namespace:** `|U.String.Polyglot.Float.*`

| Format ID | Description | Example Input | Example Output |
|-----------|-------------|---------------|----------------|
| `Currency` | Currency format (USD default) | `42.5` | `"$42.50"` |
| `Percent` | Percentage | `0.75` | `"75%"` |
| `Scientific` | Scientific notation | `1000.0` | `"1.0e3"` |
| `Fixed2` | 2 decimal places | `3.14159` | `"3.14"` |
| `Fixed4` | 4 decimal places | `3.14159` | `"3.1416"` |
| `Compact` | Compact notation | `1500000.0` | `"1.5M"` |

**Usage Example:**
```polyglot
[r] .price: pg\float << 1234.567
[r] .msg: pg\string << "Total: {.price:Currency}"
// Result: "Total: $1,234.57"
```

##### DateTime Format Identifiers

**Namespace:** `|U.String.Polyglot.DateTime.*`

| Format ID | Description | Example Input | Example Output |
|-----------|-------------|---------------|----------------|
| `ShortDate` | Short date format | `2024-01-15` | `"01/15/24"` |
| `LongDate` | Long date format | `2024-01-15` | `"January 15, 2024"` |
| `ISO8601` | ISO 8601 format | `2024-01-15T14:30:00` | `"2024-01-15T14:30:00Z"` |
| `Time24` | 24-hour time | `14:30:00` | `"14:30:00"` |
| `Time12` | 12-hour time | `14:30:00` | `"2:30 PM"` |
| `Relative` | Relative time | `2024-01-15` | `"3 days ago"` |
| `RFC3339` | RFC 3339 format | `2024-01-15T14:30:00` | `"2024-01-15T14:30:00+00:00"` |

**Usage Example:**
```polyglot
[r] .timestamp: pg\dt << DT.Now""
[r] .msg: pg\string << "Generated at {.timestamp:Time12}"
// Result: "Generated at 2:30 PM"
```

##### Boolean Format Identifiers

**Namespace:** `|U.String.Polyglot.Bool.*`

| Format ID | Description | Example Input | Example Output |
|-----------|-------------|---------------|----------------|
| `YesNo` | Yes/No format | `#Boolean.True` | `"Yes"` |
| `OnOff` | On/Off format | `#Boolean.True` | `"On"` |
| `Numeric` | 1/0 format | `#Boolean.True` | `"1"` |
| `TrueFalse` | True/False (default) | `#Boolean.True` | `"True"` |

**Usage Example:**
```polyglot
[r] .enabled: #Boolean << #Boolean.True
[r] .msg: pg\string << "Feature: {.enabled:OnOff}"
// Result: "Feature: On"
```

##### Path Format Identifiers

**Namespace:** `|U.String.Polyglot.Path.*`

| Format ID | Description | Example Input | Example Output |
|-----------|-------------|---------------|----------------|
| `Basename` | File/directory name only | `~/docs/file.txt` | `"file.txt"` |
| `Directory` | Directory path only | `~/docs/file.txt` | `"~/docs"` |
| `Extension` | File extension | `~/docs/file.txt` | `".txt"` |
| `Absolute` | Absolute path | `./file.txt` | `"/home/user/file.txt"` |
| `Normalized` | Normalized path | `~/docs/../file.txt` | `~/file.txt` |

**Usage Example:**
```polyglot
[r] .filepath: pg\path << \\Path\\.unix << "/home/user/docs/report.pdf"
[r] .msg: pg\string << "Filename: {.filepath:Basename}"
// Result: "Filename: report.pdf"
```

##### Custom Format Identifiers

**Developers can define custom format pipelines** following the naming pattern:

```polyglot
[|] U.String.Polyglot.{Type}.{CustomFormat}
[i] .formatted_argument_string: pg\string
[t] |T.String.Call
[W] |W.Polyglot.Scope
// Custom formatting logic here
[o] .result: pg\string
[X]
```

**Example - Custom Phone Format:**
```polyglot
[|] U.String.Polyglot.Int.PhoneUS
[i] .formatted_argument_string: pg\string
[t] |T.String.Call
[W] |W.Polyglot.Scope
[r] .number: pg\int << .formatted_argument_string
// Format as (XXX) XXX-XXXX
[r] .result: pg\string << // formatting logic
[o] .result: pg\string
[X]

// Usage:
[r] .phone: pg\int << 5551234567
[r] .msg: pg\string << "Call: {.phone:PhoneUS}"
// Result: "Call: (555) 123-4567"
```

##### Language-Specific Formatters

The pattern `|U.String.{language}.{type}.{format}` allows for language-specific formatting:

**Polyglot (default):** `|U.String.Polyglot.*`
**Python:** `|U.String.Python.*` (future)
**Rust:** `|U.String.Rust.*` (future)
**JavaScript:** `|U.String.JavaScript.*` (future)

Language-specific formatters can provide locale-aware or language-specific formatting behavior.

##### Type Mismatch Handling

**Error Condition:** If a format identifier is used with an incompatible type, the compiler will report:

```
Error: Pipeline not found: |U.String.Polyglot.String.Hex
Note: Format identifier 'Hex' expects type 'pg\int', but variable is 'pg\string'
```

**Why this works:** Format pipelines are named by type, so an incorrect type-format combination results in a missing pipeline definition.

##### IDE Integration

**Expected IDE behavior:**
- Autocomplete format identifiers based on variable type
- Show available formatters when typing `{.var:⎵`
- Validate format identifier exists for variable type
- Preview formatted output in hover tooltips

##### See Also

- [String Literals as Inline Pipeline Calls](../../technical/string-literals-internals.md) - Complete processing mechanics
- [Type System](../language/type-system.md) - Type inference and compatibility
- [Error Handling](../language/error-handling.md) - Handling format errors

### |U.Array.*

**Purpose:** Array manipulation and transformation operations

**Intended Scope:**
- Element access and modification
- Array filtering and mapping
- Sorting and ordering
- Array reduction and aggregation
- Array slicing and concatenation
- Element searching and indexing
- Duplicate removal
- Array set operations (union, intersection, difference)

**Example Operations (APIs TBD):**
- `|U.Array.Map`
- `|U.Array.Filter`
- `|U.Array.Reduce`
- `|U.Array.Sort`
- `|U.Array.Slice`
- `|U.Array.Concat`
- `|U.Array.Find`
- `|U.Array.Unique`
- `|U.Array.Reverse`

### |U.Set.*

**Purpose:** Set operations and manipulations

**Intended Scope:**
- Set creation and conversion
- Union, intersection, difference operations
- Subset and superset checks
- Element membership testing
- Set equality comparison
- Cartesian product
- Set size operations

**Example Operations (APIs TBD):**
- `|U.Set.Union`
- `|U.Set.Intersection`
- `|U.Set.Difference`
- `|U.Set.IsSubset`
- `|U.Set.Contains`
- `|U.Set.Size`

### |U.Math.*

**Purpose:** Mathematical operations and calculations

**Intended Scope:**
- Basic arithmetic (beyond operators)
- Trigonometric functions
- Logarithmic and exponential functions
- Rounding and precision control
- Statistical operations (mean, median, mode, etc.)
- Random number generation
- Mathematical constants
- Number validation (NaN, infinity checks)

**Example Operations (APIs TBD):**
- `|U.Math.Abs`
- `|U.Math.Round`
- `|U.Math.Ceil`
- `|U.Math.Floor`
- `|U.Math.Sqrt`
- `|U.Math.Power`
- `|U.Math.Random`
- `|U.Math.Min`
- `|U.Math.Max`
- `|U.Math.Mean`

### |U.File.*

**Purpose:** File system operations

**Intended Scope:**
- File reading and writing
- Directory operations (list, create, delete)
- Path manipulation
- File metadata (size, timestamps, permissions)
- File existence checks
- Temporary file creation
- File copying and moving
- Binary and text file handling

**Example Operations (APIs TBD):**
- `|U.File.Read`
- `|U.File.Write`
- `|U.File.Append`
- `|U.File.Exists`
- `|U.File.Delete`
- `|U.File.Copy`
- `|U.File.Move`
- `|U.File.List`
- `|U.File.Size`

### |U.Path.*

**Purpose:** Path manipulation and resolution

**Intended Scope:**
- Path joining and normalization
- Basename and directory extraction
- Extension handling
- Absolute and relative path conversion
- Path validation
- Cross-platform path handling (Unix/Windows)
- Path comparison

**Example Operations (APIs TBD):**
- `|U.Path.Join`
- `|U.Path.Normalize`
- `|U.Path.Basename`
- `|U.Path.Directory`
- `|U.Path.Extension`
- `|U.Path.IsAbsolute`
- `|U.Path.Resolve`

### |U.JSON.*

**Purpose:** JSON serialization and deserialization

**Intended Scope:**
- Parse JSON strings to serial types
- Serialize serial types to JSON strings
- Pretty-print formatting
- JSON validation
- JSON path queries (JSONPath-like)
- JSON schema validation

**Example Operations (APIs TBD):**
- `|U.JSON.Parse`
- `|U.JSON.Stringify`
- `|U.JSON.PrettyPrint`
- `|U.JSON.Validate`
- `|U.JSON.Query`

### |U.CSV.*

**Purpose:** CSV parsing and generation

**Intended Scope:**
- Parse CSV strings to array structures
- Generate CSV from array data
- Custom delimiter support
- Header row handling
- Quote and escape handling
- Type inference for parsed data

**Example Operations (APIs TBD):**
- `|U.CSV.Parse`
- `|U.CSV.Generate`
- `|U.CSV.ParseWithHeaders`
- `|U.CSV.ToArray`

### |U.Hash.*

**Purpose:** Hashing and checksum operations

**Intended Scope:**
- Cryptographic hashing (SHA-256, SHA-512, etc.)
- Non-cryptographic hashing (for hash tables)
- Checksum generation (MD5, CRC32)
- Hash comparison
- HMAC operations

**Example Operations (APIs TBD):**
- `|U.Hash.SHA256`
- `|U.Hash.SHA512`
- `|U.Hash.MD5`
- `|U.Hash.HMAC`

### |U.Encode.*

**Purpose:** Encoding and decoding operations

**Intended Scope:**
- Base64 encoding/decoding
- URL encoding/decoding
- HTML entity encoding/decoding
- UTF-8 handling
- Hex encoding/decoding

**Example Operations (APIs TBD):**
- `|U.Encode.Base64`
- `|U.Encode.Base64Decode`
- `|U.Encode.URL`
- `|U.Encode.URLDecode`
- `|U.Encode.HTMLEntities`
- `|U.Encode.Hex`

### |U.Validate.*

**Purpose:** Data validation operations

**Intended Scope:**
- Type validation
- Format validation (email, URL, phone, etc.)
- Range validation (min/max)
- Pattern matching validation (regex)
- Custom validation predicates
- Schema validation

**Example Operations (APIs TBD):**
- `|U.Validate.Email`
- `|U.Validate.URL`
- `|U.Validate.Range`
- `|U.Validate.Pattern`
- `|U.Validate.NotEmpty`

### |U.Convert.*

**Purpose:** Type conversion and casting operations

**Intended Scope:**
- String to number conversions
- Number to string formatting
- Boolean conversions
- Array/set conversions
- Date/time conversions
- Unit conversions (future consideration)

**Example Operations (APIs TBD):**
- `|U.Convert.ToInt`
- `|U.Convert.ToFloat`
- `|U.Convert.ToString`
- `|U.Convert.ToBool`
- `|U.Convert.ToArray`

### |U.Random.*

**Purpose:** Random value generation

**Intended Scope:**
- Random number generation (integers, floats)
- Random string generation
- Random element selection from collections
- UUID generation
- Seeded random number generation
- Cryptographically secure random (where needed)

**Example Operations (APIs TBD):**
- `|U.Random.Int`
- `|U.Random.Float`
- `|U.Random.String`
- `|U.Random.Element`
- `|U.Random.UUID`

### |U.Debug.*

**Purpose:** Debugging and development utilities

**Intended Scope:**
- Value inspection and pretty-printing
- Type introspection
- Performance timing
- Debug logging
- Stack trace access
- Assertion utilities

**Example Operations (APIs TBD):**
- `|U.Debug.Print`
- `|U.Debug.TypeOf`
- `|U.Debug.Time`
- `|U.Debug.Assert`
- `|U.Debug.Trace`

## Usage Patterns

While specific APIs are not yet defined, utilities will follow consistent patterns:

### Pattern 1: Single Responsibility

Each utility performs one specific operation.

```polyglot
// Conceptual example (API TBD)
[r] |U.String.ToUpper
[<] .input: pg\string << "hello"
[>] .result: pg\string >> uppercase_result
// Expected: "HELLO"
```

### Pattern 2: Composition

Utilities compose naturally in pipelines.

```polyglot
// Conceptual example (API TBD)
[r] |U.String.Trim
[<] .input: pg\string << "  hello  "
[>] .result: pg\string >> trimmed

[r] |U.String.ToUpper
[<] .input: pg\string << trimmed
[>] .result: pg\string >> final_result
// Expected: "HELLO"
```

### Pattern 3: Error Handling

Utilities will return error types for invalid operations.

```polyglot
// Conceptual example (API TBD)
[r] |U.Convert.ToInt
[<] .input: pg\string << "not-a-number"
[>] .result: pg\int >> number
[~]
[~][!] !pg.Conversion.InvalidFormat
[~][>] .message: pg\string >> error_msg
[~][r] |HandleConversionError
[~][<] .error_msg: pg\string << error_msg
```

### Pattern 4: Parallel Processing

Utilities work within parallel blocks using expansion.

```polyglot
// Conceptual example (API TBD)
[p] |ProcessStrings
[<] .items: pg\array{pg\string} << ["hello", "world", "test"]

[~][r] |U.String.ToUpper
[<] .input: pg\string << .items[*]
[>] .result: pg\string >> uppercase_results

[Y] |Y.Join
[>] uppercase_results
// Expected: ["HELLO", "WORLD", "TEST"]
```

## Integration with Standard Library

Utilities integrate with other standard library components:

**With Runtime Wrappers ([W]):**
```polyglot
// Conceptual: Pre-process data before wrapper
[r] |U.File.Read
[<] .path: pg\path << \\Path\\.unix << "/data/input.txt"
[>] .content: pg\string >> file_content

[W] |W.Python3.11
[r] |PythonAnalyze
[<] .data: pg\string << file_content
```

**With Queue Control ([Q]):**
```polyglot
// Conceptual: Validate before queuing
[r] |U.Validate.Email
[<] .input: pg\string << user_email
[>] .valid: pg\bool >> is_valid

[t] .condition: pg\bool << is_valid
[Q] |Q.Queue.Assign
[<] .queue: pg\string << #AppQueues.EmailProcessing
```

**With Error Handling:**
```polyglot
// Conceptual: Utility operation with error handling
[r] |U.File.Read
[<] .path: pg\path << file_path
[>] .content: pg\string >> file_data
[~]
[~][!] !pg.FileSystem.NotFound
[~][>] .message: pg\string >> error_msg
[~][r] |HandleFileError
[~][<] .error_msg: pg\string << error_msg
```

## Design Principles

### 1. **No Side Effects**
Utilities are pure functions - same input always produces same output.

### 2. **Type Safety**
All utilities respect Polyglot's type system and require explicit types.

### 3. **Explicit Over Implicit**
No hidden conversions or magical behavior.

### 4. **Consistent Naming**
- Verb-based naming: `ToUpper`, `Parse`, `Validate`
- Namespace organization: `|U.Category.Operation`
- Predictable parameter names: `.input`, `.result`, `.error`

### 5. **Error Transparency**
Operations that can fail return error types explicitly.

### 6. **Performance Awareness**
Utilities avoid unnecessary allocations and copies.

### 7. **Documentation First**
Complete documentation before implementation.

## Future Documentation

Upcoming documentation versions will provide:

1. **Complete API Specifications:**
   - Input parameters with types
   - Output values with types
   - Error conditions
   - Behavioral specifications

2. **Comprehensive Examples:**
   - Basic usage for each utility
   - Common patterns and combinations
   - Error handling scenarios
   - Performance considerations

3. **Best Practices:**
   - When to use utilities vs custom pipelines
   - Performance optimization patterns
   - Error handling strategies
   - Testing approaches

4. **Migration Guides:**
   - Migrating from external libraries
   - Porting code from other languages
   - Performance comparisons

## Relationship to Packages

Utilities are part of the standard library (`|U.*`), not external packages. Key differences:

**Standard Library Utilities:**
- Built-in, always available
- Stable APIs across versions
- Core functionality only
- No external dependencies

**Packages (future):**
- External, must be installed
- Domain-specific functionality
- Community contributed
- May have dependencies

**Guideline:** If an operation is fundamental and broadly applicable, it belongs in `|U.*`. If it's domain-specific or complex, it should be a package.

## Contributing to Utility Definitions

When the compiler implementation begins, utility API definitions will be designed collaboratively. Considerations will include:

- **Use Case Coverage:** Real-world needs from the community
- **API Ergonomics:** Clear, predictable interfaces
- **Performance:** Efficient implementations
- **Cross-Platform:** Consistent behavior across operating systems
- **Backwards Compatibility:** Stable APIs once defined

## Quick Reference: Utility Categories

| Namespace | Purpose | Status |
|-----------|---------|--------|
| `|U.String.*` | String manipulation | APIs TBD |
| `|U.Array.*` | Array operations | APIs TBD |
| `|U.Set.*` | Set operations | APIs TBD |
| `|U.Math.*` | Mathematical functions | APIs TBD |
| `|U.File.*` | File system operations | APIs TBD |
| `|U.Path.*` | Path manipulation | APIs TBD |
| `|U.JSON.*` | JSON processing | APIs TBD |
| `|U.CSV.*` | CSV processing | APIs TBD |
| `|U.Hash.*` | Hashing operations | APIs TBD |
| `|U.Encode.*` | Encoding/decoding | APIs TBD |
| `|U.Validate.*` | Data validation | APIs TBD |
| `|U.Convert.*` | Type conversion | APIs TBD |
| `|U.Random.*` | Random generation | APIs TBD |
| `|U.Debug.*` | Debugging utilities | APIs TBD |

## See Also

- [Standard Library Overview](overview.md) - Complete stdlib organization
- [Runtime Wrappers](runtime-wrappers.md) - External runtime integration
- [Type System](../language/type-system.md) - Understanding Polyglot types
- [Error Handling](../language/error-handling.md) - Working with errors

---

**Navigation:**
← [Queue Control](queue-control.md) | [Standard Library Index](overview.md) | [Triggers Catalog](triggers-catalog.md) →