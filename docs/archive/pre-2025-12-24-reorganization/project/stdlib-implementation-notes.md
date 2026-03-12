# Standard Library Implementation Notes

**Version:** 0.0.2
**Date:** 2025-11-22
**Status:** Planning - Implementation After Lexer/Compiler

---

## Purpose

This document tracks standard library operations and string literal pipelines discovered during syntax validation that need to be implemented AFTER the lexer and Polyglot compiler are complete.

**Implementation Priority:** Post-Epic 2 (after parser is working)

---

## String Literal Pipelines

### String Operations
These use the pattern: `Operation"{value}"` or `Operation"{.variable}"`

**Discovered in validation examples:**
- `String.Upper"{.value}"` - Convert to uppercase
- `String.Lower"{.value}"` - Convert to lowercase (inferred)
- `String.Trim"{.value}"` - Trim whitespace (mentioned in anti-patterns)
- `String.Length"{.value}"` - Get string length (mentioned in anti-patterns)

**Implementation Notes:**
- These are NOT method calls - they're string literal syntax
- The compiler should recognize the pattern `Identifier"{...}"`
- May need special handling in lexer for quote after identifier

---

## Standard Library Pipelines (`|U.*`)

### File Operations (`|U.File.*`)

**From validation examples:**
- `|U.File.Text.Read` - Read text file
  - Input: `.path: pg\path`
  - Output: `.content: pg\string`
  - Errors: `!pg.FileSystem.NotFound`

- `|U.File.Text.AppendLine` - Append line to file
  - Input: `.file: pg\path`, `.line: pg\string`
  - Output: `!No.Output`

**Needed but not yet seen:**
- `|U.File.Text.Write` - Write text file (inferred)
- `|U.File.Binary.Read` - Read binary file (inferred)
- `|U.File.Binary.Write` - Write binary file (inferred)

### CSV Operations (`|U.CSV.*`)

**From validation examples:**
- `|U.CSV.To.Rows` - Parse CSV to rows
  - Input: `.file: pg\path`
  - Output: `.rows: pg\array{pg\string}`

**Needed but not yet seen:**
- `|U.CSV.From.Rows` - Write rows to CSV (inferred)
- `|U.CSV.To.Dict` - Parse CSV with headers (inferred)

### Database Operations (`|U.DB.*`)

**From validation examples:**
- `|U.DB.Query` - Execute SQL query
  - Input: `.sql: pg\string` (uses `sql"..."` literal)
  - Output: `.data: pg\array{pg\serial}`

- `|U.DB.TargetSchema.Fetch` - Fetch table schema
  - Input: `.table: pg\string`
  - Output: `.schema: pg\serial`

**Needed but not yet seen:**
- `|U.DB.Execute` - Execute non-query SQL (inferred)
- `|U.DB.Transaction` - Transaction support (inferred)

### Logging Operations (`|U.Log.*`)

**From validation examples:**
- `|U.Log.Error` - Log error message
  - Input: `.msg: pg\string`
  - Output: `!No.Output`

**Needed but not yet seen:**
- `|U.Log.Info` - Log info message (inferred)
- `|U.Log.Warn` - Log warning message (inferred)
- `|U.Log.Debug` - Log debug message (inferred)

---

## Special Syntax Patterns

### SQL String Literals

**Pattern:** `sql"SELECT * FROM {.table}"`

**From validation examples:**
```polyglot
[<] .sql: pg\string << sql"SELECT * FROM {.source_table}"
```

**Implementation Notes:**
- Special prefix `sql` before string literal
- Enables SQL-specific syntax highlighting
- Supports variable interpolation `{.variable}`
- Should validate SQL syntax at compile time (future)

### Cron Trigger Literals

**Pattern:** `T.Cron"schedule"`

**From validation examples:**
```polyglot
[t] T.Cron"0 2 * * *"
```

**Implementation Notes:**
- NO pipe `|` before trigger name when using string literal
- Pattern: `[t] T.Type"string_value"`
- Should validate cron syntax at compile time (future)

---

## ForEach Operation

**Pattern:** `~ForEach` with explicit input/output

**From validation examples:**
```polyglot
[r] ~ForEach
[<] .my_array
[>] .current_item
[~][r] .processed: pg\string << String.Upper"{.current_item}"
[~][o] !No.Output
```

**Implementation Notes:**
- `~ForEach` is a built-in operation (NOT `|U.ForEach`)
- Takes collection as input via `[<]`
- Outputs current item variable via `[>]`
- Operations inside loop use `[~]` prefix
- Each iteration returns via `[~][o]`

