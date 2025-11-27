# Polyglot v0.0.2 - Complete Lexer Token Specification

**Generated:** 2025-11-26
**Purpose:** Comprehensive token list for lexer implementation
**Compliance:** v0.0.2 Syntax Standards

---

## Overview

This document provides a complete enumeration of all tokens, operators, and identifiers in Polyglot v0.0.2 for lexer implementation.

**Key Principles:**
- **NO KEYWORDS** - All identifiers require operator prefixes (`.`, `#`, `|`, `!`)
- **Case-sensitive** - All identifiers and tokens are case-sensitive
- **Operator-centric** - Operators define the language structure
- **Hierarchical** - All data is serialized tree structure

---

## 1. Block Markers

Block markers define pipeline structure and execution flow.

| Token | Name | Context | Purpose |
|-------|------|---------|---------|
| `[@]` | Package start | Package declaration | Start package definition |
| `[#]` | Version/Enum start | Package/Enumeration | Version number or enum definition |
| `[X]` | Block end | All blocks | Universal closing marker |
| `[\|]` | Pipeline start | Pipeline declaration | Start pipeline definition |
| `[i]` | Input | Pipeline scope | Input declaration |
| `[t]` | Trigger | Pipeline scope | Trigger declaration |
| `[Q]` | Queue config | Pipeline scope | Queue configuration |
| `[W]` | Wrapper | Pipeline scope | Runtime wrapper |
| `[\]` | Setup | Pipeline scope | Explicit setup block |
| `[/]` | Cleanup | Pipeline scope | Explicit cleanup block |
| `[o]` | Output | Pipeline scope | Output declaration |
| `[r]` | Sequential | Execution scope | Sequential execution OR variable declaration |
| `[<]` | Input binding | Within parent block | Push input to operation |
| `[>]` | Output binding | Within parent block | Pull output from operation |
| `[p]` | Parallel | Execution scope | Parallel execution (fork) |
| `[Y]` | Join | Execution scope | Join parallel results (convergence) |
| `[b]` | Background | Execution scope | Fire-and-forget execution |
| `[s]` | Streaming | Execution scope | Streaming execution |
| `[?]` | Conditional | Execution scope | Condition check |
| `[~]` | Body/Mini-scope | Conditional/Unpack | Body block delimiter |
| `[+]` | Boolean OR | Conditional logic | Logical OR |
| `[&]` | Boolean AND | Conditional logic | Logical AND |
| `[-]` | Boolean XOR | Conditional logic | Logical XOR |
| `[^]` | Boolean NAND | Conditional logic | Logical NAND |
| `[.]` | Boolean NOR | Conditional logic | Logical NOR |

### Block Marker Rules

1. **Context-dependent:** `[r]` is variable declaration at pipeline scope, sequential execution in execution context
2. **Parent requirement:** `[<]` and `[>]` ONLY appear within parent blocks (pipeline calls, conditionals, unpack)
3. **Universal closer:** `[X]` closes ALL block types
4. **Execution modes:** `[r]`, `[p]`, `[b]`, `[s]` define how code executes

---

## 2. Assignment Operators

Push and pull operators for data flow.

| Token | Name | Direction | Auto-await | Description |
|-------|------|-----------|------------|-------------|
| `<<` | Push | Right-to-left | Yes (source) | Pull from source, push to dest |
| `>>` | Pull | Left-to-right | Async | Pull from pipeline output, dest becomes Pending |
| `<~` | Default | Right-to-left | No | Assign default value, allows 1 override |

### Assignment Operator Details

**`<<` (Push)**
- **Syntax:** `.dest << .source` or `.dest << literal`
- **Context:**
  - `[r] .var: Type << value` - Variable declaration (pipeline scope)
  - `[<] .var << value` - Input binding (within parent block)
- **State effect:** Consumes 1 push from dest, auto-awaits source if Pending
- **Examples:**
  ```polyglot
  [r] .output: pg\string << "hello"
  [<] .input << .variable
  [<] .timeout << 30
  ```

**`>>` (Pull)**
- **Syntax:** `[>] .source: Type >> .dest`
- **Context:** Output binding (within parent block)
- **State effect:** Dest becomes Pending, transitions to Ready/Faulted when async completes
- **Examples:**
  ```polyglot
  [>] .result: pg\int >> .output
  [>] .user: #UserProfile >> .user_data
  ```

**`<~` (Default)**
- **Syntax:** `.variable: Type <~ default_value`
- **Context:** Input declaration with default
- **State effect:** Variable becomes DefaultReady (push_count: 1, allowed: 2)
- **Examples:**
  ```polyglot
  [i] .timeout: pg\int <~ 30
  [r] .retries: pg\int <~ 3
  ```

---

## 3. Comparison Operators

Comparison operators for conditional logic. All auto-await on operands.

| Token | Name | Description | Returns |
|-------|------|-------------|---------|
| `=?` | Equal | Test equality | `#Boolean.True` or `#Boolean.False` |
| `=!?` | Not equal | Test inequality | `#Boolean.True` or `#Boolean.False` |
| `>?` | Greater than | Numeric/comparable comparison | `#Boolean.True` or `#Boolean.False` |
| `<?` | Less than | Numeric/comparable comparison | `#Boolean.True` or `#Boolean.False` |
| `=>?` | Greater or equal | Numeric/comparable comparison | `#Boolean.True` or `#Boolean.False` |
| `=<?` | Less or equal | Numeric/comparable comparison | `#Boolean.True` or `#Boolean.False` |

### Comparison Operator Examples

```polyglot
[?] .user.role =? #Roles.Admin
[?] .status =!? #Status.Completed
[?] .count >? 100
[?] .timeout <? DT.Minutes"5"
[?] .age =>? 18
[?] .retries =<? 3
```

---

## 4. Pattern Operators

Pattern matching operators for string/pattern tests.

| Token | Name | Description | Auto-await |
|-------|------|-------------|------------|
| `*?` | Wildcard | Matches anything (catch-all) | No |
| `re?` | Regex | Test against regex pattern | Yes |

### Pattern Operator Details

