# Polyglot Lexer Test Suite - Canonical Input/Output Examples

**Version:** v0.0.2
**Last Updated:** 2025-11-26
**Purpose:** Canonical test cases for lexer implementation with expected token sequences
**Status:** Reference Implementation Tests ✅

---

## Overview

This document provides **canonical input/output examples** for the Polyglot lexer. Each test case includes:
- **Input:** Source code snippet
- **Expected Output:** Complete token sequence
- **Category:** Type of test (basic, block markers, strings, etc.)
- **Description:** What the test validates

**Total Test Cases:** 50+

---

## Test Organization

### Categories
1. **Basic Tokens** - Individual token recognition
2. **Block Markers** - All 25 block marker types
3. **Operators** - Assignment, comparison, pattern, range
4. **Identifiers** - Variables, enums, pipelines, errors
5. **String Literals** - Plain strings and interpolations
6. **Complete Statements** - Full Polyglot statements
7. **Mini Pipelines** - Small complete pipelines
8. **Edge Cases** - Boundary conditions
9. **Error Cases** - Invalid input (lexer should detect)

---

## Token Format

All test cases use this token format:

```
TOKEN_TYPE("lexeme")  [line:X, col:Y]
```

For clarity, line/column info may be omitted when not relevant to the test.

---

## 1. Basic Tokens

### Test 1.1: Variable Identifier

**Input:**
```polyglot
.user
```

**Expected Tokens:**
```
IDENTIFIER_VARIABLE(".user")
EOF
```

**Description:** Simple variable identifier with dot prefix

---

### Test 1.2: Nested Variable Identifier

**Input:**
```polyglot
.user.profile.age
```

**Expected Tokens:**
```
IDENTIFIER_VARIABLE(".user")
DELIMITER_DOT(".")
IDENTIFIER("profile")
DELIMITER_DOT(".")
IDENTIFIER("age")
EOF
```

**Description:** Multi-level variable navigation

---

### Test 1.3: Enumeration Identifier

**Input:**
```polyglot
#Boolean.True
```

**Expected Tokens:**
```
IDENTIFIER_ENUM("#Boolean")
DELIMITER_DOT(".")
IDENTIFIER("True")
EOF
```

**Description:** Enum with dot-separated field

---

### Test 1.4: Pipeline Identifier

**Input:**
```polyglot
|ProcessData
```

**Expected Tokens:**
```
IDENTIFIER_PIPELINE("|ProcessData")
EOF
```

**Description:** Pipeline identifier with pipe prefix

---

### Test 1.5: Error Identifier

**Input:**
```polyglot
!NetworkError
```

**Expected Tokens:**
```
IDENTIFIER_ERROR("!NetworkError")
EOF
```

**Description:** Error type identifier

---

### Test 1.6: Integer Literal

**Input:**
```polyglot
42
```

**Expected Tokens:**
```
LITERAL_INTEGER("42")
EOF
```

---

### Test 1.7: Negative Integer

**Input:**
```polyglot
-100
```

**Expected Tokens:**
```
OPERATOR_MINUS("-")
LITERAL_INTEGER("100")
EOF
```

**Description:** Negative sign is separate operator, not part of literal

---

### Test 1.8: Float Literal

**Input:**
```polyglot
3.14
```

**Expected Tokens:**
```
LITERAL_FLOAT("3.14")
EOF
```

---

### Test 1.9: Type Specification

**Input:**
```polyglot
pg\string
```

**Expected Tokens:**
```
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
EOF
```

---

## 2. Block Markers

### Test 2.1: Package Start

**Input:**
```polyglot
[@]
```

**Expected Tokens:**
```
BLOCK_PACKAGE_START("[@]")
EOF
```

---

### Test 2.2: Pipeline Start

**Input:**
```polyglot
[|]
```

**Expected Tokens:**
```
BLOCK_PIPELINE_START("[|]")
EOF
```

---

### Test 2.3: Input Block

**Input:**
```polyglot
[i]
```

**Expected Tokens:**
```
BLOCK_INPUT("[i]")
EOF
```

---

### Test 2.4: Sequential Block

**Input:**
```polyglot
[r]
```

