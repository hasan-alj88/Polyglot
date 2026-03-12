# Polyglot String Literals - Complete Internals

**Version:** v0.0.2
**Date:** 2025-11-26
**Status:** Canonical Reference

---

## Executive Summary

**String literals in Polyglot are NOT primitive values - they are inline pipeline calls.**

```polyglot
"hello"              // Actually: U.String"hello"
"Hello {.name}"      // Actually: U.String"Hello {.name}"
DT.Now""             // Inline pipeline call to DT.Now
DT.Minutes"5"        // Inline pipeline call to DT.Minutes
```

Every string literal `"text"` is syntactic sugar for an inline pipeline call that:
1. Processes formatted argument strings with `{.var:fmt}` placeholders
2. Calls formatting pipelines for each variable
3. Substitutes formatted values back into the string
4. Passes the result to the target pipeline
5. Returns ONE output of ANY type

---

## 1. String Literal Syntax

### Basic Form

```polyglot
Pipeline.Name"formatted argument string"
```

### Special Cases

**Plain strings** (no pipeline prefix):
```polyglot
"hello world"        // Implicitly: U.String"hello world"
```

**Empty parameters**:
```polyglot
DT.Now""             // Empty formatted argument string
Pipeline.Name""      // Empty parameters
```

### Variable Interpolation with Formatting

```polyglot
"Value: {.var:fmt}"
```

**Syntax breakdown:**
- `{` - Start placeholder
- `.var` - Variable reference
- `:fmt` - Optional format identifier
- `}` - End placeholder

**Examples:**
```polyglot
"{.count}"           // No format specifier (default formatting)
"{.number:Hex}"      // Hexadecimal format
"{.price:Currency}"  // Currency format
"{.date:ShortDate}"  // Short date format
```

---

## 2. Processing Pipeline (Background Mechanics)

### Step 1: Extract Placeholders

**Input string:**
```polyglot
"Total: {.count:Decimal} items at {.price:Currency}"
```

**Extracted:**
- Placeholder 1: `.count` with format `Decimal`
- Placeholder 2: `.price` with format `Currency`

### Step 2: Pack into Serial Array

Create `pg\array{pg\serial}` where each `pg\serial` has:
- `value` - The variable value
- `format_identifier` - The format specifier

```polyglot
[
  {value: .count,  format_identifier: "Decimal"},
  {value: .price,  format_identifier: "Currency"}
]
```

### Step 3: Format Each Variable

For each variable, run the formatting pipeline:

**Pattern:** `|U.String.{language}.{type}.{format_identifier}`

**Examples:**
```polyglot
// For {.number:Hex} where .number is pg\int
|U.String.Polyglot.Int.Hex
// Input: 17 → Output: "0x11"

// For {.price:Currency} where .price is pg\float
|U.String.Polyglot.Float.Currency
// Input: 42.50 → Output: "$42.50"

// For {.name} (no format) where .name is pg\string
|U.String.Polyglot.String.Default
// Input: "Alice" → Output: "Alice"
```

**Result:** `pg\array{pg\string}` with all formatted values
```polyglot
["42", "$19.99"]
```

### Step 4: Substitute Back

Replace placeholders with formatted values:

```
"Total: {.count:Decimal} items at {.price:Currency}"
        ↓
"Total: 42 items at $19.99"
```

### Step 5: Pass to Pipeline

Pass the final formatted string to the target pipeline as `.formatted_argument_string`.

---

## 3. String Literal Pipeline Definition

### Signature Requirements

**MANDATORY structure for string literal pipelines:**

```polyglot
[|] Pipeline.Name
[i] .formatted_argument_string: pg\string  // MANDATORY: Exact name required
                                            // NO other inputs allowed

[t] |T.String.Call                          // MANDATORY: String literal trigger

[W] |W.Polyglot.Scope                       // Or any wrapper

// Pipeline logic here

[o] .output: Type                           // MANDATORY: Exactly ONE output
                                            // Can be ANY type
[X]
```

### Key Rules