**`*?` (Wildcard)**
- **Syntax:** `[?] *?`
- **Purpose:** Exhaustive catch-all (REQUIRED in all conditional blocks)
- **Example:**
  ```polyglot
  [?] .score >? 90
  [~][r] .grade << "A"
  [~]

  [?] *?  // MANDATORY catch-all
  [~][r] .grade << "F"
  [~]
  ```

**`re?` (Regex)**
- **Syntax:** `.variable re? "pattern"`
- **Purpose:** Regular expression pattern matching
- **Example:**
  ```polyglot
  [?] .email re? "^[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}$"
  ```

---

## 5. Range Operators

Range operators for interval checking.

| Token | Name | Interval | Description |
|-------|------|----------|-------------|
| `?[` | Closed start | `[a, b]` | Both inclusive |
| `?(` | Open start | `(a, b)` | Both exclusive |
| `?]` | Half-open right | `(a, b]` | Left exclusive, right inclusive |
| `?)` | Half-open left | `[a, b)` | Left inclusive, right exclusive |

### Range Operator Examples

```polyglot
[?] .age ?[18, 65]     // 18 <= age <= 65
[?] .score ?(0, 100)   // 0 < score < 100
[?] .value ?(0, 100]   // 0 < value <= 100
[?] .index ?[0, 10)    // 0 <= index < 10
```

---

## 6. Unpack Operators

Unpack operators for collection iteration.

| Token | Name | Outputs | Description |
|-------|------|---------|-------------|
| `~ForEach` | For each | `.item` | Iterate over collection items |
| `~Enumerate` | Enumerate | `.index`, `.item` | Iterate with index |
| `~Zip` | Zip | Multiple items | Combine collections element-wise |

### Unpack Operator Details

**`~ForEach`**
- **Syntax:** `[p] ~ForEach` (parallel), `[r] ~ForEach` (sequential), `[b] ~ForEach` (background)
- **Example:**
  ```polyglot
  [r] .numbers: pg\array{pg\int} << {1, 2, 3}

  [p] ~ForEach
  [<] .numbers
  [>] .number  // Type inferred as pg\int
  [~][r] |ProcessItem
  [~][<] .item << .number
  [~]
  ```

**`~Enumerate`**
- **Syntax:** `[p] ~Enumerate`
- **Example:**
  ```polyglot
  [r] .items: pg\array{pg\string} << {"apple", "banana", "cherry"}

  [p] ~Enumerate
  [<] .items
  [>] .index   // Type: pg\int
  [>] .item    // Type: pg\string
  [~][r] .output << "{.index}: {.item}"
  [~]
  ```

**`~Zip`**
- **Syntax:** `[p] ~Zip`
- **Example:**
  ```polyglot
  [r] .list1: pg\array{pg\int} << {1, 2, 4}
  [r] .list2: pg\array{pg\string} << {"One", "Two", "Four"}

  [p] ~Zip
  [<] .list1
  [<] .list2
  [>] .num     // Type: pg\int
  [>] .str     // Type: pg\string
  [~][r] .combined << "{.num}: {.str}"
  [~]
  ```

---

## 7. Join Operators

Join operators for parallel result convergence.

| Token | Name | Output Type | Description |
|-------|------|-------------|-------------|
| `~Y.IntoArray` | Join into array | `pg\array{T}` | Collect parallel results into array |
| `~Y.IntoSerial` | Join into serial | `pg\serial` | Collect into struct/serial |
| `~Y.IntoSet` | Join into set | `pg\set{T}` | Collect into unique set (duplicates removed) |

### Join Operator Examples

```polyglot
[r] .numbers: pg\array{pg\int} << {1, 2, 3}

[p] ~ForEach
[<] .numbers
[>] .number

[~][r] |Double
[~][<] .value << .number
[~][>] .result >> .doubled

[~][Y] ~Y.IntoArray  // Join point
[~][<] .doubled
[~][>] .all_doubled  // Type: pg\array{pg\int}
[~]
```

---

## 8. Operator Prefixes

All identifiers REQUIRE operator prefixes (NO KEYWORDS).

| Prefix | Target | Description | Examples |
|--------|--------|-------------|----------|
| `.` | Variables | Define and navigate variables | `.myvar`, `.config.timeout`, `.user.name` |
| `#` | Enumerations | Define enumerations and schemas | `#UserProfile`, `#Boolean.True`, `#PgVar.States.Ready` |
| `\|` | Pipelines | Define and call pipelines | `\|ProcessData`, `\|T.Call`, `\|W.Polyglot.Scope` |
| `!` | Errors | Define error types | `!Network.Timeout`, `!Validation.Failed`, `!NoError` |

### Prefix Rules

1. **MANDATORY** - ALL identifiers must have prefix
2. **No keywords** - `FetchUser` is invalid, must be `|FetchUser`
3. **Hierarchical** - Use `.` for navigation: `.config.database.host`
4. **Case-sensitive** - `#Boolean.True` ≠ `#boolean.true`

---

## 9. Reserved Enumerations

Reserved enumerations provided by Polyglot runtime. Still require `#` prefix.

### 9.1 Variable States (`#PgVar.States`)

| Value | State | Description |
|-------|-------|-------------|
| `#PgVar.States.Declared` | Declared | Schema only, no value (push_count: 0, allowed: 1) |
| `#PgVar.States.DefaultReady` | DefaultReady | Has default, allows override (push_count: 1, allowed: 2) |
| `#PgVar.States.Pending` | Pending | Async operation in progress |
| `#PgVar.States.Ready` | Ready | Value available, immutable (push_count: final) |
| `#PgVar.States.Faulted` | Faulted | Operation failed, errors available |
| `#PgVar.States.Retrying` | Retrying | Automatic retry in progress (post-MVP) |
| `#PgVar.States.Paused` | Paused | Waiting for external trigger (post-MVP) |
| `#PgVar.States.Cached` | Cached | Cached result, may be stale (post-MVP) |
| `#PgVar.States.Dirty` | Dirty | Cache invalid, needs refresh (post-MVP) |

