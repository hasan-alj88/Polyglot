# String Literal Tokenization Strategy

**Date:** 2025-11-26
**Purpose:** Define how the lexer tokenizes string literals with interpolations
**Status:** Canonical Reference ✅

---

## Overview

String literals in Polyglot follow the pattern:

```
{pipeline}+"+Formatted_argument_string+"
```

The `formatted_argument_string` can contain any number of interpolations:

```
{value or .variable: format_identifier}
```

**CRITICAL DECISION:** The lexer MUST tokenize interpolation components, not leave them as raw string content for the parser to extract.

---

## Token Sequence Approach

The lexer outputs a **sequence of tokens** that represent the structure of the string literal, including interpolations.

### New Token Types Required

| Token Type | Description | Example |
|------------|-------------|---------|
| `STRING_START` | Opening quote of string literal | `"` |
| `STRING_CONTENT` | Static text between interpolations | `"Count: "` |
| `STRING_END` | Closing quote of string literal | `"` |
| `INTERPOLATION_START` | Opening brace of interpolation | `{` |
| `INTERPOLATION_END` | Closing brace of interpolation | `}` |
| `FORMAT_IDENTIFIER` | Format specifier in interpolation | `Hex`, `Currency` |

**Note:** `IDENTIFIER_VARIABLE` and `DELIMITER_COLON` already exist in the token specification.

---

## Examples

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
- Implicit call to `U.String"Hello, World!"`
- No PIPELINE_IDENTIFIER token since it's implicit

---

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
- Static text `"Count: "` is STRING_CONTENT
- Variable `.num` uses default formatting (no format identifier)
- Parser recognizes pattern and calls implicit `U.String`

---

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
- Parser will resolve to `|U.String.Polyglot.Int.Hex` based on type inference

---

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
- Mix of formatted (`.count:Decimal`) and unformatted (`.name`) interpolations

---

### Example 5: Explicit Pipeline Call with Empty String

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
- Explicit pipeline identifier before string
- Empty string is mandatory invocation operator

---

### Example 6: Explicit Pipeline Call with Parameter

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
- `"5"` is the formatted_argument_string parameter
- No interpolation in this case, just static content

---

### Example 7: Explicit Pipeline Call with Interpolated Parameter

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
- Pipeline identifier followed by string with interpolation
- Parser passes `.timestamp` value to `|DT.ToUTC` pipeline

---

### Example 8: Complex Interpolation

**Input:**
```polyglot
"Price: {.amount:Currency}, Tax: {.tax:Currency}, Total: {.total:Currency}"
```

**Token Sequence:**
```
STRING_START
STRING_CONTENT("Price: ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".amount")
DELIMITER_COLON
FORMAT_IDENTIFIER("Currency")
INTERPOLATION_END
STRING_CONTENT(", Tax: ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".tax")
DELIMITER_COLON
FORMAT_IDENTIFIER("Currency")
INTERPOLATION_END
STRING_CONTENT(", Total: ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".total")
DELIMITER_COLON
FORMAT_IDENTIFIER("Currency")
INTERPOLATION_END
STRING_END
```

**Notes:**
- Three interpolations, all with same format identifier
- Parser will call `|U.String.Polyglot.Float.Currency` for each (assuming float type)

---

### Example 9: Escape Sequences in String Content

**Input:**
```polyglot
"Path: C:\\Users\\{.username}\\Documents"
```

**Token Sequence:**
```
STRING_START
STRING_CONTENT("Path: C:\\Users\\")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".username")
INTERPOLATION_END
STRING_CONTENT("\\Documents")
STRING_END
```

**Notes:**
- Escape sequences (`\\`) remain in STRING_CONTENT
- Lexer processes escapes, parser receives processed content
- Backslashes are part of the static text, not delimiters

---

## Lexer State Machine

### States

```
INITIAL              → Default state
IN_STRING            → Inside string literal (after STRING_START)
IN_INTERPOLATION     → Inside interpolation (after INTERPOLATION_START)
```

### State Transitions

