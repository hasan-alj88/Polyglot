# Error Handling Philosophy - !No.Output Explicit Checks

**Session Date**: 2025-11-18
**Facilitator**: Carson (Brainstorming Coach)
**Participant**: hhj
**Session Type**: Design Philosophy & Syntax Decision

---

## Session Goal

Determine Polyglot's error handling philosophy regarding `!No.Output` and establish best practices for "success path" handling.

---

## Background Context

### Current State

**Existing Error Handling System:**
- Errors are special enumerations marked with `!`
- Three reserved fields: `.message`, `.code`, `.trace`
- Errors caught with `[!]` blocks targeting specific error types
- Default behavior is implicit success

**The Question:**
Should `!No.Output` be explicitly checkable by users, or should it remain purely implicit?

### The Problem

From v0.0.2 audit, 2 instances found in utilities-catalog.md:
```polyglot
// ✗ CURRENT VIOLATION
[?] conversion_error != \\NoError\\
[r] |HandleConversionError

[?] read_error != \\NoError\\
[r] |HandleFileError
```

**Audit Statement:**
> `!No.Output` exists implicitly but is NEVER explicitly checked. Default behavior is success.

### Questions to Resolve

1. **Should `!No.Output` be explicitly checkable?**
   - Can users test `if error == !No.Output`?
   - Can users assign `!No.Output` to error variables?

2. **How should "success path" be determined?**
   - By absence of caught errors?
   - By explicit success checks?
   - By continuation of execution?

3. **What are the best practices?**
   - When should you check for errors?
   - How should you handle "no error occurred" scenarios?

---

## Exploration

### Approach A: !No.Output is Purely Implicit (Current Philosophy)

**Philosophy**: "Success is the default; only handle failures"

**Characteristics:**
- `!No.Output` exists internally but is NOT user-accessible
- Cannot test for `!No.Output` explicitly
- Cannot assign `!No.Output` to variables
- Success path = no error caught
- Explicit error handling ONLY for specific error types

**Examples:**

**✓ CORRECT - Implicit success:**
```polyglot
[|] ProcessFile
[r] |ReadFile
[<] .path: pg\path << \\DataDir\\input.txt
[>] .content: pg\string >> .file_content

// No error caught = success, continue execution
[r] |ProcessContent
[<] .content: pg\string << .file_content
[>] .result: pg\string >> .processed

[o] .result: pg\string
[X]
```

**✓ CORRECT - Handle specific errors:**
```polyglot
[|] ProcessFile
[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> .file_content

[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Warning
[<] .msg: pg\string << "File not found, using default: {err_msg}"
[r] .file_content: pg\string << "DEFAULT_CONTENT"

// Execution continues here after error handling OR after success
[r] |ProcessContent
[<] .content: pg\string << .file_content
[X]
```

**✗ WRONG - Explicit !No.Output check:**
```polyglot
// Cannot do this:
[?] .error =? !No.Output
[~][r] |SuccessPath

// Cannot do this:
[r] .error_state: !Error << !No.Output

// Cannot do this:
[!] !No.Output  // Cannot catch "no error"
```

**Pros:**
- Clean, simple mental model
- Prevents defensive programming bloat
- Encourages handling specific errors only
- Natural control flow (errors interrupt, success continues)
- Fewer lines of code

**Cons:**
- Cannot explicitly test "did ANY error occur?"
- Cannot distinguish "no error" from "error not yet set"
- Harder to implement error aggregation patterns
- May need workarounds for complex error logic

---

### Approach B: !No.Output is Explicitly Checkable

**Philosophy**: "Explicit is better than implicit; let users check success"

**Characteristics:**
- `!No.Output` is a real, user-accessible error type
- Can test for `!No.Output` explicitly
- Can assign `!No.Output` to error variables
- Success path = explicit check for `!No.Output`
- More control over error flow

**Examples:**

**✓ ALLOWED - Explicit success check:**
```polyglot
[|] ProcessWithCheck
[r] |MightFail
[>] .error: !Error >> .operation_error

// Explicitly check if no error occurred
[?] .operation_error =? !No.Output
[~][r] |SuccessPath
[~][<] .msg: pg\string << "Operation succeeded"

// Check if any error occurred
[?] .operation_error =!? !No.Output
[~][r] |FailurePath
[~][<] .error: !Error << .operation_error
[X]
```