**Usage:**
```polyglot
[?] .variable.pgvar.state =? #PgVar.States.Ready
[?] .result.pgvar.state =? #PgVar.States.Faulted
```

### 9.2 Boolean (`#Boolean`)

| Value | Description |
|-------|-------------|
| `#Boolean.True` | Boolean true |
| `#Boolean.False` | Boolean false |

**Usage:**
```polyglot
[i] .enabled: pg\bool <~ #Boolean.True
[?] .flag =? #Boolean.False
```

### 9.3 None (`#None`)

| Value | Description |
|-------|-------------|
| `#None` | Unit type (explicit absence of value) |

**Usage:**
```polyglot
[o] #None
[i] .optional: #UserProfile | #None
```

### 9.4 Pipeline Markers

| Value | Context | Description |
|-------|---------|-------------|
| `#Pipeline.NoInput` | Input declaration | Explicit "no inputs" marker |
| `!NoError` | Output declaration | Explicit "no output" marker |

**Usage:**
```polyglot
[|] MinimalPipeline
[i] #Pipeline.NoInput  // MANDATORY if no inputs
[t] |T.Call
[W] |W.Polyglot.Scope
[o] !NoError           // MANDATORY if no output
[X]
```

---

## 10. Reserved Namespace

Reserved namespace for variable metadata (always Ready state).

### 10.1 Namespace Pattern

**Pattern:** `.*.pgvar.*`

All variables automatically have this reserved namespace for metadata access.

### 10.2 Reserved Fields

| Field | Type | Description |
|-------|------|-------------|
| `.*.pgvar.state` | `#PgVar.States.*` | Current state of variable |
| `.*.pgvar.errors` | `pg\array{!}` | Error details (if Faulted) |
| `.*.pgvar.history.{State}.at` | `pg\dt` | Timestamp of state transition |

**Examples:**
```polyglot
.user.pgvar.state                      // Current state
.result.pgvar.errors                   // Error array
.data.pgvar.history.Pending.at         // When became Pending
.output.pgvar.history.Ready.at         // When became Ready
.value.pgvar.history.Faulted.at        // When became Faulted
```

---

## 11. Type System Tokens

### 11.1 Type Namespaces

| Namespace | Language | Description |
|-----------|----------|-------------|
| `pg` | Polyglot | Native Polyglot types |
| `py` | Python | Python types |
| `rs` | Rust | Rust types |
| `go` | Go | Go types |
| `js` | JavaScript | JavaScript types |
| `node` | Node.js | Node.js types |

### 11.2 Type Separator

**Token:** `\` (backslash)

**Usage:** `namespace\type_name`

**Examples:**
```polyglot
pg\string
pg\int
pg\array{pg\string}
py\dict
rs\Vec<i32>
```

### 11.3 Primitive Types

| Type | Description | Namespace |
|------|-------------|-----------|
| `string` | String type | `pg\string` |
| `int` | Integer type | `pg\int` |
| `float` | Float type | `pg\float` |
| `bool` | Boolean type (uses `#Boolean`) | `pg\bool` |
| `dt` | Datetime type | `pg\dt` |
| `path` | File path type | `pg\path` |
| `serial` | Serialized struct/object | `pg\serial` |

### 11.4 Collection Types

| Type | Description | Syntax |
|------|-------------|--------|
| `array` | Ordered collection | `pg\array{T}` |
| `set` | Unique collection | `pg\set{T}` |

**Rules:**
- **NO nested collections:** `pg\array{pg\array{T}}` is INVALID
- **Wrapper pattern:** Use serial to wrap: `pg\array{#Row}` where `#Row` has field `pg\array{T}`
- **Delimiter:** Use `{}` not `[]` for collection literals

---

## 12. Special Identifiers

Special identifier patterns for inline pipelines and system components.

### 12.1 DateTime Pipelines (`DT`)

**Pattern:** `DT.{Operation}"parameters"`

**All inline pipelines are string literals** - even with no parameters, must include `""`

| Token | Description | Example |
|-------|-------------|---------|
| `DT.Now` | Current datetime | `DT.Now""` |
| `DT.Minutes` | Duration in minutes | `DT.Minutes"5"` |
| `DT.Hours` | Duration in hours | `DT.Hours"2"` |
| `DT.Days` | Duration in days | `DT.Days"-30"` |
| `DT.ToUTC` | Convert to UTC | `DT.ToUTC"{.timestamp}"` |

**Examples:**
```polyglot
[r] .now: pg\dt << DT.Now""
[r] .timeout: pg\dt << DT.Minutes"5"
[r] .past: pg\dt << DT.Days"-30"
[r] .utc: pg\dt << DT.ToUTC"{.local_time}"
```

### 12.2 Runtime Wrappers (`RT`)

**Pattern:** `RT.{Language}"script_or_module"`

| Token | Description | Example |
|-------|-------------|---------|
| `RT.Python` | Python wrapper | `RT.Python"script.py"` |
| `RT.Rust` | Rust wrapper | `RT.Rust"module::function"` |
| `RT.Node` | Node.js wrapper | `RT.Node"module.js"` |
| `RT.Go` | Go wrapper | `RT.Go"package.Function"` |

**Examples:**
```polyglot
[W] RT.Python"data_processor.py"
[W] RT.Rust"module::parse"
[W] RT.Node"handler.js"
```

### 12.3 Triggers (`TG`)

**Pattern:** `TG.{TriggerType}"config"`

| Token | Description | Example |
|-------|-------------|---------|
| `TG.Cron` | Time-based trigger | `TG.Cron"0 0 * * *"` |
| `TG.FileWatch` | File system trigger | `TG.FileWatch"/path/to/watch"` |
| `TG.HTTP` | HTTP endpoint trigger | `TG.HTTP"/api/endpoint"` |
| `TG.Email` | Email trigger | `TG.Email""` |

**Examples:**
```polyglot
[t] TG.Cron"0 0 * * *"
[t] TG.FileWatch"/data/input/"
[t] TG.HTTP"/api/process"
[b] TG.Email""
```

### 12.4 Trigger Types (`|T`)