1. **Input name is fixed**: MUST be `.formatted_argument_string: pg\string`
2. **Single input**: No other inputs allowed
3. **Trigger type**: MUST be `[t] |T.String.Call`
4. **Single output**: Exactly ONE output, can be any type
5. **Return type**: Not limited to strings - can return `pg\dt`, `pg\int`, etc.

---

## 4. Examples

### Example 1: Basic String Literal

**User code:**
```polyglot
[r] .greeting: pg\string << "Hello, World!"
```

**What actually happens:**
```polyglot
// Implicitly calls:
[r] U.String"Hello, World!"
[<] .formatted_argument_string: pg\string << "Hello, World!"
[>] .result: pg\string >> .greeting
```

**Pipeline definition (built-in):**
```polyglot
[|] U.String
[i] .formatted_argument_string: pg\string

[t] |T.String.Call

[W] |W.Polyglot.Scope

[r] .out: pg\string << .formatted_argument_string  // Pass through
[o] .out: pg\string
[X]
```

---

### Example 2: String with Interpolation

**User code:**
```polyglot
[r] .message: pg\string << "User {.user.name} has {.user.points:Decimal} points"
```

**Processing:**

1. **Extract placeholders:**
   - `{.user.name}` - no format
   - `{.user.points:Decimal}` - Decimal format

2. **Pack into array:**
   ```polyglot
   [
     {value: .user.name,   format_identifier: "Default"},
     {value: .user.points, format_identifier: "Decimal"}
   ]
   ```

3. **Format each:**
   ```polyglot
   // For .user.name (pg\string)
   [r] |U.String.Polyglot.String.Default
   [<] .value << .user.name
   [>] .formatted >> "Alice"

   // For .user.points (pg\int)
   [r] |U.String.Polyglot.Int.Decimal
   [<] .value << .user.points
   [>] .formatted >> "1,234"
   ```

4. **Substitute:**
   ```
   "User Alice has 1,234 points"
   ```

5. **Pass to U.String:**
   ```polyglot
   [r] U.String"User Alice has 1,234 points"
   [>] .out >> .message
   ```

---

### Example 3: Custom String Literal Pipeline

**User-defined pipeline:**
```polyglot
[|] Hello.World
[i] .formatted_argument_string: pg\string  // MANDATORY name

[t] |T.String.Call                         // MANDATORY trigger

[W] |W.Polyglot.Scope

[r] .out: pg\string << "Hello {}"
[o] .out: pg\string
[X]
```

**Usage:**
```polyglot
[r] .greeting: pg\string << Hello.World"from custom pipeline"
// Result: "Hello from custom pipeline"
```

---

### Example 4: Non-String Output (DateTime)

**DT.Now pipeline:**
```polyglot
[|] DT.Now
[i] .formatted_argument_string: pg\string  // MUST be present (even if empty)

[t] |T.String.Call                         // MANDATORY trigger

[W] RT.Rust"chrono::Utc::now"              // Native datetime

[o] .timestamp: pg\dt                      // Output is pg\dt, NOT pg\string!
[X]
```

**Usage:**
```polyglot
[r] .now: pg\dt << DT.Now""
// Returns current timestamp as pg\dt
```

---

### Example 5: Duration with Parameter

**DT.Minutes pipeline:**
```polyglot
[|] DT.Minutes
[i] .formatted_argument_string: pg\string  // Contains "5"

[t] |T.String.Call

[W] |W.Polyglot.Scope

// Parse string to int, create duration
[r] .minutes_int: pg\int << 0              // Parse .formatted_argument_string
[r] .duration: pg\dt << 0                  // Create duration from minutes

[o] .duration: pg\dt                       // Returns pg\dt
[X]
```

**Usage:**
```polyglot
[r] .timeout: pg\dt << DT.Minutes"5"
// .formatted_argument_string = "5"
// Result: 5-minute duration as pg\dt
```

---

### Example 6: Hexadecimal Formatting

**User code:**
```polyglot
[r] .number: pg\int << 17
[r] .hex_str: pg\string << "0x{.number:Hex}"
```

**Processing:**

