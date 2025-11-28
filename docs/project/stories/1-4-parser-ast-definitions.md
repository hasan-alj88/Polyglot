# Story 1.4: Parser AST Definitions

Status: ready-for-dev

## Story

As a developer,
I want AST node types defined for all Polyglot constructs,
so that the parser can build a structured representation of `.pg` files.

## Acceptance Criteria

1. **AST node types complete**
   - Defined in `polyglot-parser/src/ast.rs`
   - **Core AST Nodes**:
     - `Pipeline` - Top-level construct with metadata (name, trigger config)
     - `Block` - Contains statements, has block type (sequential, parallel, etc.)
     - `Statement` - Executable statements (assignments, pipeline calls, control flow)
     - `Expression` - Evaluable expressions (literals, identifiers, operations)
     - `Type` - Polyglot type annotations (for pipeline signatures)
   - **Supporting Structures**:
     - `Identifier` - Variable, pipeline, enum, error references
     - `Literal` - String, integer, float, datetime, collection literals
     - `Operator` - Binary and unary operators
     - `Pattern` - Wildcards, regex patterns, range expressions

2. **AST node structure**
   - Each node includes source location (`Span { start: Position, end: Position }`)
   - Recursive structures use `Box<Node>` for heap allocation
   - Collections of child nodes use `Vec<Node>`
   - Metadata fields for semantic information (pipeline name, trigger config, type annotations)
   - Implements `Debug`, `Clone`, `PartialEq` traits for all AST nodes

3. **Visitor pattern support**
   - Define `Visitor` trait for AST traversal
   - Implement `accept()` method on all AST nodes
   - Enable IR generation and analysis passes

4. **Unit tests**
   - AST construction for simple pipeline structure
   - Nested blocks (blocks within blocks)
   - All statement and expression variants
   - Source location tracking accuracy
   - Test coverage >80% for ast.rs module

5. **Error handling system**
   - Comprehensive `ParserError` enum defined in `polyglot-parser/src/error.rs`
   - **Error Categories**:
     - Lexer errors (pass-through from lexer crate)
     - Unexpected token errors (with context and expected tokens)
     - Block hierarchy violations (order, nesting, duplicates)
     - Operator misuse (string concatenation, type mismatches)
     - Type annotation errors (invalid syntax, unknown types)
     - Identifier errors (invalid format, non-ASCII, missing prefix)
     - Expression errors (unclosed delimiters, invalid ranges)
     - Statement errors (invalid targets, missing operators)
     - Pipeline call errors (invalid references, missing prefix)
     - Line continuation errors (invalid context, broken continuation)
     - EOF errors (unexpected end of file)
   - All errors include precise `Span` for location tracking
   - All errors implement `Send + Sync` for async compatibility (ADR-004)
   - Helpful error messages with hints for fixing issues
   - Unit tests for all error variants (>80% coverage for error.rs)
   - **Note**: Error *detection logic* is Story 1.5; this story defines error *types*

## Tasks / Subtasks

