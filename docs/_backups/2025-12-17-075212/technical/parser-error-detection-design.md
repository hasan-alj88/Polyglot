# Parser Error Detection Design
**Version:** 0.0.2
**Date:** 2025-11-27
**Status:** Design Phase (Gap #3 Resolution)
**Authors:** Winston (Architect), John (PM)

---

## Overview

This document defines the comprehensive error detection system for the Polyglot parser (Story 1.4). The parser must detect and report all syntax and semantic errors with precise location information and helpful error messages.

**Design Principles:**
1. **Fail Fast** - Detect errors as early as possible
2. **Precise Location** - Every error includes exact source location (line, column)
3. **Helpful Messages** - Clear explanation of what went wrong and how to fix it
4. **Error Recovery** - When possible, continue parsing to find multiple errors
5. **Context Awareness** - Errors include surrounding context for better understanding

---

## Error Categories

### 1. Lexer Errors (Pass-Through)
Errors from the lexer layer, wrapped for consistency.

**Examples:**
- Unterminated string literal
- Invalid escape sequence
- Unknown character
- Unterminated multi-line comment

**Handling:** Pass through from `LexerError` with span information preserved.

---

### 2. Unexpected Token Errors
Parser expected a specific token but found something else.

**Examples:**
- Expected `{` but found `}`
- Expected identifier but found integer literal
- Expected `)` to close expression
- Expected block marker but found operator

**Error Fields:**
- `expected: Vec<TokenKind>` - What tokens were expected
- `found: Token` - What token was actually found
- `context: String` - What the parser was doing (e.g., "parsing variable declaration")
- `span: Span` - Location of the unexpected token

---

### 3. Block Hierarchy Violations

#### 3.1 File-Level Hierarchy Errors

**Invalid Block Order:**
```polyglot
[|] |MyPipeline  // ERROR: Pipeline before package declaration
[@] MyPackage    // Package must be first
```

**Error:** `BlockOrderViolation`
- Expected order: `[@]` → `[#]` → `[!]` → `[M]` → `[|]`
- Found: `[|]` at line X before `[@]`
- Hint: "Package declaration [@] must be the first block in the file"

**Duplicate File-Level Blocks:**
```polyglot
[@] MyPackage
[@] AnotherPackage  // ERROR: Duplicate package declaration
```

**Error:** `DuplicateBlock`
- Block type: `[@]` (Package)
- First occurrence: Line X, Column Y
- Second occurrence: Line Z, Column W
- Hint: "Only one package declaration is allowed per file"

**Missing Required Blocks:**
```polyglot
[@] MyPackage
[|] |MyPipeline  // ERROR: Missing version block [#]
```

**Error:** `MissingRequiredBlock`
- Missing block: `[#]` (Version)
- Required after: `[@]` (Package)
- Location: After line X
- Hint: "Package declaration must be followed by version block [#]"

#### 3.2 Pipeline-Level Hierarchy Errors

**Invalid Pipeline Block Order:**
```polyglot
[|] |MyPipeline
[o] .output: pg\string  // ERROR: Output before input
[i] .input: pg\int
```

**Error:** `InvalidPipelineBlockOrder`
- Canonical order: `[i]` → `[t]` → `[\]` → `[r]/[p]` → `[Y]` → `[/]` → `[o]`
- Found: `[o]` at line X before `[i]`
- Hint: "Input block [i] must come before output block [o]"

**Multiple Execution Blocks:**
```polyglot
[|] |MyPipeline
[r] |Step1
[p] |Step2  // ERROR: Cannot mix [r] and [p] in same pipeline
```

**Error:** `MultipleExecutionBlocks`
- First block: `[r]` (Sequential) at line X
- Second block: `[p]` (Parallel) at line Y
- Hint: "Use only one execution block type ([r], [p], [Y], [b]) per pipeline"

**Duplicate Optional Blocks:**
```polyglot
[|] |MyPipeline
[t] TG.FileChange
[t] TG.TimeBased  // ERROR: Duplicate trigger block
```

**Error:** `DuplicateOptionalBlock`
- Block type: `[t]` (Trigger)
- First occurrence: Line X
- Second occurrence: Line Y
- Hint: "Only one trigger block [t] is allowed per pipeline"

---

### 4. Nesting Violations

**Invalid Parent-Child Relationship:**
```polyglot
[i] .input: pg\int
    [r] |Operation  // ERROR: [r] cannot be child of [i]
```

**Error:** `InvalidNesting`
- Child block: `[r]` (Sequential)
- Parent block: `[i]` (Input)
- Reason: "Input blocks [i] cannot contain executable blocks"
- Hint: "Input blocks can only contain variable declarations"

**Binding Without Pipeline Call:**
```polyglot
[r]  // No pipeline call
    [<] .input << .myVar  // ERROR: [<] requires parent to call pipeline
```

**Error:** `BindingWithoutPipelineCall`
- Binding block: `[<]` (Input Binding)
- Parent block: `[r]` at line X
- Reason: "Binding blocks [<] and [>] only allowed when parent calls a pipeline"
- Hint: "Add a pipeline call |MyPipeline to the parent [r] block"

**Orphaned Block:**
```polyglot
[@] MyPackage
    [i] .input: pg\int  // ERROR: [i] must be inside [|] pipeline
```

**Error:** `OrphanedBlock`
- Block type: `[i]` (Input)
- Parent context: `[@]` (Package)
- Reason: "Input blocks [i] must be children of pipeline blocks [|]"
- Hint: "Move [i] block inside a [|] pipeline definition"

---

### 5. Operator Misuse

**String Concatenation on Variables:**
```polyglot
[r] .msg: pg\string << .var1 +" " +" .var2  // ERROR: +" only for literals
```

**Error:** `InvalidStringConcatenation`
- Operator: `+"`
- Found: Variable identifier `.var1`
- Location: Line X, Column Y
- Hint: "Use string interpolation instead: \"{.var1} {.var2}\""

**Invalid Operator Context:**
```polyglot
[r] .result: pg\int << .num1 +" .num2  // ERROR: +" on non-strings
```

**Error:** `OperatorTypeMismatch`
- Operator: `+"` (String Concatenation)
- Left operand type: `pg\int`
- Location: Line X, Column Y
- Hint: "Operator +" requires string literals on both sides"

---

### 6. Type Annotation Errors

**Invalid Type Syntax:**
```polyglot
[r] .var: pg/string << "hello"  // ERROR: Use backslash not forward slash
```

**Error:** `InvalidTypeSyntax`
- Found: `/` at line X, column Y
- Expected: `\` (backslash)
- Hint: "Type annotations use backslash: pg\string"

**Unknown Type:**
```polyglot
[r] .var: pg\unknown << "value"
```

**Error:** `UnknownType`
- Type: `pg\unknown`
- Location: Line X, Column Y
- Valid types: `pg\string`, `pg\int`, `pg\float`, `pg\bool`, `pg\dt`, `pg\path`, `pg\serial`, `pg\array`, `pg\set`
- Hint: "Check type spelling or refer to type system documentation"

---

### 7. Identifier Errors

**Invalid Identifier Format:**
```polyglot
[r] .123invalid << "value"  // ERROR: Identifier cannot start with digit
```

**Error:** `InvalidIdentifier`
- Identifier: `.123invalid`
- Location: Line X, Column Y
- Reason: "Identifiers must start with ASCII letter or underscore"
- Hint: "Use .invalid_123 or .inv123 instead"

**Non-ASCII Identifier:**
```polyglot
[r] .变量 << "value"  // ERROR: Only ASCII allowed
```

**Error:** `NonAsciiIdentifier`
- Identifier: `.变量`
- Location: Line X, Column Y
- Reason: "Polyglot v0.0.2 only supports ASCII identifiers"
- Hint: "Use ASCII characters (a-z, A-Z, 0-9, _)"

**Missing Identifier Prefix:**
```polyglot
[r] myVar: pg\string << "hello"  // ERROR: Missing . prefix
```

**Error:** `MissingIdentifierPrefix`
- Identifier: `myVar`
- Location: Line X, Column Y
- Expected prefix: `.` for variables
- Hint: "Use .myVar for variable identifiers"

---

### 8. Expression Errors

**Unclosed Delimiter:**
```polyglot
[r] .result: pg\int << (1 + 2  // ERROR: Missing closing )
```

**Error:** `UnclosedDelimiter`
- Opening delimiter: `(` at line X, column Y
- Expected closing: `)`
- Found: EOF or newline
- Hint: "Add closing ) to complete expression"

**Invalid Range Syntax:**
```polyglot
[r] .check: pg\bool << .val ?] 1, 10 [  // ERROR: Incorrect range brackets
```

**Error:** `InvalidRangeSyntax`
- Found: `?] ... [`
- Location: Line X, Column Y
- Valid patterns: `?[`, `?(`, `?]`, `?)`
- Hint: "Use ?[ for inclusive start or ?( for exclusive start"

---

### 9. Statement Errors

**Assignment to Non-Variable:**
```polyglot
[r] |MyPipeline << "value"  // ERROR: Cannot assign to pipeline
```

**Error:** `InvalidAssignmentTarget`
- Target: `|MyPipeline` (Pipeline identifier)
- Location: Line X, Column Y
- Hint: "Only variables (.var) can be assignment targets"

**Missing Assignment Operator:**
```polyglot
[r] .var "value"  // ERROR: Missing << operator
```

**Error:** `MissingAssignmentOperator`
- Variable: `.var`
- Location: Line X, Column Y
- Hint: "Use << (push), >> (pull), or <~ (default) for assignment"

---

### 10. Pipeline Call Errors

**Invalid Pipeline Reference:**
```polyglot
[r] |123Invalid  // ERROR: Pipeline name cannot start with digit
```

**Error:** `InvalidPipelineReference`
- Pipeline: `|123Invalid`
- Location: Line X, Column Y
- Reason: "Pipeline names must follow identifier rules"
- Hint: "Use |Invalid123 instead"

**Unqualified Module Pipeline:**
```polyglot
[r] py.Print  // ERROR: Missing | prefix
```

**Error:** `MissingPipelinePrefix`
- Identifier: `py.Print`
- Location: Line X, Column Y
- Hint: "Pipeline calls require | prefix: |py.Print"

---

### 11. EOF Errors

**Unexpected End of File:**
```polyglot
[@] MyPackage
[#] 1.0.0
[|] |MyPipeline
[i] .input: pg\string
// EOF - ERROR: Missing [X] block end marker
```

**Error:** `UnexpectedEof`
- Context: "Expected [X] block end marker for pipeline"
- Started at: Line X, Column Y
- Hint: "Add [X] to close the pipeline block"

---

### 12. Line Continuation Errors

**Continuation Outside Context:**
```polyglot
[*] "continued"  // ERROR: [*] with no previous statement
```

**Error:** `InvalidLineContinuation`
- Location: Line X, Column Y
- Reason: "Line continuation [*] found without preceding statement"
- Hint: "Remove [*] or add statement before it"

**Mixed Continuation Types:**
```polyglot
[r] .msg: pg\string <<
[*] "Part 1"
[r] |Operation  // Ends continuation
[*] "Part 2"    // ERROR: New continuation cannot follow different block
```

**Error:** `BrokenContinuation`
- Continuation at: Line X
- Previous context: `[r]` block at line Y
- Reason: "Line continuation [*] must follow same statement type"
- Hint: "Remove [*] or continue the previous statement"

---

## Error Type Definitions

```rust
use thiserror::Error;
use crate::span::Span;
use polyglot_lexer::{Token, TokenKind, LexerError};

/// Comprehensive parser error types for Polyglot v0.0.2
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParserError {
    // ========================================
    // Lexer Errors (Pass-Through)
    // ========================================
    #[error("Lexer error: {source}")]
    LexerError {
        #[from]
        source: LexerError,
    },

    // ========================================
    // Token Errors
    // ========================================
    #[error("Unexpected token at {span}: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: Token,
        context: String,
        span: Span,
    },

    #[error("Unexpected end of file: {context}")]
    UnexpectedEof {
        context: String,
        started_at: Span,
    },

    // ========================================
    // Block Hierarchy Errors
    // ========================================
    #[error("Block order violation at {span}: {message}")]
    BlockOrderViolation {
        block_type: String,
        expected_order: String,
        span: Span,
        message: String,
    },

    #[error("Duplicate block at {second_span}: {block_type} already defined at {first_span}")]
    DuplicateBlock {
        block_type: String,
        first_span: Span,
        second_span: Span,
    },

    #[error("Missing required block: {block_type} expected after {after_span}")]
    MissingRequiredBlock {
        block_type: String,
        after_span: Span,
        hint: String,
    },

    #[error("Invalid pipeline block order at {span}: {message}")]
    InvalidPipelineBlockOrder {
        block_type: String,
        canonical_order: String,
        span: Span,
        message: String,
    },

    #[error("Multiple execution blocks: {first_block} at {first_span}, {second_block} at {second_span}")]
    MultipleExecutionBlocks {
        first_block: String,
        first_span: Span,
        second_block: String,
        second_span: Span,
    },

    // ========================================
    // Nesting Errors
    // ========================================
    #[error("Invalid nesting at {span}: {child_block} cannot be child of {parent_block}")]
    InvalidNesting {
        child_block: String,
        parent_block: String,
        span: Span,
        reason: String,
    },

    #[error("Binding without pipeline call at {span}: {block} requires parent to call pipeline")]
    BindingWithoutPipelineCall {
        block: String,
        parent_span: Span,
        span: Span,
    },

    #[error("Orphaned block at {span}: {block_type} must be inside {required_parent}")]
    OrphanedBlock {
        block_type: String,
        required_parent: String,
        span: Span,
    },

    // ========================================
    // Operator Errors
    // ========================================
    #[error("Invalid string concatenation at {span}: +" can only concatenate string literals")]
    InvalidStringConcatenation {
        found: String,
        span: Span,
        hint: String,
    },

    #[error("Operator type mismatch at {span}: {operator} cannot be used with {operand_type}")]
    OperatorTypeMismatch {
        operator: String,
        operand_type: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Type Errors
    // ========================================
    #[error("Invalid type syntax at {span}: {message}")]
    InvalidTypeSyntax {
        found: String,
        span: Span,
        message: String,
    },

    #[error("Unknown type at {span}: {type_name}")]
    UnknownType {
        type_name: String,
        span: Span,
        valid_types: Vec<String>,
    },

    // ========================================
    // Identifier Errors
    // ========================================
    #[error("Invalid identifier at {span}: {identifier} - {reason}")]
    InvalidIdentifier {
        identifier: String,
        span: Span,
        reason: String,
        hint: String,
    },

    #[error("Non-ASCII identifier at {span}: {identifier}")]
    NonAsciiIdentifier {
        identifier: String,
        span: Span,
        hint: String,
    },

    #[error("Missing identifier prefix at {span}: {identifier}")]
    MissingIdentifierPrefix {
        identifier: String,
        expected_prefix: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Expression Errors
    // ========================================
    #[error("Unclosed delimiter at {opening_span}: expected {closing}, found {found}")]
    UnclosedDelimiter {
        opening: char,
        opening_span: Span,
        closing: char,
        found: String,
    },

    #[error("Invalid range syntax at {span}: {message}")]
    InvalidRangeSyntax {
        found: String,
        span: Span,
        message: String,
    },

    // ========================================
    // Statement Errors
    // ========================================
    #[error("Invalid assignment target at {span}: {target}")]
    InvalidAssignmentTarget {
        target: String,
        span: Span,
        hint: String,
    },

    #[error("Missing assignment operator at {span}")]
    MissingAssignmentOperator {
        variable: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Pipeline Call Errors
    // ========================================
    #[error("Invalid pipeline reference at {span}: {pipeline}")]
    InvalidPipelineReference {
        pipeline: String,
        span: Span,
        reason: String,
    },

    #[error("Missing pipeline prefix at {span}: {identifier}")]
    MissingPipelinePrefix {
        identifier: String,
        span: Span,
        hint: String,
    },

    // ========================================
    // Line Continuation Errors
    // ========================================
    #[error("Invalid line continuation at {span}: {reason}")]
    InvalidLineContinuation {
        span: Span,
        reason: String,
        hint: String,
    },

    #[error("Broken continuation at {span}: {reason}")]
    BrokenContinuation {
        span: Span,
        previous_context: String,
        reason: String,
    },
}

// Implement Send + Sync for async compatibility (ADR-004)
// thiserror automatically implements these for all variants

impl ParserError {
    /// Get the primary span associated with this error
    pub fn span(&self) -> Span {
        match self {
            ParserError::LexerError { source } => source.span(),
            ParserError::UnexpectedToken { span, .. } => *span,
            ParserError::UnexpectedEof { started_at, .. } => *started_at,
            ParserError::BlockOrderViolation { span, .. } => *span,
            ParserError::DuplicateBlock { second_span, .. } => *second_span,
            ParserError::MissingRequiredBlock { after_span, .. } => *after_span,
            ParserError::InvalidPipelineBlockOrder { span, .. } => *span,
            ParserError::MultipleExecutionBlocks { second_span, .. } => *second_span,
            ParserError::InvalidNesting { span, .. } => *span,
            ParserError::BindingWithoutPipelineCall { span, .. } => *span,
            ParserError::OrphanedBlock { span, .. } => *span,
            ParserError::InvalidStringConcatenation { span, .. } => *span,
            ParserError::OperatorTypeMismatch { span, .. } => *span,
            ParserError::InvalidTypeSyntax { span, .. } => *span,
            ParserError::UnknownType { span, .. } => *span,
            ParserError::InvalidIdentifier { span, .. } => *span,
            ParserError::NonAsciiIdentifier { span, .. } => *span,
            ParserError::MissingIdentifierPrefix { span, .. } => *span,
            ParserError::UnclosedDelimiter { opening_span, .. } => *opening_span,
            ParserError::InvalidRangeSyntax { span, .. } => *span,
            ParserError::InvalidAssignmentTarget { span, .. } => *span,
            ParserError::MissingAssignmentOperator { span, .. } => *span,
            ParserError::InvalidPipelineReference { span, .. } => *span,
            ParserError::MissingPipelinePrefix { span, .. } => *span,
            ParserError::InvalidLineContinuation { span, .. } => *span,
            ParserError::BrokenContinuation { span, .. } => *span,
        }
    }

    /// Get a user-friendly hint for fixing the error
    pub fn hint(&self) -> Option<&str> {
        match self {
            ParserError::InvalidStringConcatenation { hint, .. } => Some(hint),
            ParserError::OperatorTypeMismatch { hint, .. } => Some(hint),
            ParserError::InvalidIdentifier { hint, .. } => Some(hint),
            ParserError::NonAsciiIdentifier { hint, .. } => Some(hint),
            ParserError::MissingIdentifierPrefix { hint, .. } => Some(hint),
            ParserError::InvalidAssignmentTarget { hint, .. } => Some(hint),
            ParserError::MissingAssignmentOperator { hint, .. } => Some(hint),
            ParserError::MissingPipelinePrefix { hint, .. } => Some(hint),
            ParserError::InvalidLineContinuation { hint, .. } => Some(hint),
            ParserError::MissingRequiredBlock { hint, .. } => Some(hint),
            _ => None,
        }
    }
}
```

---

## Validation Strategy

### Phase 1: Token-Level Validation (During Parsing)
- Unexpected tokens
- EOF errors
- Delimiter matching

### Phase 2: AST-Level Validation (After Parsing)
- Block hierarchy validation
- Nesting rules validation
- Pipeline call constraints

### Phase 3: Semantic Validation (Future - Epic 2)
- Type checking
- Variable scope validation
- Pipeline signature matching

---

## Compiler Warnings

Compiler warnings are distinct from errors. Warnings indicate non-canonical code style or potential issues that don't prevent compilation, but should be addressed for code quality and maintainability.

### Warning vs Error Distinction

| Aspect | Errors | Warnings |
|--------|--------|----------|
| **Compilation** | Prevent compilation (exit code 1) | Compilation succeeds (exit code 0) |
| **Correctness** | Code is syntactically or semantically invalid | Code is valid but not idiomatic |
| **Action Required** | MUST fix to compile | SHOULD fix for code quality |
| **Display** | Always shown | Can be suppressed with `--no-warnings` |

### Warning Categories

#### 1. Style Warnings

**Whitespace Between Variable Name and Type Colon**

**Canonical Form:**
```polyglot
[r] .count:pg.int << 42
[r] .data:pg.array.pg.string
[i] .input:pg.serial
```

**Non-Canonical Form (Triggers Warning):**
```polyglot
[r] .count :pg.int << 42        // WARNING: Unexpected whitespace before type colon
[r] .data :pg.array.pg.string   // WARNING: Unexpected whitespace before type colon
[i] .input :pg.serial            // WARNING: Unexpected whitespace before type colon
```

**Warning Details:**
- **Type:** `WhitespaceBeforeTypeColon`
- **Severity:** Warning (compilation continues)
- **Message:** "Unexpected whitespace between variable name and type colon. Remove the space for canonical format."
- **Hint:** "Canonical: `.variable:type` (no space)\nFound: `.variable :type` (with space)"
- **Span:** Location of the whitespace before colon
- **Fix:** Automated fix available (remove whitespace)

**Parser Behavior:**
- Parser MUST accept both forms (`.var:type` and `.var :type`)
- Parser MUST emit warning for non-canonical form (`.var :type`)
- Parser MUST continue compilation (warning, not error)
- Warning can be suppressed with `--no-warnings` flag

**Future Formatter:**
- Auto-formatter should normalize `.var :type` → `.var:type`
- Flag this rule for future `polyglot fmt` command implementation
- Formatter should be idempotent (running twice produces same result)

**Example Warning Output:**
```
warning: unexpected whitespace before type colon
  --> example.pg:5:14
   |
 5 | [r] .count :pg.int << 42
   |            ^ remove this whitespace
   |
   = help: canonical format is `.variable:type` with no space
   = note: for more information, run with `--explain whitespace-before-type-colon`
```

#### 2. Deprecated Syntax Warnings (Future)

Reserved for future language evolution when syntax changes require migration warnings.

### Warning Control

**CLI Flags:**
- `--no-warnings` - Suppress all warnings
- `--warn=<category>` - Enable specific warning category
- `--deny-warnings` - Treat all warnings as errors
- `--explain <warning-code>` - Show detailed explanation for warning

**Code Annotations (Future):**
```polyglot
// #[allow(whitespace-before-type-colon)]
[r] .count :pg.int << 42  // Warning suppressed
```

### Implementation Notes

**Warning System Structure:**
```rust
pub enum CompilerWarning {
    WhitespaceBeforeTypeColon {
        variable_name: String,
        span: Span,
    },
    // Future warning types...
}

impl CompilerWarning {
    pub fn code(&self) -> &str {
        match self {
            Self::WhitespaceBeforeTypeColon { .. } => "whitespace-before-type-colon",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::WhitespaceBeforeTypeColon { variable_name, .. } => {
                format!(
                    "Unexpected whitespace between variable name '{}' and type colon. \
                     Remove the space for canonical format.",
                    variable_name
                )
            }
        }
    }

    pub fn hint(&self) -> String {
        match self {
            Self::WhitespaceBeforeTypeColon { variable_name, .. } => {
                format!(
                    "Canonical: `{variable_name}:type` (no space)\n\
                     Found: `{variable_name} :type` (with space)",
                    variable_name = variable_name
                )
            }
        }
    }
}
```

**Parser Integration:**
- Parser emits warnings during type annotation parsing
- Warnings collected separately from errors
- All warnings displayed after successful compilation
- Exit code remains 0 unless `--deny-warnings` specified

---

## Error Recovery Strategies

### 1. Synchronization Points
When an error is found, recover at:
- Block markers `[X]` (always safe synchronization points)
- Newlines (for statement-level errors)
- Pipeline boundaries `[|]`

### 2. Multiple Error Reporting
- Continue parsing after recoverable errors
- Collect all errors and report together
- Limit to 10 errors per file to avoid overwhelming output

### 3. Partial AST Construction
- Build AST even with errors when possible
- Mark error nodes with `ErrorNode` variant
- Enable IDE-like experience (syntax highlighting with errors)

---

## Testing Requirements

### Error Detection Tests (Story 1.4)
Each error type must have at least:
1. **Positive test** - Error correctly detected and reported
2. **Span test** - Error location is accurate
3. **Message test** - Error message is clear and helpful
4. **Recovery test** - Parser continues after error (when applicable)

### Test Coverage Target
- >80% coverage for error.rs module
- 100% coverage for all error variants (at least one test per variant)

---

## Implementation Notes

### Story 1.4 Scope
- Define `ParserError` enum with all variants
- Implement `span()` and `hint()` methods
- Add unit tests for error construction
- **Parser implementation (error detection logic) is Story 1.5**

### Story 1.5 Scope (Parser Implementation)
- Implement recursive descent parser
- Integrate error detection at each parsing step
- Implement error recovery strategies
- Add integration tests with error cases

---

## Cross-References

- **ADR-004**: Error Handling with thiserror (docs/technical/architecture.md)
- **ADR-014**: String Concatenation Operator (docs/technical/architecture.md)
- **Block Hierarchy**: docs/technical/block-hierarchy-reference.md
- **Line Continuation**: docs/user/language/line-continuation.md
- **Story 1.4**: docs/project/stories/1-4-parser-ast-definitions.md
- **Story 1.5**: docs/project/stories/1-5-parser-implementation.md (future)

---

**End of Parser Error Detection Design**