1. **Extract:** `{.number:Hex}`

2. **Call formatter:**
   ```polyglot
   [r] |U.String.Polyglot.Int.Hex
   [<] .value: pg\int << 17
   [>] .formatted: pg\string >> "11"
   ```

3. **Substitute:**
   ```
   "0x11"
   ```

4. **Result:**
   ```polyglot
   .hex_str = "0x11"
   ```

---

## 5. Format Identifier Naming Convention

### Standard Pattern

```
|U.String.{language}.{type}.{format_identifier}
```

### Components

| Component | Description | Examples |
|-----------|-------------|----------|
| `language` | Source language | `Polyglot`, `Python`, `Rust`, `JavaScript` |
| `type` | Variable type | `Int`, `Float`, `String`, `Bool`, `DateTime` |
| `format_identifier` | Format specifier | `Hex`, `Binary`, `Currency`, `ShortDate` |

### Common Format Pipelines

**Integer formatting:**
```polyglot
|U.String.Polyglot.Int.Hex       // Hexadecimal: 255 → "FF"
|U.String.Polyglot.Int.Binary    // Binary: 5 → "101"
|U.String.Polyglot.Int.Decimal   // Decimal with commas: 1000 → "1,000"
|U.String.Polyglot.Int.Ordinal   // Ordinal: 1 → "1st", 2 → "2nd"
```

**Float formatting:**
```polyglot
|U.String.Polyglot.Float.Currency    // Currency: 42.5 → "$42.50"
|U.String.Polyglot.Float.Percent     // Percent: 0.75 → "75%"
|U.String.Polyglot.Float.Scientific  // Scientific: 1000 → "1.0e3"
```

**DateTime formatting:**
```polyglot
|U.String.Polyglot.DateTime.ShortDate   // 2024-01-15 → "01/15/24"
|U.String.Polyglot.DateTime.LongDate    // 2024-01-15 → "January 15, 2024"
|U.String.Polyglot.DateTime.Time24      // → "14:30:00"
|U.String.Polyglot.DateTime.Time12      // → "2:30 PM"
```

**Default formatting:**
```polyglot
|U.String.Polyglot.String.Default    // Pass through
|U.String.Polyglot.Int.Default       // ToString()
|U.String.Polyglot.Float.Default     // ToString()
```

---

## 6. Auto-Await Behavior

String literal processing triggers **auto-await** on all interpolated variables.

**Example:**
```polyglot
[r] |FetchUser
[>] .user: #UserProfile >> .user_data    // .user_data becomes Pending

[r] .message: pg\string << "User: {.user_data.name}"
// Runtime blocks here until .user_data is Ready or Faulted
```

**Flow:**
1. Extract `.user_data.name` from string
2. Check state: `Pending`
3. **Auto-await**: Block until `Ready` or `Faulted`
4. If `Ready`: Continue formatting
5. If `Faulted`: Propagate error

---

## 7. Lexer/Parser Implications

### Token Recognition

**The lexer sees:**
```
IDENTIFIER_PIPELINE    "DT"
DOT                    "."
IDENTIFIER             "Minutes"
STRING_LITERAL         "\"5\""
```

**Combined as:** `DT.Minutes"5"` → **INLINE_PIPELINE_CALL**

### Grammar Rule

```ebnf
inline_pipeline_call ::= PIPELINE_NAME STRING_LITERAL
PIPELINE_NAME ::= IDENTIFIER ('.' IDENTIFIER)*
STRING_LITERAL ::= '"' [^"]* '"'
```

### Important Distinctions

**These are DIFFERENT:**

1. **Variable assignment from string:**
   ```polyglot
   [r] .name: pg\string << "Alice"
   // Calls: U.String"Alice"
   ```

2. **Inline pipeline with string parameter:**
   ```polyglot
   [r] .time: pg\dt << DT.Now""
   // Calls: DT.Now with empty formatted_argument_string
   ```

3. **Pipeline call (NOT string literal):**
   ```polyglot
   [r] |ProcessData
   [<] .input: pg\string << "data"
   // Regular pipeline call, .input uses U.String"data"
   ```

