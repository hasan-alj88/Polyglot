---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "collect-errors"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*Collect.Errors"
summary: "API reference: *Collect.Errors"
keywords:
  - stdlib
  - api
  - reference

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: medium

# --- Dependency Chain ---
prereqs:
  - type-system
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#stdlib"
  - "#api"
---
# *Collect.Errors

**Collect all errors from iterations**

**Category:** Pack Operators > Collect
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *Collect.Errors
[*] <error :pg.error
[*] >errors :pg.array.pg.error
```

---

## Parameters

**Inputs:**
- `<error` :pg.error - Error from iteration scope

**Outputs:**
- `>errors` :pg.array.pg.error - Array of all collected errors in main scope

---

## Description

Collects all errors that occur during iterations into an array. Useful for batch processing where you want to continue processing despite errors and review all failures afterward.

**Key characteristics:**
- **Error aggregation** - Collects multiple errors
- **Non-blocking** - Iterations continue despite errors
- **Type-specific** - Output is always `:pg.array.pg.error`
- **Order preserved** - Sequential loops maintain error order

**Use when:**
- Batch processing with error tolerance
- Validate multiple items
- Continue despite failures
- Review all errors at once

---

## Examples

### Basic Usage - Collect Validation Errors

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [r] $validated << |ValidateItem <item << $item
      [!] !Validation.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $validation_errors
```

**Collects all validation errors, continues processing.**

---

### Process Files with Error Collection

```polyglot
[r] ~ForEach.Array
[~] <array << $file_paths
[~] >item >> $path

   [r] $content << \|File.Read <path << $path
      [!] !File.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $read_errors

   [z] $processed << \|ProcessContent <content << $content
      [!] !Processing.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $processing_errors
```

**Collects both read errors and processing errors separately.**

---

### Batch API Calls with Error Handling

```polyglot
[p] ~ForEach.Array
[~] <array << $user_ids
[~] >item >> $user_id

   [z] $user << \|API.Users.Get <id << $user_id
      [!] !API.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $api_errors

   [v] *Into.Array
   [*] <item << $user
   [*] >array >> $users
[v]

// Check if any errors occurred
[y] $api_errors.length > 0
   [r] \|Log <message << "Some API calls failed" <errors << $api_errors
```

---

### Validate Multiple Fields

```polyglot
[r] ~ForEach.Serial
[~] <serial << $form_data
[~] >path >> $field_name
[~] >item >> $value

   [z] $validated << \|ValidateField <name << $field_name <value << $value
      [!] !Validation.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $field_errors

// Report all validation errors
[y] $field_errors.length > 0
   [r] \|ReportErrors <errors << $field_errors
```

---

### Collect Errors with Context

```polyglot
[r] ~ForEach.IndexedArray
[~] <array << $records
[~] >index >> $i
[~] >item >> $record

   [z] $result << \|ProcessRecord <record << $record
      [!] *!
         [r] $err :pg.error << !Current.Error

         // Add index to error context
         [r] $enriched_err :pg.error << \|AddErrorContext <error << $err <index << $i

         [v] *Collect.Errors
         [*] <error << $enriched_err
         [*] >errors >> $processing_errors
```

---

## Empty Error Collection

**If no errors occur:**

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [z] $result << \|Process <item << $item
      [!] *!
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $errors
```

**If all succeed:** `$errors = []` (empty array)

---

## Multiple Error Collections

**Can collect different error types separately:**

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [z] $validated << \|Validate <item << $item
      [!] !Validation.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $validation_errors

   [z] $saved << \|Save <item << $validated
      [!] !Database.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $database_errors
```

**Two separate error collections.**

---

## Type

**Output is always array of errors:**

```polyglot
[*] >errors :pg.array.pg.error
```

---

## Sequential vs Parallel

### Sequential [r]

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [z] $result << \|Process <item << $item
      [!] *!
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $errors
```

**Errors collected in order of occurrence.**

### Parallel [p]

```polyglot
[p] ~ForEach.Array
[~] <array << $items
[~] >item >> $item

   [z] $result << \|Process <item << $item
      [!] *!
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $errors
[v]
```

**Error order non-deterministic.**

---

## Common Patterns

### Pattern 1: Validation with Error Report
```polyglot
[r] ~ForEach.Array
[~] <array << $inputs
[~] >item >> $input
   [z] $validated << \|Validate <input << $input
      [!] !Validation.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $errors

[y] $errors.length > 0
   [r] \|ReportValidationErrors <errors << $errors
   [r] !Validation.Failed << "Input validation failed"
```

### Pattern 2: Continue on Error
```polyglot
[r] ~ForEach.Array
[~] <array << $tasks
[~] >item >> $task
   [z] $result << \|ExecuteTask <task << $task
      [!] *!
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $task_errors
         // Continue to next task

   [v] *Into.Array
   [*] <item << $result
   [*] >array >> $results

// Log errors but don't fail
[y] $task_errors.length > 0
   [r] \|Log <level << "warning" <errors << $task_errors
```

### Pattern 3: Partial Success Reporting
```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [z] $processed << \|Process <item << $item
      [!] *!
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $errors

   [v] *Into.Array
   [*] <item << $processed
   [*] >array >> $successful

[r] $total :pg.uint << $items.length
[r] $success_count :pg.uint << $successful.length
[r] $error_count :pg.uint << $errors.length

[r] \|Report
[|] <total << $total
[|] <successful << $success_count
[|] <failed << $error_count
[|] <errors << $errors
```

### Pattern 4: Error Categorization
```polyglot
[r] ~ForEach.Array
[~] <array << $operations
[~] >item >> $op
   [z] $result << \|Execute <op << $op
      [!] !Network.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $network_errors
      [!] !Validation.*
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $validation_errors
      [!] *!
         [r] $err :pg.error << !Current.Error
         [v] *Collect.Errors
         [*] <error << $err
         [*] >errors >> $other_errors
```

---

## Performance

**Time Complexity:** O(n) where n = number of errors

**Space Complexity:** O(n) where n = number of errors

**Memory:** Errors are accumulated in memory

---

## Error Information

**Each error in the array contains:**
- Error type (e.g., `!Validation.InvalidEmail`)
- Error message
- Stack trace (if available)
- Context information

**Access error details:**
```polyglot
[r] ~ForEach.Array
[~] <array << $errors
[~] >item >> $err
   [r] $type :pg.string << $err.type
   [r] $message :pg.string << $err.message
   [r] $log :pg.string << \|U.String.Concat"{$type, \": \", $message}"
   [v] *String.Lines
   [*] <line << $log
   [*] >lines >> $error_report
```

---

## Comparison with Other Operators

| Operator | Collects | Type | Use Case |
|----------|----------|------|----------|
| **\*Collect.Errors** | Errors | `:pg.array.pg.error` | Error aggregation |
| **\*Into.Array** | Values | `:pg.array.*` | Value collection |
| **\*Join.First** | First | `*` | Single result |

**When to use \*Collect.Errors:**
- Need all error details
- Continue despite failures
- Batch error reporting
- Validation with multiple checks

**When to fail fast:**
- Don't use *Collect.Errors
- Let error propagate normally
- Stop on first error

---

## Related Operators

- [*Into.Array](../collection-building/into/into-array.md) - Collect values
- [*Join.First](./join-first.md) - Take first result

---

## See Also

- [Error Handling](../../../core-syntax/error-handling.md)
- [Loop System](../../../language/advanced/loop-system.md)
- [Pack Operators Overview](../README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