**Pattern:** `|T.{Type}`

| Token | Description | Usage |
|-------|-------------|-------|
| `\|T.Call` | Manual call | For pipelines called via `\|PipelineName` |
| `\|T.String.Call` | String literal call | For pipelines called via string literal syntax |

**Example 1: Regular pipeline call**
```polyglot
[|] HelperPipeline
[i] .input: pg\string
[t] |T.Call  // MANDATORY for called pipelines
[W] |W.Polyglot.Scope
[o] .output: pg\string
[X]
```

**Example 2: String literal pipeline**
```polyglot
[|] DT.Now
[i] .formatted_argument_string: pg\string  // MANDATORY name
[t] |T.String.Call                         // MANDATORY for string literals
[W] RT.Rust"chrono::Utc::now"
[o] .timestamp: pg\dt                      // Can return any type
[X]

// Usage:
[r] .now: pg\dt << DT.Now""
```

### 12.5 Wrappers (`|W`)

**Pattern:** `|W.{Wrapper}`

| Token | Description | Usage |
|-------|-------------|-------|
| `\|W.Polyglot.Scope` | RAII scope placeholder | When no explicit setup/cleanup needed |

**Example:**
```polyglot
[W] |W.Polyglot.Scope  // Explicit "no setup/cleanup"
```

### 12.6 String Formatting Pipelines (`U.String`)

**Pattern:** `U.String` or `|U.String.{language}.{type}.{format_identifier}`

**Purpose:** Format variables into strings for interpolation

**Naming Convention:**
```
|U.String.{language}.{type}.{format}
```

**Common Formatters:**

| Pipeline | Input | Output | Example |
|----------|-------|--------|---------|
| `U.String` | `pg\string` | `pg\string` | Plain string (default) |
| `\|U.String.Polyglot.Int.Hex` | `pg\int` | `pg\string` | `17` → `"11"` |
| `\|U.String.Polyglot.Int.Binary` | `pg\int` | `pg\string` | `5` → `"101"` |
| `\|U.String.Polyglot.Int.Decimal` | `pg\int` | `pg\string` | `1000` → `"1,000"` |
| `\|U.String.Polyglot.Float.Currency` | `pg\float` | `pg\string` | `42.5` → `"$42.50"` |
| `\|U.String.Polyglot.Float.Percent` | `pg\float` | `pg\string` | `0.75` → `"75%"` |
| `\|U.String.Polyglot.DateTime.ShortDate` | `pg\dt` | `pg\string` | ISO → `"01/15/24"` |

**Usage in interpolation:**
```polyglot
"Value: {.number:Hex}"         // Calls |U.String.Polyglot.Int.Hex
"Price: {.amount:Currency}"    // Calls |U.String.Polyglot.Float.Currency
"Progress: {.ratio:Percent}"   // Calls |U.String.Polyglot.Float.Percent
```

**Implicit call:**
```polyglot
"hello"                        // Actually: U.String"hello"
```

**See:** `/docs/technical/string-literals-internals.md` for processing mechanics

---

## 13. Delimiters and Punctuation

| Token | Name | Usage | Context |
|-------|------|-------|---------|
| `{` | Brace open | Collection start | Array/set literals, type parameters |
| `}` | Brace close | Collection end | Array/set literals, type parameters |
| `(` | Paren open | Range interval | Range operators |
| `)` | Paren close | Range interval | Range operators |
| `"` | Quote | String delimiter | String literals, inline pipeline parameters |
| `,` | Comma | Separator | Collection elements, range bounds |
| `:` | Colon | Type annotation | Variable/parameter type specification |
| `@` | At | Package/version | Package names, version specs |
| `\\` | Backslash | Type separator | Namespace\type syntax |
| `\|` | Pipe | Pipeline prefix | Pipeline identifiers |

### Delimiter Rules

1. **Collections use `{}`** - NOT `[]`
   ```polyglot
   << {}              // ✓ Correct (empty)
   << {1, 2, 3}       // ✓ Correct (array)
   << []              // ✗ Wrong
   ```

2. **Strings use `"`** - Single delimiter type
   ```polyglot
   "hello world"      // ✓ Correct
   'hello'            // ✗ Wrong (no single quotes)
   ```

3. **Type separator is `\`** - NOT `/` or `:`
   ```polyglot
   pg\string          // ✓ Correct
   pg/string          // ✗ Wrong
   pg:string          // ✗ Wrong
   ```

---

## 14. Literals

### 14.1 String Literals (Inline Pipeline Calls)

**CRITICAL:** String literals are NOT primitive values - they are **inline pipeline calls**.

**Pattern:** `Pipeline.Name"formatted_argument_string"`

**Special case:** Plain strings implicitly call `U.String`:
```polyglot
"hello world"        // Actually: U.String"hello world"
```

**Syntax Components:**
- **Pipeline name:** Optional (defaults to `U.String`)
- **String delimiter:** `"..."` (double quotes only)
- **Interpolation:** `{.variable}` or `{.variable:format}`
- **Escape sequences:** `\"`, `\\`, `\n`, `\t`

**CRITICAL TOKENIZATION:** The lexer tokenizes interpolations within string literals as separate tokens, not as raw string content. See Section 14.1.1 for details.

**Interpolation with Formatting:**
```polyglot
"{.var}"              // Default formatting
"{.var:Hex}"          // Hexadecimal format
"{.var:Currency}"     // Currency format
"{.number:Decimal}"   // Decimal with commas
```

**Examples:**
```polyglot
// Plain strings (implicit U.String call)
"hello world"                              // U.String"hello world"
"User: {.user.name}"                       // U.String with interpolation
"Path: C:\\Program Files\\App"            // With escape sequences
"Count: {.total:Decimal}"                  // With format specifier

// Explicit pipeline calls
DT.Now""                                   // Empty parameter
DT.Minutes"5"                              // Parameter: "5"
DT.ToNow"{.start}"                         // Interpolated parameter
Template.Greeting"Alice"                   // Custom pipeline
```