**Expected Tokens:**
```
BLOCK_SEQUENTIAL("[r]")
EOF
```

---

### Test 2.5: Conditional Block

**Input:**
```polyglot
[?]
```

**Expected Tokens:**
```
BLOCK_CONDITIONAL("[?]")
EOF
```

---

### Test 2.6: Block End

**Input:**
```polyglot
[X]
```

**Expected Tokens:**
```
BLOCK_END("[X]")
EOF
```

---

### Test 2.7: All Block Markers in Sequence

**Input:**
```polyglot
[@] [#] [|] [i] [t] [Q] [W] [\] [/] [o] [r] [<] [>] [p] [Y] [b] [s] [?] [~] [+] [&] [-] [^] [.] [X]
```

**Expected Tokens:**
```
BLOCK_PACKAGE_START("[@]")
BLOCK_VERSION_ENUM("[#]")
BLOCK_PIPELINE_START("[|]")
BLOCK_INPUT("[i]")
BLOCK_TRIGGER("[t]")
BLOCK_QUEUE("[Q]")
BLOCK_WRAPPER("[W]")
BLOCK_SETUP("[\\]")
BLOCK_CLEANUP("[/]")
BLOCK_OUTPUT("[o]")
BLOCK_SEQUENTIAL("[r]")
BLOCK_INPUT_BINDING("[<]")
BLOCK_OUTPUT_BINDING("[>]")
BLOCK_PARALLEL("[p]")
BLOCK_JOIN("[Y]")
BLOCK_BACKGROUND("[b]")
BLOCK_STREAMING("[s]")
BLOCK_CONDITIONAL("[?]")
BLOCK_BODY("[~]")
BLOCK_BOOL_OR("[+]")
BLOCK_BOOL_AND("[&]")
BLOCK_BOOL_XOR("[-]")
BLOCK_BOOL_NAND("[^]")
BLOCK_BOOL_NOR("[.]")
BLOCK_END("[X]")
EOF
```

**Description:** All 25 block markers

---

## 3. Operators

### Test 3.1: Push Operator

**Input:**
```polyglot
<<
```

**Expected Tokens:**
```
OP_PUSH("<<")
EOF
```

---

### Test 3.2: Pull Operator

**Input:**
```polyglot
>>
```

**Expected Tokens:**
```
OP_PULL(">+")
EOF
```

---

### Test 3.3: Equal Comparison

**Input:**
```polyglot
=?
```

**Expected Tokens:**
```
OP_EQUAL("=?")
EOF
```

---

### Test 3.4: Not Equal Comparison

**Input:**
```polyglot
=!?
```

**Expected Tokens:**
```
OP_NOT_EQUAL("=!?")
EOF
```

**Description:** Longest match - not `=` + `!` + `?`

---

### Test 3.5: Greater or Equal

**Input:**
```polyglot
=>?
```

**Expected Tokens:**
```
OP_GREATER_EQUAL("=>?")
EOF
```

**Description:** Longest match - not `=` + `>?`

---

### Test 3.6: All Comparison Operators

**Input:**
```polyglot
=? =!? >? <? =>? =<?
```

**Expected Tokens:**
```
OP_EQUAL("=?")
OP_NOT_EQUAL("=!?")
OP_GREATER(">?")
OP_LESS("<?")
OP_GREATER_EQUAL("=>?")
OP_LESS_EQUAL("=<?")
EOF
```

---

### Test 3.7: Range Operators

**Input:**
```polyglot
?[ ?( ?] ?)
```

**Expected Tokens:**
```
OP_RANGE_CLOSED("?[")
OP_RANGE_OPEN("?(")
OP_RANGE_HALF_RIGHT("?]")
OP_RANGE_HALF_LEFT("?)")
EOF
```

---

## 4. String Literals (Simple)

### Test 4.1: Plain String

**Input:**
```polyglot
"Hello, World!"
```

**Expected Tokens:**
```
STRING_START(""")
STRING_CONTENT("Hello, World!")
STRING_END(""")
EOF
```

**Description:** Plain string with no interpolation

---

### Test 4.2: Empty String

**Input:**
```polyglot
""
```

