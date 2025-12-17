---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/audit/reserved-enumeration-schema-decisions.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Reserved Enumeration Schema Decisions

**Purpose:** Document schemas for all Polyglot reserved enumerations and determine which are extendable
**Created:** 2025-11-11
**Status:** PENDING USER INPUT
**Related:** decision-log.md (Decision #1: Reserved Enumerations, Pending #5)

---

## Overview

This document catalogs all reserved enumerations in Polyglot, organized by context. For each enumeration, we document:
- **Schema:** The required keys/fields and their types
- **Extendable:** Whether users can extend this enumeration
- **Usage:** When and how to use this enumeration

---

## Path System Enumerations

### `#Path.Identifiers.*`
**Status:** ✓ SCHEMA CONFIRMED
**Extendable:** YES
**Related Decision:** decision-log.md Issue #14

**Schema:**
```polyglot
[#] Path.Identifiers.{CustomName}
[<] .unix:pg.path      // REQUIRED - Path for Unix/Linux/macOS
[<] .windows:pg.path   // REQUIRED - Path for Windows
[X]
```

**Usage Example:**
```polyglot
[#] Path.Identifiers.MyApp.DataDir
[A] DataDir
[<] .unix:pg.path << \\UnixRoot\\opt\myapp\data\
[<] .windows:pg.path << \\C\\ProgramData\MyApp\Data\
[X]

// Use the path identifier
[r] .config_file:pg.path << \\DataDir\\config.json
```

**Notes:**
- Users extend by creating new path identifiers with `.unix` and `.windows` fields
- Use `\\NoPath\\` when a path doesn't exist on a particular OS
- Both fields are REQUIRED even if one uses `\\NoPath\\`

---

### Built-in Path Root Identifiers
**Status:** ✓ SCHEMA CONFIRMED
**Extendable:** NO (System-provided roots)

These are non-extendable path roots provided by Polyglot for cross-platform path construction:

**Schema:**
```polyglot
\\UnixRoot\\          // Root for Unix/Linux/macOS paths (/)
\\WindowsRoot\\       // Root for Windows paths (typically C:\)
\\NoPath\\            // Sentinel value when path doesn't exist on OS
```

**Usage:**
```polyglot
// Define cross-platform path identifier using roots
[#] Path.Identifiers.MyApp.ConfigDir
[<] .unix:pg.path << \\UnixRoot\\etc\myapp\
[<] .windows:pg.path << \\C\\ProgramData\MyApp\Config\
[X]

// OS where path doesn't exist
[#] Path.Identifiers.MyApp.WindowsOnlyPath
[<] .unix:pg.path << \\NoPath\\
[<] .windows:pg.path << \\C\\Windows\System32\myapp\
[X]
```

**Notes:**
- These are **literal path roots**, NOT enumerations
- Cannot be extended or modified by users
- Part of Polyglot path literal syntax
- Used in path identifier definitions for cross-platform support

---

### Drive Letter Path Identifiers (Windows)
**Status:** ✓ SCHEMA CONFIRMED
**Extendable:** NO (System-provided)

Windows drive letters are referenced using backslash literal syntax:

**Schema:**
```polyglot
\\C\\          // Windows C: drive
\\D\\          // Windows D: drive
\\E\\          // Windows E: drive
// ... etc for all drive letters
```

**Usage:**
```polyglot
[#] Path.Identifiers.DataDrive
[<] .unix:pg.path << \\UnixRoot\\mnt\data\
[<] .windows:pg.path << \\D\\Data\
[X]
```

**Notes:**
- Drive letters are literal syntax, not enumerations
- Used primarily in Windows-specific paths within path identifiers
- Unix systems ignore drive letter syntax (use `\\NoPath\\` for windows field if needed)

---

## Queue System Enumerations

### `#Queues.*`
**Status:** ⚠ SCHEMA PENDING USER INPUT
**Extendable:** YES (assumed based on Decision #22)
**Related Decision:** decision-log.md Issue #22

**Current Usage (from Decision #22):**
```polyglot
[#] Queues.Background
[<] .max_concurrent:pg.int << 5
[X]
```

**Questions for User:**
1. Is `.max_concurrent` the ONLY required field?
2. Are there other optional or required fields such as:
   - `.priority:pg.int` - Default priority level for queue?
   - `.timeout:pg.dt` - Timeout duration for queued pipelines?
   - `.description:pg.string` - Human-readable description?
   - `.max_wait_time:pg.dt` - Maximum wait time before dispatch?
   - Other fields?
3. What are the complete semantics of `.max_concurrent`?

**Proposed Schema (NEEDS CONFIRMATION):**
```polyglot
[#] Queues.{CustomQueueName}
[<] .max_concurrent:pg.int << value   // REQUIRED? - Max concurrent executions
// [<] .priority:pg.int << value      // OPTIONAL? - Default priority
// [<] .timeout:pg.dt << value         // OPTIONAL? - Execution timeout
// [<] .description:pg.string << ""    // OPTIONAL? - Queue description
[X]
```

**Built-in Non-Extendable System Queues:**
- `#Queues.Pending` - System queue for pending pipelines
- `#Queues.Dispatch` - System queue for executing pipelines
- `#Queues.Pause` - System queue for paused pipelines

---

### Queue Control Pipelines (`|Q.*`)
**Status:** ⚠ CATALOG PENDING - DEFERRED TO FUTURE VERSION
**Extendable:** NO (System-provided operations)
**Related Decision:** To be documented in future documentation version

**Current Known Operations:**
```polyglot
[Q] |Q.Pause                                    // Pause instance
[Q] |Q.Resume                                   // Resume instance
[Q] |Q.Kill                                     // Kill instance
[Q] |Q.PriorityBump                             // Increase priority
[Q] |Q.Queue.Assign                             // Assign to queue
[Q] |Q.Status                                   // Get status
[Q] |Q.PauseIf.RAM.Available.LessThan          // Conditional pause
```

**Decision:**
The complete catalog of `|Q.*` pipeline operations is **not yet finalized**. This will be documented in a future version of the documentation once the queue system design is complete.

**Current Status:**
- Basic queue operations are known and in use (Pause, Resume, Kill, etc.)
- Conditional queue operations (`|Q.PauseIf.*`, `|Q.DispatchIf.*`) exist but complete catalog is TBD
- Full specification of all `|Q.*` pipelines deferred to later documentation iteration

**Action Items for Future:**
- [ ] Define complete `|Q.*` pipeline catalog
- [ ] Document input/output schemas for each queue operation
- [ ] Clarify conditional vs direct queue operations
- [ ] Document queue lifecycle and state transitions
- [ ] Provide comprehensive usage examples

**Notes:**
- Queue control pipelines use `[Q]` block marker
- These are **operations**, not enumerations
- Cannot be extended by users - system-provided only
- Used for managing pipeline execution flow and queue assignment

---

## Date/Time System Enumerations

### `#DT.*` Family
**Status:** ⚠ SCHEMAS PENDING USER INPUT
**Context:** Various date/time related enumerations for the DT system

---

### `#DT.Business.Week.*`
**Status:** ⚠ SCHEMA PENDING USER INPUT
**Extendable:** YES (assumed)
**Related Decision:** decision-log.md Decision #3 (DateTime System), Pending #5

**Questions for User:**
1. What fields are required for business week definitions?
2. Possible fields might include:
   - `.start_day:pg.string` - Which day starts the business week? (e.g., "Monday")
   - `.work_days:pg.int` - Number of working days? (e.g., 5)
   - `.holidays:pg.array{pg\dt}` - List of holiday dates?
   - `.start_time:pg.dt` - Daily business start time?
   - `.end_time:pg.dt` - Daily business end time?
3. What are common use cases for business week definitions?

**Proposed Schema (NEEDS CONFIRMATION):**
```polyglot
[#] DT.Business.Week.{CustomBusinessWeek}
// [<] .start_day:pg.string << "Monday"          // REQUIRED? - Week start day
// [<] .work_days:pg.int << 5                     // REQUIRED? - Working days per week
// [<] .start_time:pg.dt << |DT"09:00:"           // OPTIONAL? - Business hours start
// [<] .end_time:pg.dt << |DT"17:00:"             // OPTIONAL? - Business hours end
// [<] .holidays:pg.array{pg\dt} << array{}      // OPTIONAL? - Holiday dates
[X]
```

**Example Use Case (HYPOTHETICAL):**
```polyglot
[#] DT.Business.Week.Standard
[<] .start_day:pg.string << "Monday"
[<] .work_days:pg.int << 5
[<] .start_time:pg.dt << |DT"09:00:"
[<] .end_time:pg.dt << |DT"17:00:"
[X]

// Use in trigger
[t] |T.Every.BusinessDay
[<] .week_definition: #DT.Business.Week << #DT.Business.Week.Standard
```

---

### Other `#DT.*` Enumerations
**Status:** ⚠ DISCOVERY NEEDED

**Questions for User:**
1. Are there other `#DT.*` enumerations beyond `#DT.Business.Week.*`?
2. What are their schemas?
3. Are they extendable or non-extendable?

**Possible Additional DT Enumerations:**
- `#DT.Timezone.*` - Custom timezone definitions?
- `#DT.Calendar.*` - Custom calendar systems (beyond built-in Hijri, Chinese, etc.)?
- `#DT.Schedule.*` - Complex scheduling patterns?
- Other?

---

## Status System Enumerations

### `#Status.*`
**Status:** ⚠ SCHEMA PENDING USER INPUT
**Extendable:** NO (assumed non-extendable based on Decision #1)
**Related Decision:** decision-log.md Pending #1

**Questions for User:**
1. What are the complete keys/values in `#Status.*`?
2. Is this a simple set of status values or does each status have fields?
3. Common status values might include:
   - `#Status.Success`
   - `#Status.Failed`
   - `#Status.Pending`
   - `#Status.Running`
   - `#Status.Cancelled`
   - Others?

**Proposed Structure (NEEDS CONFIRMATION):**

**Option A: Simple Status Values**
```polyglot
// Built-in, non-extendable
#Status.Success
#Status.Failed
#Status.Pending
#Status.Running
#Status.Cancelled
// etc.
```

**Option B: Status with Fields**
```polyglot
// Each status has fields like .code, .message
#Status.Success.code:pg.int << 0
#Status.Success.message:pg.string << "Operation completed successfully"
// etc.
```

**Usage Example (HYPOTHETICAL):**
```polyglot
[r] .pipeline_status: #Status << #Status.Success

[?] .pipeline_status =? #Status.Success
[~][r] |U.Log.Info
[~][<] .msg:pg.string << "Pipeline completed"
```

---

## Error System Enumerations

### `#Errors.*`
**Status:** ✓ DEPRECATED/REPLACED
**Extendable:** N/A (replaced by `!Error` syntax)
**Related Decision:** decision-log.md Decision #13, Pending #3, Pending #5

**Decision:**
`#Errors.*` has been completely replaced by the `!Error` syntax in v0.0.2.

**Migration Path:**
- Old v0.0.1: `#Errors.SomeError`
- New v0.0.2: `!Package.Context.SomeError`

**Define Custom Errors:**
```polyglot
[!] !MyApp.CustomError
[<] .message:pg.string << "Error message"
[<] .code:pg.int << 5000
[<] .trace:pg.string << ""
[X]
```

**Notes:**
- All error handling now uses `!Error` types (Decision #13)
- `#Errors.*` should be documented as REMOVED in v0.0.2
- No migration code needed - syntax is completely different

---

## Comparison System Enumerations

### `#Comparison`
**Status:** ✓ SCHEMA CONFIRMED
**Extendable:** NO (System-provided)
**Related Decision:** Comparison operators replacement

**Purpose:** Provides standardized comparison result values for use with range matching operators.

**Schema:**
```polyglot
// Built-in, non-extendable comparison values
#Comparison.Less              // First value is less than second
#Comparison.Equal             // Values are equal
#Comparison.Greater           // First value is greater than second
#Comparison.LessOrEqual       // First value is less than or equal to second
#Comparison.GreaterOrEqual    // First value is greater than or equal to second
```

**Usage with Type-Aware Comparison Operators:**

Polyglot provides type-aware comparison operators (`=?`, `>?`, `<?`, `>=?`, `<=?`) for direct comparisons:

```polyglot
// Direct comparison with operators
[r] .reference:pg.dt << |DT"2024-01-15"

// Check if date is after reference (greater than)
[?] .some_date >? .reference
[~][r] |HandleAfterDate

// Check if date is before reference (less than)
[?] .some_date <? .reference
[~][r] |HandleBeforeDate

// Check if date equals reference
[?] .some_date =? .reference
[~][r] |HandleExactDate

// Range comparison (between dates)
[?] .some_date ?[|DT"2024-01-01", |DT"2024-12-31"]
[~][r] |HandleDateInRange
```

**Explicit Comparison Pipeline Pattern:**

For complex comparisons, use comparison pipelines that return `#Comparison` values:

```polyglot
[r] |U.Compare.DateTime
[<] .first:pg.dt << .date1
[<] .second:pg.dt << .date2
[>] .result: #Comparison >> .comparison

[?] .comparison =? #Comparison.Less
[~][r] |HandleEarlierDate

[?] .comparison =? #Comparison.Equal
[~][r] |HandleSameDate

[?] .comparison =? #Comparison.Greater
[~][r] |HandleLaterDate
```

**Notes:**
- **v0.0.2 Update:** Range notation (`..`, `...`) has been replaced with bracket/paren range operators (`?[`, `?(`)
- **v0.0.2 Update:** Match operator (`?>`) has been replaced with comparison operators (`=?`, `>?`, `<?`, etc.)
- `#Comparison` enum is for explicit comparison pipeline results from utility pipelines
- System-provided, cannot be extended
- Used primarily with utility comparison pipelines

---

## Boolean System Enumeration

### `#Boolean`
**Status:** ✓ SCHEMA CONFIRMED
**Extendable:** NO (System-provided)
**Related Decision:** Keyword Elimination (2025-11-16) - Replaced True/False keywords
**Keyword Replacement:** This enumeration replaces the `True` and `False` keywords

**Purpose:** Polyglot's fundamental boolean type. All boolean values use the `#Boolean` reserved enumeration instead of keywords. Supports exhaustive pattern matching for type safety.

**Schema:**
```polyglot
// Built-in boolean enumeration - exactly two variants, no fields
#Boolean.True      // Boolean true value
#Boolean.False     // Boolean false value
```

**Aliases:**
```polyglot
// Shorthand aliases for convenience
#True    → #Boolean.True
#False   → #Boolean.False
```

**Usage Patterns:**

**1. Variable Assignment:**
```polyglot
// Using full form
[r] .is_active: #Boolean << #Boolean.True
[r] .is_valid: #Boolean << #Boolean.False

// Using aliases (more common)
[r] .is_active: #Boolean << #True
[r] .is_valid: #Boolean << #False
```

**2. Conditional Switching (Exhaustive Matching Required):**
```polyglot
[|] ProcessFlag
[i] .flag: #Boolean
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Exhaustive boolean matching - BOTH branches required
[?] .flag =? #True
[~][r] |HandleTrue
[~][o] .result:pg.string << "Handled true"

[?] .flag =? #False
[~][r] |HandleFalse
[~][o] .result:pg.string << "Handled false"

[X]
```

**3. Input Parameters with Boolean Type:**
```polyglot
[|] ToggleFeature
[i] .enable: #Boolean      // Boolean input parameter
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] .enable =? #True
[~][r] |EnableFeature

[?] .enable =? #False
[~][r] |DisableFeature

[o] #None
[X]
```

**4. Pipeline Outputs:**
```polyglot
[|] ValidateData
[i] .data:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] .data =? ""
[~][o] .is_valid: #Boolean << #False

[?] .data =!? ""
[~][o] .is_valid: #Boolean << #True

[X]
```

**Exhaustive Matching Requirements:**

**Compiler/Linter Rules:**
1. **Exhaustive Boolean Switch:** When switching on a `#Boolean` typed variable, BOTH `#True` and `#False` branches MUST be present
2. **Type Safety:** Cannot assign non-boolean values to `#Boolean` typed variables
3. **No Extension:** Users cannot add new variants to `#Boolean` (e.g., no `#Maybe` variant)
4. **Alias Consistency:** `#True` and `#False` always resolve to `#Boolean.True` and `#Boolean.False`

**Compiler Error Examples:**
```polyglot
// ❌ ERROR: Non-exhaustive boolean switch
[?] .flag =? #True
[~][r] |HandleTrue
// Missing #False branch - COMPILE ERROR

// ❌ ERROR: Type mismatch
[r] .flag: #Boolean << "true"   // Cannot assign string to boolean

// ❌ ERROR: Invalid variant
[r] .flag: #Boolean << #Boolean.Maybe   // No such variant
```

**Comparison with Old Keyword Syntax:**

| v0.0.1 (Keywords) | v0.0.2 (Reserved Enumeration) |
|-------------------|-------------------------------|
| `True` | `#Boolean.True` or `#True` |
| `False` | `#Boolean.False` or `#False` |
| `[r] .flag:pg.bool << #True` | `[r] .flag: #Boolean << #True` |
| `[?] .flag ?> True` | `[?] .flag =? #True` |

**Type System Integration:**

- **Type:** `#Boolean` is both a type and an enumeration
- **Values:** Only `#Boolean.True` and `#Boolean.False` (or their aliases)
- **Serialization:** Serialize as `#Boolean.True` or `#Boolean.False` (full form preferred)
- **Validation:** Runtime validates that boolean variables only contain valid variants
- **Pattern Matching:** Exhaustive matching enforced at compile-time

**Standard Library Integration:**

Boolean operations are provided through utility pipelines:

```polyglot
// Boolean NOT operation
[r] |U.Boolean.Not
[<] .value: #Boolean << #True
[>] .result: #Boolean >> .negated     // → #False

// Boolean AND operation
[r] |U.Boolean.And
[<] .a: #Boolean << #True
[<] .b: #Boolean << #False
[>] .result: #Boolean >> .and_result  // → #False

// Boolean OR operation
[r] |U.Boolean.Or
[<] .a: #Boolean << #True
[<] .b: #Boolean << #False
[>] .result: #Boolean >> .or_result   // → #True

// Boolean XOR operation
[r] |U.Boolean.Xor
[<] .a: #Boolean << #True
[<] .b: #Boolean << #True
[>] .result: #Boolean >> .xor_result  // → #False
```

**Important Design Rationale:**

1. **Why Enumeration Instead of Keyword:**
   - Maintains Polyglot's zero-keyword philosophy
   - Enables exhaustive pattern matching
   - Consistent with other special values (`#None`, `#Status.*`, etc.)
   - Type-safe at compile-time

2. **Why Exhaustive Matching:**
   - Prevents logic errors from missing cases
   - Forces developers to handle both true and false conditions
   - Makes code more maintainable and explicit

3. **Why Aliases:**
   - `#True`/`#False` are more concise than `#Boolean.True`/`#Boolean.False`
   - Common enough to warrant shorthand
   - Still maintain namespace clarity (`#` prefix indicates enumeration)

**Semantics:**

- `#Boolean` is a **closed enumeration** - no extension allowed
- Exactly two variants: `.True` and `.False`
- No additional variants can be added (enforced by compiler)
- Cannot have fields or additional data
- Immutable singleton values
- Must use comparison operators (`=?`, `=!?`) not match operator (old `?>` removed)

**Migration from v0.0.1:**
- Replace all `True` → `#True` (or `#Boolean.True`)
- Replace all `False` → `#False` (or `#Boolean.False`)
- Replace `[?] .var ?> True` → `[?] .var =? #True`
- Change type from `:pg.bool` to `#Boolean` (if `:pg.bool` existed in v0.0.1)

---

## Special Value Enumerations

### `#None`
**Status:** ✓ SCHEMA CONFIRMED
**Extendable:** NO (System-provided)
**Related Decision:** Decision #30 (Hello World examples)

**Purpose:** Represents "no value" or absence of a value. Used primarily in pipeline output declarations when a pipeline returns no data.

**Schema:**
```polyglot
// Built-in singleton value - no fields
#None
```

**Primary Usage - Pipeline Output Declaration:**

When a pipeline performs actions but doesn't return any data:

```polyglot
[|] LogMessage
[i] .message:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] |U.Log.Info
[<] .msg:pg.string << .message

[o] #None  // ← Pipeline returns nothing
[X]
```

**Usage Patterns:**

1. **Side-Effect Pipelines** (logging, notifications, etc.):
```polyglot
[|] SendEmail
[i] .to:pg.string
[i] .subject:pg.string
[i] .body:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] |Email.Send
[<] .to:pg.string << .to
[<] .subject:pg.string << .subject
[<] .body:pg.string << .body

[o] #None  // No return value needed
[X]
```

2. **Conditional Branches with No Output:**
```polyglot
[|] ProcessData
[i] .should_process: #Boolean
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] .should_process =? #True
[~][r] |DoProcessing
[~][o] .result:pg.string

[?] .should_process =? #False
[~][o] #None  // This branch returns nothing

[X]
```

3. **Scheduled Tasks:**
```polyglot
[|] DailyBackup
[t] |T.Daily
[<] .time:pg.dt << |DT"02:00:"
[W] |W.NoSetup.NoCleanup

[r] |RunBackup

[o] #None  // Task completes but returns nothing
[X]
```

**Semantics:**

- `#None` is a **special singleton value**, not a type
- Cannot be extended or have fields
- Represents intentional absence of output
- Different from error states (errors use `!Error` types)
- Different from empty data structures (`:pg.serial` with no keys, empty arrays, etc.)

**Comparison with Other "Empty" Values:**

| Concept | Representation | Meaning |
|---------|---------------|---------|
| No output | `[o] #None` | Pipeline intentionally returns nothing |
| Empty string | `[o] .text:pg.string << ""` | Pipeline returns empty string |
| Empty array | `[o] .items: pg.array.pg.int << array{}` | Pipeline returns array with no elements |
| Empty serial | `[o] .data:pg.serial << serial{}` | Pipeline returns object with no keys |

**Notes:**
- `#None` is ONLY used in output declarations `[o]`
- Cannot assign `#None` to variables or use as a type
- If a pipeline output is `#None`, calling pipelines receive nothing
- System-provided singleton - users cannot create similar values

---

## Error Types (`!Error`)

### `!` (Generic Error Type)
**Status:** ✓ SCHEMA CONFIRMED
**Extendable:** YES (Users define custom error types)
**Related Decision:** Error syntax clarification

**Purpose:** Represents errors in Polyglot. The generic error type `!` is used for error variables, while specific error types like `!ValidationError` are defined by users or provided by the system.

**Core Mechanism:**

1. **Generic Error Type:** `!` (exclamation mark alone)
   - Used as variable type for error handling
   - Example: `[r] .error: ! << !ValidationError`

2. **Specific Error Types:** `!ErrorName`
   - User-defined or system-provided error types
   - Must have three required fields: `.message`, `.code`, `.trace`
   - Can have additional custom fields

**Schema:**

```polyglot
// Generic error type (used for variables)
!

// Specific error type definition
[!] !MyApp.ValidationError
[<] .message:pg.string << "Default validation error"
[<] .code:pg.int << 1001
[<] .trace:pg.string << ""
// Optional custom fields
[<] .field_name:pg.string << ""
[X]
```

**Required Fields (All Error Types):**
- `.message:pg.string` - Human-readable error message
- `.code:pg.int` - Numeric error code
- `.trace:pg.string` - Stack trace or empty string

**Error Raising Syntax:**

```polyglot
[|] PipelineThatCanFail
[i] .input:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] .input =? ""
[~]
[~][r] .error: ! << !ValidationError
[~][<] .message:pg.string << "Input cannot be empty"
[~][<] .code:pg.int << 1001
[~][<] .trace:pg.string << ""
[~][o] .error: !

[?] .input =!? ""
[~]
[~][r] .result:pg.string << "Success"
[~][o] .result:pg.string

[X]
```

**Error Catching Syntax:**

```polyglot
[|] SafePipeline
[i] .input:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] |PipelineThatCanFail
[<] .input:pg.string << .input
[>] .result:pg.string >> .success_result
[~]
[~][!] !ValidationError
[~][>] .message:pg.string >> .err_msg
[~][>] .code:pg.int >> .err_code
[~]
[~][r] .success_result:pg.string << "Default value"

[o] .output:pg.string << .success_result
[X]
```

**Built-in Error Types (System-Provided):**

```polyglot
// File system errors
!pg.FileSystem.NotFound
!pg.FileSystem.PermissionDenied
!pg.FileSystem.DiskFull
!pg.FileSystem.AlreadyExists

// Network errors
!pg.Network.Timeout
!pg.Network.ConnectionRefused
!pg.Network.ConnectionLost
!pg.Network.DNSResolutionFailed

// Validation errors
!pg.Validation.TypeMismatch
!pg.Validation.OutOfRange
!pg.Validation.InvalidFormat
!pg.Validation.MissingRequired

// Runtime errors
!pg.Runtime.MemoryExhausted
!pg.Runtime.StackOverflow
!pg.Runtime.Timeout
!pg.Runtime.Deadlock
```

**User-Defined Error Types:**

```polyglot
// Application-specific validation error
[!] !MyApp.ValidationError
[<] .message:pg.string << "Validation failed"
[<] .code:pg.int << 1001
[<] .trace:pg.string << ""
[<] .field:pg.string << ""
[X]

// Business logic error
[!] !MyApp.InsufficientFunds
[<] .message:pg.string << "Insufficient funds"
[<] .code:pg.int << 2001
[<] .trace:pg.string << ""
[<] .required_amount:pg.float << 0.0
[<] .available_amount:pg.float << 0.0
[X]

// Integration error
[!] !MyApp.API.RateLimitExceeded
[<] .message:pg.string << "API rate limit exceeded"
[<] .code:pg.int << 3001
[<] .trace:pg.string << ""
[<] .retry_after:pg.dt << |DT"00:00:"
[X]
```

**Error Propagation:**

Errors automatically propagate up the call stack if not caught:

```polyglot
[|] Level3
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .error: ! << !SomeError
[<] .message:pg.string << "Error at level 3"
[<] .code:pg.int << 3000
[<] .trace:pg.string << ""
[o] .error: !
[X]

[|] Level2
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] |Level3
// No error catch - error propagates up
[X]

[|] Level1
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] |Level2
[~]
[~][!] !SomeError  // Catches error from Level3
[~][>] .message:pg.string >> .err_msg
[~][r] |HandleError
[X]
```

**Key Semantics:**

1. **Generic Type `!`**: Used for error variables
   - `[r] .error: !` declares a variable that can hold any error type
   - Cannot be used directly in type declarations (use specific error types)

2. **Specific Error Types**: Define concrete error types
   - `!ErrorType` references a specific error
   - Must be defined with `[!] !ErrorType` before use
   - Naming convention: Use namespace dotted notation (e.g., `!MyApp.Module.ErrorName`)

3. **Implicit Error Output**: All pipelines have an implicit error output
   - Defaults to `!NoError` on success (never explicitly checked or assigned)
   - Explicitly outputting an error with `[o] .error: !` raises it

4. **Error Matching**: Errors are caught by specific type
   - `[!] !ValidationError` catches only `!ValidationError`
   - Parent pipelines can catch errors from child pipelines
   - If not caught, error propagates up the call stack

**Notes:**
- `!` is the ONLY type operator that has dual usage (generic type and type marker)
- All error types must have `.message`, `.code`, and `.trace` fields
- Custom fields are allowed and encouraged for context
- Error types are **NOT** enumerations - they use `!` operator, not `#`
- System-provided error types use `!pg.*` namespace
- User error types should use application namespace (e.g., `!MyApp.*`)

---

## Complete Catalog of Reserved Enumerations

**Status Summary:**

| Enumeration/Type | Context | Extendable | Schema Status |
|------------------|---------|------------|---------------|
| `#Boolean` | Boolean System | ✗ NO | ✓ CONFIRMED |
| `#Path.Identifiers.*` | Path System | ✓ YES | ✓ CONFIRMED |
| `#Queues.*` | Queue System | ✓ YES | ⚠ PENDING |
| `#Queues.Pending` | Queue System | ✗ NO | ⚠ PENDING |
| `#Queues.Dispatch` | Queue System | ✗ NO | ⚠ PENDING |
| `#Queues.Pause` | Queue System | ✗ NO | ⚠ PENDING |
| `#DT.Business.Week.*` | Date/Time | ✓ YES | ⚠ PENDING |
| `#DT.*` (other) | Date/Time | ? | ⚠ DISCOVERY NEEDED |
| `#Status.*` | Status Values | ✗ NO | ⚠ PENDING |
| `#Comparison` | Comparison System | ✗ NO | ✓ CONFIRMED |
| `#None` | Special Values | ✗ NO | ✓ CONFIRMED |
| `!` (Generic Error) | Error Handling | N/A | ✓ CONFIRMED |
| `!ErrorType` | Error Handling | ✓ YES | ✓ CONFIRMED |
| `!pg.*` (Built-in Errors) | Error Handling | ✗ NO | ✓ CONFIRMED |
| `#Errors.*` | Error Handling | N/A | ✓ DEPRECATED (replaced by `!Error`) |

---

## Questions for User Review

**High Priority:**

1. **`#Queues.*` Schema:**
   - What is the complete schema for custom queue definitions?
   - Is `.max_concurrent` the only required field or are there others?

2. **`#DT.Business.Week.*` Schema:**
   - What fields define a business week?
   - Common use cases and examples?

3. **`#Status.*` Definition:**
   - Is this a simple set of status values or do they have fields?
   - What are all the built-in status values?

4. **`#None` Definition:**
   - What exactly is `#None` and how is it used?
   - Is it a type, a value, or both?

**Medium Priority:**

5. **Other `#DT.*` Enumerations:**
   - Are there other date/time enumerations beyond `#DT.Business.Week.*`?
   - What are their schemas?

6. **Are there any other built-in reserved enumerations** not yet discovered or documented?

---

## Next Steps

1. **User Review:** User (Hasan) reviews this document and provides:
   - Confirmed schemas for pending enumerations
   - Answers to all questions
   - Additional reserved enumerations not yet listed

2. **Update Documentation:**
   - Update this file with confirmed schemas
   - Update decision-log.md with references
   - Create examples for each reserved enumeration

3. **Validate Decisions:**
   - Ensure all schemas are consistent with syntax decisions
   - Verify extendability rules match Decision #1
   - Cross-reference with related decisions

4. **Generate v0.0.2 Docs:**
   - Include reserved enumeration reference in standard library docs
   - Document schema enforcement rules
   - Provide complete usage examples

---

## Notes

- This document is organized by **context** (Path, Queue, DateTime, Status, etc.) for easy navigation
- Each enumeration should have clear schema definition before v0.0.2 finalization
- Extendability rules must be explicitly documented
- All schemas enforce type safety at compile-time
- User cannot create new reserved enumerations (only Polyglot defines them)