**Key Rules:**
1. Empty parameters MUST include `""`: `DT.Now""` not `DT.Now`
2. Outputs can be ANY type, not just strings
3. All string literals trigger format processing for `{.var:fmt}`
4. Auto-await triggered on interpolated variables

**See:**
- `/docs/technical/string-literals-internals.md` for complete mechanics
- `/docs/project/examples/STRING-LITERAL-TOKENIZATION-STRATEGY.md` for lexer tokenization

#### 14.1.1 String Literal Token Sequence

The lexer emits a **sequence of tokens** for string literals, not a single `LITERAL_STRING` token.

**Token Types for String Literals:**

| Token | Description | Example |
|-------|-------------|---------|
| `STRING_START` | Opening quote | `"` |
| `STRING_CONTENT` | Static text between interpolations | `"Count: "` |
| `STRING_END` | Closing quote | `"` |
| `INTERPOLATION_START` | Opening brace | `{` |
| `INTERPOLATION_END` | Closing brace | `}` |
| `FORMAT_IDENTIFIER` | Format specifier | `Hex`, `Currency` |

**Example Token Sequence:**

```polyglot
Input: "Count: {.num:Hex}"

Token sequence:
  STRING_START
  STRING_CONTENT("Count: ")
  INTERPOLATION_START
  IDENTIFIER_VARIABLE(".num")
  DELIMITER_COLON
  FORMAT_IDENTIFIER("Hex")
  INTERPOLATION_END
  STRING_END
```

**Pattern (EBNF-style):**
```
STRING_LITERAL = STRING_START
                 (STRING_CONTENT | INTERPOLATION)*
                 STRING_END

INTERPOLATION = INTERPOLATION_START
                IDENTIFIER_VARIABLE
                (DELIMITER_COLON FORMAT_IDENTIFIER)?
                INTERPOLATION_END
```

**See:** `/docs/project/examples/STRING-LITERAL-TOKENIZATION-STRATEGY.md` for complete details

### 14.2 Integer Literals

**Pattern:** `-?[0-9]+`

**Examples:**
```polyglot
0
42
-10
1000000
```

### 14.3 Float Literals

**Pattern:** `-?[0-9]+\.[0-9]+`

**Examples:**
```polyglot
3.14
-0.5
100.0
0.001
```

### 14.4 Boolean Literals

**Values:** `#Boolean.True`, `#Boolean.False`

**Note:** These are enumeration values, not primitive literals

**Examples:**
```polyglot
[i] .enabled: pg\bool <~ #Boolean.True
[?] .active =? #Boolean.False
```

### 14.5 DateTime Literals

**Pattern:** `DT"{ISO8601}"`

**ISO8601 Format:** `YYYY-MM-DDTHH:MM:SS[.fff][Z|±HH:MM]`

**Examples:**
```polyglot
DT"2024-01-15T14:30:00Z"
DT"2024-01-15T14:30:00.123Z"
DT"2024-01-15T14:30:00+05:30"
```

### 14.6 Duration Literals

**Pattern:** `DT.{Unit}"{number}"`

**Units:** `Minutes`, `Hours`, `Days`, `Weeks`, `Months`, `Years`

**Examples:**
```polyglot
DT.Minutes"30"
DT.Hours"2"
DT.Days"-7"
```

### 14.7 Collection Literals

**Pattern:** `{[element[, element]*]?}`

**Examples:**
```polyglot
{}                              // Empty collection
{1, 2, 3}                       // Integer array
{"a", "b", "c"}                 // String array
{#Status.Pending, #Status.Done} // Enum array
```

### 14.8 None Literal

**Value:** `#None`

**Usage:** Represents explicit absence of value

**Examples:**
```polyglot
[o] #None
[r] .result: #UserProfile | #None << #None
```

---

## 15. Comments

### 15.1 Single-line Comments

**Pattern:** `//[^\n]*`

**Examples:**
```polyglot
// This is a comment
[r] .output: pg\string << "hello"  // Inline comment
```

### 15.2 Multi-line Comments

**Pattern:** `/\*.*?\*/`

**Examples:**
```polyglot
/* This is a
   multi-line
   comment */

[r] .value: pg\int << 42  /* Can be inline too */
```

---

## 16. Identifiers

### 16.1 Base Identifier Pattern

**Pattern:** `[a-zA-Z_][a-zA-Z0-9_]*`

**Rules:**
- Must start with letter or underscore
- Can contain letters, digits, underscores
- Case-sensitive
- MUST have operator prefix (`.`, `#`, `|`, `!`)

**Examples:**
```polyglot
user_name          // ✗ Wrong (no prefix)
.user_name         // ✓ Correct (variable)
#UserProfile       // ✓ Correct (enum)
|ProcessData       // ✓ Correct (pipeline)
!ValidationError   // ✓ Correct (error)
```

### 16.2 Variable Identifiers

**Pattern:** `\.{IDENTIFIER}(\.{IDENTIFIER})*`

**Examples:**
```polyglot
.user
.config
.database.host
.user.profile.age
.*.pgvar.state
```

### 16.3 Enumeration Identifiers

**Pattern:** `#{IDENTIFIER}(\.{IDENTIFIER})*`

**Examples:**
```polyglot
#UserProfile
#Boolean.True
#PgVar.States.Ready
#Status.Completed
#Config.Mode.Production
```

### 16.4 Pipeline Identifiers

**Pattern:** `\|{IDENTIFIER}(\.{IDENTIFIER})*`

**Examples:**
```polyglot
|ProcessData
|FetchUser
|T.Call
|W.Polyglot.Scope
```

### 16.5 Error Identifiers

**Pattern:** `!{IDENTIFIER}(\.{IDENTIFIER})*`

**Examples:**
```polyglot
!NetworkError
!Validation.Failed
!Database.ConnectionTimeout
!NoError
```

### 16.6 Version Identifiers

**Pattern:** `[0-9]+\.[0-9]+\.[0-9]+`

**Usage:** Package version specifications

**Examples:**
```polyglot
1.0.0
0.0.2
2.1.15
```

---

## 17. Token Priority and Ambiguity Resolution

### 17.1 Multi-character Operators

