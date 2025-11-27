# Polyglot Lexer Pattern Trees - Complete Reference

**Version:** v0.0.2
**Last Updated:** 2025-11-26
**Status:** Canonical Reference for Lexer Implementation

---

## Overview

This document provides **comprehensive pattern trees** for all Polyglot block markers, showing exactly what token sequences the lexer should expect after each block marker, organized by context.

**Purpose:**
- Guide lexer implementation with precise token expectations
- Enable accurate error detection and reporting
- Document all valid contexts for each block marker
- Support parser development with clear structural patterns

**Key Principles:**
1. **Lexer tokenizes, Parser validates** - Lexer recognizes patterns, Parser checks relationships
2. **Context determines meaning** - Same block marker can have different patterns in different scopes
3. **Operator prefixes enable unambiguous tokenization** - `.`, `#`, `|`, `!` prefixes make identifiers distinct
4. **Block markers are discrete tokens** - `[X]` is ONE token, not three (`[`, `X`, `]`)

---

## Pattern Tree Notation

```
BLOCK_MARKER → Context: Description
               │
               └─→ TOKEN_TYPE
                   → TOKEN_TYPE
                   → (OPTIONAL_TOKEN)?
                   → NEWLINE
```

**Symbols:**
- `→` - Must be followed by
- `│` - Context branch
- `└─→` - Pattern sequence begins
- `*` - Zero or more repetitions
- `?` - Optional (zero or one)
- `|` - Alternatives (one of several options)
- `(...)` - Grouping

---

## Complete Pattern Trees

### **[@]** Package Declaration

```
[@] → Context: File start (package declaration)
      │
      └─→ IDENTIFIER (registry name)
          → @ (delimiter)
          → IDENTIFIER (package name)
          → : (delimiter)
          → VERSION (x.y.z format)
          → NEWLINE
```

**Example:**
```polyglot
[@] Local@Example:1.0.0
```

**Tokens:**
- `BLOCK_PACKAGE_START` (`[@]`)
- `IDENTIFIER` (`Local`)
- `DELIMITER_AT` (`@`)
- `IDENTIFIER` (`Example`)
- `DELIMITER_COLON` (`:`)
- `VERSION` (`1.0.0`)
- `NEWLINE`

---

### **[#]** Two Distinct Contexts

```
[#] → Context 1: File Number (within [@] scope)
      │
      └─→ INTEGER (file number)
          → NEWLINE

    → Context 2: Enumeration Definition (top-level)
      │
      └─→ IDENTIFIER (enumeration name)
          → NEWLINE
```

**Context 1 Example:**
```polyglot
[@] Local@Example:1.0.0
[#] 1
[X]
```

**Context 2 Example:**
```polyglot
[#] UserRole
[<] .Admin
[<] .User
[X]
```

---

### **[<]** Four Distinct Contexts

```
[<] → Context 1: Package Import (within [@] scope)
      │
      └─→ @ (import marker)
          → IDENTIFIER (import alias)
          → << (push operator)
          → IDENTIFIER (registry)
          → @ (delimiter)
          → IDENTIFIER (package name)
          → : (delimiter)
          → VERSION
          → NEWLINE

    → Context 2: Macro Include (within [@] scope)
      │
      └─→ IDENTIFIER (Macro)
          → . (delimiter)
          → IDENTIFIER (MustInclude)
          → STRING_LITERAL (path, may contain \/)
          → NEWLINE

    → Context 3: Enumeration Field (within [#] enum scope)
      │
      ├─→ . (variable prefix)
      │   → IDENTIFIER (field name)
      │   → (. → IDENTIFIER)* (nested fields)
      │   → NEWLINE
      │   // Enum field (no type)
      │
      └─→ . (variable prefix)
          → IDENTIFIER (field name)
          → (. → IDENTIFIER)* (nested fields)
          → : (type delimiter)
          → TYPE_NAMESPACE
          → \ (type separator)
          → TYPE_NAME
          → NEWLINE
          // Serial field (has type)

    → Context 4: Input/Variable Binding (within parent block: [r], [p], [Q], [?])
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → (:)? (optional type annotation)
          → (TYPE_NAMESPACE → \ → TYPE_NAME)?
          → << (push operator)
          → VALUE_EXPRESSION
          → NEWLINE
```

