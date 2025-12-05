# Error Handling

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Polyglot's error handling system uses **`!Error` types** - special enumerations with three reserved fields designed for structured error management. This system provides compile-time type safety while allowing flexible error handling patterns.

**Key Characteristics:**
- Errors are special enumerations marked with `!`
- Three reserved fields: `.message`, `.code`, `.trace`
- Strong type safety with compile-time checking
- Support for custom fields beyond reserved three
- Multiple error handling patterns (minimal, detailed, partial)

---

## Table of Contents

1. [Error Type Basics](#error-type-basics)
2. [Reserved Fields](#reserved-fields)
3. [Defining Custom Errors](#defining-custom-errors)
4. [Catching Errors](#catching-errors)
5. [Error Field Extraction](#error-field-extraction)
6. [Error Handling Patterns](#error-handling-patterns)
7. [Built-in Error Types](#built-in-error-types)
8. [Error vs Enumeration](#error-vs-enumeration)
9. [Best Practices](#best-practices)
10. [Migration from v0.0.1](#migration-from-v001)

---

## Error Type Basics

### What are Error Types?

**Error types** (`!Error`) are special enumerations designed specifically for error handling.

**Key Differences from Regular Enumerations:**
- Marked with `!` operator instead of `#`
- Have three reserved fields that must be present
- Can have additional custom fields
- Used in error catching syntax

---

### Error Marker: `!`

The `!` operator marks error types:

```polyglot
// Define error type
[!] !MyApp.CustomError
[X]

// Catch error type
[!] !pg.FileSystem.NotFound

// Reference error type
[i] .error: !ErrorType
```

---

### Why Special Error Types?

**Benefits over generic error handling:**
- **Type Safety:** Catch specific error types at compile-time
- **Structured Data:** Three reserved fields provide consistent error information
- **Extensibility:** Add custom fields for context-specific errors
- **Pattern Matching:** Different handling for different error types
- **Debugging:** Stack traces and error codes built-in

---

## Reserved Fields

### Three Required Fields

Every error type MUST have three reserved fields:

| Field | Type | Purpose |
|-------|------|---------|
| `.message` | `pg\string` | Human-readable error message |
| `.code` | `pg\int` | Numeric error code |
| `.trace` | `pg\string` | Stack trace information |

---

### Reserved Field Semantics

**`.message: pg\string`**
- Human-readable description of the error
- Should be clear and actionable
- May include context about what went wrong

**`.code: pg\int`**
- Numeric error code for programmatic handling
- Unique within error hierarchy
- Follows error code conventions (e.g., HTTP-style codes)

**`.trace: pg\string`**
- Stack trace or execution path
- Automatically populated by runtime
- Empty string if no trace available

---

### Example with Reserved Fields

```polyglot
[!] !MyApp.ValidationError
[<] .message: pg\string << "Validation failed"
[<] .code: pg\int << 4000
[<] .trace: pg\string << ""
[X]
```

---

## Defining Custom Errors

### Definition Syntax

**Block Markers:**
- `[!]` - Start error definition
- `[X]` - End error definition

**Format:**
```polyglot
[!] !ErrorName
[<] .message: pg\string << "default message"
[<] .code: pg\int << error_code
[<] .trace: pg\string << ""
[<] .custom_field: type << value  // Optional
[X]
```

---

### Minimal Custom Error

```polyglot
[!] !MyApp.CustomError
[<] .message: pg\string << "An error occurred"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""
[X]
```

---

### Error with Custom Fields

Custom errors can have additional fields beyond the three reserved:

```polyglot
[!] !MyApp.DatabaseError
[<] .message: pg\string << "Database operation failed"
[<] .code: pg\int << 5100
[<] .trace: pg\string << ""
// Custom fields
[<] .query: pg\string << ""
[<] .affected_rows: pg\int << 0
[<] .connection_id: pg\string << ""
[X]
```

---

### Error Naming Conventions

**Recommended format:**
```
!Package.Context.ErrorName
```

**Examples:**
```polyglot
[!] !MyApp.Validation.InvalidInput
[!] !MyApp.Database.ConnectionFailed
[!] !MyApp.API.RateLimitExceeded
[!] !DataProcessor.Transform.TypeMismatch
[!] !FileHandler.IO.PermissionDenied
```

---

### Error with Alias

Errors support aliases like enumerations:

```polyglot
[!] !MyApp.Authentication.InvalidCredentials
[A] !InvalidCreds  // Alias usable within package
[<] .message: pg\string << "Invalid username or password"
[<] .code: pg\int << 4010
[<] .trace: pg\string << ""
[<] .username_attempted: pg\string << ""
[<] .login_attempts: pg\int << 0
[X]
```

**Usage with alias:**
```polyglot
// Catch with full name
[!] !MyApp.Authentication.InvalidCredentials

// Catch with alias (package-scoped)
[!] !InvalidCreds
```

---

## Catching Errors

### Basic Error Catching

**Syntax:**
```polyglot
[r] |MightFail
[!] !ErrorType
// Handle error here
```

**Example:**
```polyglot
[|] ProcessFile
[i] .file_path: pg\path
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> .file_content
[~]
[~][!] !pg.FileSystem.NotFound
[~][>] .message: pg\string >> .err_msg
[~]
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "File not found: {.err_msg}"

[o] .content: pg\string
[X]
```

---

### Multiple Error Types

Catch different error types separately:

```polyglot
// Define custom database connection using reserved extendable enumeration
// #Database.connections.* is a reserved extendable enumeration
[#] Database.connections.User.Defined.Connection
[A] Connection1  // Can use #Connection1 as alias
[s] << @yaml \\FileDir\\dbconnection.yaml  // Load from file
[~][!] Default  // Default error handling
[X]




[|] DatabaseOperation
[i] #None
[t] |T.Call
[W] |W.DB.connect
[<] .connection: #Database.connections << #Connection1
[>] .handle: pg\db >> .db
[~]
[~][!] !pg.Database.ConnectionFailed
[~][>] .message: pg\string >> .err_msg
[~]
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "Could not connect: {.err_msg}"
[~]
[~][!] !pg.Database.Timeout
[~][>] .message: pg\string >> .timeout_err
[~]
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "Timeout: {.timeout_err}"

[r] |U.DB.Query
[<] .db: pg\db << .db
[<] .sql: pg\string << sql"SELECT * FROM Table1"
[>] .results: pg\serial >> .table1

[o] .table1: pg\serial
[X]
```

---

### Error Flow Control

Errors interrupt normal flow:

```polyglot
[|] ProcessData
[r] |Step1  // If this fails...

[!] !SomeError
[r] |HandleError  // ...this runs

[r] |Step2  // This will NOT run if Step1 failed
[r] |Step3  // This will NOT run if Step1 failed
[X]
```

**Note:** After catching an error, subsequent operations after the error handler do not execute. The pipeline continues from the error handler.

---

### Serial Block Error Handling (`[s][!]`)

**Purpose:** Catch errors from Serial Load Blocks (`[s]`) with scope-level error handling

**Syntax:**
```polyglot
[s] .variable << Format"path"
[s][!] !ErrorType
// Handle serial block errors
```

**Key Characteristics:**
- Catches errors from **ALL** `[s]` blocks at the same scope/level
- Two-level error handling: variable-level (`.var.error`) and scope-level (`[s][!]`)
- Error precedence: `[~][!]` (specific) overrides `[s][!]` (general)
- Partial success model: successful loads complete even when others fail

---

#### Basic Serial Error Handling

```polyglot
[|] LoadConfig
[i] .env: #Environment
[t] |T.Call

[s] .db_config << JSON"db.json"
[s] .api_config << JSON"api.json"

[s][!] !File.NotFound
[>] .message >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << "Config file not found: {err_msg}"
[o] !ConfigurationError

[s][!] !JSON.ParseError
[r] |HandleParseError

[o] .db_config: pg\serial
[X]
```

---

#### Multiple Error Types

```polyglot
[s] .base_config << JSON"base.json"
[s] .env_config << JSON"{.env}.json"
[s] .secrets << JSON.FilenameKey"secrets/*.json"

[s][!] !File.NotFound         // Catches NotFound for ALL serial blocks
[>] .message >> err_msg
[r] |UseDefaultConfig

[s][!] !JSON.ParseError       // Catches ParseError for ALL serial blocks
[>] .message >> parse_err
[r] |U.Log.Error
[<] .msg: pg\string << "Invalid JSON: {parse_err}"
[o] !ConfigurationError

[s][!] !Serial.ReservedEnumeration.MissingField  // Validation errors
[r] |HandleValidationError
```

---

#### Variable-Level vs Scope-Level

**Variable-Level Error Checking:**
```polyglot
[s] .config << JSON"config.json"

// Check specific variable's error
[!] .config.error =? !File.NotFound
[r] |UseDefaultConfig

// Or fail pipeline with the error
[o] .config.error
```

**Scope-Level Error Handling:**
```polyglot
[s] .config1 << JSON"c1.json"
[s] .config2 << JSON"c2.json"

// Catches errors from BOTH serial blocks
[s][!] !File.NotFound
[r] |HandleMissing
```

---

#### Error Precedence

```polyglot
[s] .critical_config << JSON"critical.json"
[s] .optional_config << JSON"optional.json"

// Specific handler for critical config (takes precedence)
[~][!] !File.NotFound
[r] |FailPipeline

// General handler for all other serial blocks
[s][!] !File.NotFound
[r] |UseDefaults

// [~][!] runs for critical_config
// [s][!] runs for optional_config
```

---

#### Error-Carrying Variables

**All serial load variables carry error state:**

```polyglot
[s] .config << JSON"config.json"

// Variable states:
// Success: .config = data, .config.error = !NoError
// Failure: .config = #None.ErrorState, .config.error = !File.NotFound

// Check error state
[!] .config.error =? !NoError
[r] |ProcessConfig
[<] .data: pg\serial << .config

// Handle error state
[!] .config.error =? !File.NotFound
[r] |UseDefaultConfig
```

---

#### Scope-Based Error Handling

```polyglot
// Same scope - shared error handling
[s] .config1 << JSON"c1.json"
[s] .config2 << JSON"c2.json"
[s][!] !File.NotFound   // Handles errors from both

// Different scope - separate handling
[?] .env =? #Production
[~] [s] .config << JSON"prod.json"
[~] [s][!] !File.NotFound
[~] [r] |FailCritical

[?] .env =? #Development
[~] [s] .config << JSON"dev.json"
[~] [s][!] !File.NotFound
[~] [r] |UseDefaults
```

---

#### Implicit Error Notification

**If no explicit `[s][!]` handler:**
- Automatic logging/console output
- Context-aware (console display vs. log files)
- Critical for automated pipelines (failure visibility required)

```polyglot
[s] .config << JSON"config.json"
// No [s][!] handler → automatic error notification if load fails
```

---

#### Common Serial Error Types

| Error Type | When Raised |
|------------|-------------|
| `!File.NotFound` | File not found or empty (0 bytes) |
| `!JSON.ParseError` | Invalid JSON syntax |
| `!YAML.ParseError` | Invalid YAML syntax |
| `!TOML.ParseError` | Invalid TOML syntax |
| `!XML.ParseError` | Invalid XML syntax |
| `!Serial.ReservedEnumeration.MissingField` | Required field missing in reserved type |
| `!Serial.ReservedEnumeration.FieldMismatch` | Field type mismatch in reserved type |

---

## Error Field Extraction

### Extracting Error Fields

Use `[>]` with `>>` operator to extract error fields:

**Syntax:**
```polyglot
[!] !ErrorType
[>] .field: type >> variable_name
```

---

### Extract All Reserved Fields

```polyglot
[|] DetailedErrorHandling
[i] #None
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |MightFail
[>] .result: pg\string >> .success_result
[~]
[~][!] !pg.FileSystem.NotFound
[~][>] .message: pg\string >> .err_message
[~][>] .code: pg\int >> .err_code
[~][>] .trace: pg\string >> .err_trace
[~]
[~][r] |U.Log.Error
[~][<] .msg: pg\string << .err_message
[~][<] .code: pg\int << .err_code
[~][<] .trace: pg\string << .err_trace

[o] .result: pg\string
[X]
```

---

### Extract Custom Fields

```polyglot
[!] !MyApp.DatabaseError
[<] .message: pg\string << "Query failed"
[<] .code: pg\int << 5100
[<] .trace: pg\string << ""
[<] .query: pg\string << ""
[<] .affected_rows: pg\int << 0
[X]

// Later, catch and extract
[|] ProcessQuery
[r] |ExecuteQuery

[!] !MyApp.DatabaseError
[>] .message: pg\string >> err_msg
[>] .query: pg\string >> failed_query
[>] .affected_rows: pg\int >> rows

[r] |U.Log.Error
[<] .msg: pg\string << "Query failed: {failed_query}, rows: {rows}"

[X]
```

---

### Partial Field Extraction

Extract only the fields you need:

```polyglot
[|] MinimalErrorHandling
[r] |MightFail

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
// Don't need .code or .trace

[r] |U.Log.Error
[<] .msg: pg\string << err_msg

[X]
```

---

### Optional vs Required Extraction

**Question for consideration:**
- Are all three reserved fields always required in extraction?
- Or can you extract only what you need?

**Current Decision (Pending #3):** Partial extraction is allowed. You extract only the fields you need.

---

## Error Handling Patterns

### Pattern 1: Minimal Handling

**Use case:** Simple error logging, don't need details

```polyglot
[|] SimpleOperation
[r] |MightFail

[!] !pg.FileSystem.NotFound
[r] |U.Log.Error
[<] .msg: pg\string << "File not found"

[X]
```

**Characteristics:**
- No field extraction
- Simple response
- Log or ignore

---

### Pattern 2: Detailed Handling

**Use case:** Need full error information for debugging

```polyglot
[|] DetailedOperation
[r] |MightFail

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_message
[>] .code: pg\int >> err_code
[>] .trace: pg\string >> err_trace

[r] |U.Log.Error
[<] .msg: pg\string << "Error {err_code}: {err_message}"
[<] .trace: pg\string << err_trace

[r] |NotifyAdmin
[<] .error_details: pg\string << "Full error information..."

[X]
```

**Characteristics:**
- Extract all reserved fields
- Detailed logging
- May trigger alerts

---

### Pattern 3: Partial Field Extraction

**Use case:** Only need specific error information

```polyglot
[|] PartialOperation
[r] |MightFail

[!] !MyApp.DatabaseError
[>] .message: pg\string >> err_msg
[>] .query: pg\string >> failed_query
// Don't need .code, .trace, or .affected_rows

[r] |RetryWithDifferentQuery
[<] .original_query: pg\string << failed_query

[X]
```

**Characteristics:**
- Extract only needed fields
- Cleaner code
- Better performance

---

### Pattern 4: Error Recovery

**Use case:** Attempt recovery or fallback

```polyglot
[|] WithFallback
[r] |TryPrimaryMethod

[!] !pg.Network.ConnectionFailed
[>] .message: pg\string >> err_msg
[r] |U.Log.Warning
[<] .msg: pg\string << "Primary failed, trying fallback"

[r] |TryFallbackMethod

[!] !pg.Network.ConnectionFailed
[r] |U.Log.Error
[<] .msg: pg\string << "Both methods failed"

[X]
```

**Characteristics:**
- Try alternative approaches
- Graceful degradation
- User-friendly failure

---

### Pattern 5: Error Transformation

**Use case:** Catch low-level error, throw high-level error

```polyglot
[|] HighLevelOperation
[r] |LowLevelOperation

[!] !pg.FileSystem.PermissionDenied
[>] .message: pg\string >> low_level_msg

// Transform to application-level error
[r] |ThrowError
[<] .error: !MyApp.AccessDenied
[<] .message: pg\string << "User does not have access to this resource"
[<] .original_error: pg\string << low_level_msg

[X]
```

**Characteristics:**
- Abstraction layer
- Hide implementation details
- Consistent error interface

---

### Pattern 6: Error Aggregation

**Use case:** Collect multiple errors before reporting

```polyglot
[|] ValidateMultipleFields
[r] .errors: pg.mutable\array{pg\string} << array{}

[r] |ValidateUsername
[!] !MyApp.ValidationError
[>] .message: pg\string >> err_msg
[r] .errors.append(err_msg)

[r] |ValidateEmail
[!] !MyApp.ValidationError
[>] .message: pg\string >> err_msg
[r] .errors.append(err_msg)

[r] |ValidatePassword
[!] !MyApp.ValidationError
[>] .message: pg\string >> err_msg
[r] .errors.append(err_msg)

// Report all errors at once
[?] .errors.length > 0
[~][r] |ReportValidationErrors
[~][<] .errors: pg\array{pg\string} << .errors

[X]
```

**Characteristics:**
- Multiple validations
- Collect all issues
- Single report

---

## Built-in Error Types

### Standard Library Errors

Polyglot provides built-in error types in the `pg` namespace:

---

### File System Errors

```polyglot
!pg.FileSystem.NotFound
!pg.FileSystem.PermissionDenied
!pg.FileSystem.AlreadyExists
!pg.FileSystem.IsDirectory
!pg.FileSystem.IsFile
!pg.FileSystem.IOError
```

**Example:**
```polyglot
[r] |ReadFile
[<] .path: pg\path << "data.txt"

[!] !pg.FileSystem.NotFound
[r] |U.Log.Error
[<] .msg: pg\string << "File not found"

[!] !pg.FileSystem.PermissionDenied
[r] |U.Log.Error
[<] .msg: pg\string << "Permission denied"
```

---

### Network Errors

```polyglot
!pg.Network.ConnectionFailed
!pg.Network.Timeout
!pg.Network.HostNotFound
!pg.Network.ConnectionReset
!pg.Network.SSLError
```

**Example:**
```polyglot
[r] |HTTP.Get
[<] .url: pg\string << "https://api.example.com"

[!] !pg.Network.Timeout
[r] |U.Log.Warning
[<] .msg: pg\string << "Request timed out, retrying..."
```

---

### Database Errors

```polyglot
!pg.Database.ConnectionFailed
!pg.Database.AuthenticationFailed
!pg.Database.QueryError
!pg.Database.Timeout
!pg.Database.ConstraintViolation
!pg.Database.DeadlockDetected
```

**Example:**
```polyglot
[r] |Database.Query
[<] .sql: pg\string << "SELECT * FROM users"

[!] !pg.Database.QueryError
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << "Query failed: {err_msg}"
```

---

### Type Errors

```polyglot
!pg.Type.Mismatch
!pg.Type.ConversionFailed
!pg.Type.Overflow
!pg.Type.InvalidCast
```

---

### Runtime Errors

```polyglot
!pg.Runtime.OutOfMemory
!pg.Runtime.StackOverflow
!pg.Runtime.NullReference
!pg.Runtime.IndexOutOfBounds
!pg.Runtime.DivisionByZero
```

---

## Error vs Enumeration

### Comparison Table

| Feature | Error (`!Error`) | Enumeration (`#Enum`) |
|---------|-----------------|----------------------|
| **Marker** | `!` | `#` |
| **Reserved fields** | 3 required (`.message`, `.code`, `.trace`) | None |
| **Custom fields** | Yes (beyond 3 reserved) | All fields are custom |
| **Definition** | `[!]...[X]` blocks | `[#]...[X]` blocks |
| **Usage** | Error catching | Data structures |
| **Immutable** | Yes | Yes |
| **Type safety** | Strong | Strong |
| **Extendable** | No (create new types) | Some (reserved only) |

---

### Similarities

Both are:
- Immutable data structures
- Defined with block syntax
- Support custom fields
- Support aliases with `[A]`
- Have compile-time type checking
- Use dot notation for field access

---

### Key Difference

**Errors** have three reserved fields that are automatically managed by the runtime:

```polyglot
// Error - MUST have 3 reserved fields
[!] !MyError
[<] .message: pg\string << "Error"
[<] .code: pg\int << 5000
[<] .trace: pg\string << ""  // Runtime fills this
[X]

// Enumeration - no reserved fields
[#] MyEnum
[<] .field1: pg\string << "value"
[<] .field2: pg\int << 42
[X]
```

---

### When to Use What

**Use Errors (`!Error`) when:**
- Handling error conditions
- Need stack trace information
- Want error codes
- Need structured error handling

**Use Enumerations (`#Enum`) when:**
- Defining configuration
- Defining constants
- Modeling data structures
- Not error-related

---

## Best Practices

### 1. Use Descriptive Error Messages

**Good:**
```polyglot
[!] !MyApp.ValidationError
[<] .message: pg\string << "Username must be between 3 and 20 characters"
[<] .code: pg\int << 4001
[<] .trace: pg\string << ""
[X]
```

**Avoid:**
```polyglot
[!] !MyApp.ValidationError
[<] .message: pg\string << "Error"  // Too vague
[<] .code: pg\int << 4001
[<] .trace: pg\string << ""
[X]
```

---

### 2. Use Meaningful Error Codes

**Good:**
```polyglot
// Use HTTP-style or domain-specific codes
[!] !MyApp.NotFound
[<] .code: pg\int << 404

[!] !MyApp.Unauthorized
[<] .code: pg\int << 401

[!] !MyApp.ValidationError
[<] .code: pg\int << 4000
```

**Avoid:**
```polyglot
// Random or sequential codes
[!] !MyApp.Error1
[<] .code: pg\int << 1

[!] !MyApp.Error2
[<] .code: pg\int << 2
```

---

### 3. Organize Errors Hierarchically

**Good:**
```polyglot
[!] !MyApp.Database.ConnectionFailed
[!] !MyApp.Database.QueryError
[!] !MyApp.Database.Timeout

[!] !MyApp.Validation.InvalidEmail
[!] !MyApp.Validation.InvalidPassword
```

**Avoid:**
```polyglot
[!] !DBConnectionFailed
[!] !DBQueryError
[!] !InvalidEmail
[!] !InvalidPassword
```

---

### 4. Add Context with Custom Fields

**Good:**
```polyglot
[!] !MyApp.DatabaseError
[<] .message: pg\string << "Query execution failed"
[<] .code: pg\int << 5100
[<] .trace: pg\string << ""
[<] .query: pg\string << ""
[<] .table: pg\string << ""
[<] .affected_rows: pg\int << 0
[X]
```

**Avoid:**
```polyglot
// No context - harder to debug
[!] !MyApp.DatabaseError
[<] .message: pg\string << "Query failed"
[<] .code: pg\int << 5100
[<] .trace: pg\string << ""
[X]
```

---

### 5. Extract Only What You Need

**Good:**
```polyglot
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
// Only need the message
```

**Avoid:**
```polyglot
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[>] .code: pg\int >> err_code
[>] .trace: pg\string >> err_trace
// Extracting fields you won't use
```

---

### 6. Handle Errors at Appropriate Level

**Good:**
```polyglot
// Low-level operation
[|] ReadConfigFile
[i] .path: pg\path
[r] |ReadFile
[<] .path: pg\path << .path
// Let error bubble up - don't handle here
[X]

// High-level operation
[|] Initialize
[r] |ReadConfigFile
[<] .path: pg\path << "config.json"
[!] !pg.FileSystem.NotFound
[r] |UseDefaultConfig  // Handle at appropriate level
[X]
```

---

### 7. Don't Swallow Errors Silently

**Good:**
```polyglot
[!] !pg.FileSystem.NotFound
[r] |U.Log.Warning
[<] .msg: pg\string << "Config not found, using defaults"
[r] |UseDefaultConfig
```

**Avoid:**
```polyglot
[!] !pg.FileSystem.NotFound
// No logging, no handling - error disappeared!
```

---

### 8. Use Error Recovery Patterns

**Good:**
```polyglot
[r] |TryPrimaryService
[!] !pg.Network.ConnectionFailed
[r] |U.Log.Warning
[<] .msg: pg\string << "Primary failed, trying backup"
[r] |TryBackupService
```

**Avoid:**
```polyglot
[r] |TryPrimaryService
[!] !pg.Network.ConnectionFailed
[r] |U.Log.Error
[<] .msg: pg\string << "Failed"
// Give up immediately - no fallback
```

---

## Migration from v0.0.1

### Deprecated: `#Errors.*`

In v0.0.1, errors were handled with `#Errors.*` enumerations. This has been **completely replaced** in v0.0.2.

---

### Old v0.0.1 Syntax (Deprecated)

```polyglot
// ✗ OLD - DO NOT USE
[r] |MightFail
[!] #Errors.FileNotFound

// Access error fields
[r] .msg: pg\string << #Errors.FileNotFound.message
```

---

### New v0.0.2 Syntax (Current)

```polyglot
// ✓ NEW - Use this
[r] |MightFail
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg

[r] |HandleError
[<] .msg: pg\string << err_msg
```

---

### Migration Steps

1. **Replace `#Errors.*` with `!ErrorType`**
   - Old: `#Errors.FileNotFound`
   - New: `!pg.FileSystem.NotFound`

2. **Use `!` marker instead of `#`**
   - Old: `[!] #Errors.SomeError`
   - New: `[!] !pg.SomeError`

3. **Extract fields with `[>]` and `>>`**
   - Old: Access via enumeration field
   - New: Extract with `[>] .field >> variable`

4. **Define custom errors with `[!]...[X]`**
   - Old: Use `#Errors.*` enumeration
   - New: Define with `[!] !CustomError...[X]`

---

### No Automatic Migration

**Important:** There is no automatic migration from v0.0.1 to v0.0.2 for error handling. The syntax is completely different.

All error handling code must be manually updated to use the new `!Error` syntax.

---

## See Also

### Language Specification
- [Type System](type-system.md) - Error types
- [Enumerations](enumerations.md) - Relationship to enumerations
- [Complete Syntax Reference](syntax-complete.md) - `[!]` syntax

### Examples
- [Error Handling Examples](../examples/error-handling.md) - All patterns with examples

### Planning
- [Decision Log](../decision-log.md) - Error handling decisions (#13, Pending #3)

---

**End of Error Handling Reference**