When lexing, longest match wins:

| Order | Token | Not |
|-------|-------|-----|
| 1 | `=!?` | Not `=!` + `?` |
| 2 | `=>?` | Not `=` + `>?` |
| 3 | `=<?` | Not `=` + `<?` |
| 4 | `=?` | Not `=` + `?` |
| 5 | `<<` | Not `<` + `<` |
| 6 | `>>` | Not `>` + `>` |
| 7 | `<~` | Not `<` + `~` |

### 17.2 Block Markers vs Operators

Context matters:

```polyglot
[<]    // Block marker (input binding) - at line start or after block
 <?    // Operator (less than) - in expression
```

### 17.3 Prefix Ambiguity

All prefixes must be followed by identifier:

```polyglot
.user      // ✓ Variable
. user     // ✗ Invalid (space)
.123       // ✗ Invalid (starts with digit)
```

---

## 18. Whitespace and Layout

### 18.1 Significant Whitespace

**Polyglot is NOT whitespace-significant** - indentation is stylistic only.

### 18.2 Line Continuation

No explicit line continuation character. Statements can span multiple lines:

```polyglot
[r] .long_string: pg\string <<
    "This is a very long string that spans multiple lines"
```

### 18.3 Whitespace Tokens

| Token | Name | Usage |
|-------|------|-------|
| Space | ` ` | Token separator |
| Tab | `\t` | Token separator (stylistic indentation) |
| Newline | `\n` | Token separator |
| Carriage return | `\r` | Token separator (Windows) |

---

## 19. Lexer State Machine Hints

### 19.1 States

1. **INITIAL** - Default state
2. **IN_STRING** - Inside string literal (after STRING_START)
3. **IN_INTERPOLATION** - Inside interpolation (after INTERPOLATION_START)
4. **IN_COMMENT** - Inside multi-line comment
5. **IN_BLOCK_MARKER** - Inside `[...]` block marker

### 19.2 Transitions

```
INITIAL:
  '"'             -> Emit STRING_START
                  -> Enter IN_STRING
                  -> Initialize content buffer
  '/*'            -> IN_COMMENT
  '//'            -> SINGLE_LINE_COMMENT
  '['             -> IN_BLOCK_MARKER
  'DT.', 'RT.', 'TG.' -> Emit PIPELINE_IDENTIFIER
                      -> Stay in INITIAL
  [.|#|!]         -> IDENTIFIER
  [0-9]           -> NUMBER

IN_STRING:
  '{'             -> Emit STRING_CONTENT (buffered content if non-empty)
                  -> Emit INTERPOLATION_START
                  -> Enter IN_INTERPOLATION
                  -> Clear buffer
  '"'             -> Emit STRING_CONTENT (buffered content if non-empty)
                  -> Emit STRING_END
                  -> Return to INITIAL
  '\\'            -> Process escape sequence
                  -> Add to buffer
                  -> Stay in IN_STRING
  any other char  -> Add to buffer
                  -> Stay in IN_STRING

IN_INTERPOLATION:
  '.'             -> Parse IDENTIFIER_VARIABLE
                  -> Stay in IN_INTERPOLATION
  ':'             -> Emit DELIMITER_COLON
                  -> Stay in IN_INTERPOLATION
  identifier      -> Emit FORMAT_IDENTIFIER
                  -> Stay in IN_INTERPOLATION
  '}'             -> Emit INTERPOLATION_END
                  -> Return to IN_STRING
                  -> Clear buffer

IN_BLOCK_MARKER:
  ']'             -> INITIAL (emit BLOCK_MARKER)

IN_COMMENT:
  '*/'            -> INITIAL
```

**Key Points:**
- STRING_CONTENT is accumulated in buffer during IN_STRING state
- Buffer is emitted when entering interpolation or ending string
- Empty STRING_CONTENT is not emitted
- Escape sequences are processed in IN_STRING state before buffering

---

## 20. Complete Token Enumeration

### 20.1 Token Categories

