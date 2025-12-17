---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/syntax/operators.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Operators Reference

**Version:** 0.0.6
**Complete Reference:** All Operators with Examples
**Last Updated:** 2025-12-03

**Philosophy:** Polyglot's operators encode **direction** and perform **dual operations** simultaneously. Think in terms of data flow, not traditional assignment.

---

## Operator Table

| Operator | Name                    | Category        | Purpose                   | Example           |
| -------- | ----------------------- | --------------- | ------------------------- | ----------------- |
| `\|`     | Pipeline                | Call            | Call pipeline             | `\|ProcessData`   |
| `~`      | Unpack                  | Iteration       | Expand collections        | `~ForEach`        |
| `~Y.`    | Join                    | Synchronization | Join parallel tasks       | `~Y.IntoArray`    |
| `@`      | Package                 | Import          | Package access            | `@utils`          |
| `#`      | Enumeration             | Type            | Enum identifier           | `#UserRole`       |
| `!`      | Error                   | Type            | Error identifier          | `!NetworkTimeout` |
| `.`      | Variable                | Identifier      | Variable prefix           | `.myvar`          |
| `<`      | Input Argument          | Pipeline I/O    | Pipeline input parameter  | `<input`          |
| `>`      | Output Argument         | Pipeline I/O    | Pipeline output parameter | `>output`         |
| `<<`     | Push/Pull Left          | Data Flow       | Bidirectional flow (→    | `.x << 42`        |
| `<~`     | Push/Pull Default Left  | Data Flow       | Default bidirectional (→ | `.x <~ 30`        |
| `>>`     | Push/Pull Right         | Data Flow       | Bidirectional flow (←    | `.x >> .result`   |
| `~>`     | Push/Pull Default Right | Data Flow       | Default bidirectional (← | `.x ~> .source`   |
| `=?`     | Equal                   | Comparison      | Equality check            | `.x =? .y`        |
| `=!?`    | Not Equal               | Comparison      | Inequality check          | `.x =!? .y`       |
| `>?`     | Greater                 | Comparison      | Greater than              | `.x >? 10`        |
| `<?`     | Less                    | Comparison      | Less than                 | `.x <? 100`       |
| `=>?`    | Greater/Equal           | Comparison      | Greater or equal          | `.x =>? 5`        |
| `=<?`    | Less/Equal              | Comparison      | Less or equal             | `.x =<? 20`       |
| `?[a,b]` | Range Inclusive         | Range           | Inclusive range           | `.x ?[0,100]`     |
| `?(a,b` | Range Exclusive         | Range           | Exclusive range           | `.x ?(0,100`     |
| `*?`     | Wildcard                | Pattern         | Match any                 | `[?] *?`          |
| `re?`    | Regex                   | Pattern         | Regex match               | `.str re? "\\d+"` |
| `{.var` | Interpolation           | String          | Variable in string        | `"{.name"`       |
| `+"`     | Concatenation           | String          | Continue string           | `[*] +"more"`     |
| `:`      | Type                    | Declaration     | Type separator            | `.x:pg.int`      |
| `\`      | Type Namespace          | Declaration     | Namespace separator       | `:pg.string`       |

---

## Pipeline Operator `|`

Calls pipelines. **MUST** start with `|` prefix.

```polyglot
|ProcessData                       // Pipeline call
|T.Daily                           // Trigger
|W.Python                          // Wrapper
|Q.Priority                        // Queue
```

---

## Unpack Operator `~`

Expands collections for iteration.

```polyglot
[p] ~ForEach                       // Unpack each item
[<] .items
[>] .item
[~][r] |ProcessItem
[~]
```

**Common Unpack Operators:**
- `~ForEach` - Iterate over collection
- `~Enumerate` - Iterate with index
- `~Zip` - Combine collections

---

## Join Operator `~Y.`

Synchronizes and collects parallel results.

```polyglot
[Y] ~Y.IntoArray                   // Join into array
[<] .results
[>] .combined

[Y] ~Y.IntoSet                     // Join into set
[Y] ~Y.IntoSerial                  // Join into serial
```

---

## Package Operator `@`

Accesses packages.

```polyglot
[@] @Local::MyProject:1.0.0.0
[<] @utils << @Community::StringUtils:2.0.0.0
[X]
```

---

## Pipeline I/O Operators `<` and `>`

**Critical Distinction:** Pipeline arguments use `<` and `>` prefixes, NOT `.` prefix like variables.

### `<` Input Argument Prefix

Identifies a pipeline **input argument** (parameter passed TO the pipeline.

```polyglot
[r] |HttpGet
[<] <url:pg.string << .my_url
[>] >response:pg.dict >> .result
```

**Namespace:** `<url` lives in `|HttpGet.Inputs.url`, NOT in the caller's scope.

**Characteristics:**
- Used with `[<]` block marker
- Receives data from caller's variables (using `<<`
- Not accessible after pipeline call
- Distinct from `.variable` (scope variable

**Example:**
```polyglot
[r] |ProcessData
[<] <input:pg.string << .data          // Input argument <input
[<] <config: #Config << .settings       // Input argument <config
[>] >result:pg.dict >> .output         // Output argument >result
```

**Reading:** "Input argument `<input` receives value from variable `.data`"

---

### `>` Output Argument Prefix

Identifies a pipeline **output argument** (value returned FROM the pipeline.

```polyglot
[r] |FetchData
[>] >data:pg.string >> .result
[>] >status:pg.int >> .status_code
```

**Namespace:** `>data` lives in `|FetchData.Outputs.data`, NOT in the caller's scope.

**Characteristics:**
- Used with `[>]` block marker
- Pushes data into caller's variables (using `>>`
- Becomes Pending during async execution
- Becomes Final or Faulted when pipeline completes
- Distinct from `.variable` (scope variable

**Example:**
```polyglot
[r] |Calculate
[<] <a:pg.int << .x
[<] <b:pg.int << .y
[>] >sum:pg.int >> .total              // Output argument >sum
[>] >product:pg.int >> .mult           // Output argument >product

// .total and .mult are Pending until |Calculate completes
```

**Reading:** "Output argument `>sum` pushes value into variable `.total`"

---

### Complete Pipeline Call Example

```polyglot
[r] |Transform
[<] <input:pg.string << .raw_data      // < prefix: input argument
[<] <format:pg.string << "json"        // < prefix: input argument
[>] >output:pg.dict >> .parsed         // > prefix: output argument
[>] >errors:pg.array.pg.string >> .errs  // > prefix: output argument

// After call:
// - <input and <format DON'T exist (pipeline arguments, not variables
// - .parsed and .errs DO exist (variables in current scope
```

---

### Why Not Use `.` Prefix?

**Problem with `.` prefix:**
```polyglot
[r] |Process
[<] <input:pg.string     // CONFUSING: Looks like a variable
[>] >output:pg.dict >> .result
// Is .input accessible here? NO! It's a pipeline argument.
```

**Clarity with `<` and `>` prefixes:**
```polyglot
[r] |Process
[<] <input:pg.string     // CLEAR: Pipeline input argument
[>] >output:pg.dict >> .result
// <input is obviously not a variable in current scope
```

**Namespace Distinction:**
- `.variable` → Current scope (`Scope.variable`
- `<input` → Pipeline input (`|Pipeline.Inputs.input`
- `>output` → Pipeline output (`|Pipeline.Outputs.output`

---

### Operator Summary

| Prefix | Purpose | Scope | Example |
|--------|---------|-------|---------|
| `.` | Variable | Current scope | `.myvar:pg.int << 42` |
| `<` | Input argument | Pipeline inputs | `<input:pg.string` |
| `>` | Output argument | Pipeline outputs | `>result:pg.dict` |

---

## Push/Pull Data Flow Operators

**Critical Concept:** These operators are **NOT** traditional assignment. They perform **dual operations simultaneously** and encode **direction**.

### Bidirectional Reading

Every Push/Pull operator can be read in **two ways**:

```polyglot
.x >> .y
```

**Reading 1 (Push perspective:** "`.x` pushed into `.y`" (data flows from x to y
**Reading 2 (Pull perspective:** "`.y` pulled from `.x`" (y gets data from x

**Both readings are correct!** The operator encodes direction and performs both operations.

---

### `<<` Push/Pull Left

**Dual operation:** Data flows **right-to-left** (←

```polyglot
[r] .x << 42                         // Read: "42 pushed into .x" OR ".x pulled from 42"
[r] .x:pg.int << .y                 // Read: ".y pushed into .x" OR ".x pulled from .y"
```

**Perspectives:**
- **From right:** Value/expression pushes INTO left variable
- **From left:** Variable pulls value FROM right expression

**State transition:** Pending → Final (if sync or stays Pending (if async

**Examples:**

```polyglot
[r] .version:pg.string << "1.0.0"   // Literal pushes in / Variable pulls from literal
[r] .count:pg.int << 42             // Final state immediately

[r] |Fetch                           // Pipeline pushes in / Variable pulls from pipeline
[>] >data:pg.string >> .result      // Pending → Final (when complete
```

---

### `<~` Push/Pull Default Left

**Dual operation:** Default value flows **right-to-left** (←, can override once

```polyglot
[i] .timeout:pg.int <~ 30           // Read: "30 pushed into .timeout" OR ".timeout pulled from 30"
```

**Perspectives:**
- **From right:** Default literal pushes INTO variable
- **From left:** Variable pulls default FROM literal

**State:** Creates **Default** (override once, then Final

**Examples:**

```polyglot
[i] .timeout:pg.int <~ 30           // Default with value 30
[i] .retries:pg.int <~ 3            // Default with value 3

[r] .timeout << 60                   // Override: Default → Final (value: 60
```

---

### `>>` Push/Pull Right

**Dual operation:** Data flows **left-to-right** (→

```polyglot
.x >> .y
```

**Read:** "`.x` pushed into `.y`" OR "`.y` pulled from `.x`"

**Perspectives:**
- **From left:** Variable pushes INTO right target
- **From right:** Target pulls value FROM left variable

**Auto-await:** If left variable is Pending, waits until Final or Faulted

**Examples:**

```polyglot
[r] |HttpGet
[<] <url:pg.string
[>] >response:pg.dict >> .http_response  // Pending

// Using the response (auto-awaits
[r] .status:pg.int << U.Dict.Get"{.http_response, 'status_code'"
// Auto-awaits .http_response if still Pending

// Read the >> operation:
// 1. "|HttpGet output argument >response pushed into .http_response"
// 2. ".http_response pulled from |HttpGet output argument >response"
```

---

### `~>` Push/Pull Default Right

**Dual operation:** Default value flows **left-to-right** (→, can override once

```polyglot
.x ~> .source
```

**Read:** "`.source` pushed into `.x`" OR "`.x` pulled from `.source`"

**Perspectives:**
- **From left:** Source variable pushes INTO target
- **From right:** Target pulls default FROM source

**State:** Creates **Default** (override once, then Final

**Examples:**

```polyglot
[i] .defaultConfig: #Config <~ .config   // Default from source
[i] .timeout:pg.int <~ .settings.timeout  // Default from source

// Read both ways:
// 1. ".defaultConfig pushed into .config"
// 2. ".config pulled from .defaultConfig"

[r] .timeout << 60                   // Override: Default → Final
```

---

### Operator Pairs Summary

| Operator | Direction | Dual Reading | State |
|----------|-----------|--------------|-------|
| `<<` | Right→Left (← | "Value pushed into Var" / "Var pulled from Value" | Final or Pending |
| `<~` | Right→Left (← | "Default pushed into Var" / "Var pulled from Default" | Default |
| `>>` | Left→Right (→ | "Source pushed into Target" / "Target pulled from Source" | Final or Pending |
| `~>` | Left→Right (→ | "Source pushed into Var" / "Var pulled from Source" | Default |

**Key Insight:** Direction is encoded in the operator (`<<` vs `>>`, NOT in left/right position. This is fundamentally different from traditional assignment (`=`.

---

### Critical Rule: Operator Direction MUST Match Data Flow

**Block markers dictate allowed operators:**

| Marker | Purpose | Data Flow | ✅ Allowed | ❌ Forbidden |
|--------|---------|-----------|-----------|-------------|
| `[i]` | Input | INTO pipeline (← | `<<`, `<~` | `>>`, `~>` |
| `[<]` | Pipeline input | INTO pipeline (← | `<<`, `<~` | `>>`, `~>` |
| `[>]` | Pipeline output | OUT OF pipeline (→ | `>>`, `~>` | `<<`, `<~` |

**Examples:**

```polyglot
// ✅ CORRECT: [i] uses << or <~
[i] .timeout:pg.int <~ 30               // Data flows INTO input
[i] .defaultConfig: #Config <~ .config  // Data flows INTO input

// ✅ CORRECT: [<] uses << or <~
[r] |ProcessData
[<] <input:pg.string << .data           // Data flows INTO pipeline
[<] <config: #Config <~ .default         // Data flows INTO pipeline

// ✅ CORRECT: [>] uses >> or ~>
[r] |FetchData
[>] >output:pg.string >> .result        // Data flows OUT OF pipeline
[>] >config: #Config ~> .fallback        // Data flows OUT with default

// ❌ WRONG: Direction mismatch
[i] .config: #Config ~> .source          // WRONG: ~> is outward!
[<] <input:pg.string >> .data           // WRONG: >> is outward!
[>] >output:pg.string << .result        // WRONG: << is inward!
```

**Mnemonic:** Arrows point to data destination:
- `<<` and `<~` point LEFT (← into variable/input
- `>>` and `~>` point RIGHT (→ out of pipeline

---

## Comparison Operators

**Note:** Numeric comparisons (`>?`, `<?`, `=>?`, `=<?` only work with `:pg.int`, `:pg.float`, `:pg.dt`, `:pg.string` (lexicographic.

### `=?` Equal To

```polyglot
[?] .status =? #Status.Active
[~][r] |ProcessActive
[~]
```

### `=!?` Not Equal To

```polyglot
[?] .value =!? #None
[~][o] .value:pg.int
[~]
```

### `>?` Greater Than

```polyglot
[?] .count >? 100
[~][o] .high:pg.bool << #Boolean.True
[~]
```

### `<?` Less Than

```polyglot
[?] .age <? 18
[~][o] .minor:pg.bool << #Boolean.True
[~]
```

### `=>?` Greater Than or Equal

```polyglot
[?] .score =>? 90
[~][o] .grade:pg.string << "A"
[~]
```

### `=<?` Less Than or Equal

```polyglot
[?] .retries =<? 3
[~][r] |AttemptAgain
[~]
```

---

## Range Operators

### `?[a,b]` Inclusive Range

Both endpoints included.

```polyglot
[?] .value ?[0,100]                // 0 <= value <= 100
[~][o] .valid:pg.bool << #Boolean.True
[~]
```

### `?(a,b` Exclusive Range

Both endpoints excluded.

```polyglot
[?] .age ?(0,120                  // 0 < age < 120
[~][o] .valid:pg.bool << #Boolean.True
[~]
```

---

## Pattern Matching Operators

### `*?` Wildcard Match (Catch-All

Matches anything. **REQUIRED** in conditional blocks.

```polyglot
[?] .value >? 100
[~][o] .result:pg.string << "high"
[~]

[?] *?                             // REQUIRED catch-all
[~][o] .result:pg.string << "other"
[~]
```

### `re?` Regex Match

Matches regular expression pattern.

```polyglot
[?] .email re? "^[\\w.]+@[\\w.]+\\.[a-z]{2,$"
[~][o] .valid:pg.bool << #Boolean.True
[~]
```

---

## String Operators

### `{.var` String Interpolation

Embeds variable in string.

```polyglot
[r] .name:pg.string << "Alice"
[r] .greeting:pg.string << "Hello, {.name!"  // "Hello, Alice!"
```

**With format specifiers:**
```polyglot
[r] .count:pg.int << 255
[r] .hex:pg.string << "{.count:Hex"          // "FF"
[r] .binary:pg.string << "{.count:Binary"    // "11111111"
```

### `+"` String Concatenation (Line Continuation

Continues string across lines.

```polyglot
[r] .long_message:pg.string << "This is line one"
[*] +"This is line two"
[*] +"This is line three"
```

---

## Type Operators

### `:` Type Declaration

Declares variable type.

```polyglot
.user:pg.string                   // Type:pg.string
.count:pg.int                     // Type:pg.int
.data: rs\Vec                      // Type: rs\Vec (Rust
```

### `\` Type Namespace Separator

Separates namespace from type. **REQUIRED - use backslash, not slash!**

```polyglot
:pg.string                          // ✓ Correct
:pg.int                             // ✓ Correct
rs\HashMap                         // ✓ Correct
py\dict                            // ✓ Correct

pg/string                          // ✗ WRONG: Forward slash
pg:string                          // ✗ WRONG: Colon
```

---

## Operator Precedence

Polyglot has minimal expression syntax. Operators are evaluated **in context**:

1. **Type operators** (`:`, `\` - During type resolution
2. **PUSH operators** (`<<`, `<~` - At variable assignment
3. **PULL operators** (`>>` - At variable extraction
4. **Comparison operators** (`=?`, `>?`, etc. - In conditionals
5. **Boolean operators** (`[&]`, `[+]`, etc. - In logic chains

**No arithmetic operators** - use foreign runtime (Python/Rust for math.

---

## Common Operator Patterns

### Push and Pull Pattern

```polyglot
[r] |FetchUser
[<] <id:pg.int << .user_id        // PUSH into pipeline
[>] >user: #UserProfile >> .result // PULL from pipeline
```

### Conditional with Boolean Logic

```polyglot
[?] .age =>? 18
[&] .verified =? #Boolean.True     // AND operator
[~][r] |GrantAccess
[~]
```

### Range Validation

```polyglot
[?] .score ?[0,100]                // Inclusive range
[~][o] .valid:pg.bool << #Boolean.True
[~]

[?] *?                             // Catch-all
[~][o] .valid:pg.bool << #Boolean.False
[~]
```

### Collection Unpacking

```polyglot
[p] ~ForEach
[<] .items
[>] .item
[~][r] |ProcessItem
[~][<] <data:pg.any << .item
[~][>] >result:pg.any >> .processed
[~]
[~][Y] ~Y.IntoArray
[~][<] .processed
[~][>] .all_results
[~]
```

---

## Reserved Operators

These operators have special meaning and cannot be overridden:

- `|` - Pipeline call
- `~` - Unpack/expansion
- `#` - Enumeration
- `!` - Error
- `@` - Package
- `.` - Variable prefix
- `<` - Input argument prefix
- `>` - Output argument prefix
- `<<` - PUSH immediate
- `<~` - PUSH default
- `>>` - PULL
- All comparison operators
- All pattern matching operators

---

## See Also

- [Block Markers](block-markers.md - All block markers
- [Type System](type-system.md - Type declarations
- [Syntax Overview](overview.md - Core principles

---

**Next:** [Type System →](type-system.md