**VALUE_EXPRESSION expansion:**
```
VALUE_EXPRESSION:
    → STRING_LITERAL_SEQUENCE (see String Literal Pattern below)
    → LITERAL_INTEGER
    → LITERAL_FLOAT
    → . → IDENTIFIER (→ . → IDENTIFIER)* (variable reference)
    → PIPELINE_IDENTIFIER → STRING_LITERAL_SEQUENCE (inline pipeline call)
    → # → IDENTIFIER (→ . → IDENTIFIER)* (enum value)
    → { → COLLECTION_ITEMS → } (collection literal)

STRING_LITERAL_SEQUENCE:
    → STRING_START
    → (STRING_CONTENT | INTERPOLATION)*
    → STRING_END

INTERPOLATION:
    → INTERPOLATION_START
    → IDENTIFIER_VARIABLE
    → (DELIMITER_COLON → FORMAT_IDENTIFIER)?
    → INTERPOLATION_END

Note: STRING_LITERAL is tokenized as a sequence, not a single token.
      See "String Literal Tokenization Pattern" section below for details.
```

**Context 1 Example (Import):**
```polyglot
[@] Local@Example:1.0.0
[<] @utils << npm@lodash:4.17.21
[X]
```

**Context 2 Example (Macro):**
```polyglot
[@] Local@Example:1.0.0
[<] Macro.MustInclude"common/types.pg"
[X]
```

**Context 3 Example (Enum Field):**
```polyglot
[#] Config
[<] .mode.Production
[<] .mode.Development
[<] .timeout: pg\int
[X]
```

**Context 4 Example (Variable Binding):**
```polyglot
[r] |ProcessData
[<] .input << .source_data
[<] .timeout: pg\int << 30
```

---

### **[>]** Output Binding

```
[>] → Context: Within parent block ([r], [p], [b], [s] pipeline calls)
      │
      └─→ . (variable prefix)
          → IDENTIFIER (source variable)
          → (. → IDENTIFIER)* (nested path)
          → (:)? (optional type annotation)
          → (TYPE_NAMESPACE → \ → TYPE_NAME)?
          → >> (pull operator)
          → . (variable prefix)
          → IDENTIFIER (destination variable)
          → (. → IDENTIFIER)* (nested path)
          → NEWLINE
```

**Example:**
```polyglot
[r] |FetchUser
[<] .user_id << .id
[>] .user: #UserProfile >> .current_user
```

---

### **[|]** Pipeline Definition

```
[|] → Context: Top-level pipeline definition
      │
      └─→ PIPELINE_IDENTIFIER (can contain dots: Utils.String.Parse)
          → NEWLINE
```

**Example:**
```polyglot
[|] ProcessData
[|] Utils.String.Transform
```

---

### **[i]** Input Declaration

```
[i] → Context 1: No inputs required (UPDATED)
      │
      └─→ NEWLINE
          // Empty - pipeline requires no inputs

    → Context 2: Variable input declaration
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → : (type delimiter)
          → TYPE_NAMESPACE
          → \ (type separator)
          → TYPE_NAME
          → (ASSIGNMENT_OP)? (optional <<, <~, or >>)
          → (VALUE_EXPRESSION)? (if assignment present)
          → NEWLINE

    → Context 3: Enumeration input
      │
      └─→ # (enum prefix)
          → IDENTIFIER (enum type)
          → (. → IDENTIFIER)* (nested enum path)
          → (<<)? (optional push)
          → (# → IDENTIFIER (→ . → IDENTIFIER)*)? (enum value)
          → NEWLINE
```

**ASSIGNMENT_OP:**
- `<<` - Push (immediate value)
- `<~` - Default (allows one override)
- `>>` - Pull (from pipeline, rare in [i])

**Context 1 Example (No Inputs - UPDATED):**
```polyglot
[|] GenerateReport
[i]
[t] |T.Daily
[W] |W.Polyglot.Scope
[o] .report: pg\string
[X]
```

**Context 2 Example (Variable Input):**
```polyglot
[|] ProcessUser
[i] .user_id: pg\string
[i] .timeout: pg\int <~ 30
[i] .config: #Config << #Config
```