**✓ ALLOWED - Initialize with no error:**
```polyglot
[|] InitializeErrorState
[r] .validation_error: !Error << !No.Output

// Later, might set to actual error
[?] .input_invalid
[~][r] .validation_error: !Error << !MyApp.ValidationError

// Check at end
[?] .validation_error =? !No.Output
[~][o] .success: pg\bool << true
[X]
```

**✓ ALLOWED - Catch "no error" case:**
```polyglot
[|] ExplicitNoErrorHandling
[r] |SomeOperation
[>] .error: !Error >> .op_error

[!] !No.Output  // Catch the "no error" case
[r] |U.Log.Info
[<] .msg: pg\string << "Operation completed successfully"
[X]
```

**Pros:**
- Explicit control over success/failure paths
- Can distinguish "no error" from "uninitialized"
- Easier error aggregation and tracking
- More familiar to developers from other languages
- Can implement complex error logic

**Cons:**
- Verbose, more code to write
- Encourages defensive programming
- Success path requires explicit checking
- Contradicts "errors interrupt flow" model
- More boilerplate code

---

### Approach C: Hybrid - Implicit Default with Explicit Option

**Philosophy**: "Implicit by default, explicit when needed"

**Characteristics:**
- `!No.Output` is accessible but discouraged
- Success path = no error caught (implicit)
- CAN explicitly check `!No.Output` if absolutely needed
- Best practice: use implicit success
- Use explicit checks only for error aggregation or special cases

**Examples:**

**✓ PREFERRED - Implicit success (90% of cases):**
```polyglot
[|] NormalOperation
[r] |MightFail
[<] .input: pg\string << .data

[!] !pg.FileSystem.NotFound
[r] |HandleNotFound

// No error = success, continue
[r] |NextStep
[X]
```

**✓ ALLOWED - Explicit check (special cases only):**
```polyglot
[|] ErrorAggregation
[r] .errors: pg.mutable\array{!Error} << array{}

[r] |Operation1
[>] .error: !Error >> .err1

// Only add to errors array if not NoError
[?] .err1 =!? !No.Output
[~][r] .errors.append(.err1)

[r] |Operation2
[>] .error: !Error >> .err2

[?] .err2 =!? !No.Output
[~][r] .errors.append(.err2)

// Check if any errors occurred
[?] .errors.length >? 0
[~][r] |ReportAllErrors
[~][<] .errors: pg\array{!Error} << .errors
[X]
```

**Pros:**
- Flexibility for edge cases
- Maintains clean implicit model for common cases
- Supports advanced patterns when needed
- Doesn't force explicit checks everywhere

**Cons:**
- Inconsistent - two ways to do the same thing
- Developers might overuse explicit checks
- Requires documentation of "when to use which"
- Could lead to style inconsistencies

---

## Design Decision Questions

### Question 1: Core Philosophy

**What is Polyglot's error handling philosophy?**

**Option A**: "Success is implicit; only failures are explicit"
- Errors interrupt flow
- Success = execution continues
- No need to check for "no error"

**Option B**: "Both success and failure are explicit"
- Must check error state explicitly
- Success and failure are symmetric
- More control, more code

**Option C**: "Implicit by default, explicit when needed"
- Prefer implicit success
- Allow explicit checks for special cases
- Best of both worlds?

**Recommendation**: ?

---

### Question 2: !No.Output Accessibility

**Should `!No.Output` be user-accessible?**

**Option A**: NO - Purely internal
- Cannot reference `!No.Output` in user code
- Cannot assign or compare to `!No.Output`
- Compile error if used

**Option B**: YES - Fully accessible
- Can assign: `.error: !Error << !No.Output`
- Can compare: `.error =? !No.Output`
- Can catch: `[!] !No.Output`

**Option C**: LIMITED - Accessible but discouraged
- Can use in comparisons only: `.error =? !No.Output`
- Cannot assign or catch
- Style guide discourages use

**Recommendation**: ?

---

### Question 3: Success Path Patterns

**How should developers handle the "success path"?**

**Option A**: By continuation
```polyglot
[r] |MightFail
// If we're here, it succeeded
[r] |NextStep
```

**Option B**: By explicit check
```polyglot
[r] |MightFail
[>] .error: !Error >> .err

[?] .err =? !No.Output
[~][r] |SuccessPath
```

**Option C**: Both patterns allowed
```polyglot
// Preferred:
[r] |MightFail
[r] |NextStep

// Allowed for special cases:
[r] |MightFail
[>] .error: !Error >> .err
[?] .err =!? !No.Output
[~][r] |ErrorAggregation
```

**Recommendation**: ?

---

### Question 4: Error Aggregation