**Expected Tokens:**
```
STRING_START(""")
STRING_END(""")
EOF
```

**Description:** Empty string - no STRING_CONTENT token

---

### Test 4.3: String with Escape Sequences

**Input:**
```polyglot
"Path: C:\\Users\\Alice"
```

**Expected Tokens:**
```
STRING_START(""")
STRING_CONTENT("Path: C:\\Users\\Alice")
STRING_END(""")
EOF
```

**Description:** Escape sequences processed, backslashes in content

---

## 5. String Literals (With Interpolation)

### Test 5.1: String with Simple Interpolation

**Input:**
```polyglot
"Hello, {.name}!"
```

**Expected Tokens:**
```
STRING_START(""")
STRING_CONTENT("Hello, ")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".name")
INTERPOLATION_END("}")
STRING_CONTENT("!")
STRING_END(""")
EOF
```

**Description:** Single interpolation without format specifier

---

### Test 5.2: String with Formatted Interpolation

**Input:**
```polyglot
"Count: {.num:Hex}"
```

**Expected Tokens:**
```
STRING_START(""")
STRING_CONTENT("Count: ")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".num")
DELIMITER_COLON(":")
FORMAT_IDENTIFIER("Hex")
INTERPOLATION_END("}")
STRING_END(""")
EOF
```

**Description:** Interpolation with format specifier

---

### Test 5.3: String with Multiple Interpolations

**Input:**
```polyglot
"User {.name} has {.count:Decimal} items"
```

**Expected Tokens:**
```
STRING_START(""")
STRING_CONTENT("User ")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".name")
INTERPOLATION_END("}")
STRING_CONTENT(" has ")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".count")
DELIMITER_COLON(":")
FORMAT_IDENTIFIER("Decimal")
INTERPOLATION_END("}")
STRING_CONTENT(" items")
STRING_END(""")
EOF
```

**Description:** Multiple interpolations, mixed formats

---

### Test 5.4: String Only Interpolation

**Input:**
```polyglot
"{.variable}"
```

**Expected Tokens:**
```
STRING_START(""")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".variable")
INTERPOLATION_END("}")
STRING_END(""")
EOF
```

**Description:** No STRING_CONTENT - entire string is interpolation

---

### Test 5.5: Adjacent Interpolations

**Input:**
```polyglot
"{.first}{.second}"
```

**Expected Tokens:**
```
STRING_START(""")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".first")
INTERPOLATION_END("}")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".second")
INTERPOLATION_END("}")
STRING_END(""")
EOF
```

**Description:** No STRING_CONTENT between interpolations

---

## 6. Inline Pipeline Calls

### Test 6.1: Explicit Pipeline with Empty String

**Input:**
```polyglot
DT.Now""
```

**Expected Tokens:**
```
PIPELINE_IDENTIFIER("DT.Now")
STRING_START(""")
STRING_END(""")
EOF
```

**Description:** Empty string is mandatory invocation operator

---

### Test 6.2: Explicit Pipeline with Parameter

**Input:**
```polyglot
DT.Minutes"5"
```

**Expected Tokens:**
```
PIPELINE_IDENTIFIER("DT.Minutes")
STRING_START(""")
STRING_CONTENT("5")
STRING_END(""")
EOF
```

---

### Test 6.3: Pipeline with Interpolation

**Input:**
```polyglot
DT.ToUTC"{.timestamp}"
```

**Expected Tokens:**
```
PIPELINE_IDENTIFIER("DT.ToUTC")
STRING_START(""")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".timestamp")
INTERPOLATION_END("}")
STRING_END(""")
EOF
```

---

## 7. Complete Statements

### Test 7.1: Variable Declaration with Assignment

**Input:**
```polyglot
[r] .message: pg\string << "Hello"
```

**Expected Tokens:**
```
BLOCK_SEQUENTIAL("[r]")
IDENTIFIER_VARIABLE(".message")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
OP_PUSH("<<")
STRING_START(""")
STRING_CONTENT("Hello")
STRING_END(""")
EOF
```

**Description:** Complete variable declaration statement

---

### Test 7.2: Input Declaration

**Input:**
```polyglot
[i] .name: pg\string
```