**Context 3 Example (Enum Input):**
```polyglot
[|] HandleRole
[i] .role: #UserRole << #UserRole.User
```

---

### **[t]** Trigger Declaration

```
[t] → Context 1: Pipeline trigger (|T.*)
      │
      └─→ | (pipeline prefix)
          → IDENTIFIER (T)
          → . (delimiter)
          → IDENTIFIER (Call, Daily, etc.)
          → (STRING_LITERAL)? (optional parameter)
          → NEWLINE

    → Context 2: Special trigger (TG.*)
      │
      └─→ IDENTIFIER (TG)
          → . (delimiter)
          → IDENTIFIER (Cron, FileWatch, HTTP, etc.)
          → STRING_LITERAL (parameter, often required)
          → NEWLINE
```

**Context 1 Example:**
```polyglot
[t] |T.Call
[t] |T.Daily
```

**Context 2 Example:**
```polyglot
[t] TG.Cron"0 2 * * *"
[t] TG.FileWatch"/data/uploads"
[t] TG.HTTP"/api/webhook"
```

---

### **[Q]** Queue Control

```
[Q] → Context: Queue configuration block (within [|] pipeline)
      │
      └─→ NEWLINE
          // Followed by [<] bindings for queue configuration
```

**Example:**
```polyglot
[Q]
[<] .priority: pg\int << 7
[<] .timeout: pg\dt << DT.Minutes"5"
[<] .retry_count: pg\int << 3
```

---

### **[W]** Wrapper

```
[W] → Context 1: Runtime wrapper (RT.*)
      │
      └─→ IDENTIFIER (RT)
          → . (delimiter)
          → IDENTIFIER (Python, Rust, Go, Node, etc.)
          → STRING_LITERAL (module/crate/file name)
          → NEWLINE

    → Context 2: Polyglot wrapper (|W.*)
      │
      └─→ | (pipeline prefix)
          → IDENTIFIER (W)
          → . (delimiter)
          → IDENTIFIER (→ . → IDENTIFIER)* (Polyglot.Scope, etc.)
          → NEWLINE
```

**Context 1 Example:**
```polyglot
[W] RT.Python"analysis.py"
[W] RT.Rust"data_processor"
[W] RT.Node"server.js"
```

**Context 2 Example:**
```polyglot
[W] |W.Polyglot.Scope
```

---

### **[\]** Setup Block

```
[\] → Context: Setup/initialization block (within [|] pipeline)
      │
      └─→ NEWLINE
          // Execution blocks follow
          // Must eventually close with [/]
```

**Example:**
```polyglot
[\]
[r] .connection: #DbConnection << |DB.Connect
[<] .host << "localhost"
[>] .conn >> .connection
[/]
```

---

### **[/]** Cleanup Block

```
[/] → Context: Closes setup block (matches [\])
      │
      └─→ NEWLINE
```

**Example:**
```polyglot
[\]
[r] .resource << |Allocate
[/]
[r] |Process
[<] .resource << .resource
```

---

### **[o]** Output Declaration

```
[o] → Context 1: Variable output
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → : (type delimiter)
          → TYPE_NAMESPACE
          → \ (type separator)
          → TYPE_NAME
          → NEWLINE

    → Context 2: Error output
      │
      └─→ ! (error prefix)
          → IDENTIFIER (error type)
          → (. → IDENTIFIER)* (nested error path)
          → (:)? (optional type for custom errors)
          → (TYPE_NAMESPACE → \ → TYPE_NAME)?
          → NEWLINE

    → Context 3: No error (reserved)
      │
      └─→ ! (error prefix)
          → IDENTIFIER (NoError - reserved)
          → NEWLINE
```

**Examples:**
```polyglot
[o] .result: pg\string
[o] .data: #UserData
[o] !ValidationError
[o] !Database.ConnectionTimeout
[o] !NoError
```

---

### **[r]** Sequential Execution

```
[r] → Context 1: Variable declaration
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → : (type delimiter)
          → TYPE_NAMESPACE
          → \ (type separator)
          → TYPE_NAME
          → ASSIGNMENT_OP (<<, <~, or >>)
          → VALUE_EXPRESSION
          → NEWLINE

    → Context 2: Pipeline call
      │
      └─→ | (pipeline prefix)
          → IDENTIFIER (pipeline name)
          → (. → IDENTIFIER)* (nested pipeline path)
          → NEWLINE
          // Followed by [<] and/or [>] bindings
```