**How should developers collect multiple errors?**

**Current problem**: With implicit success, how do you know if an error occurred?

**Option A**: Check for specific errors at each step
```polyglot
[r] .has_errors: pg\bool << false

[r] |Validate1
[!] !MyApp.ValidationError
[r] .has_errors: pg\bool << true

[r] |Validate2
[!] !MyApp.ValidationError
[r] .has_errors: pg\bool << true

[?] .has_errors
[~][r] |ReportErrors
```

**Option B**: Extract errors to array, check length
```polyglot
[r] .errors: pg.mutable\array{!Error} << array{}

[r] |Validate1
[!] !MyApp.ValidationError
[>] .message: pg\string >> .err1_msg
[r] .errors.append(!MyApp.ValidationError{.message: .err1_msg})

[?] .errors.length >? 0
[~][r] |ReportErrors
```

**Option C**: Allow explicit !No.Output checks for aggregation
```polyglot
[r] .errors: pg.mutable\array{!Error} << array{}

[r] |Validate1
[>] .error: !Error >> .err1
[?] .err1 =!? !No.Output
[~][r] .errors.append(.err1)

[?] .errors.length >? 0
[~][r] |ReportErrors
```

**Recommendation**: ?

---

## Confirmed Understanding

### Error State Model

**All pipelines have implicit error state:**
- Default: `!No.Output` (success)
- Raise error: `[o] !Error.Type`
- Catch error: `[~][!] !Error.Type` (scoped to previous block element)

### Key Behaviors

**1. Error Catching with `[~][!]`**
```polyglot
[r] |MightFail
[~][!] !SomeError  // ✓ Scoped error handler
[~][r] |HandleError
```

**2. Error State After Catching**
- **Pipeline that raised error**: Remains in error state
- **Pipeline that caught error**: Has `!No.Output` (unless it raises new error)

```polyglot
[|] CallerPipeline

[r] |CalleeRaisesError  // Callee ends with: !SomeError
[~][!] !SomeError
[~][r] |HandleError
// Caller error state: !No.Output (error was handled)

[r] |NextOperation  // Sees !No.Output from caller
[X]
```

**3. !No.Output is Accessible**
- `!No.Output` is a reserved enumeration with error handling responsibilities
- Can be referenced anywhere (comparisons, assignments, catch blocks)
- Typically not needed, but allowed

```polyglot
// ✓ Valid (but unusual)
[?] .some_error =? !No.Output
[r] .error_state: !Error << !No.Output
[~][!] !No.Output  // Catch success case
```

**4. Explicit `[o] !No.Output`**
- Valid but not needed
- Implicit behavior does the same
- Error state is `!No.Output` by default

---

## Design Decisions

### Decision 1: Core Philosophy ✓

**CHOSEN: Approach C - Hybrid (Implicit by default, explicit when needed)**

**Philosophy Statement:**
> "Success is implicit and preferred; explicit error checking is allowed for special cases"

**Rationale:**
- `!No.Output` is a reserved enumeration - can be accessed like any enumeration
- Default behavior is implicit success (less code, cleaner)
- Explicit checks available when needed (error aggregation, special logic)
- Flexibility without forcing verbosity

---

### Decision 2: !No.Output Accessibility ✓

**CHOSEN: Fully Accessible**

`!No.Output` can be:
- ✓ Compared: `[?] .error =? !No.Output`
- ✓ Assigned: `[r] .error: !Error << !No.Output`
- ✓ Caught: `[~][!] !No.Output`
- ✓ Output: `[o] !No.Output` (valid but redundant)

**Rationale:**
- It's a reserved enumeration with special responsibilities
- No artificial restrictions
- Users typically won't use it (implicit is easier)
- Available for edge cases

---

### Decision 3: Success Path Pattern ✓

**CHOSEN: Implicit continuation (preferred)**

**Preferred Pattern (90% of cases):**
```polyglot
[r] |MightFail
[~][!] !SomeError
[~][r] |HandleError

// Execution continues here (implicit !No.Output)
[r] |NextStep
```

**Allowed Pattern (special cases):**
```polyglot
[r] |MightFail
[~][!] !No.Output
[~][r] |U.Log.Info
[~][<] .msg: pg\string << "Success"
```

**Discouraged Pattern (verbose):**
```polyglot
[r] |MightFail
// Storing error to check later (usually unnecessary)
[>] .error: !Error >> .operation_error

[?] .operation_error =? !No.Output
[~][r] |SuccessPath
```

