# Macro System

**Version:** 0.0.2
**Last Updated:** 2025-11-18
**Status:** Complete

---

## Overview

Polyglot's macro system provides **compile-time inline code templates** for reusable setup/cleanup patterns. Macros are NOT runtime constructs - they inline directly at the unwrap site during compilation.

**Key Principles:**
- Compile-time inline insertion (NOT runtime)
- Blocks insert by TYPE, not position
- Type safety enforced at unwrap site
- RAII-like resource management (FIFO setup, LIFO cleanup)
- No keywords - pure block-based syntax

---

## Table of Contents

1. [Macro Definition](#macro-definition)
2. [Macro Unwrap](#macro-unwrap)
3. [Scope Flow](#scope-flow)
4. [Block Type Declaration](#block-type-declaration)
5. [Multiple Macros](#multiple-macros)
6. [Type Safety](#type-safety)
7. [Complete Examples](#complete-examples)
8. [Best Practices](#best-practices)

---

## Macro Definition

### `[M]` Block

**Purpose:** Define a macro template

**Syntax:**
```polyglot
[M] MacroName
[<] Macro.include"<chars+"  // Declare which block types included
// Macro body...
[X]
```

**Example:**
```polyglot
[M] DatabaseSetup
[<] Macro.include"{\/"     // Contains: [{], [}], [\], [/]
[{] .db_host: pg\string    // Scope input
[}] .db_conn: pg\db        // Scope output

[\] |U.DB.Connect
[<] .host: pg\string << .db_host
[>] .connection: pg\db >> .db_conn

[/] |U.DB.Disconnect
[<] .conn: pg\db << .db_conn
[X]
```

---

## Macro Unwrap

### `[W]` Block

**Purpose:** Inline macro at unwrap site

**Syntax:**
```polyglot
[W] |MacroName
[<] .input_var << value    // Wire inputs
[>] .output_var >> result  // Wire outputs
```

**Example:**
```polyglot
[|] MyPipeline
[W] |DatabaseSetup
[<] .db_host << "localhost"
[>] .db_conn >> .db

// .db is now available in pipeline scope!
[r] |QueryUsers
[<] .conn: pg\db << .db
[X]
```

**Behavior:**
- Macro blocks insert by TYPE at unwrap site
- All `[\]` setup blocks execute first
- All `[/]` cleanup blocks execute last (LIFO order)
- Type checking happens at unwrap site

---

## Scope Flow

### `[{]` - Scope Input

**Purpose:** Variables flowing INTO macro from caller

**Syntax:**
```polyglot
[{] .variable_name: pg\type
```

**Example:**
```polyglot
[M] Logger
[{] .log_level: pg\string    // IN from caller
[{] .log_file: pg\path       // IN from caller
[}] .log_handle: pg\file     // OUT to caller

[\] |U.Log.Open
[<] .level: pg\string << .log_level
[<] .path: pg\path << .log_file
[>] .handle: pg\file >> .log_handle
[X]
```

**Flow:** Caller MUST provide all `[{]` variables at unwrap site

---

### `[}]` - Scope Output

**Purpose:** Variables flowing OUT of macro to caller

**Syntax:**
```polyglot
[}] .variable_name: pg\type
```

**Flow:** All `[}]` variables become available in caller after macro unwrap

---

### `[i]` - Constant Input

**Purpose:** Immutable constants (replaces `Fixed` keyword)

**Syntax:**
```polyglot
[i] .variable_name: pg\type << value
```

**Example:**
```polyglot
[M] RateLimiter
[i] .max_requests: pg\int << 1000    // Immutable constant
[{] .window_seconds: pg\int          // Mutable input
[}] .limiter: pg\db                  // Output

[\] |U.RateLimit.Create
[<] .max: pg\int << .max_requests
[<] .window: pg\int << .window_seconds
[>] .handle: pg\db >> .limiter
[X]
```

**Behavior:**
- Value cannot be reassigned within macro
- Compiler enforces immutability
- No `Fixed` keyword needed

---

## Block Type Declaration

### `Macro.include"<chars+"` Syntax

**Purpose:** Explicitly declare which block types the macro contains

**Syntax:**
```polyglot
[<] Macro.include"<block_chars+"
```

**Block Characters:**
- `{` = Contains `[{]` scope input blocks
- `}` = Contains `[}]` scope output blocks
- `\` = Contains `[\]` setup blocks
- `/` = Contains `[/]` cleanup blocks
- `=` = Contains `[i]` constant blocks

**Examples:**
```polyglot
// Macro with scope flow and setup/cleanup
[<] Macro.include"{\/"

// Macro with only setup
[<] Macro.include"\"

// Macro with scope flow only
[<] Macro.include"{}"

// Macro with everything
[<] Macro.include"{}\=/="
```

**Compiler Validation:**
- At unwrap site, compiler verifies declared blocks match actual content
- Prevents macro misuse
- Enables type checking

---

## Multiple Macros

### FIFO Setup, LIFO Cleanup

When multiple macros unwrapped in same pipeline:

**Setup blocks (`[\]`):** Execute in FIFO order (first unwrapped, first setup)
**Cleanup blocks (`[/]`):** Execute in LIFO order (last unwrapped, first cleanup)

**Example:**
```polyglot
[|] ComplexPipeline
[W] |DatabaseSetup     // Setup 1st, Cleanup 3rd
[W] |CacheSetup        // Setup 2nd, Cleanup 2nd
[W] |LoggerSetup       // Setup 3rd, Cleanup 1st

// Execution order:
// 1. DatabaseSetup [\] blocks
// 2. CacheSetup [\] blocks
// 3. LoggerSetup [\] blocks
// ... pipeline body ...
// 4. LoggerSetup [/] blocks  (LIFO)
// 5. CacheSetup [/] blocks
// 6. DatabaseSetup [/] blocks
[X]
```

**Pattern:** RAII-like resource management
- Last acquired, first released
- Natural cleanup order
- Prevents resource leaks

---

## Type Safety

### Enforcement at Unwrap Site

**Type checking occurs at unwrap site, not definition:**

```polyglot
[M] Processor
[{] .input: pg\string    // Expects string
[}] .output: pg\int      // Produces int

[\] |Convert
[<] .in: pg\string << .input
[>] .out: pg\int >> .output
[X]

// Valid unwrap
[|] Pipeline1
[W] |Processor
[<] .input << "123"              // ✓ Correct type
[>] .output >> .result: pg\int   // ✓ Correct type
[X]

// Invalid unwrap (compile error)
[|] Pipeline2
[W] |Processor
[<] .input << 456                // ✗ Wrong type (int, not string)
[>] .output >> .result: pg\string // ✗ Wrong type (expects int)
[X]
```

---

### Variable Renaming

**Flexibility:** Variables can be renamed at unwrap site

```polyglot
[M] Validator
[{] .data: pg\string
[}] .is_valid: pg\bool

[\] |U.Validate
[<] .input: pg\string << .data
[>] .result: pg\bool >> .is_valid
[X]

// Different variable names at unwrap site
[|] CheckInput
[W] |Validator
[<] .data << .user_input        // Renamed from caller's perspective
[>] .is_valid >> .validation_result
[X]
```

---

## Complete Examples

### Example 1: Database Connection Management

```polyglot
[M] DatabaseSetup
[<] Macro.include"{\/"
[{] .db_host: pg\string
[{] .db_port: pg\int
[{] .db_name: pg\string
[}] .db_conn: pg\db

[\] |U.DB.Connect
[<] .host: pg\string << .db_host
[<] .port: pg\int << .db_port
[<] .database: pg\string << .db_name
[>] .connection: pg\db >> .db_conn

[/] |U.DB.Disconnect
[<] .conn: pg\db << .db_conn
[X]

// Usage
[|] ProcessUsers
[W] |DatabaseSetup
[<] .db_host << "localhost"
[<] .db_port << 5432
[<] .db_name << "users_db"
[>] .db_conn >> .db

[r] |QueryUsers
[<] .conn: pg\db << .db
[>] .users: pg\array{pg\string} >> .user_list
[X]
```

---

### Example 2: Logging with Error Handling

```polyglot
[M] Logger
[<] Macro.include"{\/"
[{] .log_level: pg\string
[{] .log_path: pg\path
[}] .log_handle: pg\file

[\] |U.Log.Open
[<] .level: pg\string << .log_level
[<] .path: pg\path << .log_path
[>] .handle: pg\file >> .log_handle

[!] !pg.FileSystem.PermissionDenied
[r] |U.Log.OpenFallback
[<] .fallback_path: pg\path << \\TempDir\\fallback.log
[>] .handle: pg\file >> .log_handle

[/] |U.Log.Close
[<] .handle: pg\file << .log_handle
[X]

// Usage
[|] Application
[W] |Logger
[<] .log_level << "INFO"
[<] .log_path << \\LogDir\\app.log
[>] .log_handle >> .logger

[r] |U.Log.Write
[<] .handle: pg\file << .logger
[<] .message: pg\string << "Application started"
[X]
```

---

### Example 3: Cache Management

```polyglot
[M] CacheSetup
[<] Macro.include"{\/"
[i] .cache_ttl: pg\int << 3600      // 1 hour constant
[{] .cache_size: pg\int
[}] .cache_handle: pg\db

[\] |U.Cache.Initialize
[<] .size: pg\int << .cache_size
[<] .ttl: pg\int << .cache_ttl
[>] .handle: pg\db >> .cache_handle

[/] |U.Cache.Shutdown
[<] .handle: pg\db << .cache_handle
[X]

// Usage
[|] CachedAPI
[W] |CacheSetup
[<] .cache_size << 1000
[>] .cache_handle >> .cache

[r] |U.Cache.Get
[<] .handle: pg\db << .cache
[<] .key: pg\string << "user:123"
[>] .value: pg\string >> .cached_value

[?] .cached_value =!? ""
[r] |FetchFromDatabase
[X]
```

---

### Example 4: Multiple Macros (RAII Pattern)

```polyglot
[|] CompleteWorkflow
// Setup order: DB → Cache → Logger
// Cleanup order: Logger → Cache → DB (LIFO)

[W] |DatabaseSetup
[<] .db_host << "localhost"
[<] .db_port << 5432
[<] .db_name << "main_db"
[>] .db_conn >> .db

[W] |CacheSetup
[<] .cache_size << 5000
[>] .cache_handle >> .cache

[W] |Logger
[<] .log_level << "DEBUG"
[<] .log_path << \\LogDir\\workflow.log
[>] .log_handle >> .logger

// Main workflow logic
[r] |ProcessData
[<] .db: pg\db << .db
[<] .cache: pg\db << .cache
[<] .logger: pg\file << .logger

// Automatic cleanup in LIFO order
[X]
```

---

## Best Practices

### 1. Explicit Block Declaration

**Always declare included blocks:**
```polyglot
// ✓ GOOD
[M] MyMacro
[<] Macro.include"{\/"
// ...
[X]

// ✗ BAD - Missing declaration
[M] MyMacro
// ...
[X]
```

---

### 2. Consistent Scope Flow

**Input/Output should be logically paired:**
```polyglot
// ✓ GOOD - Clear flow
[{] .config_path: pg\path    // IN
[}] .config: #Config         // OUT

// ✗ BAD - Confusing flow
[}] .result: pg\int          // OUT before IN?
[{] .input: pg\string        // IN after OUT?
```

---

### 3. Use Constants for Fixed Values

**Prefer `[i]` over `[{]` for immutable values:**
```polyglot
// ✓ GOOD
[i] .max_connections: pg\int << 100

// ✗ BAD - Mutable when should be constant
[{] .max_connections: pg\int
```

---

### 4. Match Setup/Cleanup Pairs

**Every `[\]` should have corresponding `[/]`:**
```polyglot
// ✓ GOOD - Paired
[\] |U.File.Open
[<] .path: pg\path << .file_path
[>] .handle: pg\file >> .file_handle

[/] |U.File.Close
[<] .handle: pg\file << .file_handle

// ✗ BAD - Setup without cleanup
[\] |U.File.Open
[<] .path: pg\path << .file_path
[>] .handle: pg\file >> .file_handle
// Missing [/] cleanup!
```

---

### 5. Descriptive Macro Names

**Use clear, descriptive names:**
```polyglot
// ✓ GOOD
[M] DatabaseConnectionSetup
[M] CacheInitialization
[M] LoggerWithRotation

// ✗ BAD - Vague names
[M] DB
[M] Setup
[M] Thing
```

---

### 6. Document Complex Macros

**Add comments for non-obvious behavior:**
```polyglot
[M] DistributedLock
[<] Macro.include"{\/"

// Acquires distributed lock across cluster
// Automatically releases on cleanup or error
// Timeout: 30 seconds

[{] .resource_id: pg\string
[{] .timeout_ms: pg\int
[}] .lock_handle: pg\db

[\] |U.Lock.Acquire
// ...
[X]
```

---

## See Also

- [Block Markers](06-block-markers.md) - Macro-specific blocks: `[M]`, `[W]`, `[{]`, `[}]`, `[i]`
- [Operators](05-operators.md) - Assignment operators used in macros
- [Type System](02-type-system.md) - Type safety in macro scope flow

---

**Session Document:** `docs/project/agent-sessions/carson-2025-11-18-macro-system-spec.md`
**Design Decisions:** 9 major decisions documented
**Status:** Production-ready specification

---

**End of Macro System Documentation**