**Questions for implementation:**
- Is `ForEach` a reserved word or a standard library operation?
- How does the iteration state get managed?
- Is there a `~ForEachIndexed` variant with index?

---

## Macros (`[W]` Unwrap)

### No-Op Setup/Cleanup

**From validation examples:**
```polyglot
[W] |W.Polyglot.Scope
```

**Implementation Notes:**
- Macro that provides no-op `[\]` setup and `[/]` cleanup blocks
- Satisfies canonical pipeline requirements when no setup/cleanup needed
- Should be in standard macro library

**Alternative names seen:**
- `W.NoSetup.NoCleanp` (typo in example - should be `NoCleanup`)

---

## Package Management

### Reserved Enumeration Libraries

**From validation examples:**
```polyglot
[<] .queue: pg\string << @Queuelib#Queues.Background
```

**Pattern:** `@PackageAlias#Enumeration.Variant`

**Implementation Notes:**
- `@Queuelib` is imported package providing `#Queues` enumeration
- Standard packages should provide common enumerations
- Need to define standard package namespaces

**Potential Standard Packages:**
- `@Queuelib` - Queue management enumerations
- `@PathLib` - Path identifier enumerations
- `@StatusLib` - Status code enumerations
- `@ErrorLib` - Common error types

---

## Error Types

### File System Errors

**From validation examples:**
- `!pg.FileSystem.NotFound` - File not found
- `!pg.FileSystem.PermissionDenied` - Permission denied (inferred from hierarchy doc)

### Standard Errors

**From validation examples:**
- `!No.Output` - No error (success case)

**Needed but not yet seen:**
- `!pg.Network.ConnectionFailed` - Network errors (inferred)
- `!pg.Database.QueryFailed` - Database errors (inferred)
- `!pg.Serialization.ParseError` - Parsing errors (inferred)

---

## Implementation Checklist

### Phase 1: Core Syntax (Epic 1-2)
- [ ] Lexer recognizes block markers
- [ ] Parser handles block hierarchy
- [ ] Compiler generates IR for pipelines

### Phase 2: String Literal Pipelines (Epic 3-4)
- [ ] Lexer recognizes `Identifier"{value}"` pattern
- [ ] Parser handles string literal pipeline syntax
- [ ] Implement `String.Upper`, `String.Lower`, `String.Trim`, `String.Length`

### Phase 3: Standard Library Operations (Epic 5-6)
- [ ] `|U.File.*` - File operations
- [ ] `|U.CSV.*` - CSV parsing
- [ ] `|U.DB.*` - Database operations
- [ ] `|U.Log.*` - Logging operations

### Phase 4: Special Literals (Epic 7-8)
- [ ] `sql"..."` - SQL string literals
- [ ] `T.Cron"..."` - Cron trigger literals
- [ ] Syntax validation for special literals

### Phase 5: ForEach & Iteration (Epic 9)
- [ ] `~ForEach` operation
- [ ] Iteration state management
- [ ] Loop variable scoping

### Phase 6: Standard Macros (Epic 10)
- [ ] `W.NoSetup.NoCleanup` macro
- [ ] Other common setup/cleanup patterns

### Phase 7: Standard Packages (Epic 11-12)
- [ ] `@Queuelib` - Queue enumerations
- [ ] `@PathLib` - Path identifiers
- [ ] `@ErrorLib` - Common errors

---

## Questions for Clarification

### ForEach Implementation
1. Is `~ForEach` a built-in primitive or standard library?
2. How does iteration state get managed between iterations?
3. Is there support for `~ForEachIndexed` with index access?
4. Can you break/continue from loops?

### String Literal Pipelines
1. Are these compile-time or runtime operations?
2. Should they support method chaining: `String.Upper.Trim"{.value}"`?
3. Are there numeric literal pipelines: `Int.Parse"{.value}"`?

### Error Handling
1. What's the complete set of reserved error types (`!pg.*`)?
2. Can users extend `!pg.*` namespace or only create custom errors?
3. Is there error wrapping/chaining support?

### SQL Literals
1. Should SQL syntax be validated at compile time?
2. Are there other special literal types (regex, json, yaml)?
3. How does SQL parameterization work for injection prevention?

---

## References

- Validation examples: See `/docs/user/common-mistakes-antipatterns.md`
- User corrections: See conversation 2025-11-22
- Standard library catalog: `/docs/user/standard-library/utilities-catalog.md`

---

**Status:** 📋 Planning Document - Review Before Epic 3+
