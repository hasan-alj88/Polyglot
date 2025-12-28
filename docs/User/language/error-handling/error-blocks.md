# Error Blocks Guide

**Version:** v0.0.4
**Category:** Language Guide
**Audience:** Intermediate users
**Prerequisites:** Basic error handling, fork patterns

---

## Overview

Error blocks (`[!]`) provide **nested error handling** with **pattern matching** on error types. They allow you to handle errors inline and convert them to typed values like booleans, enums, or custom types.

**Key Concepts:**
- `[!]` - Error block marker (nested inside operations)
- `!` - Current error variable (only exists in `[!]` blocks)
- `[?]` - Pattern match operator
- `!*` - Wildcard for any error

---

## Basic Syntax

### Error Block Structure

```polyglot
[r] |Operation
(|) <input << $data
   [!] $result:type << !           // ! = current error
      [?] error_pattern ? value    // Pattern match
      [?] !* ? default_value       // Wildcard
```

**Pattern:**
1. Nest `[!]` block inside operation
2. Assign error to variable: `$var << !`
3. Pattern match with `[?]`
4. Return typed values based on error type

---

## Simple Example

### Convert Error to Boolean

```polyglot
[r] |File.Load
(|) <path << $file_path
   [!] $success:pg.bool << !
      [?] !NoError ? #True
      [?] !* ? #False

// $success is now #True or #False based on error
[f] $success =? #True
   [r] // Handle success
{x}
```

**What happens:**
1. `|File.Load` attempts to load file
2. If no error (`!NoError`), return `#True`
3. If any other error (`!*`), return `#False`
4. `$success` now contains the result

---

## The Error Variable `!`

### Scope and Usage

```polyglot
[!] $status:pg.bool << !    // ! represents current error
```

**Important:**
- `!` variable **only exists inside `[!]` blocks**
- Represents the current error being handled
- Has type `!ErrorType`

**Outside error block:**
```polyglot
[r] $x << !    // ❌ ERROR: ! doesn't exist here
```

**Inside error block:**
```polyglot
[!] $x << !    // ✅ OK: ! is available
```

---

## Pattern Matching with `[?]`

### Basic Pattern Matching

```polyglot
[?] error_pattern ? return_value
```

**Examples:**
```polyglot
[?] !NoError ? #True                    // Match specific error
[?] !File.NotFound ? #False             // Match file error
[?] !Network.Timeout ? #Retry           // Match network error
[?] !* ? #Unknown                       // Match any error (wildcard)
```

### Multiple Patterns

```polyglot
[!] $status:pg.error_code << !
   [?] !NoError ? #StatusCode.Success
   [?] !File.NotFound ? #StatusCode.NotFound
   [?] !Permission.Denied ? #StatusCode.Forbidden
   [?] !Network.Timeout ? #StatusCode.Timeout
   [?] !* ? #StatusCode.InternalError
```

**Pattern order matters** - first match wins!

---

## Error Blocks in Loops

### Collecting Success/Failure Per Iteration

```polyglot
[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [r] |File.Process
   (|) <path << $file
      [!] $success:pg.bool << !
         [?] !NoError ? #True
         [?] !* ? #False

[*] *Into.Array
(*) <item << $success

// $success is now array of booleans
[r] |U.Boolean.All
(|) <array << $success
(|) >result >> $all_succeeded
```

**Use case:** Track which iterations succeeded/failed.

---

## Error Blocks with Pipeline Composition

### Handling Chain Errors

```polyglot
[r] |Step1
(|) <input << $data

[|] |> |Step2
(|) >result >> <input

[|] |>
(|) >output >> $result
   [!] $chain_ok:pg.bool << !
      [?] !NoError ? #True
      [?] !* ? #False

[f] $chain_ok =? #False
   [r] >error << !ChainFailed
{x}
```

**Pattern:** Apply error block to final chain output.

---

## Complete Example: Multi-Operation Error Handling