```
TOKENS = {
  // Block Markers (27 tokens)
  BLOCK_PACKAGE_START,      // [@]
  BLOCK_VERSION_ENUM,       // [#]
  BLOCK_END,                // [X]
  BLOCK_PIPELINE_START,     // [|]
  BLOCK_INPUT,              // [i]
  BLOCK_TRIGGER,            // [t]
  BLOCK_QUEUE,              // [Q]
  BLOCK_WRAPPER,            // [W]
  BLOCK_SETUP,              // [\]
  BLOCK_CLEANUP,            // [/]
  BLOCK_OUTPUT,             // [o]
  BLOCK_SEQUENTIAL,         // [r]
  BLOCK_INPUT_BINDING,      // [<]
  BLOCK_OUTPUT_BINDING,     // [>]
  BLOCK_PARALLEL,           // [p]
  BLOCK_JOIN,               // [Y]
  BLOCK_BACKGROUND,         // [b]
  BLOCK_STREAMING,          // [s]
  BLOCK_CONDITIONAL,        // [?]
  BLOCK_BODY,               // [~]
  BLOCK_BOOL_OR,            // [+]
  BLOCK_BOOL_AND,           // [&]
  BLOCK_BOOL_XOR,           // [-]
  BLOCK_BOOL_NAND,          // [^]
  BLOCK_BOOL_NOR,           // [.]

  // Assignment Operators (3 tokens)
  OP_PUSH,                  // <<
  OP_PULL,                  // >>
  OP_DEFAULT,               // <~

  // Comparison Operators (6 tokens)
  OP_EQUAL,                 // =?
  OP_NOT_EQUAL,             // =!?
  OP_GREATER,               // >?
  OP_LESS,                  // <?
  OP_GREATER_EQUAL,         // =>?
  OP_LESS_EQUAL,            // =<?

  // Pattern Operators (2 tokens)
  OP_WILDCARD,              // *?
  OP_REGEX,                 // re?

  // Range Operators (4 tokens)
  OP_RANGE_CLOSED,          // ?[
  OP_RANGE_OPEN,            // ?(
  OP_RANGE_HALF_RIGHT,      // ?]
  OP_RANGE_HALF_LEFT,       // ?)

  // Delimiters (10 tokens)
  DELIMITER_BRACE_OPEN,     // {
  DELIMITER_BRACE_CLOSE,    // }
  DELIMITER_PAREN_OPEN,     // (
  DELIMITER_PAREN_CLOSE,    // )
  DELIMITER_QUOTE,          // "
  DELIMITER_COMMA,          // ,
  DELIMITER_COLON,          // :
  DELIMITER_AT,             // @
  DELIMITER_BACKSLASH,      // \
  DELIMITER_PIPE,           // |

  // Identifiers (5 categories)
  IDENTIFIER_VARIABLE,      // .identifier(.identifier)*
  IDENTIFIER_ENUM,          // #identifier(.identifier)*
  IDENTIFIER_PIPELINE,      // |identifier(.identifier)*
  IDENTIFIER_ERROR,         // !identifier(.identifier)*
  IDENTIFIER_UNPACK,        // ~identifier
  IDENTIFIER_JOIN,          // ~Y.identifier

  // Reserved Enumerations (11 tokens)
  RESERVED_PGVAR_DECLARED,     // #PgVar.States.Declared
  RESERVED_PGVAR_DEFAULTREADY, // #PgVar.States.DefaultReady
  RESERVED_PGVAR_PENDING,      // #PgVar.States.Pending
  RESERVED_PGVAR_READY,        // #PgVar.States.Ready
  RESERVED_PGVAR_FAULTED,      // #PgVar.States.Faulted
  RESERVED_BOOLEAN_TRUE,       // #Boolean.True
  RESERVED_BOOLEAN_FALSE,      // #Boolean.False
  RESERVED_NONE,               // #None
  RESERVED_PIPELINE_NOINPUT,   // #Pipeline.NoInput
  RESERVED_NOERROR,            // !NoError

  // String Literal Tokens (6 tokens) - NEW: Interpolation support
  STRING_START,             // " (opening quote)
  STRING_CONTENT,           // Static text between interpolations
  STRING_END,               // " (closing quote)
  INTERPOLATION_START,      // { (opening brace in string)
  INTERPOLATION_END,        // } (closing brace in string)
  FORMAT_IDENTIFIER,        // Format specifier (e.g., Hex, Currency)

  // Other Literals (5 categories)
  LITERAL_INTEGER,          // -?[0-9]+
  LITERAL_FLOAT,            // -?[0-9]+.[0-9]+
  LITERAL_DATETIME,         // DT"ISO8601"
  LITERAL_DURATION,         // DT.Unit"number"
  LITERAL_COLLECTION,       // {...}

  // Type Tokens (8 tokens)
  TYPE_NAMESPACE,           // pg, py, rs, go, js, node
  TYPE_STRING,              // string
  TYPE_INT,                 // int
  TYPE_FLOAT,               // float
  TYPE_BOOL,                // bool
  TYPE_DATETIME,            // dt
  TYPE_PATH,                // path
  TYPE_SERIAL,              // serial
  TYPE_ARRAY,               // array
  TYPE_SET,                 // set

  // Special Identifiers (4 categories)
  SPECIAL_DATETIME,         // DT.{Operation}
  SPECIAL_RUNTIME,          // RT.{Language}
  SPECIAL_TRIGGER,          // TG.{Type}
  SPECIAL_TRIGGER_TYPE,     // |T.{Type}
  SPECIAL_WRAPPER,          // |W.{Wrapper}

  // Comments (2 tokens)
  COMMENT_SINGLE,           // //...
  COMMENT_MULTI,            // /*...*/

  // Whitespace (4 tokens)
  WHITESPACE_SPACE,         // ' '
  WHITESPACE_TAB,           // '\t'
  WHITESPACE_NEWLINE,       // '\n'
  WHITESPACE_CR,            // '\r'

  // Version (1 token)
  VERSION,                  // [0-9]+.[0-9]+.[0-9]+

  // End of file
  EOF
}
```

### 20.2 Token Count Summary

| Category | Count |
|----------|-------|
| Block Markers | 25 |
| Assignment Operators | 3 |
| Comparison Operators | 6 |
| Pattern Operators | 2 |
| Range Operators | 4 |
| Delimiters | 10 |
| Identifier Categories | 6 |
| Reserved Enumerations | 10 |
| String Literal Tokens | 6 |
| Other Literal Types | 5 |
| Type Tokens | 10 |
| Special Identifiers | 5 |
| Comments | 2 |
| Whitespace | 4 |
| Version | 1 |
| EOF | 1 |
| **TOTAL** | **100 token types** |

---

## 21. Implementation Notes

### 21.1 Lexer Requirements

1. **Longest match** - Always prefer longest matching token
2. **Context-aware** - Block markers vs operators require context
3. **String interpolation tokenization** - CRITICAL: Must tokenize `{.variable:format}` as separate tokens (STRING_START, STRING_CONTENT, INTERPOLATION_START, IDENTIFIER_VARIABLE, DELIMITER_COLON, FORMAT_IDENTIFIER, INTERPOLATION_END, STRING_END)
4. **String content buffering** - Accumulate static text in buffer, emit STRING_CONTENT when entering interpolation or ending string
5. **Escape sequences** - Must handle `\"`, `\\`, `\n`, `\t` in strings before buffering
6. **Multi-line comments** - Must track nesting depth
7. **Reserved namespace** - `.*.pgvar.*` pattern recognition
8. **Pipeline identifiers** - Must handle `DT.Operation`, `RT.Language`, `TG.Type` as PIPELINE_IDENTIFIER tokens
9. **Case-sensitive** - All identifiers and tokens are case-sensitive
10. **State machine** - Must implement INITIAL, IN_STRING, IN_INTERPOLATION, IN_COMMENT, IN_BLOCK_MARKER states

### 21.2 Error Detection

Lexer should detect and report:
1. Unterminated strings
2. Unterminated multi-line comments
3. Invalid identifier patterns (e.g., `.123`, `#`)
4. Invalid escape sequences in strings
5. Malformed version numbers
6. Unrecognized characters

