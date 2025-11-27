# Parser Considerations for Polyglot v0.0.2 Lexer

**Date:** 2025-11-27
**Lexer Version:** v0.0.2
**For:** Parser Implementation Team

---

## Overview

The Polyglot lexer correctly handles **tokenization** and defers **syntax validation** to the parser. This document outlines specific edge cases and validation rules that the parser must implement.

---

## 1. Line Continuation `[*]` - Edge Case ⚠️

### Lexer Behavior

The `[*]` block marker (BlockLineContinuation) triggers special lexer behavior:

1. **Token emitted:** `BlockLineContinuation` with lexeme `"[*]"`
2. **Newline skipped:** The newline immediately following `[*]` is consumed and **NOT** emitted as a Newline token
3. **Line tracking:** Line/column counters are updated but no Newline token appears in the token stream

### Example

**Source code:**
```polyglot
[r] .x: pg\int << \
[*]
42
```

**Token stream:**
```
OpPush (<<)
DelimiterBackslash (\)
Newline             ← Before [*]
BlockLineContinuation ([*])
                    ← NO NEWLINE TOKEN HERE (skipped by lexer)
LiteralInteger (42)
```

### Parser Responsibilities

The parser MUST validate:

1. **Context validity:** `[*]` should only appear where line breaks are syntactically valid
   - ✅ Valid: After operators (`<<`, `>>`, etc.)
   - ✅ Valid: After delimiters (`,`, `:`, etc.)
   - ❌ Invalid: In the middle of identifiers
   - ❌ Invalid: Inside string literals (different mechanism)

2. **Semantic continuity:** The tokens before and after `[*]` must form a valid expression
   ```polyglot
   // Valid
   [r] .x: pg\int << \
   [*]
   42

   // Invalid - incomplete expression
   [r] .x: pg\int
   [*]
   << 42
   ```

3. **Nesting rules:** Define if `[*]` can appear inside blocks, conditionals, etc.
   ```polyglot
   [?] .x >? 10
   [*]
   && .y <? 20  // Is this valid?
   ```

### No Predictable Pattern After `[*]`

⚠️ **Important:** The lexer does NOT enforce what can follow `[*]` because:
- Any valid token may appear on the continuation line
- The lexer's job is tokenization, not syntax validation
- The parser must validate that the continuation makes semantic sense

**Example - Lexer accepts, Parser validates:**
```polyglot
[r] .name: pg\string
[*]
<< "Hello"  // Parser validates this completes the assignment
```

---

## 2. Identifier Validation (Parser Responsibility)

The lexer tokenizes individual elements but defers **identifier placement** validation to the parser.

### Cases Lexer Accepts, Parser Must Reject

#### 2.1. Variable Starting with Number (`.123abc`)

**Lexer behavior:**
```
Tokens: DelimiterDot (.) → LiteralFloat (0.123) → Identifier (abc)
```

**Parser must:**
- Detect that `.` followed by number is a float literal, not a variable identifier
- Reject `abc` appearing after the float without an operator

#### 2.2. Empty Identifier After Dot (`.`)

**Lexer behavior:**
```
Tokens: DelimiterDot (.)
```

**Parser must:**
- Expect an identifier after `.` in variable context
- Reject standalone `.` where variable identifier is expected

#### 2.3. Double Prefix (`..variable`)

**Lexer behavior:**
```
Tokens: DelimiterDot (.) → DelimiterDot (.) → IdentifierVariable (.variable)
```

**Parser must:**
- Reject double-dot syntax (not valid in Polyglot)
- Validate single `.` prefix for variables

#### 2.4. Multiple Decimal Points (`3.14.15`)

**Lexer behavior:**
```
Tokens: LiteralFloat (3.14) → LiteralFloat (0.15)
```

**Parser must:**
- Reject adjacent float literals without operator
- Validate expression structure

#### 2.5. Number Followed by Identifier (`42abc`)

**Lexer behavior:**
```
Tokens: LiteralInteger (42) → Identifier (abc)
```

