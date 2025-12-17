---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/language/11-comments.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Comments

**Version:** 0.0.2
**Last Updated:** 2025-11-11
**Status:** Complete

---

## Overview

Polyglot supports two types of comments: single-line and multi-line. Comments use forward slash `/` syntax, which is distinct from the backslash `\` used in other parts of the language.

**Key Points:**
- Single-line comments: `//`
- Multi-line comments: `/* */`
- Forward slash `/` (NOT backslash `\`)
- Comments are ignored by compiler
- Can appear anywhere whitespace is allowed

---

## Table of Contents

1. [Single-Line Comments](#single-line-comments)
2. [Multi-Line Comments](#multi-line-comments)
3. [Comment Placement](#comment-placement)
4. [Important Distinctions](#important-distinctions)
5. [Documentation Comments](#documentation-comments)
6. [Best Practices](#best-practices)
7. [Common Mistakes](#common-mistakes)

---

## Single-Line Comments

### Syntax

Use double forward slash `//`:

```polyglot
// This is a single-line comment
```

---

### Standalone Comments

Comments on their own line:

```polyglot
// This is a comment explaining the next operation
[r] |ProcessData

// Another comment
[r] |ValidateResults
```

---

### Inline Comments

Comments at the end of a line:

```polyglot
[r] .count:pg.int << 42  // Initialize counter
[r] .name:pg.string << "Alice"  // Set user name
```

---

### Multiple Single-Line Comments

```polyglot
// This is the first line of a comment
// This is the second line
// This is the third line
[r] |Operation
```

---

### Complete Example

```polyglot
[|] ProcessFile
[i] .file_path:pg.path

// Read the file from disk
[r] |ReadFile
[<] .path:pg.path << .file_path
[>] .content:pg.string >> file_data  // Store content

// Validate the content
[r] |ValidateData
[<] .data:pg.string << file_data

[X]
```

---

## Multi-Line Comments

### Syntax

Use C-style block comments `/* */`:

```polyglot
/*
 * This is a multi-line comment
 * that spans multiple lines
 */
```

---

### Basic Multi-Line Comment

```polyglot
/* This is a simple multi-line comment */
[r] |Operation
```

---

### Formatted Multi-Line Comment

```polyglot
/*
 * This is a formatted multi-line comment
 * with a consistent style using asterisks
 * on each line for readability
 */
[r] |ProcessData
```

---

### Inline Multi-Line Comment

```polyglot
[r] |Operation  /* This is an inline multi-line comment */
```

---

### Comment Blocks

```polyglot
/*
 * =============================================================================
 * File Processing Pipeline
 * =============================================================================
 *
 * This pipeline reads a file, validates its content, and processes it
 * according to the specified rules.
 *
 * Input: file_path (pg\path) - Path to the file to process
 * Output: result (pg\string) - Processed content
 *
 * Author: Team Name
 * Date: 2024-01-15
 * =============================================================================
 */
[|] ProcessFile
[i] .file_path:pg.path
[r] |ReadAndProcess
[X]
```

---

### Nested Content in Comments

```polyglot
/*
 * This comment can contain anything:
 *
 * - Code snippets (not executed):
 *   [r] |SomeOperation
 *   [<] .param << value
 *
 * - Special characters: @#$%^&*
 *
 * - Multiple languages:
 *   // Single-line style inside multi-line
 *   /* This would need escaping in some languages */
 */
```

---

## Comment Placement

### Before Declarations

```polyglot
// Define the main pipeline
[|] MainPipeline

// Declare input parameter
[i] .data:pg.string

// Set up trigger
[t] |T.Daily
[<] .time:pg.dt << |DT"09:00:"

[X]
```

---

### Between Operations

```polyglot
[r] |Step1

// Intermediate comment between steps
[r] |Step2

/*
 * Another comment with more detail
 */
[r] |Step3
```

---

### Inside Complex Structures

```polyglot
[#] Configuration
[<] .host:pg.string << "localhost"  // Server host
[<] .port:pg.int << 8080            // Server port
[<] .debug:pg.bool << #False         // Debug mode flag
[X]
```

---

### In Parallel Blocks

```polyglot
[p] |ProcessPartA
[<] .data:pg.string << input
// Comment inside parallel block
[~][r] |TransformA
[~][<] .input:pg.string << .data
[>] .result >> result_a

// Comment between parallel blocks
[p] |ProcessPartB
[<] .data:pg.string << input
[>] .result >> result_b
```

---

### Section Separators

```polyglot
[|] ComplexPipeline

// ==================== INPUTS ====================
[i] .file_path:pg.path
[i] Default .max_size:pg.int << 1024

// ==================== TRIGGERS ====================
[t] |T.Daily
[<] .time:pg.dt << |DT"09:00:"

// ==================== OPERATIONS ====================
[r] |ReadFile
[r] |ProcessData
[r] |WriteResults

// ==================== ERROR HANDLING ====================
[!] !pg.FileSystem.NotFound
[r] |HandleFileNotFound

[X]
```

---

## Important Distinctions

### Comments Use Forward Slash `/`

```polyglot
// ✓ CORRECT - Forward slash for comments
// This is a comment
/* This is also a comment */

// ✗ WRONG - Backslash is NOT for comments
\\ This is not a comment
\* This is not a comment *\
```

---

### Type Separator Uses Backslash `\`

```polyglot
// Comments use forward slash /
// Types use backslash \

[r] .count:pg.int << 42  // Type separator: \
```

---

### Path Identifiers Use Backslash `\`

```polyglot
// Comments use forward slash
// Path identifiers use backslash

[r] .file:pg.path << \\DataDir\\file.txt  // Path identifier: \\
```

---

### Visual Distinction

```polyglot
// Comment - forward slash
[r] .type:pg.int       // Type - backslash
[r] .path:pg.path << \\Path\\   // Path identifier - backslash
```

**Summary:**
- **Comments:** `/` forward slash
- **Types:** `\` backslash
- **Paths:** `\\` double backslash

---

## Documentation Comments

### Pipeline Documentation

```polyglot
/*
 * ProcessUserData
 *
 * Processes user data from input file and generates a report.
 *
 * Inputs:
 *   - file_path: Path to the input CSV file
 *   - format: Output format (json, xml, csv)
 *
 * Outputs:
 *   - report: Generated report content
 *
 * Errors:
 *   - !pg.FileSystem.NotFound: Input file doesn't exist
 *   - !MyApp.ValidationError: Data validation failed
 *
 * Example:
 *   [r] |ProcessUserData
 *   [<] .file_path:pg.path << "users.csv"
 *   [<] .format:pg.string << "json"
 */
[|] ProcessUserData
[i] .file_path:pg.path
[i] .format:pg.string
[o] .report:pg.string
[X]
```

---

### Enumeration Documentation

```polyglot
/*
 * Application Configuration
 *
 * Contains all configuration settings for the application.
 *
 * Fields:
 *   - host: Database host address
 *   - port: Database port number
 *   - debug: Enable debug logging
 *   - timeout: Connection timeout in seconds
 */
[#] AppConfig
[<] .host:pg.string << "localhost"
[<] .port:pg.int << 5432
[<] .debug:pg.bool << #False
[<] .timeout:pg.int << 30
[X]
```

---

### Error Documentation

```polyglot
/*
 * Database Connection Error
 *
 * Raised when unable to establish connection to the database.
 *
 * Reserved Fields:
 *   - message: Error description
 *   - code: Error code (5100)
 *   - trace: Stack trace
 *
 * Custom Fields:
 *   - host: Database host that failed
 *   - port: Database port
 *   - retry_count: Number of connection attempts made
 */
[!] !MyApp.Database.ConnectionError
[<] .message:pg.string << "Database connection failed"
[<] .code:pg.int << 5100
[<] .trace:pg.string << ""
[<] .host:pg.string << ""
[<] .port:pg.int << 0
[<] .retry_count:pg.int << 0
[X]
```

---

## Best Practices

### 1. Write Clear, Concise Comments

```polyglot
// ✓ GOOD - Clear and helpful
// Calculate the total price including tax
[r] |CalculateTotal
[<] .subtotal:pg.float << price
[<] .tax_rate:pg.float << 0.08

// ✗ POOR - Obvious or redundant
// Call the CalculateTotal pipeline
[r] |CalculateTotal  // This operation calculates the total
```

---

### 2. Explain Why, Not What

```polyglot
// ✓ GOOD - Explains reasoning
// Use 10 second timeout to prevent hanging on slow networks
[i] Default .timeout:pg.int << 10

// ✗ POOR - Just repeats the code
// Set timeout to 10
[i] Default .timeout:pg.int << 10
```

---

### 3. Keep Comments Updated

```polyglot
// ✓ GOOD - Comment matches code
// Retry up to 3 times before failing
[i] Default .max_retries:pg.int << 3

// ✗ WRONG - Comment doesn't match code
// Retry up to 5 times before failing
[i] Default .max_retries:pg.int << 3  // Comment is outdated!
```

---

### 4. Use Multi-Line for Complex Explanations

```polyglot
// ✓ GOOD - Multi-line for detailed explanation
/*
 * This operation implements exponential backoff for retries.
 * Each retry waits longer than the previous one:
 * 1st retry: 1 second
 * 2nd retry: 2 seconds
 * 3rd retry: 4 seconds
 * This prevents overwhelming the server while allowing recovery.
 */
[r] |RetryWithBackoff

// ✗ POOR - Single-line too long
// This operation implements exponential backoff for retries where each retry waits longer than the previous one with 1st retry 1 second 2nd retry 2 seconds 3rd retry 4 seconds to prevent overwhelming the server
[r] |RetryWithBackoff
```

---

### 5. Comment Complex Logic

```polyglot
// ✓ GOOD - Explains non-obvious logic
[p] |ProcessChunk1
[<] .data: pg.array.pg.int << data[0:1000]
[>] .result >> result1

[p] |ProcessChunk2
[<] .data: pg.array.pg.int << data[1000:2000]
[>] .result >> result2

/*
 * Join must wait for both chunks to complete before combining.
 * This ensures data consistency and prevents race conditions.
 */
[Y] |Y.Join
[>] result1
[>] result2
```

---

### 6. Use Section Headers for Organization

```polyglot
// ✓ GOOD - Organized with section headers
[|] ComplexPipeline

// ========================================
// CONFIGURATION
// ========================================
[i] .input:pg.string
[i] Default .debug:pg.bool << #False

// ========================================
// DATA PROCESSING
// ========================================
[r] |ValidateInput
[r] |TransformData

// ========================================
// ERROR HANDLING
// ========================================
[!] !ValidationError
[r] |HandleError

[X]
```

---

### 7. Document Assumptions and Constraints

```polyglot
// ✓ GOOD - Documents assumptions
/*
 * ASSUMPTIONS:
 * - Input file is UTF-8 encoded
 * - File size does not exceed 100MB
 * - Data is in CSV format with header row
 *
 * CONSTRAINTS:
 * - Must complete within 60 seconds
 * - Maximum memory usage: 512MB
 */
[|] ProcessLargeFile
[i] .file_path:pg.path
[X]
```

---

### 8. Avoid Commented-Out Code

```polyglot
// ✗ POOR - Commented-out code should be removed
[r] |ProcessData
// [r] |OldProcessData  // Don't leave old code commented
// [r] |DeprecatedStep  // Remove instead of commenting

// ✓ BETTER - Remove old code, use version control
[r] |ProcessData
```

---

## Common Mistakes

### Mistake 1: Using Backslash for Comments

```polyglot
// ✗ WRONG - Backslash doesn't create comments
\\ This is not a comment
\* This is not a comment *\

// ✓ CORRECT - Use forward slash
// This is a comment
/* This is a comment */
```

---

### Mistake 2: Confusing Comments with Path Identifiers

```polyglot
// ✗ CONFUSING - Looks like path but it's a comment
// \\DataDir\\file.txt

// ✓ CLEAR - Distinguish comments from paths
// File path: \\DataDir\\file.txt
[r] .file:pg.path << \\DataDir\\file.txt
```

---

### Mistake 3: Over-Commenting Obvious Code

```polyglot
// ✗ POOR - Too obvious
// Declare input of type string
[i] .input:pg.string

// Assign 5 to variable x
[r] .x:pg.int << 5

// Call ProcessData pipeline
[r] |ProcessData

// ✓ BETTER - Only comment when adding value
[i] .input:pg.string

// Process input with custom validation rules
[r] |ProcessData
[<] .data:pg.string << .input
```

---

### Mistake 4: Not Closing Multi-Line Comments

```polyglot
// ✗ SYNTAX ERROR - Unclosed comment
/*
 * This comment is not closed
[r] |Operation

// ✓ CORRECT - Properly closed
/*
 * This comment is properly closed
 */
[r] |Operation
```

---

### Mistake 5: Nesting Multi-Line Comments (If Not Supported)

```polyglot
// ✗ MAY CAUSE ISSUES - Nested multi-line comments
/*
 * Outer comment
 * /* Inner comment */
 * More outer comment
 */

// ✓ SAFER - Use single-line within multi-line
/*
 * Outer comment
 * // Inner comment (single-line)
 * More outer comment
 */
```

---

## See Also

### Language Specification
- [Complete Syntax Reference](01-syntax-complete.md) - Comments overview
- [Type System](02-type-system.md) - Type separator `\` vs comment `/`

### Best Practices
- Write self-documenting code
- Use comments to explain "why" not "what"
- Keep comments updated with code changes
- Remove commented-out code

### Planning
- [Decision Log](../decision-log.md) - Comment syntax decisions (#16)

---

**End of Comments Reference**