**Expected Tokens:**
```
BLOCK_INPUT("[i]")
IDENTIFIER_VARIABLE(".name")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
EOF
```

---

### Test 7.3: Output Declaration

**Input:**
```polyglot
[o] .result: pg\int
```

**Expected Tokens:**
```
BLOCK_OUTPUT("[o]")
IDENTIFIER_VARIABLE(".result")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("int")
EOF
```

---

### Test 7.4: Pipeline Call with Input Binding

**Input:**
```polyglot
[<] .input << .source
```

**Expected Tokens:**
```
BLOCK_INPUT_BINDING("[<]")
IDENTIFIER_VARIABLE(".input")
OP_PUSH("<<")
IDENTIFIER_VARIABLE(".source")
EOF
```

---

### Test 7.5: Pipeline Call with Output Binding

**Input:**
```polyglot
[>] .output: pg\string >> .result
```

**Expected Tokens:**
```
BLOCK_OUTPUT_BINDING("[>]")
IDENTIFIER_VARIABLE(".output")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
OP_PULL(">+")
IDENTIFIER_VARIABLE(".result")
EOF
```

---

### Test 7.6: Conditional Check

**Input:**
```polyglot
[?] .count >? 10
```

**Expected Tokens:**
```
BLOCK_CONDITIONAL("[?]")
IDENTIFIER_VARIABLE(".count")
OP_GREATER(">?")
LITERAL_INTEGER("10")
EOF
```

---

### Test 7.7: Enum Comparison

**Input:**
```polyglot
[?] .status =? #Status.Complete
```

**Expected Tokens:**
```
BLOCK_CONDITIONAL("[?]")
IDENTIFIER_VARIABLE(".status")
OP_EQUAL("=?")
IDENTIFIER_ENUM("#Status")
DELIMITER_DOT(".")
IDENTIFIER("Complete")
EOF
```

---

## 8. Mini Pipelines

### Test 8.1: Minimal Pipeline

**Input:**
```polyglot
[|] HelloWorld
[i] #None
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .msg: pg\string << "Hello"
[o] .msg: pg\string
[X]
```

**Expected Tokens:**
```
BLOCK_PIPELINE_START("[|]")
IDENTIFIER("HelloWorld")
NEWLINE
BLOCK_INPUT("[i]")
RESERVED_NONE("#None")
NEWLINE
BLOCK_TRIGGER("[t]")
SPECIAL_TRIGGER_TYPE("|T.Call")
NEWLINE
BLOCK_WRAPPER("[W]")
SPECIAL_WRAPPER("|W.Polyglot.Scope")
NEWLINE
BLOCK_SEQUENTIAL("[r]")
IDENTIFIER_VARIABLE(".msg")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
OP_PUSH("<<")
STRING_START(""")
STRING_CONTENT("Hello")
STRING_END(""")
NEWLINE
BLOCK_OUTPUT("[o]")
IDENTIFIER_VARIABLE(".msg")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
NEWLINE
BLOCK_END("[X]")
EOF
```

**Description:** Complete minimal pipeline

---

### Test 8.2: Pipeline with String Interpolation

**Input:**
```polyglot
[|] Greet
[i] .name: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .greeting: pg\string << "Hello, {.name}!"
[o] .greeting: pg\string
[X]
```

**Expected Tokens:**
```
BLOCK_PIPELINE_START("[|]")
IDENTIFIER("Greet")
NEWLINE
BLOCK_INPUT("[i]")
IDENTIFIER_VARIABLE(".name")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
NEWLINE
BLOCK_TRIGGER("[t]")
SPECIAL_TRIGGER_TYPE("|T.Call")
NEWLINE
BLOCK_WRAPPER("[W]")
SPECIAL_WRAPPER("|W.Polyglot.Scope")
NEWLINE
BLOCK_SEQUENTIAL("[r]")
IDENTIFIER_VARIABLE(".greeting")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
OP_PUSH("<<")
STRING_START(""")
STRING_CONTENT("Hello, ")
INTERPOLATION_START("{")
IDENTIFIER_VARIABLE(".name")
INTERPOLATION_END("}")
STRING_CONTENT("!")
STRING_END(""")
NEWLINE
BLOCK_OUTPUT("[o]")
IDENTIFIER_VARIABLE(".greeting")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
NEWLINE
BLOCK_END("[X]")
EOF
```