```polyglot
{@} @Local:Examples.ErrorHandling:0.0.0.1
{x}



{|} |ProcessWithErrors
[%] %Doc << "Demonstrate error block pattern matching"

[|] <input_file:pg.path
[|] <output_file:pg.path
[|] >status:pg.string
[|] >error <~ !NoError

[r] |File.Load
(|) <path << $input_file
   [!] $load_status:pg.error_code << !
      [?] !NoError ? #Status.LoadSuccess
      [?] !File.NotFound ? #Status.FileNotFound
      [?] !Permission.Denied ? #Status.PermissionDenied
      [?] !* ? #Status.LoadFailed

[f] $load_status =? #Status.LoadSuccess
   [r] $content :pg.string << |File.GetContent"{$input_file}"

   [r] |Data.Transform
   (|) <input << $content
      [!] $transform_status:pg.error_code << !
         [?] !NoError ? #Status.TransformSuccess
         [?] !Data.Invalid ? #Status.InvalidData
         [?] !* ? #Status.TransformFailed

   [f] $transform_status =? #Status.TransformSuccess
      [r] $transformed :pg.string << |Data.GetResult""

      [r] |File.Write
      (|) <path << $output_file
      (|) <content << $transformed
         [!] $write_status:pg.error_code << !
            [?] !NoError ? #Status.WriteSuccess
            [?] !Permission.Denied ? #Status.CannotWrite
            [?] !Disk.Full ? #Status.DiskFull
            [?] !* ? #Status.WriteFailed

      [f] $write_status =? #Status.WriteSuccess
         [r] "All operations succeeded" >> >status
         [r] >error << !NoError
      {x}

      [f] *?
         [r] "Write failed: {$write_status}" >> >status
         [r] >error << !WriteError
      {x}
   {x}

   [f] *?
      [r] "Transform failed: {$transform_status}" >> >status
      [r] >error << !TransformError
   {x}
{x}

[f] *?
   [r] "Load failed: {$load_status}" >> >status
   [r] >error << !LoadError
{x}
{x}
```

**Data Flow:**
1. Try load file → error block converts to status enum
2. If success, try transform → error block converts to status
3. If success, try write → error block converts to status
4. Each level checks result and handles appropriately

---

## Error Types Reference

### Common Error Types

**Success:**
```polyglot
!NoError                    // No error occurred
```

**File Errors:**
```polyglot
!File.NotFound             // File doesn't exist
!File.PermissionDenied     // Cannot access file
!File.AlreadyExists        // File exists (when shouldn't)
!File.ReadError            // Error reading file
!File.WriteError           // Error writing file
```

**Network Errors:**
```polyglot
!Network.Timeout           // Request timed out
!Network.ConnectionRefused // Cannot connect
!Network.HostUnreachable   // Cannot reach host
```

**Data Errors:**
```polyglot
!Data.Invalid              // Data validation failed
!Data.ParseError           // Cannot parse data
!Data.SchemaViolation      // Doesn't match schema
```

**System Errors:**
```polyglot
!Permission.Denied         // Insufficient permissions
!Disk.Full                 // No disk space
!Memory.OutOfMemory        // Out of memory
```

---

## Comparison with Other Error Handling

### Error Output Parameters

```polyglot
// Traditional: Error output parameter
[|] >error <~ !NoError

[r] |Operation""

[f] $result.var_state =? #True
   [r] >error << !OperationFailed
{x}
```

**Use when:** Need to propagate error to caller

### Error Blocks

```polyglot
// Modern: Error blocks
[r] |Operation""
   [!] $success:pg.bool << !
      [?] !NoError ? #True
      [?] !* ? #False
```

**Use when:** Need immediate inline handling/conversion

### Choosing Between Them

| Use Case | Recommended Approach |
|----------|---------------------|
| Propagate to caller | Error output parameters |
| Inline conversion | Error blocks |
| Loop error aggregation | Error blocks |
| Chain error handling | Error blocks on final output |
| Multiple error paths | Fork patterns + error outputs |

---

## Advanced Patterns

### Pattern 1: Error Classification

```polyglot
[!] $error_class:pg.error_category << !
   [?] !File.NotFound ? #Category.NotFound
   [?] !File.PermissionDenied ? #Category.Forbidden
   [?] !Network.Timeout ? #Category.Timeout
   [?] !Network.ConnectionRefused ? #Category.Unavailable
   [?] !Data.Invalid ? #Category.BadRequest
   [?] !* ? #Category.InternalError
```

### Pattern 2: Retry Decision

```polyglot
[!] $should_retry:pg.bool << !
   [?] !NoError ? #False                      // Don't retry success
   [?] !Network.Timeout ? #True               // Retry timeouts
   [?] !Network.Temporary ? #True             // Retry temporary
   [?] !File.NotFound ? #False                // Don't retry not found
   [?] !* ? #False                            // Don't retry others
```