---

## 8. String Literal vs Regular Pipeline Calls

### String Literal Inline Pipeline

**Syntax:**
```polyglot
Pipeline.Name"parameters"
```

**Characteristics:**
- Uses `"..."` string literal syntax
- Single input: `.formatted_argument_string`
- Trigger: `|T.String.Call`
- Can be called inline in expressions
- Processed by string formatting system

### Regular Pipeline Call

**Syntax:**
```polyglot
[r] |Pipeline.Name
[<] .input1: Type << value
[<] .input2: Type << value
[>] .output: Type >> .dest
```

**Characteristics:**
- Uses `[r]` block marker
- Multiple inputs allowed
- Trigger: `|T.Call` (for called pipelines)
- Cannot be inline in expressions
- No automatic string formatting

---

## 9. Edge Cases and Special Behaviors

### Empty String Literal

```polyglot
""               // U.String""
DT.Now""         // Empty parameter to DT.Now
```

All inline pipelines MUST include `""` even with no parameters.

**Why is the empty string mandatory?**

Because inline pipeline calls are invoked via string literal syntax. The string - even if empty - is the syntactic marker that distinguishes:
- `DT.Now""` (inline pipeline call - VALID)
- `DT.Now` (bare pipeline reference - INVALID)

Without the string literal syntax, the parser cannot differentiate between:
1. **Pipeline call:** `DT.Now""` → Execute the pipeline
2. **Pipeline reference:** `DT.Now` → Reference to the pipeline itself (not a call)

The empty string `""` serves as the mandatory **invocation operator** for inline pipeline calls, similar to how `()` works in function calls in other languages.

### Escaping Braces

**To include literal `{` or `}` in output:**
```polyglot
"{{.var}}"       // Escaped: outputs "{.var}" literally
"Value: {{ }}"   // Escaped: outputs "Value: { }"
```

### Nested Interpolation (Not Supported)

```polyglot
"Outer {.var1 + {.var2}}"    // ❌ INVALID - no nesting allowed
```

**Workaround:**
```polyglot
[r] .combined: pg\string << "{.var1}{.var2}"
[r] .final: pg\string << "Outer {.combined}"
```

### Format Identifier Without Variable

```polyglot
"{:Hex}"         // ❌ INVALID - must have variable
```

---

## 10. Implementation Checklist

### For Lexer

- [ ] Recognize `PIPELINE.NAME"string"` as single token or token sequence
- [ ] Distinguish between inline pipeline calls and regular strings
- [ ] Handle escape sequences inside string literals
- [ ] Recognize empty string parameters: `""`

### For Parser

- [ ] Parse inline pipeline call syntax
- [ ] Extract placeholder patterns: `{.var:fmt}`
- [ ] Build AST node for string literal processing
- [ ] Validate format identifier syntax

### For Compiler/Runtime

- [ ] Extract all `{.var:fmt}` placeholders
- [ ] Pack into `pg\array{pg\serial}` structure
- [ ] Call formatting pipelines: `|U.String.{lang}.{type}.{fmt}`
- [ ] Substitute formatted values back
- [ ] Pass to target pipeline as `.formatted_argument_string`
- [ ] Implement auto-await for interpolated variables
- [ ] Handle escaped braces: `{{` → `{`

### For Type Checker

- [ ] Verify target pipeline has correct signature
- [ ] Check input: `.formatted_argument_string: pg\string`
- [ ] Check trigger: `|T.String.Call`
- [ ] Check single output (any type)
- [ ] Validate format identifiers exist for variable types

---

## 11. Common Patterns

### Pattern 1: Simple Logging

```polyglot
[r] .log: pg\string << "Processing {.count} items"
```

### Pattern 2: Current Timestamp

```polyglot
[r] .now: pg\dt << DT.Now""
```

### Pattern 3: Duration Creation

```polyglot
[r] .timeout: pg\dt << DT.Minutes"30"
[r] .delay: pg\dt << DT.Hours"2"
[r] .window: pg\dt << DT.Days"7"
```