```
INITIAL state:
  See '"' → Emit STRING_START
         → Enter IN_STRING state
         → Initialize content buffer

  See 'DT.', 'RT.', 'TG.' → Emit PIPELINE_IDENTIFIER
                           → Stay in INITIAL
                           → Next '"' will trigger STRING_START

IN_STRING state:
  See '{' → Emit STRING_CONTENT (buffered content)
         → Emit INTERPOLATION_START
         → Enter IN_INTERPOLATION state
         → Clear buffer

  See '"' → Emit STRING_CONTENT (buffered content if non-empty)
         → Emit STRING_END
         → Return to INITIAL state

  See '\\' → Process escape sequence
          → Add to buffer
          → Stay in IN_STRING

  See any other char → Add to buffer
                     → Stay in IN_STRING

IN_INTERPOLATION state:
  See '.' → Emit IDENTIFIER_VARIABLE
         → Stay in IN_INTERPOLATION

  See ':' → Emit DELIMITER_COLON
         → Stay in IN_INTERPOLATION

  See identifier → Emit FORMAT_IDENTIFIER
                → Stay in IN_INTERPOLATION

  See '}' → Emit INTERPOLATION_END
         → Return to IN_STRING state
         → Clear buffer
```

---

## Parser Responsibilities

Once the lexer emits the token sequence, the parser:

1. **Recognizes patterns:**
   - `STRING_START ... STRING_END` → Plain string or implicit `U.String` call
   - `PIPELINE_IDENTIFIER STRING_START ... STRING_END` → Explicit pipeline call

2. **Extracts interpolations:**
   - Collect all `INTERPOLATION_START ... INTERPOLATION_END` sequences
   - Extract variable/value and optional format identifier

3. **Builds processing workflow:**
   - Pack interpolated variables into array
   - Call format pipelines for each variable
   - Substitute results into string
   - Pass formatted string to pipeline

4. **Type inference:**
   - Infer types of interpolated variables
   - Resolve format pipeline names
   - Validate format pipelines exist

---

## Edge Cases

### Empty String Content

**Input:**
```polyglot
"{.var}"
```

**Token Sequence:**
```
STRING_START
INTERPOLATION_START
IDENTIFIER_VARIABLE(".var")
INTERPOLATION_END
STRING_END
```

**Notes:**
- No STRING_CONTENT tokens
- String is entirely interpolation

---

### Adjacent Interpolations

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

---

### Escaped Braces (Future)

**Input:**
```polyglot
"Value: \\{.var\\}"
```

**Expected behavior:**
- Escaped braces should appear as literal `{` and `}` in output
- Exact escape syntax TBD (likely `\\{` and `\\}`)
- **Status:** Design pending

---

## Format Identifier Resolution

The lexer tokenizes format identifiers as simple identifiers (e.g., `Hex`, `Currency`). The parser/semantic analyzer is responsible for:

1. **Type inference:**
   ```
   .num: pg\int  →  Type is pg\int
   ```

2. **Pipeline name construction:**
   ```
   Format: Hex
   Type: pg\int
   Language: Polyglot (default)

   Pipeline name: |U.String.Polyglot.Int.Hex
   ```

3. **Pipeline lookup:**
   - Check if pipeline exists in registry
   - If not found, emit compile error with suggestion

4. **Call format pipeline:**
   - Pass variable value as input
   - Receive formatted string as output

---

## Implementation Checklist

### Lexer Implementation

- [ ] Add new token types to enum:
  - `STRING_START`
  - `STRING_CONTENT`
  - `STRING_END`
  - `INTERPOLATION_START`
  - `INTERPOLATION_END`
  - `FORMAT_IDENTIFIER`

- [ ] Implement state machine:
  - `INITIAL` → `IN_STRING` on `"`
  - `IN_STRING` → `IN_INTERPOLATION` on `{`
  - `IN_INTERPOLATION` → `IN_STRING` on `}`
  - `IN_STRING` → `INITIAL` on `"`

- [ ] Handle escape sequences:
  - `\"`, `\\`, `\n`, `\t`, `\r`
  - Process in `IN_STRING` state

- [ ] Buffer management:
  - Accumulate characters in `IN_STRING` state
  - Emit STRING_CONTENT on interpolation start or string end
  - Clear buffer after emitting

- [ ] Error detection:
  - Unterminated string (EOF in `IN_STRING`)
  - Unterminated interpolation (EOF in `IN_INTERPOLATION`)
  - Invalid escape sequences
  - Nested interpolations (not supported)

### Parser Implementation

- [ ] Recognize string literal patterns:
  - Plain string: `STRING_START ... STRING_END`
  - Explicit pipeline: `PIPELINE_IDENTIFIER STRING_START ... STRING_END`

- [ ] Extract interpolations:
  - Collect `INTERPOLATION_START ... INTERPOLATION_END` sequences
  - Parse variable and format identifier

- [ ] Build AST nodes:
  - StringLiteral node with:
    - Static content parts
    - Interpolation expressions
    - Format identifiers

- [ ] Validate structure:
  - Ensure interpolations are well-formed
  - Detect nested interpolations (error)

### Semantic Analyzer Implementation