### Pattern 3: Logging Level

```polyglot
[!] $log_level:pg.log_level << !
   [?] !NoError ? #Log.Debug
   [?] !Data.Invalid ? #Log.Warning
   [?] !Network.Timeout ? #Log.Error
   [?] !* ? #Log.Critical
```

---

## Best Practices

### ✅ 1. Always Include Wildcard

```polyglot
// ✅ GOOD: Wildcard catches unexpected errors
[!] $status:pg.bool << !
   [?] !NoError ? #True
   [?] !File.NotFound ? #False
   [?] !* ? #False                // Safety net

// ❌ RISKY: Missing wildcard
[!] $status:pg.bool << !
   [?] !NoError ? #True
   [?] !File.NotFound ? #False
   // What if different error occurs?
```

### ✅ 2. Use Specific Error Types

```polyglot
// ✅ GOOD: Specific matching
[?] !File.NotFound ? #HandleNotFound
[?] !File.PermissionDenied ? #HandlePermission

// ❌ AVOID: Too generic
[?] !Error ? #HandleError
```

### ✅ 3. Order Patterns Specific to General

```polyglot
// ✅ GOOD: Specific first, wildcard last
[?] !File.NotFound ? #Specific1
[?] !File.PermissionDenied ? #Specific2
[?] !* ? #General

// ❌ WRONG: Wildcard first (catches everything!)
[?] !* ? #General
[?] !File.NotFound ? #Specific   // Never reached!
```

### ✅ 4. Convert to Appropriate Types

```polyglot
// ✅ GOOD: Boolean for simple success/fail
[!] $success:pg.bool << !

// ✅ GOOD: Enum for multiple states
[!] $status:pg.error_code << !

// ✅ GOOD: String for logging
[!] $message:pg.string << !
```

---

## Troubleshooting

### Issue 1: Using `!` Outside Error Block

**Error:** `!` variable doesn't exist

```polyglot
// ❌ WRONG
[r] $x << !    // ! not available here
```

**Solution:** Only use `!` inside `[!]` blocks:

```polyglot
// ✅ RIGHT
[r] |Operation""
   [!] $x << !    // ! available here
```

### Issue 2: Unreachable Patterns

**Warning:** Wildcard placed too early

```polyglot
// ❌ WRONG: Wildcard catches everything
[!] $status << !
   [?] !* ? #Default          // Catches all
   [?] !Specific ? #Value     // Never reached!
```

**Solution:** Put wildcard last:

```polyglot
// ✅ RIGHT
[!] $status << !
   [?] !Specific ? #Value
   [?] !* ? #Default          // Catches remaining
```

### Issue 3: Type Mismatch

**Error:** Return values have different types

```polyglot
// ❌ WRONG: #True is bool, "error" is string
[!] $result:pg.bool << !
   [?] !NoError ? #True
   [?] !* ? "error"           // Type error!
```

**Solution:** Ensure all return values match declared type:

```polyglot
// ✅ RIGHT: All return booleans
[!] $result:pg.bool << !
   [?] !NoError ? #True
   [?] !* ? #False
```

---

## Quick Reference

```
┌─────────────────────────────────────────┐
│ ERROR BLOCKS                            │
├─────────────────────────────────────────┤
│                                         │
│  BASIC SYNTAX                           │
│  [!] $variable:type << !                │
│     [?] error_pattern ? value           │
│     [?] !* ? default                    │
│                                         │
│  ERROR VARIABLE                         │
│  ! = current error (only in [!] blocks) │
│                                         │
│  PATTERN MATCHING                       │
│  [?] !NoError ? #True                   │
│  [?] !File.NotFound ? #False            │
│  [?] !* ? #Default (wildcard)           │
│                                         │
│  COMMON PATTERNS                        │
│  Boolean: Success/failure               │
│  Enum: Multiple status codes            │
│  String: Error messages                 │
│                                         │
└─────────────────────────────────────────┘
```

---

## See Also

- [Error Handling Basics](./basics.md) - Traditional error handling
- [Fork Patterns](../control-flow/fork-patterns.md) - Conditional execution
- [Loop System](../control-flow/loops.md) - Error blocks in loops
- [Pipeline Composition](../advanced/pipeline-composition.md) - Chain error handling

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-27
**Confidence:** ✅ Verified - All patterns human-validated