**Best Practice:**
- Prefer implicit continuation
- Only catch specific errors you need to handle
- Execution continuing = success
- Use explicit `!No.Output` checks only for error aggregation or logging

---

### Decision 4: Error Aggregation Pattern ✓

**RECOMMENDED: Extract error details to collection**

**Pattern A: Collect error messages (Preferred)**
```polyglot
[|] ValidateMultipleFields
[r] .errors: pg.mutable\array{pg\string} << array{}

[r] |ValidateUsername
[~][!] !MyApp.ValidationError
[~][>] .message: pg\string >> .err_msg
[~][r] .errors.append(.err_msg)

[r] |ValidateEmail
[~][!] !MyApp.ValidationError
[~][>] .message: pg\string >> .err_msg
[~][r] .errors.append(.err_msg)

[r] |ValidatePassword
[~][!] !MyApp.ValidationError
[~][>] .message: pg\string >> .err_msg
[~][r] .errors.append(.err_msg)

// Check if any validation failed
[?] .errors.length >? 0
[~][r] |ReportValidationErrors
[~][<] .errors: pg\array{pg\string} << .errors

[o] .validation_errors: pg\array{pg\string}
[X]
```

**Pattern B: Use boolean flag (Simpler)**
```polyglot
[|] ValidateMultipleFields
[r] .has_errors: pg\bool << false

[r] |ValidateUsername
[~][!] !MyApp.ValidationError
[~][r] .has_errors: pg\bool << true

[r] |ValidateEmail
[~][!] !MyApp.ValidationError
[~][r] .has_errors: pg\bool << true

[?] .has_errors
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "Validation failed"

[X]
```

**Pattern C: Explicit !No.Output check (Allowed but verbose)**
```polyglot
[|] ValidateMultipleFields
[r] .errors: pg.mutable\array{!Error} << array{}

[r] |ValidateUsername
[>] .error: !Error >> .err1

[?] .err1 =!? !No.Output
[~][r] .errors.append(.err1)

[r] |ValidateEmail
[>] .error: !Error >> .err2

[?] .err2 =!? !No.Output
[~][r] .errors.append(.err2)

[?] .errors.length >? 0
[~][r] |ReportAllErrors
[~][<] .errors: pg\array{!Error} << .errors

[X]
```

**Recommendation**: Use Pattern A or B. Pattern C is allowed but unnecessarily verbose.

---

## Best Practices

### ✓ DO: Use Implicit Success (Continuation)

```polyglot
[r] |ReadFile
[<] .path: pg\path << .file_path
[>] .content: pg\string >> .file_content

[~][!] !pg.FileSystem.NotFound
[~][r] |U.Log.Warning
[~][<] .msg: pg\string << "File not found, using default"
[~][r] .file_content: pg\string << "DEFAULT_CONTENT"

// If we're here, either succeeded or error was handled
[r] |ProcessContent
[<] .content: pg\string << .file_content
```

---

### ✓ DO: Catch Specific Errors Only

```polyglot
[r] |DatabaseQuery
[<] .query: pg\string << sql"SELECT * FROM users"

[~][!] !pg.Database.QueryError
[~][r] |HandleQueryError

[~][!] !pg.Database.ConnectionFailed
[~][r] |HandleConnectionError

// Success path continues automatically
[r] |ProcessResults
```

---

### ✓ DO: Use Error Aggregation for Multiple Validations

```polyglot
[r] .error_messages: pg.mutable\array{pg\string} << array{}

[r] |ValidateField1
[~][!] !ValidationError
[~][>] .message: pg\string >> .err
[~][r] .error_messages.append(.err)

[r] |ValidateField2
[~][!] !ValidationError
[~][>] .message: pg\string >> .err
[~][r] .error_messages.append(.err)

[?] .error_messages.length >? 0
[~][o] !MyApp.AggregatedValidationError
```

---

### ⚠️ AVOID: Checking !No.Output Explicitly (Usually Unnecessary)

```polyglot
// ✗ Verbose - usually unnecessary
[r] |MightFail
[>] .error: !Error >> .op_error

[?] .op_error =? !No.Output
[~][r] |SuccessPath

[?] .op_error =!? !No.Output
[~][r] |ErrorPath

// ✓ Prefer this
[r] |MightFail
[~][!] !SpecificError
[~][r] |HandleError

[r] |SuccessPath  // Implicit success
```

---

### ✓ DO: Catch !No.Output for Success Logging (When Needed)