### 21.3 Token Metadata

Each token should carry:
1. **Type** - Token category/type
2. **Lexeme** - Actual text matched
3. **Line** - Line number (1-indexed)
4. **Column** - Column number (1-indexed)
5. **Length** - Length of lexeme

---

## 22. Example Token Stream

### Input Code

```polyglot
[@] Local@Example:1.0.0
[#] 1
[X]

[|] HelloWorld
[i] .name: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .greeting: pg\string << "Hello, {.name}!"
[o] .greeting: pg\string
[X]
```

### Token Stream

```
BLOCK_PACKAGE_START       '[@]'         line:1 col:1
IDENTIFIER_VARIABLE       'Local'       line:1 col:5
DELIMITER_AT              '@'           line:1 col:10
IDENTIFIER_VARIABLE       'Example'     line:1 col:11
DELIMITER_COLON           ':'           line:1 col:18
VERSION                   '1.0.0'       line:1 col:19
BLOCK_VERSION_ENUM        '[#]'         line:2 col:1
LITERAL_INTEGER           '1'           line:2 col:5
BLOCK_END                 '[X]'         line:3 col:1

BLOCK_PIPELINE_START      '[|]'         line:5 col:1
IDENTIFIER_PIPELINE       '|HelloWorld' line:5 col:5
BLOCK_INPUT               '[i]'         line:6 col:1
IDENTIFIER_VARIABLE       '.name'       line:6 col:5
DELIMITER_COLON           ':'           line:6 col:10
TYPE_NAMESPACE            'pg'          line:6 col:12
DELIMITER_BACKSLASH       '\'           line:6 col:14
TYPE_STRING               'string'      line:6 col:15
BLOCK_TRIGGER             '[t]'         line:7 col:1
SPECIAL_TRIGGER_TYPE      '|T.Call'     line:7 col:5
BLOCK_WRAPPER             '[W]'         line:8 col:1
SPECIAL_WRAPPER           '|W.Polyglot.Scope' line:8 col:5
BLOCK_SEQUENTIAL          '[r]'         line:9 col:1
IDENTIFIER_VARIABLE       '.greeting'   line:9 col:5
DELIMITER_COLON           ':'           line:9 col:14
TYPE_NAMESPACE            'pg'          line:9 col:16
DELIMITER_BACKSLASH       '\'           line:9 col:18
TYPE_STRING               'string'      line:9 col:19
OP_PUSH                   '<<'          line:9 col:26
LITERAL_STRING            '"Hello, {.name}!"' line:9 col:29
BLOCK_OUTPUT              '[o]'         line:10 col:1
IDENTIFIER_VARIABLE       '.greeting'   line:10 col:5
DELIMITER_COLON           ':'           line:10 col:14
TYPE_NAMESPACE            'pg'          line:10 col:16
DELIMITER_BACKSLASH       '\'           line:10 col:18
TYPE_STRING               'string'      line:10 col:19
BLOCK_END                 '[X]'         line:11 col:1
EOF
```

---

## 23. Quick Reference Tables

### 23.1 All Operators

| Operator | Name | Type |
|----------|------|------|
| `<<` | Push | Assignment |
| `>>` | Pull | Assignment |
| `<~` | Default | Assignment |
| `=?` | Equal | Comparison |
| `=!?` | Not equal | Comparison |
| `>?` | Greater | Comparison |
| `<?` | Less | Comparison |
| `=>?` | Greater/equal | Comparison |
| `=<?` | Less/equal | Comparison |
| `*?` | Wildcard | Pattern |
| `re?` | Regex | Pattern |
| `?[` | Range closed | Range |
| `?(` | Range open | Range |
| `?]` | Range half-right | Range |
| `?)` | Range half-left | Range |
| `~ForEach` | For each | Unpack |
| `~Enumerate` | Enumerate | Unpack |
| `~Zip` | Zip | Unpack |
| `~Y.IntoArray` | Join array | Join |
| `~Y.IntoSerial` | Join serial | Join |
| `~Y.IntoSet` | Join set | Join |

### 23.2 All Block Markers

| Marker | Name | Context |
|--------|------|---------|
| `[@]` | Package start | Package |
| `[#]` | Version/Enum | Package/Enum |
| `[X]` | End | Universal |
| `[\|]` | Pipeline start | Pipeline |
| `[i]` | Input | Pipeline |
| `[t]` | Trigger | Pipeline |
| `[Q]` | Queue | Pipeline |
| `[W]` | Wrapper | Pipeline |
| `[\]` | Setup | Pipeline |
| `[/]` | Cleanup | Pipeline |
| `[o]` | Output | Pipeline |
| `[r]` | Sequential | Execution |
| `[<]` | Input binding | Parent block |
| `[>]` | Output binding | Parent block |
| `[p]` | Parallel | Execution |
| `[Y]` | Join | Execution |
| `[b]` | Background | Execution |
| `[s]` | Streaming | Execution |
| `[?]` | Conditional | Execution |
| `[~]` | Body | Conditional/Unpack |
| `[+]` | Boolean OR | Boolean logic |
| `[&]` | Boolean AND | Boolean logic |
| `[-]` | Boolean XOR | Boolean logic |
| `[^]` | Boolean NAND | Boolean logic |
| `[.]` | Boolean NOR | Boolean logic |

### 23.3 All Prefixes

| Prefix | Target | Example |
|--------|--------|---------|
| `.` | Variable | `.user` |
| `#` | Enumeration | `#Status` |
| `\|` | Pipeline | `\|Process` |
| `!` | Error | `!Timeout` |

---

**END OF TOKEN SPECIFICATION**

---

**Next Steps for Lexer Implementation:**

1. Implement token enumeration from Section 20.1
2. Build state machine from Section 19
3. Handle token priority from Section 17
4. Implement error detection from Section 21.2
5. Test with examples from Section 22
6. Use quick reference tables (Section 23) for validation

---

**Document Version:** 1.0
**Polyglot Version:** v0.0.2
**Last Updated:** 2025-11-26
**Status:** Complete ✅