### Pattern 4: Formatted Output

```polyglot
[r] .hex: pg\string << "Value: 0x{.number:Hex}"
[r] .price: pg\string << "Total: {.amount:Currency}"
[r] .percent: pg\string << "Progress: {.ratio:Percent}"
```

### Pattern 5: Custom String Pipeline

```polyglot
[|] Template.Greeting
[i] .formatted_argument_string: pg\string
[t] |T.String.Call
[W] |W.Polyglot.Scope
[r] .out: pg\string << "Hello, {.formatted_argument_string}!"
[o] .out: pg\string
[X]

// Usage:
[r] .greeting: pg\string << Template.Greeting"Alice"
// Result: "Hello, Alice!"
```

---

## 12. Critical Rules Summary

1. ✅ **All string literals are inline pipeline calls**
2. ✅ **Plain `"text"` implicitly calls `U.String"text"`**
3. ✅ **Format: `Pipeline.Name"formatted_argument_string"`**
4. ✅ **Interpolation: `{.var:fmt}` syntax**
5. ✅ **Input name: `.formatted_argument_string` (fixed)****
6. ✅ **Trigger: `|T.String.Call` (mandatory)**
7. ✅ **Single output: Any type, not just strings**
8. ✅ **Empty params: `""` required even with no parameters**
9. ✅ **Auto-await: Triggered on all interpolated variables**
10. ✅ **Format pipelines: `|U.String.{lang}.{type}.{fmt}`**

---

## 13. FAQ

### Q: Can I use parentheses for parameters?

**A: No.** Only string literal syntax is allowed.

```polyglot
DT.Minutes(5)     // ❌ INVALID
DT.Minutes"5"     // ✅ CORRECT
```

### Q: Can I omit the `""` for pipelines with no parameters?

**A: No.** Empty string is mandatory.

```polyglot
DT.Now            // ❌ INVALID
DT.Now""          // ✅ CORRECT
```

### Q: Can string literal pipelines return non-string types?

**A: Yes!** The output can be any type.

```polyglot
DT.Now""          // Returns pg\dt
DT.Minutes"5"     // Returns pg\dt
"hello"           // Returns pg\string
```

### Q: Can I define my own format identifiers?

**A: Yes!** Define pipelines following the naming convention:

```polyglot
[|] U.String.Polyglot.Int.MyCustomFormat
[i] .value: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope
// Custom formatting logic
[o] .formatted: pg\string
[X]
```

### Q: What happens if a format pipeline doesn't exist?

**A: Compile error.** The compiler constructs the pipeline name `|U.String.{lang}.{type}.{fmt}` and verifies it exists in the pipeline registry. If not found, compilation fails with a helpful error message.

**Example error:**
```
Error: Pipeline not found: |U.String.Polyglot.Int.InvalidFormat
Note: Format identifier 'InvalidFormat' not defined for type 'pg\int'
Suggestion: Available formats for 'pg\int': [Hex, Binary, Decimal, Ordinal, ...]
```

### Q: What if I use the wrong format for a type (e.g., `{.string_var:Hex}`)?

**A: Compile error - missing pipeline definition.** Since format pipelines are namespaced by type, using `Hex` (which expects `pg\int`) on a `pg\string` variable will result in the compiler looking for `|U.String.Polyglot.String.Hex`, which doesn't exist.

**Example:**
```polyglot
[r] .name: pg\string << "Alice"
[r] .msg: pg\string << "{.name:Hex}"  // ❌ ERROR!
```

**Compiler output:**
```
Error: Pipeline not found: |U.String.Polyglot.String.Hex
Note: Format identifier 'Hex' expects type 'pg\int', but variable '.name' is 'pg\string'
Suggestion: Available formats for 'pg\string': [ToUpper, ToLower, Trim, ...]
```

**Why this works elegantly:** No special type-checking logic needed - format pipelines are naturally namespaced by type, so incorrect type-format combinations simply don't exist as pipeline definitions.

---

**Document Version:** 1.0
**Last Updated:** 2025-11-26
**Status:** Canonical Reference ✅