```polyglot
[r] |CriticalOperation
[~][!] !No.Output
[~][r] |U.Log.Info
[~][<] .msg: pg\string << "Critical operation succeeded"

[~][!] !SomeError
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "Critical operation failed"
```

---

### ✗ DON'T: Output !No.Output Explicitly (It's Implicit)

```polyglot
// ✗ Redundant - error state is already !No.Output
[r] |MightFail
[~][!] !SomeError
[~][r] |HandleError

[o] !No.Output  // ✗ Not needed - already implicit

// ✓ Just let it be implicit
[r] |MightFail
[~][!] !SomeError
[~][r] |HandleError
// Implicit !No.Output
```

---

## Documentation Updates Needed

### 1. Update Error Handling Documentation

**File**: `docs/user/language/error-handling.md`

**Add Section**: "!No.Output - The Success State"

**Content**:
```markdown
## !No.Output - The Success State

### Overview

`!No.Output` is a reserved error enumeration representing the "no error" or "success" state. All pipelines have an implicit error state that defaults to `!No.Output`.

**Key Characteristics:**
- Default error state for all pipelines
- Can be referenced explicitly (but typically not needed)
- Best practice: use implicit continuation rather than explicit checks

### Implicit Success (Preferred)

```polyglot
[r] |MightFail
[~][!] !SomeError
[~][r] |HandleError

// Execution continues here = implicit !No.Output
[r] |NextStep
```

### Explicit !No.Output (Allowed)

```polyglot
// Valid: Catch success case for logging
[r] |CriticalOperation
[~][!] !No.Output
[~][r] |U.Log.Info
[~][<] .msg: pg\string << "Operation succeeded"

// Valid: Check if no error occurred
[?] .operation_result =? !No.Output
[~][r] |SuccessPath

// Valid but redundant: Output !No.Output
[o] !No.Output  // Same as default
```

### When to Use Explicit !No.Output

**Use explicit `!No.Output` for:**
- Success case logging in critical operations
- Error aggregation patterns
- Complex conditional logic based on success/failure

**Don't use explicit `!No.Output` for:**
- Normal success path (use continuation)
- Checking if operation succeeded (execution continuing = success)
- Clearing error state (it's implicit after catching)

### Best Practice

**Prefer implicit continuation:**
```polyglot
// ✓ GOOD - Clean, idiomatic
[r] |Operation
[~][!] !SpecificError
[~][r] |HandleError

[r] |NextStep  // Implicit success
```

**Avoid unnecessary explicit checks:**
```polyglot
// ✗ VERBOSE - Usually unnecessary
[r] |Operation
[>] .error: !Error >> .op_error

[?] .op_error =? !No.Output
[~][r] |NextStep
```
```

---

### 2. Update Code Violations

**File**: `docs/user/audit/code-violations-log.md`

**Update Status**: Those 2 violations checking `!= \\NoError\\` should be updated:
- Not violations of philosophy (explicit checks are allowed)
- But violations of syntax:
  - Use `=!?` not `!=`
  - Use `!No.Output` not `\\NoError\\`
  - Use `[~][!]` for error catching, not `[?]`

**Correct syntax:**
```polyglot
// If really needed (unusual):
[?] .conversion_error =!? !No.Output
[~][r] |HandleConversionError
```

---

### 3. Update Best Practices

**File**: `docs/user/language/error-handling.md`

**Add to Best Practices Section:**

```markdown
### 9. Prefer Implicit Success

**Good:**
```polyglot
[r] |MightFail
[~][!] !SomeError
[~][r] |HandleError

// Implicit success - execution continues
[r] |NextStep
```

**Avoid (usually unnecessary):**
```polyglot
[r] |MightFail
[~][!] !No.Output
[~][r] |SuccessPath

// Only use for special cases like logging
```

---

### 10. Use !No.Output Only When Meaningful

**Good (meaningful use case):**
```polyglot
// Logging critical operation success
[r] |DeployToProduction
[~][!] !No.Output
[~][r] |U.Log.Audit
[~][<] .msg: pg\string << "Deployment succeeded"
```

**Avoid (meaningless):**
```polyglot
// Checking success just to continue
[?] .error =? !No.Output
[~][r] |NextStep  // Just let execution continue!
```
```

---

## Summary

### Final Design: Hybrid Approach

**Philosophy**: "Success is implicit and preferred; explicit error checking is allowed for special cases"