**Context 1 Example:**
```polyglot
[r] .output: pg\string << "computed value"
[r] .count: pg\int <~ 0
[r] .timestamp: pg\dt << DT.Now""
```

**Context 2 Example:**
```polyglot
[r] |FetchData
[<] .id << .user_id
[>] .data >> .result
```

---

### **[p]** Parallel Execution

```
[p] → Context 1: Parallel pipeline call
      │
      └─→ | (pipeline prefix)
          → IDENTIFIER (pipeline name)
          → (. → IDENTIFIER)* (nested path)
          → NEWLINE
          // Followed by [<] and/or [>] bindings

    → Context 2: Unpack operator
      │
      └─→ ~ (unpack prefix)
          → IDENTIFIER (ForEach, Enumerate, Zip)
          → NEWLINE
          // Followed by [<] inputs, [>] outputs, [~] mini-scope
```

**Context 1 Example:**
```polyglot
[p] |ProcessAsync
[<] .data << .batch
[>] .result >> .processed
```

**Context 2 Example:**
```polyglot
[p] ~ForEach
[<] .items
[>] .item
[~]
[r] |ProcessItem
[<] .value << .item
[~]
```

---

### **[Y]** Join Point

```
[Y] → Context: Convergence point for parallel operations
      │
      └─→ ~ (join prefix)
          → IDENTIFIER (Y)
          → . (delimiter)
          → IDENTIFIER (IntoArray, IntoSerial, IntoSet)
          → NEWLINE
          // Followed by [<] inputs, [>] output
```

**Example:**
```polyglot
[Y] ~Y.IntoArray
[<] .results
[>] .combined_array
```

---

### **[b]** Background Execution

```
[b] → Context: Fire-and-forget pipeline call
      │
      └─→ | (pipeline prefix)
          → IDENTIFIER (pipeline name)
          → (. → IDENTIFIER)* (nested path)
          → NEWLINE
          // Followed by [<] inputs (optional [>] outputs)
```

**Example:**
```polyglot
[b] |LogEvent
[<] .message << "Background task started"
[<] .severity << #Severity.Info
```

---

### **[s]** Streaming Execution

```
[s] → Context: Stream-aware pipeline call
      │
      └─→ | (pipeline prefix)
          → IDENTIFIER (pipeline name)
          → (. → IDENTIFIER)* (nested path)
          → NEWLINE
          // Followed by [<] stream input, [>] chunk outputs
```

**Example:**
```polyglot
[s] |ProcessStream
[<] .stream << .data_stream
[>] .chunk >> .processed_chunk
```

---

### **[?]** Conditional

```
[?] → Context 1: Catch-all (exhaustive requirement)
      │
      └─→ * (wildcard)
          → ? (pattern operator)
          → NEWLINE
          // Followed by [~] body

    → Context 2: Comparison condition
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → COMPARISON_OP (=?, =!?, >?, <?, =>?, =<?)
          → VALUE_EXPRESSION (right operand)
          → NEWLINE
          // Optional: [&], [+], etc. for compound conditions
          // Followed by [~] body

    → Context 3: Pattern condition
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → PATTERN_OP (*?, re?)
          → (STRING_LITERAL)? (pattern for re?)
          → NEWLINE
          // Followed by [~] body

    → Context 4: Range condition
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → RANGE_OP (?[, ?(, ?], ?))
          → VALUE_EXPRESSION (start)
          → , (delimiter)
          → VALUE_EXPRESSION (end)
          → NEWLINE
          // Followed by [~] body
```

**Operators:**

**COMPARISON_OP:**
- `=?` - Equal
- `=!?` - Not equal
- `>?` - Greater than
- `<?` - Less than
- `=>?` - Greater or equal
- `=<?` - Less or equal

**PATTERN_OP:**
- `*?` - Wildcard (matches anything)
- `re?` - Regex pattern

**RANGE_OP:**
- `?[` - Closed both: [a, b]
- `?(` - Open both: (a, b)
- `?]` - Half-open right: (a, b]
- `?)` - Half-open left: [a, b)

