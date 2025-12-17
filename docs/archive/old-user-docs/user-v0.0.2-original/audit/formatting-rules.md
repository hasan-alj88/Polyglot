# Polyglot Formatting Rules

**Version:** 0.0.2
**Last Updated:** 2025-11-12
**Status:** Complete
**Purpose:** Comprehensive code formatting and style guidelines for Polyglot

---

## Overview

This document defines the official formatting rules for Polyglot v0.0.2 code. Consistent formatting improves readability, maintainability, and collaboration across projects.

**Key Principles:**
- Consistency over personal preference
- Readability is paramount
- Visual structure reflects logical structure
- Whitespace has semantic meaning

---

## Table of Contents

1. [Blank Line Rules](#1-blank-line-rules)
2. [Pipeline Structure Formatting](#2-pipeline-structure-formatting)
3. [Enumeration Formatting](#3-enumeration-formatting)
4. [Error Definition Formatting](#4-error-definition-formatting)
5. [Comment Formatting](#5-comment-formatting)
6. [String Formatting](#6-string-formatting)
7. [Nested Operations Formatting](#7-nested-operations-formatting)
8. [Code Organization](#8-code-organization)
9. [Indentation and Spacing](#9-indentation-and-spacing)
10. [Examples of Well-Formatted Code](#10-examples-of-well-formatted-code)
11. [Examples of Poorly-Formatted Code](#11-examples-of-poorly-formatted-code)
12. [Common Formatting Mistakes](#12-common-formatting-mistakes)

---

## 1. Blank Line Rules

### Rule 1.1: Before Block Elements with Pipeline Calls

**REQUIRED:** 1 blank line BEFORE block element with pipeline call

**Rationale:** Visually separates pipeline calls from other operations for easier scanning

```polyglot
// VALID
[|] ProcessData
[i] .input: pg\string

[r] |FirstOperation
[<] .data: pg\string << .input

[r] |SecondOperation
[<] .data: pg\string << .input
[X]

// INVALID - Missing blank line before pipeline calls
[|] ProcessData
[i] .input: pg\string
[r] |FirstOperation
[<] .data: pg\string << .input
[r] |SecondOperation
[<] .data: pg\string << .input
[X]
```

---

### Rule 1.2: After Block Elements with Pipeline Calls

**REQUIRED:** 1 blank line AFTER block element with pipeline call (including its children)

**Rationale:** Creates visual separation between distinct operations

```polyglot
// VALID
[r] |FirstOperation
[<] .data: pg\string << .input
[>] .result: pg\string >> output1

[r] |SecondOperation
[<] .data: pg\string << output1
[>] .result: pg\string >> output2

// INVALID - Missing blank line after pipeline call group
[r] |FirstOperation
[<] .data: pg\string << .input
[>] .result: pg\string >> output1
[r] |SecondOperation
[<] .data: pg\string << output1
```

---

### Rule 1.3: Before Pipeline Definitions

**REQUIRED:** 4 blank lines BEFORE pipeline definition `[|]`

**Rationale:** Major visual separation between pipeline definitions (top-level constructs)

**Exception:** First pipeline in file may have 0-2 blank lines before it

```polyglot
// VALID
[|] FirstPipeline
[i] .input: pg\string
[X]




[|] SecondPipeline
[i] .input: pg\string
[X]

// INVALID - Only 1 blank line
[|] FirstPipeline
[X]

[|] SecondPipeline
[X]
```

---

### Rule 1.4: Before Enumeration Definitions

**REQUIRED:** 4 blank lines BEFORE enumeration definition `[#]`

**Rationale:** Major visual separation between enumeration definitions

**Exception:** First enumeration in file may have 0-2 blank lines before it

```polyglot
// VALID
[#] FirstEnumeration
[<] .field: pg\string << "value"
[X]




[#] SecondEnumeration
[<] .field: pg\string << "value"
[X]

// INVALID - Only 2 blank lines
[#] FirstEnumeration
[X]


[#] SecondEnumeration
[X]
```

---

### Rule 1.5: Before Error Definitions

**REQUIRED:** 4 blank lines BEFORE error definition `[!]`

**Rationale:** Consistency with pipeline and enumeration separation

```polyglot
// VALID
[!] !FirstError
[<] .message: pg\string << "Error 1"
[<] .code: pg\int << 5001
[<] .trace: pg\string << ""
[X]




[!] !SecondError
[<] .message: pg\string << "Error 2"
[<] .code: pg\int << 5002
[<] .trace: pg\string << ""
[X]
```

---

### Rule 1.6: Within Pipeline Body

**RECOMMENDED:** Blank lines to separate logical sections

```polyglot
// VALID - Sections separated by blank lines
[|] ComplexPipeline
// Input declarations
[i] .file_path: pg\path
[i] Default .max_size: pg\int << 1024

// Trigger configuration
[t] |T.File.Modified
[<] .path: pg\path << .file_path

// Operations
[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> data

[r] |ProcessData
[<] .input: pg\string << data
[>] .result: pg\string >> output

// Error handling
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << err_msg

[X]
```

---

### Rule 1.7: No Blank Lines Inside Operation Groups

**REQUIRED:** Do NOT add blank lines between block elements that form a single operation group

```polyglot
// VALID - No blank lines within operation group
[r] |ProcessData
[<] .input: pg\string << data
[<] .max_size: pg\int << 1024
[>] .result: pg\string >> output
[>] .error: !Error >> err

// INVALID - Blank lines inside operation group
[r] |ProcessData

[<] .input: pg\string << data

[<] .max_size: pg\int << 1024

[>] .result: pg\string >> output
```

---

## 2. Pipeline Structure Formatting

### Rule 2.1: Pipeline Name Placement

**REQUIRED:** Pipeline name on same line as `[|]`

```polyglot
// VALID
[|] PipelineName

// INVALID
[|]
PipelineName
```

---

### Rule 2.2: Input Declarations First

**REQUIRED:** All `[i]` input declarations come immediately after `[|]`, before any operations

```polyglot
// VALID
[|] ProcessData
[i] .input: pg\string
[i] Default .max_size: pg\int << 1024

[r] |DoWork
[X]

// INVALID - Operations before inputs
[|] ProcessData
[r] |DoWork
[i] .input: pg\string  // Wrong position
[X]
```

---

### Rule 2.3: Trigger After Inputs

**RECOMMENDED:** Trigger declarations `[t]` come after inputs, before operations

```polyglot
// VALID
[|] ScheduledTask
[i] .data: pg\string

[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

[r] |DoWork
[X]
```

---

### Rule 2.4: Operations in Middle

**RECOMMENDED:** Sequential operations `[r]` and parallel blocks `[p]` in middle section

```polyglot
[|] Pipeline
[i] .input: pg\string

[r] |Operation1
[r] |Operation2

[p] |ParallelOp
[X]
```

---

### Rule 2.5: Error Handlers After Operations

**RECOMMENDED:** Error catching blocks `[!]` come after the operations they protect

```polyglot
// VALID
[r] |MightFail

[!] !SomeError
[r] |HandleError
```

---

### Rule 2.6: Output Declaration Last

**REQUIRED:** Output declaration `[o]` comes last, immediately before `[X]`

```polyglot
// VALID
[|] Transform
[i] .input: pg\string

[r] |Process
[>] .result: pg\string >> output

[o] .result: pg\string
[X]

// INVALID - Output not last
[|] Transform
[o] .result: pg\string

[i] .input: pg\string
[r] |Process
[X]
```

---

### Rule 2.7: End Marker Alignment

**REQUIRED:** `[X]` at same indentation level as `[|]`

```polyglot
// VALID
[|] Pipeline
    [i] .input: pg\string
[X]

// INVALID - Misaligned
[|] Pipeline
    [i] .input: pg\string
    [X]
```

---

## 3. Enumeration Formatting

### Rule 3.1: Enumeration Structure

**REQUIRED:** Follow this order:
1. `[#]` with enumeration name
2. `[A]` alias (if present)
3. `[<]` field definitions
4. `[X]` end marker

```polyglot
// VALID
[#] MyApp.Configuration
[A] AppConfig
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[<] .debug: pg\bool << #False
[X]
```

---

### Rule 3.2: Field Alignment

**RECOMMENDED:** Align field values for readability (optional but preferred)

```polyglot
// VALID - Aligned
[#] Configuration
[<] .host:    pg\string << "localhost"
[<] .port:    pg\int    << 8080
[<] .debug:   pg\bool   << #False
[<] .timeout: pg\int    << 30
[X]

// VALID - Not aligned (acceptable)
[#] Configuration
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[<] .debug: pg\bool << #False
[<] .timeout: pg\int << 30
[X]
```

---

### Rule 3.3: No Blank Lines Between Fields

**REQUIRED:** No blank lines between field definitions in enumerations

```polyglot
// VALID
[#] Config
[<] .field1: pg\string << "value1"
[<] .field2: pg\string << "value2"
[<] .field3: pg\string << "value3"
[X]

// INVALID - Blank lines between fields
[#] Config
[<] .field1: pg\string << "value1"

[<] .field2: pg\string << "value2"

[<] .field3: pg\string << "value3"
[X]
```

---

### Rule 3.4: Grouping Related Fields

**RECOMMENDED:** Use comments to group related fields

```polyglot
// VALID
[#] AppConfig
// Database settings
[<] .db_host: pg\string << "localhost"
[<] .db_port: pg\int << 5432

// Cache settings
[<] .cache_enabled: pg\bool << #True
[<] .cache_ttl: pg\int << 3600
[X]
```

---

## 4. Error Definition Formatting

### Rule 4.1: Required Fields Order

**REQUIRED:** Reserved fields MUST appear in this order:
1. `.message: pg\string`
2. `.code: pg\int`
3. `.trace: pg\string`
4. Custom fields (if any)

```polyglot
// VALID
[!] !MyApp.CustomError
[<] .message: pg\string << "Error occurred"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[<] .custom_field: pg\string << ""
[X]

// INVALID - Wrong order
[!] !MyApp.CustomError
[<] .code: pg\int << 5000
[<] .message: pg\string << "Error occurred"
[<] .trace: pg\string << ""
[X]
```

---

### Rule 4.2: Custom Fields After Reserved

**REQUIRED:** All custom fields come after the three reserved fields

```polyglot
// VALID
[!] !DatabaseError
[<] .message: pg\string << "Database failed"
[<] .code: pg\int << 5100
[<] .trace: pg\string << ""
[<] .query: pg\string << ""
[<] .affected_rows: pg\int << 0
[X]
```

---

### Rule 4.3: Error Alias Placement

**REQUIRED:** `[A]` alias comes immediately after `[!]` definition line

```polyglot
// VALID
[!] !MyApp.Authentication.InvalidCredentials
[A] !InvalidCreds
[<] .message: pg\string << "Invalid credentials"
[<] .code: pg\int << 4010
[<] .trace: pg\string << ""
[X]
```

---

## 5. Comment Formatting

### Rule 5.1: Single-Line Comments

**REQUIRED:** Space after `//`

```polyglot
// VALID - Space after //
// This is a comment

// INVALID - No space
//This is a comment
```

---

### Rule 5.2: Inline Comments

**REQUIRED:** At least 2 spaces before `//` on same line as code

```polyglot
// VALID
[r] .count: pg\int << 42  // Initialize counter

// INVALID - Only 1 space
[r] .count: pg\int << 42 // Initialize counter
```

---

### Rule 5.3: Multi-Line Comment Style

**RECOMMENDED:** Use consistent asterisk style for multi-line comments

```polyglot
// VALID
/*
 * This is a multi-line comment
 * with consistent asterisk style
 * for better readability
 */

// VALID (alternate style)
/* This is a simple
   multi-line comment */
```

---

### Rule 5.4: Section Headers

**RECOMMENDED:** Use comment separators for major sections

```polyglot
// VALID
[|] ComplexPipeline

// ========================================
// INPUT DECLARATIONS
// ========================================
[i] .input: pg\string
[i] Default .max_size: pg\int << 1024

// ========================================
// DATA PROCESSING
// ========================================
[r] |ProcessData
[r] |ValidateData

[X]
```

---

### Rule 5.5: Standalone vs Inline

**RECOMMENDED:**
- Use standalone comments for operation groups
- Use inline comments for individual parameters

```polyglot
// VALID
// Process the data with validation
[r] |ProcessData
[<] .input: pg\string << data
[<] .strict: pg\bool << #True  // Enable strict validation
[<] .timeout: pg\int << 30    // Timeout in seconds
```

---

## 6. String Formatting

### Rule 6.1: String Continuation with `[^]`

**REQUIRED:** When using `[^]` for string continuation in `pg\serial`, indent continuation lines

```polyglot
// VALID
[r] .config: pg\serial << serial{
[^]  "host": "localhost",
[^]  "port": 8080,
[^]  "debug": false
[^]}

// INVALID - No indentation
[r] .config: pg\serial << serial{
[^]"host": "localhost",
[^]"port": 8080
[^]}
```

---

### Rule 6.2: Continuation Marker Alignment

**REQUIRED:** `[^]` markers should align vertically

```polyglot
// VALID
[r] .data: pg\serial << serial{
[^]  "name": "Alice",
[^]  "age": 30,
[^]  "email": "alice@example.com"
[^]}

// INVALID - Misaligned
[r] .data: pg\serial << serial{
[^]  "name": "Alice",
  [^]  "age": 30,
[^]  "email": "alice@example.com"
[^]}
```

---

### Rule 6.3: Nested Serial Objects

**REQUIRED:** Increase indentation for nested serial objects

```polyglot
// VALID
[r] .config: pg\serial << serial{
[^]  "database": {
[^]    "host": "localhost",
[^]    "port": 5432
[^]  },
[^]  "cache": {
[^]    "enabled": true,
[^]    "ttl": 3600
[^]  }
[^]}
```

---

### Rule 6.4: Array Formatting in Serial

**RECOMMENDED:** Each array element on separate line for readability

```polyglot
// VALID - Multi-line array
[r] .items: pg\serial << serial{
[^]  "names": [
[^]    "Alice",
[^]    "Bob",
[^]    "Charlie"
[^]  ]
[^]}

// VALID - Single-line for short arrays
[r] .items: pg\serial << serial{
[^]  "codes": [1, 2, 3]
[^]}
```

---

## 7. Nested Operations Formatting

### Rule 7.1: Expansion Marker Indentation

**REQUIRED:** Do NOT increase indentation for `[~]` prefix - keep same level

**Rationale:** `[~]` is a marker, not a scope. Indentation represents logical nesting, not marker depth.

```polyglot
// VALID - No extra indentation for [~]
[p] |ParallelBlock
[<] .data: pg\string << input
[~][r] |NestedOperation
[~][<] .input: pg\string << .data
[~][>] .result >> temp

// INVALID - Extra indentation
[p] |ParallelBlock
[<] .data: pg\string << input
    [~][r] |NestedOperation
    [~][<] .input: pg\string << .data
```

---

### Rule 7.2: Multiple Nesting Levels

**REQUIRED:** Each `[~]` adds to marker prefix, not indentation

```polyglot
// VALID
[r] ~Array.ForEach
[~][r] .item: pg\string << current
[~][r] ~String.Split
[~][~][r] |ProcessToken
[~][~][<] .token: pg\string << current_token

// INVALID - Increasing indentation
[r] ~Array.ForEach
    [~][r] .item: pg\string << current
    [~][r] ~String.Split
        [~][~][r] |ProcessToken
```

---

### Rule 7.3: Nested Operations in Parallel Blocks

**REQUIRED:** Use `[~]` for operations within parallel blocks

```polyglot
// VALID
[p] |ProcessPartA
[<] .data: pg\string << input
[~][r] |TransformData
[~][<] .input: pg\string << .data
[~][>] .result >> temp
[>] .output >> result_a
```

---

## 8. Code Organization

### Rule 8.1: File Organization Order

**RECOMMENDED:** Organize Polyglot files in this order:
1. File header comment
2. Enumeration definitions
3. Error definitions
4. Pipeline definitions

```polyglot
/*
 * File: data_processor.pg
 * Purpose: Data processing pipelines
 * Author: Team Name
 */

// ========================================
// ENUMERATIONS
// ========================================

[#] DataProcessor.Config
[<] .max_size: pg\int << 1024
[X]




// ========================================
// ERROR DEFINITIONS
// ========================================

[!] !DataProcessor.ValidationError
[<] .message: pg\string << "Validation failed"
[<] .code: pg\int << 4000
[<] .trace: pg\string << ""
[X]




// ========================================
// PIPELINES
// ========================================

[|] ProcessData
[i] .input: pg\string
[r] |ValidateData
[X]
```

---

### Rule 8.2: Group Related Definitions

**RECOMMENDED:** Group related enumerations, errors, and pipelines together

```polyglot
// Database-related definitions
[#] Database.Config
[X]

[!] !Database.ConnectionError
[X]

[|] ConnectToDatabase
[X]




// Cache-related definitions
[#] Cache.Config
[X]

[!] !Cache.ConnectionError
[X]

[|] ConnectToCache
[X]
```

---

### Rule 8.3: Within Pipeline Organization

**REQUIRED:** Follow this order within pipelines:
1. Input declarations `[i]`
2. **Triggers `[t]` (MANDATORY - compiler error if missing)**
3. Queue control `[Q]` (if at start)
4. Wrapper context `[W]` (if needed)
5. Operations `[r]`, `[p]`
6. Error handlers `[!]`
7. Output declaration `[o]`

**CRITICAL:** ALL pipelines MUST have a trigger declaration `[t] |T.*` or compiler will throw error.

```polyglot
[|] CompleteExample
// 1. Inputs
[i] .input: pg\string
[i] Default .timeout: pg\int << 30

// 2. Triggers (REQUIRED - compiler error if missing)
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"

// 3. Queue control
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 512

// 4. Wrapper (if needed)
[W] |W.Python3.11

// 5. Operations
[r] |ProcessData
[<] .data: pg\string << .input

// 6. Error handlers
[!] !pg.FileSystem.NotFound
[r] |HandleError

// 7. Output
[o] .result: pg\string
[X]
```

---

## 9. Indentation and Spacing

### Rule 9.1: No Indentation for Block Markers

**REQUIRED:** All block markers start at column 0 (no indentation)

**Rationale:** Block markers define structure; indentation would be redundant and confusing

```polyglot
// VALID - All markers at column 0
[|] Pipeline
[i] .input: pg\string
[r] |Operation
[<] .data: pg\string << .input
[X]

// INVALID - Indented markers
[|] Pipeline
    [i] .input: pg\string
    [r] |Operation
        [<] .data: pg\string << .input
[X]
```

---

### Rule 9.2: Consistent Spacing Around Operators

**REQUIRED:** Single space around assignment operators `<<` and `>>`

```polyglot
// VALID
[r] .x: pg\int << 42
[>] .result: pg\string >> output

// INVALID - No spaces
[r] .x: pg\int<<42
[>] .result: pg\string>>output

// INVALID - Multiple spaces
[r] .x: pg\int  <<  42
```

---

### Rule 9.3: Space After Colon in Type Declarations

**REQUIRED:** Single space after colon before type

```polyglot
// VALID
[i] .input: pg\string
[r] .count: pg\int << 42

// INVALID - No space
[i] .input:pg\string

// INVALID - Multiple spaces
[r] .count:  pg\int << 42
```

---

### Rule 9.4: No Space Inside Brackets

**REQUIRED:** No spaces inside square brackets for block markers

```polyglot
// VALID
[r] |Operation
[<] .input: pg\string << value

// INVALID - Spaces inside brackets
[ r ] |Operation
[ < ] .input: pg\string << value
```

---

### Rule 9.5: Space After Pipeline Operator

**REQUIRED:** No space between `|` and pipeline name

```polyglot
// VALID
[r] |ProcessData
[t] |T.Daily

// INVALID - Space after |
[r] | ProcessData
[t] | T.Daily
```

---

## 10. Examples of Well-Formatted Code

### Example 10.1: Simple Pipeline

```polyglot
[|] GreetUser
[i] .name: pg\string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[o] .greeting: pg\string << "Hello, {.name}!"
[X]
```

---

### Example 10.2: Pipeline with Operations

```polyglot
[|] ProcessFile
[i] .file_path: pg\path
[i] Default .max_size: pg\int << 1024
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> file_data

[r] |ValidateData
[<] .data: pg\string << file_data
[<] .max_size: pg\int << .max_size
[>] .is_valid: pg\bool >> valid

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << err_msg

[o] .result: pg\string
[X]
```

---

### Example 10.3: Configuration Enumeration

```polyglot
[#] MyApp.Database.Configuration
[A] DBConfig
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 5432
[<] .database: pg\string << "myapp_db"
[<] .ssl_enabled: pg\bool << #True
[<] .connection_timeout: pg\int << 30
[X]
```

---

### Example 10.4: Error Definition

```polyglot
[!] !MyApp.Database.ConnectionError
[A] !DBConnError
[<] .message: pg\string << "Failed to connect to database"
[<] .code: pg\int << 5100
[<] .trace: pg\string << ""
[<] .host: pg\string << ""
[<] .port: pg\int << 0
[<] .retry_count: pg\int << 0
[X]
```

---

### Example 10.5: Parallel Execution

```polyglot
[|] ParallelProcessing
[i] .data: pg\string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[p] |ProcessPartA
[<] .input: pg\string << .data
[~][r] |TransformA
[~][<] .data: pg\string << .input
[>] .result >> result_a

[p] |ProcessPartB
[<] .input: pg\string << .data
[~][r] |TransformB
[~][<] .data: pg\string << .input
[>] .result >> result_b

[Y] |Y.Join
[>] result_a
[>] result_b

[r] |CombineResults
[<] .a: pg\string << result_a
[<] .b: pg\string << result_b
[>] .final: pg\string >> output

[o] .result: pg\string
[X]
```

---

### Example 10.6: Complex Pipeline with All Elements

```polyglot
/*
 * CompleteWorkflow
 *
 * Demonstrates all major formatting patterns in a single pipeline
 */
[|] CompleteWorkflow
// Input declarations
[i] .file_path: pg\path
[i] Default .timeout: pg\int << 30
[i] Default .debug: pg\bool << #False

// Trigger configuration
[t] |T.File.Modified
[<] .path: pg\path << .file_path

// Queue control
[Q] |Q.PauseIf.RAM.Available.LessThan
[<] .mb: pg\uint << 512

// Read file
[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> file_data

// Process data
[r] |ProcessData
[<] .data: pg\string << file_data
[<] .timeout: pg\int << .timeout
[>] .result: pg\string >> processed

// Error handling
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << "File not found: {err_msg}"

[!] !pg.FileSystem.PermissionDenied
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << "Access denied: {err_msg}"

// Output
[o] .result: pg\string
[X]
```

---

## 11. Examples of Poorly-Formatted Code

### Example 11.1: Missing Blank Lines

```polyglot
// INVALID - No blank lines before pipeline calls
[|] ProcessData
[i] .input: pg\string
[r] |FirstOperation
[<] .data: pg\string << .input
[r] |SecondOperation
[<] .data: pg\string << .input
[X]
```

**Problem:** Pipeline calls not visually separated

**Fix:** Add blank line before each pipeline call group

---

### Example 11.2: Inconsistent Spacing

```polyglot
// INVALID - Inconsistent spacing around operators
[r] .x:pg\int<< 42
[r] .y: pg\int  <<  10
[r] .z : pg\int <<  5
```

**Problem:** Inconsistent spacing around `:` and `<<`

**Fix:** Use single space consistently: `.x: pg\int << 42`

---

### Example 11.3: Wrong Definition Separation

```polyglot
// INVALID - Only 1 blank line between pipelines
[|] FirstPipeline
[X]

[|] SecondPipeline
[X]
```

**Problem:** Should have 4 blank lines between top-level definitions

**Fix:** Use 4 blank lines between pipeline definitions

---

### Example 11.4: Incorrect Indentation

```polyglot
// INVALID - Indented block markers
[|] Pipeline
    [i] .input: pg\string
    [r] |Operation
        [<] .data: pg\string << .input
[X]
```

**Problem:** Block markers should not be indented

**Fix:** All block markers at column 0

---

### Example 11.5: Wrong Field Order in Error

```polyglot
// INVALID - Wrong order of reserved fields
[!] !CustomError
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[<] .message: pg\string << "Error"
[X]
```

**Problem:** Reserved fields not in required order

**Fix:** Order must be: `.message`, `.code`, `.trace`

---

### Example 11.6: Blank Lines in Operation Group

```polyglot
// INVALID - Blank lines within operation group
[r] |ProcessData

[<] .input: pg\string << data

[>] .result: pg\string >> output
```

**Problem:** Blank lines break up operation group

**Fix:** No blank lines between operation and its parameters

---

### Example 11.7: Messy Parallel Block

```polyglot
// INVALID - Poor parallel block formatting
[p] |ProcessPartA
[<] .data: pg\string << input
    [~][r] |Transform
        [~][<] .input: pg\string << .data
[>] .result >> result_a
```

**Problem:** Incorrect indentation, missing blank lines

**Fix:** No indentation for `[~]`, blank line before `[>]`

---

## 12. Common Formatting Mistakes

### Mistake 12.1: Forgetting Blank Lines Before Definitions

**Problem:**
```polyglot
[#] FirstEnum
[X]
[#] SecondEnum  // Only 0 blank lines
[X]
```

**Fix:**
```polyglot
[#] FirstEnum
[X]




[#] SecondEnum  // 4 blank lines
[X]
```

---

### Mistake 12.2: Indenting Block Markers

**Problem:**
```polyglot
[|] Pipeline
    [i] .input: pg\string  // Indented
    [r] |Operation         // Indented
[X]
```

**Fix:**
```polyglot
[|] Pipeline
[i] .input: pg\string  // No indentation
[r] |Operation         // No indentation
[X]
```

---

### Mistake 12.3: Wrong Comment Spacing

**Problem:**
```polyglot
//No space after slashes
[r] .x: pg\int << 42 //Only 1 space before inline comment
```

**Fix:**
```polyglot
// Space after slashes
[r] .x: pg\int << 42  // At least 2 spaces before inline comment
```

---

### Mistake 12.4: Mixing Output Placement

**Problem:**
```polyglot
[|] Pipeline
[o] .result: pg\string  // Output at top

[i] .input: pg\string

[r] |Process
[X]
```

**Fix:**
```polyglot
[|] Pipeline
[i] .input: pg\string

[r] |Process

[o] .result: pg\string  // Output last
[X]
```

---

### Mistake 12.5: Inconsistent String Continuation

**Problem:**
```polyglot
[r] .config: pg\serial << serial{
[^]"host": "localhost",
  [^]  "port": 8080,
[^]"debug": false
[^]}
```

**Fix:**
```polyglot
[r] .config: pg\serial << serial{
[^]  "host": "localhost",
[^]  "port": 8080,
[^]  "debug": false
[^]}
```

---

### Mistake 12.6: Extra Indentation for Nested Operations

**Problem:**
```polyglot
[p] |Parallel
[<] .data: pg\string << input
    [~][r] |Nested  // Extra indentation
    [~][<] .input: pg\string << .data
```

**Fix:**
```polyglot
[p] |Parallel
[<] .data: pg\string << input
[~][r] |Nested  // Same level
[~][<] .input: pg\string << .data
```

---

### Mistake 12.7: Missing Blank Line After Pipeline Call

**Problem:**
```polyglot
[r] |FirstOperation
[<] .data: pg\string << input
[>] .result: pg\string >> output
[r] |SecondOperation  // No blank line
```

**Fix:**
```polyglot
[r] |FirstOperation
[<] .data: pg\string << input
[>] .result: pg\string >> output

[r] |SecondOperation  // Blank line added
```

---

## Summary of Key Rules

### Blank Lines
- **1 blank line** before block element with pipeline call
- **1 blank line** after block element with pipeline call group
- **4 blank lines** before `[|]` pipeline definition
- **4 blank lines** before `[#]` enumeration definition
- **4 blank lines** before `[!]` error definition
- **0 blank lines** within operation groups or field definitions

### Structure
- All block markers at column 0 (no indentation)
- Inputs first, operations middle, output last
- Reserved error fields in order: `.message`, `.code`, `.trace`
- `[X]` at same level as opening marker

### Spacing
- Single space after `:` in type declarations
- Single space around `<<` and `>>`
- At least 2 spaces before inline comments
- Space after `//` in comments

### Organization
- Enumerations, then errors, then pipelines
- Group related definitions
- Consistent order within pipelines

---

## Automated Formatting

**Note:** These rules can be enforced by an automated formatter tool. If available, use the official Polyglot formatter to ensure compliance.

---

## See Also

### Language Specification
- [Complete Syntax Reference](../language/01-syntax-complete.md)
- [Block Markers](../language/06-block-markers.md)
- [Comments](../language/11-comments.md)

### Examples
- [Hello World](../examples/01-hello-world.md)
- [Complete Workflows](../examples/complete-workflows.md)

---

**End of Formatting Rules**