**Key Points:**
1. ✓ `!No.Output` is fully accessible (reserved enumeration)
2. ✓ Default behavior is implicit success
3. ✓ Can explicitly check/catch `!No.Output` when needed
4. ✓ Best practice: use implicit continuation
5. ✓ Error state after catching: `!No.Output` (unless new error raised)
6. ✓ Callee keeps its error state; caller handles independently

**This provides:**
- Clean, idiomatic code by default (implicit)
- Flexibility for edge cases (explicit)
- Natural error handling flow
- No artificial restrictions
- Minimal boilerplate

---

## Complete Examples

### Example 1: Simple Error Handling (Implicit Success)

```polyglot
[|] ReadAndProcessFile
[i] .file_path: pg\path
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |U.File.Read
[<] .path: pg\path << .file_path
[>] .content: pg\string >> .file_content

[~][!] !pg.FileSystem.NotFound
[~][r] |U.Log.Warning
[~][<] .msg: pg\string << "File not found, using default"
[~][r] .file_content: pg\string << "DEFAULT_CONTENT"

[~][!] !pg.FileSystem.PermissionDenied
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "Permission denied"
[~][o] !MyApp.FileAccessError

// Implicit success - execution continues
[r] |ProcessContent
[<] .content: pg\string << .file_content
[>] .result: pg\string >> .processed_content

[o] .result: pg\string
[X]
```

---

### Example 2: Error Aggregation

```polyglot
[|] ValidateUserInput
[i] .username: pg\string
[i] .email: pg\string
[i] .password: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope

[r] .validation_errors: pg.mutable\array{pg\string} << array{}

[r] |ValidateUsername
[<] .username: pg\string << .username
[~][!] !MyApp.ValidationError
[~][>] .message: pg\string >> .err_msg
[~][r] .validation_errors.append(.err_msg)

[r] |ValidateEmail
[<] .email: pg\string << .email
[~][!] !MyApp.ValidationError
[~][>] .message: pg\string >> .err_msg
[~][r] .validation_errors.append(.err_msg)

[r] |ValidatePassword
[<] .password: pg\string << .password
[~][!] !MyApp.ValidationError
[~][>] .message: pg\string >> .err_msg
[~][r] .validation_errors.append(.err_msg)

// Check if any validation failed
[?] .validation_errors.length >? 0
[~][r] |U.Log.Error
[~][<] .msg: pg\string << "Validation failed with {.validation_errors.length} errors"
[~][o] !MyApp.AggregatedValidationError

// Implicit success - no validation errors
[o] .success: pg\bool << true
[X]
```

---

### Example 3: Explicit !No.Output for Critical Operation Logging

```polyglot
[|] CriticalDatabaseMigration
[i] .migration_script: pg\string
[t] |T.Call
[W] |W.DB.Transaction

[r] |U.DB.ExecuteMigration
[<] .script: pg\string << .migration_script

[~][!] !No.Output
[~][r] |U.Log.Audit
[~][<] .level: pg\string << "CRITICAL"
[~][<] .msg: pg\string << "Migration succeeded"
[~][r] |NotifyAdmins
[~][<] .subject: pg\string << "Migration Success"

[~][!] !pg.Database.QueryError
[~][r] |U.Log.Audit
[~][<] .level: pg\string << "CRITICAL"
[~][<] .msg: pg\string << "Migration failed - rolling back"
[~][r] |NotifyAdmins
[~][<] .subject: pg\string << "Migration Failed"
[~][o] !MyApp.MigrationFailed

[o] .success: pg\bool << true
[X]
```

---

### Example 4: Nested Error Handling

```polyglot
[|] ProcessWithFallback
[i] .data: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |TryPrimaryMethod
[<] .input: pg\string << .data
[>] .result: pg\string >> .primary_result

[~][!] !pg.Network.ConnectionFailed
[~][r] |U.Log.Warning
[~][<] .msg: pg\string << "Primary method failed, trying fallback"
[~]
[~][r] |TryFallbackMethod
[~][<] .input: pg\string << .data
[~][>] .result: pg\string >> .fallback_result
[~]
[~][~][!] !pg.Network.ConnectionFailed
[~][~][r] |U.Log.Error
[~][~][<] .msg: pg\string << "Both primary and fallback failed"
[~][~][o] !MyApp.AllMethodsFailed
[~]
[~]// Fallback succeeded
[~][r] .primary_result: pg\string << .fallback_result

// Success path (either primary or fallback worked)
[o] .result: pg\string << .primary_result
[X]
```

---

**Session Complete**
**Status**: Philosophy confirmed and documented
**Next Steps**: Update documentation files with new !No.Output guidance

---