**Examples:**
```polyglot
[?] .count >? 100
[~]
[r] .status << "high"
[~]

[?] .role =? #Roles.Admin
[&] .verified =? #Boolean.True
[~]
[r] |GrantAccess
[~]

[?] .email re? "^[a-zA-Z0-9]+@.*$"
[~]
[r] |ValidateEmail
[~]

[?] .age ?[18, 65]
[~]
[r] .eligible << #Boolean.True
[~]

[?] *?
[~]
[r] |HandleDefault
[~]
```

---

### **[~]** Conditional Body

```
[~] → Context 1: Body start (within [?] conditional)
      │
      └─→ NEWLINE
          // Execution blocks follow

    → Context 2: Body end (paired with start)
      │
      └─→ NEWLINE
```

**Example:**
```polyglot
[?] .condition
[~]                    ← Body start
[r] .result << "yes"
[~]                    ← Body end
```

---

### **[&]** Boolean AND

```
[&] → Context: Compound condition (all must be true)
      │       (within [?] scope, after initial condition)
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → COMPARISON_OP
          → VALUE_EXPRESSION
          → NEWLINE
          // Can chain more boolean operators
```

**Example:**
```polyglot
[?] .status =? #Status.Active
[&] .verified =? #Boolean.True
[&] .age =>? 18
[~]
[r] |ProcessUser
[~]
```

---

### **[+]** Boolean OR

```
[+] → Context: Compound condition (any must be true)
      │       (within [?] scope, after initial condition)
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → COMPARISON_OP
          → VALUE_EXPRESSION
          → NEWLINE
```

**Example:**
```polyglot
[?] .role =? #Roles.Admin
[+] .role =? #Roles.SuperUser
[~]
[r] |GrantFullAccess
[~]
```

---

### **[-]** Boolean XOR

```
[-] → Context: Compound condition (exactly one must be true)
      │       (within [?] scope, after initial condition)
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → COMPARISON_OP
          → VALUE_EXPRESSION
          → NEWLINE
```

**Example:**
```polyglot
[?] .payment_method =? #Payment.Card
[-] .payment_method =? #Payment.Cash
[~]
[r] |ProcessSinglePayment
[~]
```

---

### **[^]** Boolean NAND

```
[^] → Context: Compound condition (not all are true)
      │       (within [?] scope, after initial condition)
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → COMPARISON_OP
          → VALUE_EXPRESSION
          → NEWLINE
```

**Example:**
```polyglot
[?] .feature1_enabled =? #Boolean.True
[^] .feature2_enabled =? #Boolean.True
[~]
[r] |HandleConflict
[~]
```

---

### **[.]** Boolean NOR

```
[.] → Context: Compound condition (none are true)
      │       (within [?] scope, after initial condition)
      │
      └─→ . (variable prefix)
          → IDENTIFIER (variable name)
          → (. → IDENTIFIER)* (nested path)
          → COMPARISON_OP
          → VALUE_EXPRESSION
          → NEWLINE
```

**Example:**
```polyglot
[?] .option1 =? #Boolean.True
[.] .option2 =? #Boolean.True
[~]
[r] |NoOptionsSelected
[~]
```

---

### **[X]** Block End

```
[X] → Context 1: Closes package declaration ([@])
      │
      └─→ NEWLINE

    → Context 2: Closes enumeration ([#])
      │
      └─→ NEWLINE

    → Context 3: Closes pipeline ([|])
      │
      └─→ NEWLINE

    → Context 4: End of file
      │
      └─→ EOF
```

**Example:**
```polyglot
[@] Local@Example:1.0.0
[#] 1
[X]                    ← Closes package

[#] UserRole
[<] .Admin
[X]                    ← Closes enum

[|] ProcessData
[i] .data: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\string
[X]                    ← Closes pipeline
```

---

## Context Summary Table