**Description:** Pipeline with string interpolation in assignment

---

## 9. Comments

### Test 9.1: Single-Line Comment

**Input:**
```polyglot
// This is a comment
.variable
```

**Expected Tokens:**
```
IDENTIFIER_VARIABLE(".variable")
EOF
```

**Description:** Comments are skipped, not emitted as tokens

---

### Test 9.2: Multi-Line Comment

**Input:**
```polyglot
/* This is a
   multi-line
   comment */
.variable
```

**Expected Tokens:**
```
IDENTIFIER_VARIABLE(".variable")
EOF
```

**Description:** Multi-line comments are skipped

---

### Test 9.3: Inline Comment

**Input:**
```polyglot
[r] .msg: pg\string << "Hi"  // Inline comment
```

**Expected Tokens:**
```
BLOCK_SEQUENTIAL("[r]")
IDENTIFIER_VARIABLE(".msg")
DELIMITER_COLON(":")
TYPE_NAMESPACE("pg")
DELIMITER_BACKSLASH("\\")
TYPE_NAME("string")
OP_PUSH("<<")
STRING_START(""")
STRING_CONTENT("Hi")
STRING_END(""")
EOF
```

**Description:** Inline comments after statements

---

## 10. Edge Cases

### Test 10.1: Collection Literal

**Input:**
```polyglot
{1, 2, 3}
```

**Expected Tokens:**
```
DELIMITER_BRACE_OPEN("{")
LITERAL_INTEGER("1")
DELIMITER_COMMA(",")
LITERAL_INTEGER("2")
DELIMITER_COMMA(",")
LITERAL_INTEGER("3")
DELIMITER_BRACE_CLOSE("}")
EOF
```

---

### Test 10.2: Empty Collection

**Input:**
```polyglot
{}
```

**Expected Tokens:**
```
DELIMITER_BRACE_OPEN("{")
DELIMITER_BRACE_CLOSE("}")
EOF
```

---

### Test 10.3: Reserved Variable State

**Input:**
```polyglot
.user.pgvar.state
```

**Expected Tokens:**
```
IDENTIFIER_VARIABLE(".user")
DELIMITER_DOT(".")
IDENTIFIER("pgvar")
DELIMITER_DOT(".")
IDENTIFIER("state")
EOF
```

---

### Test 10.4: Version Number

**Input:**
```polyglot
1.0.0
```

**Expected Tokens:**
```
VERSION("1.0.0")
EOF
```

---

### Test 10.5: Package Declaration

**Input:**
```polyglot
[@] Local@Example:1.0.0
```

**Expected Tokens:**
```
BLOCK_PACKAGE_START("[@]")
IDENTIFIER("Local")
DELIMITER_AT("@")
IDENTIFIER("Example")
DELIMITER_COLON(":")
VERSION("1.0.0")
EOF
```

---

## 11. Error Cases (Lexer Should Detect)

### Test 11.1: Unterminated String

**Input:**
```polyglot
"Hello
```

**Expected Error:**
```
LexerError: Unterminated string literal at line 1, column 1
```

---

### Test 11.2: Unterminated Interpolation

**Input:**
```polyglot
"Value: {.num"
```

**Expected Error:**
```
LexerError: Unterminated interpolation at line 1, column 8: expected '}', got '"'
```

---

### Test 11.3: Invalid Identifier (Starts with Digit)

**Input:**
```polyglot
.123variable
```

**Expected Error:**
```
LexerError: Invalid identifier at line 1, column 1: identifier cannot start with digit
```

---

### Test 11.4: Unterminated Block Marker

**Input:**
```polyglot
[r .variable
```

**Expected Error:**
```
LexerError: Unterminated block marker at line 1, column 1: expected ']', got ' '
```

---

### Test 11.5: Unknown Block Marker

**Input:**
```polyglot
[z]
```

**Expected Error:**
```
LexerError: Unknown block marker at line 1, column 1: '[z]'
```

---

### Test 11.6: Invalid Escape Sequence

