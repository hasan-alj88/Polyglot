# Error Handling - v0.0.5

## Error Handling Marker: `[!]`

The `[!]` marker handles errors from pipelines, similar to how `[f]` handles conditionals.

## Basic Pattern

```polyglot
[r] |Pipeline
 |  <input << $value
   [!] !SpecificError
      %% Handle this specific error

   [!] !AnotherError
      %% Handle another specific error

   [!] !*
      %% Catch-all for remaining cases (including success)
```

## Exhaustiveness Requirement

Like fork conditionals `[f]`, error handling must be **exhaustive**:

❌ **Compile Error - Missing catch-all:**
```polyglot
[r] |File.Write
 |  <path << $file
   [!] !File.Write.Error
      %% Handle write error
%% ERROR: Missing [!] !* for other cases
```

✅ **Correct - Exhaustive handling:**
```polyglot
[r] |File.Write
 |  <path << $file
   [!] !File.Write.Error
      %% Handle write error

   [!] !*
      %% Handle success and other errors
```

## Output Handling in Error Cases

When pipelines have outputs, all error paths must handle the output.

### Example: File Write with Report

```polyglot
[>] >report:serial

[r] |U.File.Text.Write
 |  <path:path << $config.output-file
 |  <content:string << $reportYAML
   [!] !File.Write.Error
      [>] >report
         [.] .status:error << !File.Write.Error
         [.] .message:string << "Failed to write report"
         [.] .total_errors:uint << $totalErrors

   [!] !*
      [>] >report
         [.] .status:error << !Success
         [.] .message:string << "Report generated successfully"
         [.] .total_errors:uint << $totalErrors
```

## Output Field Syntax

### Pattern 1: Subfield Markers (Recommended)

```polyglot
[>] >report
   [.] .status:error << !Success
   [.] .total_errors:uint << $totalErrors
   [.] .total_warnings:uint << $totalWarnings
```

### Pattern 2: Dotted Output Names

```polyglot
[>] >report.status:error << !Success
[>] >report.total_errors:uint << $totalErrors
[>] >report.total_warnings:uint << $totalWarnings
```

### Pattern 3: Inline Serial with Continuation (Valid but not recommended)

```polyglot
[>] >report << {
[+]  .status: !Success,
[+]  .total_errors: $totalErrors,
[+]  .total_warnings: $totalWarnings
[+] }
```

## Error Types

### Built-in Error Types

- `!Success` - Operation succeeded (not an error, but used in `!*` catch-all)
- `!File.Write.Error` - File write operation failed
- `!File.Read.Error` - File read operation failed
- `!DB.Connection.Error` - Database connection failed
- `!DB.Query.Error` - Database query failed
- `!Email.Send.Error` - Email send failed
- `!HTTP.Request.Error` - HTTP request failed
- `!Validation.Error` - Validation failed

### Error Transformations

Errors can be transformed/wrapped:

```polyglot
[w] |W.DB.Connect
 |  <host << $host
   [!] !DB.Connection.Error -> !StartupError
      %% Transform DB error to startup error
      [>] >result << !StartupError
```

## Wrapper Error Handling

Wrappers can have error handlers using `[w][!]`:

```polyglot
[w] |W.DB.Connect
 |  <host:string << $config.db-host
 |  <port:int << $config.db-port
 |  >db:serial >> $db

[w][!]
   [!] !DB.Connection.Error
      [>] >result
         [.] .status:error << !DB.Connection.Error
         [.] .message:string << "Failed to connect to database"

   [!] !*
      %% Success - continue normally
```

## Nested Error Handling

Error handlers can be nested within iteration:

```polyglot
[p] ~ForEach.Array
 ~  <array << $files
 ~  >item >> $file
   [r] |U.File.Text.Read
    |  <file:path << $file
    |  >content:string >> $content
      [!] !File.Read.Error
         %% Skip this file, continue iteration
         [*] *Aggregate.Sum
          *  <inc << 1
          *  >sum >> $failedFiles

      [!] !*
         %% Process file successfully
         [r] $processed << |Process"{$content}"
         [*] *Into.Array
          *  <item << $processed
          *  >array >> $results
```

## Error Propagation

If an error is not handled, it propagates up to the calling context:

```polyglot
{|} |ProcessData
[t] |T.Function

[r] |LoadFile
 |  <path << $inputPath
 |  >data >> $data
%% If LoadFile fails and no [!] handler, error propagates to caller
```

## Best Practices

### 1. Always Use Catch-All

```polyglot
[!] !SpecificError
   %% Handle specific case

[!] !*
   %% Handle everything else (required)
```

### 2. Use Subfield Markers for Structured Outputs

✅ **Good:**
```polyglot
[>] >result
   [.] .status:error << !Success
   [.] .data:serial << $processedData
```

❌ **Less Clear:**
```polyglot
[>] >result << { .status: !Success, .data: $processedData }
```

### 3. Handle Errors at Appropriate Level

- Handle expected errors locally
- Let unexpected errors propagate
- Transform errors to appropriate domain

### 4. Document Error Cases

```polyglot
%% This pipeline can fail with:
%% - !File.Read.Error if input file missing
%% - !Validation.Error if data format invalid
[r] |LoadAndValidate
 |  <path << $inputPath
   [!] !File.Read.Error
      %% Log and use default data

   [!] !Validation.Error
      %% Report validation failure

   [!] !*
      %% Success case
```

## Comparison with Traditional Try-Catch

### Traditional (JavaScript)
```javascript
try {
  const data = await writeFile(path, content);
  return { status: "success", data };
} catch (err) {
  if (err instanceof FileWriteError) {
    return { status: "error", message: err.message };
  }
  throw err; // Re-throw unknown errors
}
```

### Polyglot
```polyglot
[r] |U.File.Text.Write
 |  <path << $path
 |  <content << $content
   [!] !File.Write.Error
      [>] >result
         [.] .status:error << !File.Write.Error
         [.] .message:string << "Write failed"

   [!] !*
      [>] >result
         [.] .status:error << !Success
         [.] .data:serial << $writeData
```

## Universal Subfield Marker

The `[.]` marker is used consistently across:

1. **Enum field definitions**
2. **Input/Output field definitions**
3. **Serial field access**

This creates a unified syntax for hierarchical data.

---

**Version:** 0.0.5
**Last Updated:** 2026-01-01
**Status:** Core language specification