| Block Marker | Valid Contexts | Context Count | Notes |
|--------------|----------------|---------------|-------|
| `[@]` | File start only | 1 | Package declaration |
| `[#]` | Within `[@]` (file number)<br>Top-level (enum def) | 2 | Context changes meaning |
| `[<]` | Within `[@]` (import)<br>Within `[@]` (macro)<br>Within `[#]` (enum field)<br>Within parent block (binding) | 4 | Most context-sensitive |
| `[>]` | Within parent block only | 1 | Output binding |
| `[&#124;]` | Top-level only | 1 | Pipeline definition |
| `[i]` | Within `[&#124;]` only | 3 | Empty, variable, or enum |
| `[t]` | Within `[&#124;]`, after `[i]` | 2 | Pipeline or special trigger |
| `[Q]` | Within `[&#124;]`, after `[t]` | 1 | Queue control (optional) |
| `[W]` | Within `[&#124;]`, after `[Q]` or `[t]` | 2 | Runtime or Polyglot wrapper |
| `[\]` | Within `[&#124;]`, alternative to `[W]` | 1 | Setup block |
| `[/]` | Closes `[\]` only | 1 | Cleanup marker |
| `[o]` | Within `[&#124;]`, before `[X]` | 3 | Variable, error, or no-error |
| `[r]` | Within pipeline body | 2 | Variable decl or pipeline call |
| `[p]` | Within pipeline body | 2 | Parallel call or unpack |
| `[Y]` | Within pipeline body, after `[p]` | 1 | Join point |
| `[b]` | Within pipeline body | 1 | Background execution |
| `[s]` | Within pipeline body | 1 | Streaming execution |
| `[?]` | Within pipeline body | 4 | Catch-all, comparison, pattern, range |
| `[~]` | Within `[?]` only | 2 | Body start/end markers |
| `[&]` `[+]` `[-]` `[^]` `[.]` | Within `[?]`, after condition | 1 each | Boolean operators |
| `[X]` | Closes `[@]`, `[#]`, `[&#124;]` | 4 | Universal closer |

---

## Lexer Implementation Guidance

### Token Recognition Strategy

**1. Block Markers** - Use state machine:
```rust
INITIAL state:
  See '[' → Enter IN_BLOCK_MARKER state

IN_BLOCK_MARKER state:
  Read character
  See ']' → Emit appropriate BLOCK_* token
  Return to INITIAL state
```

**2. Operators** - Use longest match:
```rust
// Check longer operators first!
if current == '=' && peek == '!' && peek2 == '?' {
    emit OPERATOR_NOT_EQUAL  // =!?
} else if current == '=' && peek == '?' {
    emit OPERATOR_EQUAL      // =?
}
```

**3. Identifiers** - Check prefix first:
```rust
match current {
    '.' => lex_variable_identifier(),
    '#' => lex_enum_identifier(),
    '|' => lex_pipeline_identifier(),
    '!' => lex_error_identifier(),
    _ => lex_plain_identifier()  // For special cases
}
```

**4. String Literals** - Tokenize interpolations:
```rust
// CRITICAL: Lexer tokenizes interpolations as separate tokens

// Example: "Count: {.num:Hex}"
// Token sequence:
STRING_START
STRING_CONTENT("Count: ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".num")
DELIMITER_COLON
FORMAT_IDENTIFIER("Hex")
INTERPOLATION_END
STRING_END

// With explicit pipeline: DT.Now""
PIPELINE_IDENTIFIER("DT.Now")
STRING_START
STRING_END
```

---

## String Literal Tokenization Pattern

**CRITICAL:** String literals are tokenized as **token sequences**, not single tokens. The lexer must parse interpolations `{.variable:format}` and emit structured tokens.

### Basic Pattern

```
STRING_LITERAL_SEQUENCE:
    → STRING_START
    → (STRING_CONTENT | INTERPOLATION)*
    → STRING_END

INTERPOLATION:
    → INTERPOLATION_START
    → IDENTIFIER_VARIABLE
    → (DELIMITER_COLON → FORMAT_IDENTIFIER)?
    → INTERPOLATION_END
```

### Example 1: Plain String (No Interpolation)

**Input:**
```polyglot
"Hello, World!"
```

**Token Sequence:**
```
STRING_START
STRING_CONTENT("Hello, World!")
STRING_END
```

**Notes:**
- Implicit `U.String` call
- No interpolations, single STRING_CONTENT

### Example 2: String with Single Interpolation

**Input:**
```polyglot
"Count: {.num}"
```