**Input:**
```polyglot
"Invalid \x escape"
```

**Expected Error:**
```
LexerError: Invalid escape sequence at line 1, column 9: '\x'
```

---

## 12. Whitespace and Newlines

### Test 12.1: Multiple Spaces

**Input:**
```polyglot
.var1    .var2
```

**Expected Tokens:**
```
IDENTIFIER_VARIABLE(".var1")
IDENTIFIER_VARIABLE(".var2")
EOF
```

**Description:** Whitespace is skipped, only used for token separation

---

### Test 12.2: Tabs and Spaces

**Input:**
```polyglot
[r]		.variable
```

**Expected Tokens:**
```
BLOCK_SEQUENTIAL("[r]")
IDENTIFIER_VARIABLE(".variable")
EOF
```

**Description:** Tabs treated same as spaces

---

### Test 12.3: Newlines

**Input:**
```polyglot
[r] .var1
[r] .var2
```

**Expected Tokens:**
```
BLOCK_SEQUENTIAL("[r]")
IDENTIFIER_VARIABLE(".var1")
NEWLINE
BLOCK_SEQUENTIAL("[r]")
IDENTIFIER_VARIABLE(".var2")
EOF
```

**Description:** Newlines emitted as tokens (statement separators)

---

## Summary Statistics

| Category | Test Count |
|----------|------------|
| Basic Tokens | 9 |
| Block Markers | 7 |
| Operators | 7 |
| String Literals (Simple) | 3 |
| String Literals (Interpolation) | 5 |
| Inline Pipeline Calls | 3 |
| Complete Statements | 7 |
| Mini Pipelines | 2 |
| Comments | 3 |
| Edge Cases | 5 |
| Error Cases | 6 |
| Whitespace | 3 |
| **TOTAL** | **60 tests** |

---

## Test Implementation Format

### Recommended Structure (Rust)

```rust
#[test]
fn test_variable_identifier() {
    let input = ".user";
    let expected = vec![
        Token::IdentifierVariable(".user".to_string()),
        Token::EOF,
    ];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_string_with_interpolation() {
    let input = r#""Hello, {.name}!""#;
    let expected = vec![
        Token::StringStart,
        Token::StringContent("Hello, ".to_string()),
        Token::InterpolationStart,
        Token::IdentifierVariable(".name".to_string()),
        Token::InterpolationEnd,
        Token::StringContent("!".to_string()),
        Token::StringEnd,
        Token::EOF,
    ];
    assert_eq!(lex(input), expected);
}
```

---

## Additional Test Resources

### Test Data Extraction

All examples from user documentation can serve as integration tests:
- `/docs/user/examples/hello-world.md` - 6 complete examples
- `/docs/user/examples/data-processing.md` - Data transformation examples
- `/docs/user/examples/error-handling.md` - Error scenarios

### Property-Based Testing

Consider property-based tests:
- All valid identifiers should tokenize correctly
- String interpolations should nest properly
- Block markers should always produce single tokens
- Longest match should always be preferred

---

## Test Execution Strategy

### Phase 1: Unit Tests (Individual Tokens)
Run Tests 1-4 (Basic tokens, block markers, operators, identifiers)

### Phase 2: String Literal Tests
Run Tests 4-6 (String tokenization with interpolations)

### Phase 3: Statement Tests
Run Tests 7 (Complete statements)

### Phase 4: Integration Tests
Run Tests 8 (Complete pipelines)

### Phase 5: Edge Cases and Errors
Run Tests 9-11 (Comments, edge cases, error detection)

### Phase 6: Regression Tests
Process all examples from `/docs/user/examples/`

---

## Maintenance

When adding new language features:
1. Add canonical test case to appropriate category
2. Include both valid and error cases
3. Update summary statistics
4. Run full test suite to ensure no regressions

---

**Next Steps:**
1. Implement test harness (Rust test framework)
2. Extract tests from this document into code
3. Run tests against lexer implementation
4. Achieve 100% pass rate before moving to parser

---

**Document Status:** Canonical Test Suite ✅
**Last Updated:** 2025-11-26
**Total Tests:** 60
**Coverage:** All token types, all block markers, string interpolations, error cases