**Parser must:**
- Reject adjacent number and identifier without operator
- Validate token sequence rules

---

## 3. Block Structure Validation

The lexer recognizes all 26 block markers but does NOT validate:

### 3.1. Block Pairing

**Parser must validate:**
- Every `[|]` (BlockPipelineStart) has matching `[X]` (BlockEnd)
- Proper nesting of blocks
- Block order rules (e.g., `[i]` before `[r]`)

### 3.2. Block Completeness

**Parser must validate:**
- Required blocks are present (e.g., pipeline must have output)
- Optional blocks appear in correct positions
- Block content is appropriate for block type

---

## 4. String Continuation with `>""` (Design Decision Needed)

### Current State

The lexer does NOT implement string continuation across lines using `>"` syntax.

### Example (NOT YET IMPLEMENTED)

```polyglot
[r] .msg: pg\string << "Hello \
[*]
>"World"

// Should become: "Hello World"
```

### Options

**Option A: Lexer handles it**
- Recognize `>"` in IN_STRING state after `[*]`
- Merge string content across lines
- Emit single StringContent token

**Option B: Parser handles it**
- Lexer emits: StringEnd, BlockLineContinuation, StringStart
- Parser recognizes pattern and merges strings

**Recommendation:** Option A (Lexer) - String tokenization is lexer responsibility

---

## 5. Reserved Enumeration Validation

The lexer recognizes 10 reserved enumerations as special tokens.

**Parser must validate:**
- Context-appropriate usage (e.g., `#Boolean.True` only in boolean contexts)
- Type compatibility with operations
- State transitions for `#PgVar.States.*`

---

## 6. Error Recovery

**Lexer error types** that stop tokenization:
1. `UnterminatedString` - Parser cannot recover, must report
2. `UnterminatedComment` - Parser cannot recover
3. `UnterminatedBlockMarker` - Parser cannot recover
4. `UnknownBlockMarker` - Parser cannot recover
5. `InvalidEscapeSequence` - Parser cannot recover
6. `UnexpectedCharacter` - Parser cannot recover

**Parser's role:** Report lexer errors to user with context

---

## 7. Lookahead Requirements

Some grammar rules may require parser lookahead:

1. **Identifier vs. Reserved:** Lexer pre-classifies, parser uses token type
2. **Range operators:** Lexer provides OpRange* tokens, parser validates usage
3. **Block markers:** Lexer provides Block* tokens, parser validates structure

---

## 8. ASCII-Only Identifiers

✅ **Implemented in lexer** (2025-11-27)

The lexer enforces strict ASCII-only identifiers:
- Valid: `a-z`, `A-Z`, `0-9`, `_`
- Invalid: Any non-ASCII characters (é, ñ, 中, etc.)

**Parser assumption:** All identifiers in token stream are ASCII-only

---

## Summary

| Validation Type | Lexer | Parser |
|-----------------|-------|--------|
| Token recognition | ✅ | - |
| ASCII-only identifiers | ✅ | - |
| Escape sequences | ✅ | - |
| Unterminated constructs | ✅ | - |
| Line continuation `[*]` newline skip | ✅ | - |
| `[*]` context validity | - | ✅ |
| Identifier placement | - | ✅ |
| Expression structure | - | ✅ |
| Block pairing/nesting | - | ✅ |
| Type compatibility | - | ✅ |
| Operator usage | - | ✅ |

---

## Implementation Priority

For parser development, validate in this order:

1. **High Priority:**
   - Block structure (pairing, nesting)
   - Identifier placement rules
   - Expression structure validation

2. **Medium Priority:**
   - `[*]` context validation
   - Reserved enumeration usage
   - Type compatibility

3. **Future:**
   - String continuation `>"` (after design decision)
   - Advanced error recovery
   - Optimization hints

---

**Document Version:** 1.0
**Last Updated:** 2025-11-27
**Status:** Ready for Parser Implementation