- [ ] Type inference for interpolated variables

- [ ] Format pipeline resolution:
  - Construct pipeline name from type + format
  - Lookup pipeline in registry
  - Emit error if not found

- [ ] Validate format pipeline signatures:
  - Must accept variable type as input
  - Must return `pg\string` as output

---

## Rationale

### Why Tokenize Interpolations in Lexer?

**Option A: Lexer tokenizes interpolations (CHOSEN)**
- ✅ Clean separation: Lexer handles syntax, parser handles semantics
- ✅ Easier error reporting: Lexer can report unterminated interpolations
- ✅ Simpler parser: Parser receives structured tokens
- ✅ Consistent with block marker strategy: `[X]` is one token, `{...}` is token sequence

**Option B: Lexer outputs raw string, parser extracts interpolations**
- ❌ Parser becomes more complex (must parse string content)
- ❌ Error reporting harder (parser must track positions in string)
- ❌ Mixing lexical and syntactic analysis in parser

**Decision:** Lexer tokenizes interpolations for cleaner architecture.

---

## Performance Considerations

### Lexer Overhead

**Tokenizing interpolations adds:**
- State machine complexity (3 states instead of 2)
- Buffer management for STRING_CONTENT
- Additional tokens per interpolation (~4-5 tokens)

**Mitigation:**
- Modern lexers handle this easily
- Token overhead is negligible
- Cleaner parser logic offsets lexer complexity

### Parser Simplification

**Benefits from structured tokens:**
- No string parsing logic in parser
- Direct AST construction from tokens
- Simpler error handling

**Net effect:** Overall system complexity is LOWER with lexer tokenization.

---

## Examples from Polyglot Codebase

### Example from Pipeline Definition

```polyglot
[r] .greeting: pg\string << "Hello, {.name}!"
```

**Lexer output for string part:**
```
STRING_START
STRING_CONTENT("Hello, ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".name")
INTERPOLATION_END
STRING_CONTENT("!")
STRING_END
```

**Parser interpretation:**
- Implicit `U.String` call
- Extract `.name` variable
- Default formatting (no format identifier)
- Call `|U.String.Polyglot.String` (assuming `.name` is `pg\string`)
- Substitute result into "Hello, {result}!"

---

### Example from Conditional Logic

```polyglot
[?] .score >? 90
[~][r] .message << "Excellent! Score: {.score:Decimal}"
[~]
```

**Lexer output for string part:**
```
STRING_START
STRING_CONTENT("Excellent! Score: ")
INTERPOLATION_START
IDENTIFIER_VARIABLE(".score")
DELIMITER_COLON
FORMAT_IDENTIFIER("Decimal")
INTERPOLATION_END
STRING_END
```

**Parser interpretation:**
- Implicit `U.String` call
- Extract `.score` variable with `Decimal` format
- Call `|U.String.Polyglot.Int.Decimal` (assuming `.score` is `pg\int`)
- Substitute result into "Excellent! Score: {result}"

---

## Future Enhancements

### Nested Interpolations (Likely Disallowed)

**Example:**
```polyglot
"Outer {DT.Now\\\"\\\"}}"  // DT.Now inside interpolation
```

**Status:** To be determined
**Recommendation:** Disallow for simplicity

---

### Literal Braces in String Content

**Example:**
```polyglot
"Show literal braces: \\{ and \\}"
```

**Status:** Escape syntax to be defined
**Recommendation:** `\\{` and `\\}` for literal braces

---

## Summary

**Key Decision:** Lexer tokenizes string literal interpolations into structured token sequences.

**Token sequence pattern:**
```
STRING_START
  STRING_CONTENT*
  (INTERPOLATION_START
    IDENTIFIER_VARIABLE
    (DELIMITER_COLON FORMAT_IDENTIFIER)?
   INTERPOLATION_END)*
  STRING_CONTENT*
STRING_END
```

**Benefits:**
- Clean lexer/parser separation
- Better error reporting
- Simpler parser logic
- Consistent with overall architecture

**Next steps:**
1. Update token enumeration in LEXER-TOKEN-SPECIFICATION.md
2. Update pattern trees in LEXER-PATTERN-TREES.md
3. Implement lexer state machine
4. Implement parser string literal handling

---

**Document Status:** Canonical Reference ✅
**Last Updated:** 2025-11-26
**Related Documents:**
- `/docs/project/examples/LEXER-TOKEN-SPECIFICATION.md`
- `/docs/project/examples/LEXER-PATTERN-TREES.md`
- `/docs/technical/string-literals-internals.md`
- `/docs/technical/architecture.md`