**Token Sequence:**
```
STRING_START
STRING_CONTENT("Count: ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".num")
INTERPOLATION_END
STRING_END
```

**Notes:**
- Static text "Count: " becomes STRING_CONTENT
- Variable `.num` uses default formatting (no format identifier)

### Example 3: String with Formatted Interpolation

**Input:**
```polyglot
"Value: {.num:Hex}"
```

**Token Sequence:**
```
STRING_START
STRING_CONTENT("Value: ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".num")
DELIMITER_COLON
FORMAT_IDENTIFIER("Hex")
INTERPOLATION_END
STRING_END
```

**Notes:**
- Colon separates variable from format identifier
- `Hex` is tokenized as FORMAT_IDENTIFIER
- Parser resolves to `|U.String.Polyglot.Int.Hex`

### Example 4: String with Multiple Interpolations

**Input:**
```polyglot
"User {.name} has {.count:Decimal} items"
```

**Token Sequence:**
```
STRING_START
STRING_CONTENT("User ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".name")
INTERPOLATION_END
STRING_CONTENT(" has ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".count")
DELIMITER_COLON
FORMAT_IDENTIFIER("Decimal")
INTERPOLATION_END
STRING_CONTENT(" items")
STRING_END
```

**Notes:**
- Multiple STRING_CONTENT tokens for text between interpolations
- Mix of formatted and unformatted interpolations

### Example 5: Explicit Pipeline Call

**Input:**
```polyglot
DT.Minutes"5"
```

**Token Sequence:**
```
PIPELINE_IDENTIFIER("DT.Minutes")
STRING_START
STRING_CONTENT("5")
STRING_END
```

**Notes:**
- PIPELINE_IDENTIFIER emitted separately
- String follows as sequence
- Parser recognizes inline pipeline call pattern

### Example 6: Explicit Pipeline with Interpolation

**Input:**
```polyglot
DT.ToUTC"{.timestamp}"
```

**Token Sequence:**
```
PIPELINE_IDENTIFIER("DT.ToUTC")
STRING_START
INTERPOLATION_START
IDENTIFIER_VARIABLE(".timestamp")
INTERPOLATION_END
STRING_END
```

**Notes:**
- No STRING_CONTENT (entire string is interpolation)
- Pipeline receives interpolated variable value

### Example 7: Empty String

**Input:**
```polyglot
DT.Now""
```

**Token Sequence:**
```
PIPELINE_IDENTIFIER("DT.Now")
STRING_START
STRING_END
```

**Notes:**
- No STRING_CONTENT token (empty string)
- Empty string is mandatory invocation operator

### Example 8: Adjacent Interpolations

**Input:**
```polyglot
"{.first}{.second}"
```

**Token Sequence:**
```
STRING_START
INTERPOLATION_START
IDENTIFIER_VARIABLE(".first")
INTERPOLATION_END
INTERPOLATION_START
IDENTIFIER_VARIABLE(".second")
INTERPOLATION_END
STRING_END
```

**Notes:**
- No STRING_CONTENT between interpolations
- Parser concatenates results

### Lexer State Machine for Strings

**States:**
- `INITIAL` - Default state
- `IN_STRING` - Inside string literal (after STRING_START)
- `IN_INTERPOLATION` - Inside interpolation (after INTERPOLATION_START)

**Transitions:**

```
INITIAL:
  See '"' → Emit STRING_START
         → Enter IN_STRING
         → Initialize content buffer

IN_STRING:
  See '{' → Emit STRING_CONTENT (buffered content if non-empty)
         → Emit INTERPOLATION_START
         → Enter IN_INTERPOLATION
         → Clear buffer

  See '"' → Emit STRING_CONTENT (buffered content if non-empty)
         → Emit STRING_END
         → Return to INITIAL

  See '\\' → Process escape sequence
          → Add to buffer
          → Stay in IN_STRING

  Any other char → Add to buffer
                → Stay in IN_STRING

IN_INTERPOLATION:
  See '.' → Parse IDENTIFIER_VARIABLE
         → Stay in IN_INTERPOLATION

  See ':' → Emit DELIMITER_COLON
         → Stay in IN_INTERPOLATION

  See identifier → Emit FORMAT_IDENTIFIER
                → Stay in IN_INTERPOLATION

  See '}' → Emit INTERPOLATION_END
         → Return to IN_STRING
         → Clear buffer
```