- [ ] Create polyglot-parser crate structure (AC: #1)
  - [ ] Add `polyglot-parser` to workspace Cargo.toml members
  - [ ] Create `polyglot-parser/Cargo.toml` with dependencies
  - [ ] Create `polyglot-parser/src/lib.rs` with module exports
  - [ ] Add dependency on `polyglot-lexer` for Token types
  - [ ] Verify `cargo build` succeeds

- [ ] Define Position and Span types (AC: #2)
  - [ ] Create `span.rs` module
  - [ ] Define `Position` struct with line, column, offset fields
  - [ ] Define `Span` struct with start and end positions
  - [ ] Implement `Debug`, `Clone`, `PartialEq`, `Eq` traits
  - [ ] Add helper methods: `new()`, `merge()`, `contains()`

- [ ] Define core AST node enums (AC: #1)
  - [ ] Create `ast.rs` module
  - [ ] Define `Pipeline` struct with fields:
    - `name: String`
    - `input: Option<Vec<InputParameter>>`
    - `output: Option<TypeAnnotation>`
    - `trigger: Option<TriggerConfig>`
    - `queue: Option<QueueConfig>`
    - `wrapper: Option<WrapperConfig>`
    - `body: Block`
    - `span: Span`
  - [ ] Define `Block` struct with fields:
    - `block_type: BlockType` (Sequential, Parallel, Conditional, etc.)
    - `statements: Vec<Statement>`
    - `span: Span`
  - [ ] Define `BlockType` enum variants:
    - `Sequential` - `[r]` blocks
    - `Parallel` - `[p]` blocks
    - `Conditional` - `[?]` blocks
    - `Join` - `[Y]` blocks
    - `Background` - `[b]` blocks
    - `Streaming` - `[s]` blocks

- [ ] Define Statement enum (AC: #1)
  - [ ] Create `Statement` enum with variants:
    - `VariableDeclaration { name: Identifier, init: Option<Expression>, span: Span }`
    - `Assignment { target: Identifier, value: Expression, span: Span }`
    - `PipelineCall { pipeline: Identifier, args: Vec<Expression>, span: Span }`
    - `Return { value: Option<Expression>, span: Span }`
    - `Block { block: Block, span: Span }`
    - `Conditional { condition: Expression, then_branch: Block, else_branch: Option<Block>, span: Span }`
    - `ErrorCatch { error_type: Identifier, handler: Block, span: Span }`
  - [ ] Implement `Debug`, `Clone`, `PartialEq` traits

- [ ] Define Expression enum (AC: #1)
  - [ ] Create `Expression` enum with variants:
    - `Literal { value: Literal, span: Span }`
    - `Identifier { name: Identifier, span: Span }`
    - `PipelineCall { pipeline: Identifier, args: Vec<Expression>, span: Span }`
    - `BinaryOp { left: Box<Expression>, op: BinaryOperator, right: Box<Expression>, span: Span }`
    - `UnaryOp { op: UnaryOperator, operand: Box<Expression>, span: Span }`
    - `RangeCheck { value: Box<Expression>, range: RangeExpression, span: Span }`
    - `PatternMatch { value: Box<Expression>, pattern: Pattern, span: Span }`
    - `Unpack { value: Box<Expression>, span: Span }`
  - [ ] Implement `Debug`, `Clone`, `PartialEq` traits

- [ ] Define Identifier types (AC: #1)
  - [ ] Create `Identifier` enum with variants:
    - `Variable(String)` - `.var_name`
    - `Pipeline(String)` - `|pipeline_name` or `|Module.Pipeline`
    - `Enum(String)` - `#Enum.Variant`
    - `Error(String)` - `!ErrorType`
    - `JoinPoint(String)` - `~Y.join_name`
    - `Package(String)` - `@package_name`
  - [ ] Add helper methods: `as_str()`, `is_qualified()`, `module_path()`

- [ ] Define Literal types (AC: #1)
  - [ ] Create `Literal` enum with variants:
    - `String(String)` - String literals with interpolation metadata
    - `Integer(i64)`
    - `Float(f64)`
    - `Boolean(bool)` - Represented as `#Boolean.True` or `#Boolean.False`
    - `Datetime(String)` - `DT"..."` literals (string form, parsed later)
    - `Duration(String, DurationUnit)` - `DT.Minutes"5"`
    - `Collection(Vec<Expression>)` - `{1, 2, 3}`
    - `None` - `#None`

- [ ] Define operator enums (AC: #1)
  - [ ] Create `BinaryOperator` enum:
    - Comparison: `Equal`, `NotEqual`, `Greater`, `Less`, `GreaterEqual`, `LessEqual`
    - Logical: `And`, `Or`, `Xor`, `Nand`, `Nor`
    - Data flow: `Push`, `Pull`, `DefaultFrom`, `DefaultTo`
  - [ ] Create `UnaryOperator` enum:
    - `Negate`, `Not`, `Unpack`

- [ ] Define pattern and range types (AC: #1)
  - [ ] Create `Pattern` enum:
    - `Wildcard` - `*?`
    - `Regex(String)` - `re?"pattern"`
    - `EnumVariant(Identifier)` - `#Enum.Variant`
  - [ ] Create `RangeExpression` struct:
    - `start: Box<Expression>`
    - `end: Box<Expression>`
    - `inclusive_start: bool` - true for `?[`, false for `?(`
    - `inclusive_end: bool` - true for `]`, false for `)`

- [ ] Define type annotation structures (AC: #1)
  - [ ] Create `TypeAnnotation` enum:
    - `Named(String)` - `pg\string`, `pg\int`
    - `Pipeline(String)` - Pipeline-typed values
    - `Enum(String)` - Enumeration types
    - `Collection(Box<TypeAnnotation>)` - `{pg\int}` collections
    - `Optional(Box<TypeAnnotation>)` - Optional types
  - [ ] Create `InputParameter` struct:
    - `name: String`
    - `type_annotation: Option<TypeAnnotation>`
    - `default: Option<Expression>`
    - `span: Span`

- [ ] Define configuration structures (AC: #1)
  - [ ] Create `TriggerConfig` struct for `[t]` blocks
  - [ ] Create `QueueConfig` struct for `[Q]` blocks
  - [ ] Create `WrapperConfig` struct for `[W]` blocks

- [ ] Implement Visitor trait (AC: #3)
  - [ ] Create `visitor.rs` module
  - [ ] Define `Visitor` trait with visit methods for each AST node type
  - [ ] Implement `accept()` method on all AST node types
  - [ ] Add example visitor implementation (e.g., `PrintVisitor` for debugging)

- [ ] Implement helper methods and traits (AC: #2)
  - [ ] Implement `Display` trait for major AST nodes (debugging)
  - [ ] Add `span()` method to all node types returning `&Span`
  - [ ] Add AST builder pattern (optional, for ergonomic test construction)

- [ ] Write comprehensive unit tests (AC: #4)
  - [ ] Test AST construction for simple pipeline
  - [ ] Test nested block structures
  - [ ] Test all statement variants
  - [ ] Test all expression variants
  - [ ] Test source location tracking
  - [ ] Test visitor pattern with example visitor
  - [ ] Verify >80% code coverage with cargo-tarpaulin

- [ ] Define comprehensive error types (AC: #5)
  - [ ] Create `error.rs` module
  - [ ] Define `ParserError` enum with all error categories:
    - `LexerError` (pass-through from lexer crate)
    - `UnexpectedToken` (expected vs found)
    - `UnexpectedEof` (context and start location)
    - `BlockOrderViolation` (file-level hierarchy)
    - `DuplicateBlock` (multiple same blocks)
    - `MissingRequiredBlock` (required block not found)
    - `InvalidPipelineBlockOrder` (pipeline canonical order)
    - `MultipleExecutionBlocks` (conflicting execution types)
    - `InvalidNesting` (invalid parent-child)
    - `BindingWithoutPipelineCall` ([<]/[>] without pipeline)
    - `OrphanedBlock` (block outside required parent)
    - `InvalidStringConcatenation` (+" on variables)
    - `OperatorTypeMismatch` (operator on wrong type)
    - `InvalidTypeSyntax` (type annotation errors)
    - `UnknownType` (unrecognized type name)
    - `InvalidIdentifier` (format violations)
    - `NonAsciiIdentifier` (non-ASCII characters)
    - `MissingIdentifierPrefix` (missing . | # ! prefix)
    - `UnclosedDelimiter` (missing closing bracket/paren)
    - `InvalidRangeSyntax` (malformed range operators)
    - `InvalidAssignmentTarget` (assign to non-variable)
    - `MissingAssignmentOperator` (missing <<, >>, <~)
    - `InvalidPipelineReference` (malformed pipeline name)
    - `MissingPipelinePrefix` (missing | prefix)
    - `InvalidLineContinuation` ([*] without context)
    - `BrokenContinuation` ([*] after different block)
  - [ ] Add `#[derive(Error, Debug, Clone, PartialEq)]` to enum
  - [ ] Implement custom error messages with thiserror
  - [ ] Add `span()` method returning primary error location
  - [ ] Add `hint()` method returning optional fix suggestions
  - [ ] Ensure all errors include `Span` for precise location tracking
  - [ ] Verify `Send + Sync` bounds (automatic with thiserror)

- [ ] Write error handling unit tests (AC: #5)
  - [ ] Test error construction for each variant
  - [ ] Test `span()` method returns correct location
  - [ ] Test `hint()` method returns appropriate suggestions
  - [ ] Test error message formatting
  - [ ] Verify >80% coverage for error.rs module
  - [ ] Test Send + Sync bounds (compile-time verification)

- [ ] Update module exports (AC: #1, #2, #3, #5)
  - [ ] Export all AST types from lib.rs
  - [ ] Export ParserError and related error types
  - [ ] Export Visitor trait and related types
  - [ ] Add module-level documentation
  - [ ] Verify cargo clippy passes with no warnings

## Dev Notes

### Architecture Context

**From Architecture Document** [Source: docs/technical/architecture.md]

- **Crate**: `polyglot-parser` (library crate, to be created in this story)
- **Dependencies**: `polyglot-lexer` (for Token types), `serde` (for serialization)
- **Error Handling**: Use `thiserror` for structured error types per ADR-004
- **Critical Requirement**: All errors must be `Send + Sync` for async compatibility
- **Technology**: Rust 2021 Edition with workspace dependency inheritance
- **Testing**: Unit tests in `#[cfg(test)]` modules, target >80% coverage (NFR-M2)

### Learnings from Previous Stories

**From Story 1.2 (Status: done)** [Source: docs/project/stories/1-2-lexer-token-definitions.md]

- **Token Foundation**: Lexer provides 102+ token types with source location tracking
- **Pattern to Follow**: Define types first (Story 1.2), implement logic later (Story 1.3)
- **Quality Standards**: Comprehensive traits (`Debug`, `Clone`, `PartialEq`), >80% test coverage
- **Error Design**: Use thiserror, include location and context in all errors

**From Story 1.3 (Status: done)** [Source: Implementation]

- **Workspace Setup**: Adding new crates to workspace is straightforward
- **Integration**: Lexer crate integration works smoothly with workspace dependencies
- **Testing**: Unit tests with `#[cfg(test)]` modules work well

### Project Structure Notes

```
polyglot-parser/
├── Cargo.toml                    # NEW: Configure with workspace deps
├── src/
│   ├── lib.rs                    # NEW: Export AST, Visitor, Error, and utilities
│   ├── ast.rs                    # NEW: Core AST node definitions
│   ├── span.rs                   # NEW: Position and Span types
│   ├── error.rs                  # NEW: Comprehensive ParserError types
│   ├── visitor.rs                # NEW: Visitor trait for AST traversal
│   └── tests.rs                  # NEW: Unit tests (or tests/ directory)
```

### Implementation Guidance

1. **AST Design Principles**:
   - Keep nodes as simple data structures (no behavior, just data)
   - Use `Box<>` for recursive types to avoid infinite size
   - Every node MUST have span for error reporting
   - Visitor pattern enables multiple analysis passes without modifying nodes

2. **v0.0.2 Syntax Coverage**:
   - **Block Markers**: Each block type maps to AST node (e.g., `[r]` → `BlockType::Sequential`)
   - **Operators**: Binary and unary operators map to expression nodes
   - **Identifiers**: Different identifier prefixes (`.`, `|`, `#`, `!`, `~Y`) are distinct types
   - **NO KEYWORDS**: v0.0.2 eliminated keywords, use enumeration-based booleans

3. **Span Design**:
   - Position: `{ line: usize, column: usize, offset: usize }`
   - Span: `{ start: Position, end: Position }`
   - Every node should have a span for error messages
   - Spans enable precise error reporting ("Error at line 5, column 12")

4. **Testing Strategy**:
   - Unit tests in `ast.rs` and related modules using `#[cfg(test)]`
   - Test each AST node type construction
   - Test visitor pattern with example visitor
   - Consider builder pattern for ergonomic test construction

5. **Performance Considerations**:
   - AST nodes are created once per parse, performance less critical than lexer
   - Use `Box<>` to keep enum sizes reasonable
   - Spans are small (3 usizes), safe to copy

6. **Parser Integration Preview** (Story 1.5):
   - Parser will consume tokens from lexer
   - Parser will construct these AST nodes
   - Error recovery will use spans for helpful messages
   - Visitor pattern enables IR generation in Epic 2

7. **Error Handling Design**:
   - **Comprehensive Coverage**: 26+ error types covering all syntax and semantic violations
   - **Location Tracking**: Every error includes `Span` for precise source location
   - **Helpful Messages**: Use thiserror's `#[error("...")]` with interpolated fields
   - **Hints**: Provide `hint()` method for actionable fix suggestions
   - **Async Compatible**: All errors must be `Send + Sync` (automatic with thiserror)
   - **Error Categories**:
     * **Lexer pass-through**: Wrap `LexerError` for consistency
     * **Token errors**: Expected vs. found, EOF
     * **Hierarchy violations**: Block order, nesting, duplicates
     * **Operator misuse**: String concatenation, type mismatches
     * **Type errors**: Invalid syntax, unknown types
     * **Identifier errors**: Format, ASCII, prefix
     * **Expression errors**: Delimiters, ranges
     * **Statement errors**: Assignment targets, operators
     * **Pipeline errors**: References, prefixes
     * **Line continuation**: Invalid context, broken continuation
   - **Design Reference**: See `docs/technical/parser-error-detection-design.md` for complete specification
   - **Story 1.4 Scope**: Define error *types* only; Story 1.5 implements error *detection logic*

### References

- [Source: docs/project/epics.md#Story-1.4]
- [Source: docs/technical/architecture.md#Technology-Stack-Details]
- [Source: docs/technical/architecture.md#ADR-004-Error-Handling]
- [Source: docs/technical/architecture.md#ADR-014-String-Concatenation-Operator]
- [Source: docs/technical/parser-error-detection-design.md] - **Comprehensive error detection system**
- [Source: docs/technical/block-hierarchy-reference.md] - Block nesting validation rules
- [Source: docs/user/language/00-quick-start.md] - Syntax reference
- [Source: docs/user/language/08-line-continuation.md] - Line continuation and `+"` operator
- [Source: docs/project/stories/1-2-lexer-token-definitions.md] - Similar pattern

## Dev Agent Record

### Context Reference

<!-- Path(s) to story context XML will be added here by story-context workflow -->

### Agent Model Used

_To be filled by dev agent_

### Debug Log References

_To be filled by dev agent during implementation_

### Completion Notes List

_To be filled by dev agent upon story completion_

### File List

_To be filled by dev agent with files created/modified/deleted_