### Error Detection for String Literals

**Unterminated string:**
```polyglot
.msg << "hello
```
**Error:** `Unterminated string literal at line 1`

**Unterminated interpolation:**
```polyglot
.msg << "Value: {.num"
```
**Error:** `Unterminated interpolation at line 1: expected '}', got '"'`

**Nested interpolation (not supported):**
```polyglot
.msg << "Outer {DT.Now\\\"\\\"}"
```
**Error:** `Nested interpolation not supported`

**Invalid format identifier:**
```polyglot
.msg << "Value: {.num:123}"
```
**Error:** `Invalid format identifier: must be identifier, got integer`

### Complete Reference

**See:** `/docs/project/examples/STRING-LITERAL-TOKENIZATION-STRATEGY.md` for comprehensive documentation including:
- Rationale for lexer tokenization
- Parser responsibilities
- Edge cases
- Performance considerations
- Implementation checklist

---

## Error Detection Patterns

### Lexer-Level Errors (Token Formation)

**Unknown block marker:**
```polyglot
[z] .something    ← [z] is not valid
```
**Error:** `Unknown block marker: [z]`

**Unterminated string:**
```polyglot
.msg << "hello
```
**Error:** `Unterminated string literal at line 1`

**Invalid identifier:**
```polyglot
.123variable      ← Can't start with digit
```
**Error:** `Invalid identifier: cannot start with digit`

**Malformed block marker:**
```polyglot
[r .variable      ← Missing ]
```
**Error:** `Unterminated block marker: expected ']', got ' '`

---

### Parser-Level Errors (Structure Validation)

**Missing block marker:**
```polyglot
.variable: pg\int << 5    ← Should be [r] .variable...
```
**Error:** `Expected block marker at start of statement`

**Wrong block order:**
```polyglot
[|] Pipeline
[t] |T.Call       ← [t] before [i]
[i] .input        ← Wrong order!
```
**Error:** `Expected [i] inputs before [t] triggers`

**[<] without parent:**
```polyglot
[<] .var << value    ← No parent block
```
**Error:** `[<] input binding requires parent block ([r], [p], [Q], etc.)`

**Non-exhaustive conditions:**
```polyglot
[?] .value >? 100
[~][r] .result << "high"
[~]
// Missing [?] *?
```
**Error:** `Conditional block missing required [?] *? catch-all`

---

## Evolution and Extensibility

### Easy Changes (Lexer Level)

✅ **Adding new block markers**
Effort: Trivial (1 line in switch statement)

✅ **Adding new operators**
Effort: Easy (add to pattern matching)

✅ **Adding new token types**
Effort: Easy (add enum variant + lexing logic)

✅ **Removing block markers**
Effort: Easy (remove case or emit deprecation warning)

### Medium Changes (Lexer Level)

⚠️ **Changing operator precedence**
Effort: Medium (reorder checks, extensive testing)

⚠️ **Adding context-sensitive tokens**
Effort: Medium (track state, test all contexts)

⚠️ **Extending string literal handling**
Effort: Medium (new state machine paths)

### Hard Changes (Lexer Level)

❌ **Fundamental syntax changes**
Effort: Hard (breaks core design, massive migration)

❌ **Changing identifier rules**
Effort: Hard (breaks unambiguous tokenization)

❌ **Making whitespace significant**
Effort: Hard (fundamental lexing strategy change)

---

## Version History

**v0.0.2** (2025-11-26)
- Initial comprehensive pattern trees
- Updated `[i]` pattern (empty input: `[i]` with newline)
- Added `[<]` macro pattern (`Macro.MustInclude"path"`)
- Documented all 25 block markers with contexts
- Added extensibility guidance

---

## References

- [Lexer Token Specification](LEXER-TOKEN-SPECIFICATION.md) - Complete token enumeration
- [Grammar](../../ai-context/grammar.ebnf) - Formal EBNF grammar
- [Constraints](../../ai-context/constraints.yaml) - Validation rules
- [Architecture](../../technical/architecture.md) - System design

---

**Document Status:** Canonical Reference ✅
**Last Updated:** 2025-11-26
**Maintained By:** Polyglot Language Team
