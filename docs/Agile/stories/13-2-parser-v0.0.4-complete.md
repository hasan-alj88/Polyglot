# Story 13.2: Complete v0.0.4 Parser Implementation

**Epic:** Epic 13 - v0.0.4 Syntax Migration
**Story ID:** 13.2
**Status:** Ready for Implementation
**Complexity:** High
**Estimated Effort:** 3-4 weeks
**Architecture:** [Parser Architecture v0.0.4](../architecture/parser-architecture-v0.0.4.md)
**Dependency:** Story 13.1 (Lexer v0.0.4) must be complete

---

## User Story

As a developer,
I want a complete v0.0.4 parser that builds ASTs from token streams,
So that all modern Polyglot features (pipeline composition, loops, enum definitions, trigger I/O) can be compiled and executed.

---

## Acceptance Criteria

**Given** a token stream from the v0.0.4 lexer
**When** I invoke `Parser::new(tokens).parse()`
**Then** a valid `Program` AST is returned

**And** parser supports:
- **NEW:** Pipeline composition blocks with `|>` operator
- **NEW:** Loop statements with indentation-based bodies (`[~]` ... `[*]`)
- **NEW:** Enum definition blocks with serial loads (`{#}` ... `{x}`)
- **NEW:** Trigger I/O wiring (`>output >> <input`)
- **NEW:** Error blocks with pattern matching (`[!]` with `[?]` patterns)
- **UPDATED:** Three-phase pipeline resolution (current file → same package → registry)

**And** parser validates:
- Type annotations with nested `pg.` prefix (e.g., `:pg.array.pg.string`)
- Pipeline composition chains (output-to-input wirings)
- Loop parameter bindings (unpack/pack symmetry)
- Enum serial load formats (YAML, JSON, TOML)
- Scope-wide error handling in enums (`[s][!]`)

**And** parser errors:
- Report line and column numbers
- Provide helpful suggestions
- Collect multiple errors (don't stop at first)
- Use synchronization points for recovery

**And** integration tests verify:
- Complete pipeline composition examples parse correctly
- Loop examples with nested error blocks parse correctly
- Enum definitions with multiple serial loads parse correctly
- Multi-file compilation with Phase 2 resolution works

---

## Prerequisites

- Story 13.1 (Lexer v0.0.4) ✅ Must be complete
- Story 1.5 (v0.0.3 Parser) ✅ Complete
- [Parser Architecture v0.0.4](../architecture/parser-architecture-v0.0.4.md) ✅ Complete

---

## Implementation Tasks

### Task 1: Update AST Type Definitions

**File:** `polyglot-parser/src/ast.rs`

Add new AST node types for v0.0.4 features:

```rust
// NEW: Pipeline Composition
pub struct PipelineComposition {
    pub operator: CompositionOperator,
    pub next_pipeline: Option<String>,
    pub wirings: Vec<OutputToInputWiring>,
    pub span: Span,
}

pub struct OutputToInputWiring {
    pub output_param: String,
    pub input_param: String,
    pub span: Span,
}

// NEW: Loop Statement
pub struct LoopStatement {
    pub marker: Marker,
    pub unpack_operator: UnpackCall,
    pub body: Vec<Statement>,
    pub pack_operator: PackCall,
    pub span: Span,
}

pub struct UnpackCall {
    pub stdlib_name: String,
    pub params: LoopParams,
    pub span: Span,
}

pub struct PackCall {
    pub stdlib_name: String,
    pub params: PackParams,
    pub span: Span,
}

// NEW: Enum Block with Serial Loads
pub struct EnumBlock {
    pub name: String,
    pub entries: Vec<EnumEntry>,
    pub aliases: Vec<AliasDeclaration>,
    pub serial_loads: Vec<SerialLoad>,
    pub error_handler: Option<ErrorBlock>,  // Scope-wide
    pub span: Span,
}

pub struct SerialLoad {
    pub format: SerialFormat,
    pub file_path: Expression,
    pub field_accessor: Option<FieldAccessor>,
    pub span: Span,
}

pub enum SerialFormat {
    YAML,
    JSON,
    TOML,
}

pub struct FieldAccessor {
    pub path: Vec<String>,
    pub span: Span,
}

// NEW: Error Block with Pattern Matching
pub struct ErrorBlock {
    pub marker: Marker,
    pub handlers: Vec<ErrorHandler>,
    pub span: Span,
}

pub struct ErrorHandler {
    pub pattern: ErrorPattern,
    pub body: Vec<Statement>,
    pub span: Span,
}

pub enum ErrorPattern {
    Specific { error_type: String },
    Category { prefix: String },
    Conversion { from: String, to: String },
    Wildcard,
}

// UPDATED: Pipeline Block with Trigger I/O
pub struct PipelineBlock {
    pub name: String,
    pub trigger: Option<TriggerDef>,
    pub inputs: Vec<InputParameter>,
    pub outputs: Vec<OutputParameter>,
    pub statements: Vec<Statement>,
    pub span: Span,
}

pub struct OutputParameter {
    pub name: String,
    pub type_annotation: Type,
    pub wiring: Option<WiringTarget>,  // For >> operator
    pub span: Span,
}

pub struct WiringTarget {
    pub target_pipeline: Option<String>,
    pub target_input: String,
    pub span: Span,
}
```

**Acceptance:**
- All AST types compile
- Types derive `Debug, Clone, PartialEq` (no Serialize - AST is internal)
- Span tracking for all nodes

---

### Task 2: Implement Pipeline Composition Parsing

**File:** `polyglot-parser/src/pipeline.rs`

```rust
impl Parser {
    fn parse_pipeline_composition(&mut self) -> Result<PipelineComposition, ParserError> {
        // [|] |>
        self.expect(TokenKind::MarkerPipelineDef)?;
        self.expect(TokenKind::PipelineComposition)?;

        // Optional next pipeline name
        let next_pipeline = if self.check(TokenKind::OpPipe) {
            self.advance();
            Some(self.expect_identifier()?)
        } else {
            None  // Final step in chain
        };

        // (|) >output >> <input wirings
        let mut wirings = Vec::new();
        if self.check(TokenKind::ParenPipeline) {
            self.advance();

            while !self.check_statement_end() {
                let wiring = self.parse_output_to_input_wiring()?;
                wirings.push(wiring);
            }
        }

        Ok(PipelineComposition {
            operator: CompositionOperator::Compose,
            next_pipeline,
            wirings,
            span: self.span(),
        })
    }

    fn parse_output_to_input_wiring(&mut self) -> Result<OutputToInputWiring, ParserError> {
        // >output:type >> <input
        self.expect(TokenKind::Output)?;
        let output_param = self.expect_identifier()?;

        self.expect(TokenKind::PrefixType)?;
        let _output_type = self.parse_type()?;

        self.expect(TokenKind::OpWire)?;  // >>

        self.expect(TokenKind::Input)?;
        let input_param = self.expect_identifier()?;

        Ok(OutputToInputWiring {
            output_param,
            input_param,
            span: self.span(),
        })
    }
}
```

**Acceptance:**
- `[|] |> |NextPipeline` parses correctly
- `(|) >output >> <input` wirings parse correctly
- Chains with multiple steps parse correctly
- Final step (no next pipeline) parses correctly

**Test Cases:**
```rust
#[test]
fn test_pipeline_composition() {
    let source = r#"
[|] |> |Step2
(|) >result:pg.string >> <input2
"#;

    let ast = parse(source).unwrap();
    assert_matches!(
        ast.statements[0],
        Statement::PipelineComposition(PipelineComposition {
            next_pipeline: Some("Step2"),
            wirings: vec![
                OutputToInputWiring {
                    output_param: "result",
                    input_param: "input2",
                    ..
                }
            ],
            ..
        })
    );
}
```

---

### Task 3: Implement Loop Statement Parsing with Indentation

**File:** `polyglot-parser/src/loop.rs`

```rust
impl Parser {
    fn parse_loop_statement(&mut self, marker: Marker) -> Result<LoopStatement, ParserError> {
        // [p] ~ForEach.Array
        self.expect(TokenKind::MarkerLoop)?;

        let unpack_operator = self.parse_unpack_call()?;

        // (~) parameters
        let loop_params = self.parse_loop_params()?;

        // Expect INDENT
        self.expect(TokenKind::Indent)?;

        // Parse loop body (indented statements)
        let mut body = Vec::new();
        while !self.check(TokenKind::Dedent) && !self.check(TokenKind::MarkerPack) {
            body.push(self.parse_statement()?);
        }

        // Expect DEDENT
        if self.check(TokenKind::Dedent) {
            self.advance();
        }

        // [*] *Into.Array
        self.expect(TokenKind::MarkerPack)?;

        let pack_operator = self.parse_pack_call()?;

        // (*) parameters
        let pack_params = self.parse_pack_params()?;

        Ok(LoopStatement {
            marker,
            unpack_operator,
            body,
            pack_operator,
            span: self.span(),
        })
    }

    fn parse_unpack_call(&mut self) -> Result<UnpackCall, ParserError> {
        // ~ForEach.Array
        self.expect(TokenKind::OpUnpack)?;

        let stdlib_name = self.parse_qualified_name()?;  // ForEach.Array

        Ok(UnpackCall {
            stdlib_name,
            params: LoopParams::default(),  // Parsed separately
            span: self.span(),
        })
    }

    fn parse_pack_call(&mut self) -> Result<PackCall, ParserError> {
        // *Into.Array
        self.expect(TokenKind::MarkerPack)?;

        let stdlib_name = self.parse_qualified_name()?;  // Into.Array

        Ok(PackCall {
            stdlib_name,
            params: PackParams::default(),  // Parsed separately
            span: self.span(),
        })
    }

    fn parse_loop_params(&mut self) -> Result<LoopParams, ParserError> {
        // (~) $collection >> $item or (~) $collection >> $item, $index
        self.expect(TokenKind::ParenLoop)?;

        let source = self.parse_expression()?;

        self.expect(TokenKind::OpWire)?;  // >>

        let item_var = self.expect_variable()?;

        let index_var = if self.check(TokenKind::Comma) {
            self.advance();
            Some(self.expect_variable()?)
        } else {
            None
        };

        Ok(LoopParams {
            source,
            item_var: Some(item_var),
            index_var,
            span: self.span(),
        })
    }
}
```

**Acceptance:**
- `[p] ~ForEach.Array` parses correctly
- Loop params `(~) $collection >> $item` parse correctly
- Indented loop body parses correctly
- `[*] *Into.Array` parses correctly
- Pack params `(*) $target << $item` parse correctly
- Nested loops parse correctly

**Test Cases:**
```rust
#[test]
fn test_loop_statement() {
    let source = r#"
[p] ~ForEach.Array
(~) $items >> $item
    [r] $result << |ProcessItem
        (|) <item:pg.string << $item
[*] *Into.Array
(*) $output << $result
"#;

    let ast = parse(source).unwrap();
    assert_matches!(
        ast.statements[0],
        Statement::Loop(LoopStatement {
            unpack_operator: UnpackCall { stdlib_name: "ForEach.Array", .. },
            body: _,
            pack_operator: PackCall { stdlib_name: "Into.Array", .. },
            ..
        })
    );
}
```

---

### Task 4: Implement Enum Block Parsing with Serial Loads

**File:** `polyglot-parser/src/enum.rs`

```rust
impl Parser {
    fn parse_enum_block(&mut self) -> Result<EnumBlock, ParserError> {
        // {#} EnumName
        self.expect(TokenKind::BlockEnumStart)?;

        let name = self.expect_identifier()?;

        let mut entries = Vec::new();
        let mut aliases = Vec::new();
        let mut serial_loads = Vec::new();
        let mut error_handler = None;

        while !self.check(TokenKind::BlockEnd) {
            if self.check(TokenKind::MarkerAlias) {
                // [A] alias_name = #Type.Value
                aliases.push(self.parse_alias_declaration()?);
            } else if self.check(TokenKind::MarkerSerialLoad) {
                // [s] yaml|json|toml path [.] field.path
                serial_loads.push(self.parse_serial_load()?);
            } else if self.check(TokenKind::MarkerError) {
                // [s][!] - Scope-wide error handler
                if !serial_loads.is_empty() {
                    error_handler = Some(self.parse_error_block()?);
                } else {
                    return Err(self.error(
                        "[s][!] scope-wide error handler requires [s] serial loads"
                    ));
                }
            } else {
                // Enum entry
                entries.push(self.parse_enum_entry()?);
            }
        }

        self.expect(TokenKind::BlockEnd)?;  // {x}

        Ok(EnumBlock {
            name,
            entries,
            aliases,
            serial_loads,
            error_handler,
            span: self.span(),
        })
    }

    fn parse_serial_load(&mut self) -> Result<SerialLoad, ParserError> {
        // [s] yaml|json|toml "path/to/file.yaml"
        self.expect(TokenKind::MarkerSerialLoad)?;

        let format = self.parse_serial_format()?;

        let file_path = self.parse_expression()?;

        // Optional [.] field.accessor.path
        let field_accessor = if self.check(TokenKind::MarkerSubfield) {
            self.advance();
            Some(self.parse_field_accessor()?)
        } else {
            None
        };

        Ok(SerialLoad {
            format,
            file_path,
            field_accessor,
            span: self.span(),
        })
    }

    fn parse_serial_format(&mut self) -> Result<SerialFormat, ParserError> {
        let format_name = self.expect_identifier()?;

        match format_name.to_lowercase().as_str() {
            "yaml" => Ok(SerialFormat::YAML),
            "json" => Ok(SerialFormat::JSON),
            "toml" => Ok(SerialFormat::TOML),
            _ => Err(self.error(&format!(
                "Unknown serial format '{}'. Expected yaml, json, or toml",
                format_name
            ))),
        }
    }

    fn parse_field_accessor(&mut self) -> Result<FieldAccessor, ParserError> {
        // Dot-separated path: field1.field2.field3
        let mut path = vec![self.expect_identifier()?];

        while self.check(TokenKind::Dot) {
            self.advance();
            path.push(self.expect_identifier()?);
        }

        Ok(FieldAccessor {
            path,
            span: self.span(),
        })
    }
}
```

**Acceptance:**
- `{#} EnumName` ... `{x}` parses correctly
- `[s] yaml "config.yaml"` parses correctly
- `[.] field.path` field accessor parses correctly
- `[A] alias_name = #Type.Value` parses correctly
- `[s][!]` scope-wide error handler parses correctly
- Multiple serial loads in one enum parse correctly

**Test Cases:**
```rust
#[test]
fn test_enum_with_serial_loads() {
    let source = r#"
{#} Config
[A] DefaultMode = #Config.Development

[s] yaml "config/base.yaml"
[s] json "config/overrides.json"
    [.] database.settings

[s][!]
    [?] #Error.FileNotFound
        [r] $fallback << "default_config"
{x}
"#;

    let ast = parse(source).unwrap();
    assert_matches!(
        ast.enums[0],
        EnumBlock {
            name: "Config",
            aliases: vec![_],
            serial_loads: vec![_, _],
            error_handler: Some(_),
            ..
        }
    );
}
```

---

### Task 5: Implement Error Block Parsing with Pattern Matching

**File:** `polyglot-parser/src/error.rs`

```rust
impl Parser {
    fn parse_error_block(&mut self) -> Result<ErrorBlock, ParserError> {
        // [!]
        self.expect(TokenKind::MarkerError)?;

        let mut handlers = Vec::new();

        while self.check(TokenKind::MarkerSwitch) {
            // [?] pattern
            handlers.push(self.parse_error_handler()?);
        }

        Ok(ErrorBlock {
            marker: Marker::Error,
            handlers,
            span: self.span(),
        })
    }

    fn parse_error_handler(&mut self) -> Result<ErrorHandler, ParserError> {
        // [?] pattern
        self.expect(TokenKind::MarkerSwitch)?;

        let pattern = self.parse_error_pattern()?;

        // Parse handler body (until next [?] or end of block)
        let mut body = Vec::new();
        while !self.check(TokenKind::MarkerSwitch)
            && !self.check_block_end()
        {
            body.push(self.parse_statement()?);
        }

        Ok(ErrorHandler {
            pattern,
            body,
            span: self.span(),
        })
    }

    fn parse_error_pattern(&mut self) -> Result<ErrorPattern, ParserError> {
        match self.current_token()?.kind {
            TokenKind::PrefixEnum => {
                // #Error.FileNotFound or #Error.* or #Error.File*
                let error_type = self.parse_enum_value()?;

                if error_type.contains('*') {
                    // Category pattern: #Error.File*
                    Ok(ErrorPattern::Category {
                        prefix: error_type.trim_end_matches('*').to_string(),
                    })
                } else if self.peek()?.kind == TokenKind::PrefixEnum {
                    // Conversion: #Error.* #Warning.*
                    let from = error_type;
                    self.advance();
                    let to = self.parse_enum_value()?;
                    Ok(ErrorPattern::Conversion { from, to })
                } else {
                    // Specific: #Error.FileNotFound
                    Ok(ErrorPattern::Specific { error_type })
                }
            }
            _ if self.check_wildcard() => {
                // *? wildcard
                self.advance();
                Ok(ErrorPattern::Wildcard)
            }
            _ => Err(self.error("Expected error pattern (#Error.Type or *?)")),
        }
    }
}
```

**Acceptance:**
- `[!]` marker parses correctly
- `[?] #Error.FileNotFound` specific pattern parses correctly
- `[?] #Error.File*` category pattern parses correctly
- `[?] #Error.* #Warning.*` conversion pattern parses correctly
- `[?] *?` wildcard pattern parses correctly
- Multiple handlers in one error block parse correctly

---

### Task 6: Implement Type Parsing with Nested Prefix Validation

**File:** `polyglot-parser/src/type.rs`

```rust
impl Parser {
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        // :pg.type or :pg.array.pg.element_type
        self.expect(TokenKind::Identifier)?;  // "pg"

        if self.current_token()?.lexeme != "pg" {
            return Err(self.error("Type must start with 'pg.'"));
        }

        self.expect(TokenKind::Dot)?;

        let type_name = self.expect_identifier()?;

        match type_name.as_str() {
            "string" => Ok(Type::Primitive(PrimitiveType::String)),
            "int" => Ok(Type::Primitive(PrimitiveType::Int)),
            "float" => Ok(Type::Primitive(PrimitiveType::Float)),
            "boolean" => Ok(Type::Primitive(PrimitiveType::Boolean)),
            "serial" => Ok(Type::Serial),
            "path" => Ok(Type::Path),
            "array" => {
                // CRITICAL: Must have SECOND pg. prefix
                self.expect(TokenKind::Dot)?;

                let pg2 = self.expect_identifier()?;
                if pg2 != "pg" {
                    return Err(self.error(
                        "Array element type must have pg. prefix (e.g., pg.array.pg.string)"
                    ));
                }

                self.expect(TokenKind::Dot)?;

                let element_type = self.parse_type_simple()?;

                Ok(Type::Array(Box::new(ArrayType {
                    element_type,
                    span: self.span(),
                })))
            }
            _ => {
                // Enum or foreign type
                Ok(Type::Enum(type_name))
            }
        }
    }
}
```

**Acceptance:**
- `:pg.string` parses correctly
- `:pg.array.pg.string` parses correctly (nested prefix)
- `:pg.array.string` returns error (missing second `pg.`)
- `:pg.serial` parses correctly
- `:pg.CustomEnum` parses as enum type

**Test Cases:**
```rust
#[test]
fn test_nested_type_annotations() {
    assert_ok!(parse_type(":pg.string"));
    assert_ok!(parse_type(":pg.array.pg.string"));
    assert_ok!(parse_type(":pg.array.pg.int"));

    assert_err!(parse_type(":pg.array.string"));  // Missing second pg.
    assert_err!(parse_type(":string"));            // Missing pg. prefix
}
```

---

### Task 7: Implement Three-Phase Pipeline Resolution

**File:** `polyglot-parser/src/resolution.rs`

```rust
impl Parser {
    fn resolve_pipeline_reference(&mut self, name: &str) -> Result<PipelineSignature, ParserError> {
        // PHASE 1: Current file namespace
        if let Some(pipeline) = self.current_file_pipelines.get(name) {
            return Ok(pipeline.clone());
        }

        // PHASE 2: Same package, different files (by [#] order)
        let current_package = self.get_current_package();
        let same_package_files = self.find_same_package_files(current_package)?;
        self.validate_file_ordering(&same_package_files)?;

        for file in same_package_files {
            if let Some(pipeline) = file.pipelines.get(name) {
                return Ok(pipeline.clone());
            }
        }

        // PHASE 3: External packages (registry/database)
        if let Some(pipeline) = self.registry.lookup(name)? {
            return Ok(pipeline);
        }

        Err(ParserError::UndeclaredPipeline {
            name: name.to_string(),
            available: self.suggest_similar_pipelines(name),
            span: self.span(),
        })
    }

    fn validate_file_ordering(&self, files: &[ParsedFile]) -> Result<(), ParserError> {
        let mut seen_numbers = std::collections::HashSet::new();

        for file in files {
            if let Some(order) = file.order_number {
                if !seen_numbers.insert(order) {
                    return Err(ParserError::DuplicateFileOrder {
                        number: order,
                        file1: file.path.clone(),
                        file2: files.iter()
                            .find(|f| f.order_number == Some(order) && f.path != file.path)
                            .unwrap().path.clone(),
                        span: file.span.clone(),
                    });
                }
            }
        }

        Ok(())
    }
}
```

**Acceptance:**
- Phase 1 searches current file first
- Phase 2 searches same-package files in `[#]` order
- Phase 3 queries external registry
- File ordering validates (no duplicate `[#]` numbers)
- Pipeline in file `[#] 2` cannot be called from file `[#] 1`
- Helpful error with suggestions if pipeline not found

---

### Task 8: Implement Error Recovery with Synchronization

**File:** `polyglot-parser/src/recovery.rs`

```rust
impl Parser {
    fn synchronize(&mut self) {
        // Panic mode recovery: skip tokens until next statement
        while !self.is_at_end() {
            match self.current_token().map(|t| &t.kind) {
                Ok(TokenKind::MarkerRun)
                | Ok(TokenKind::MarkerParallel)
                | Ok(TokenKind::MarkerFork)
                | Ok(TokenKind::MarkerLoop)
                | Ok(TokenKind::MarkerError)
                | Ok(TokenKind::BlockEnd) => {
                    return;  // Found statement start or block end
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn parse_with_recovery<T, F>(&mut self, f: F) -> Result<T, ParserError>
    where
        F: FnOnce(&mut Self) -> Result<T, ParserError>,
    {
        match f(self) {
            Ok(value) => Ok(value),
            Err(e) => {
                self.errors.push(e.clone());
                self.synchronize();
                Err(e)
            }
        }
    }

    pub fn parse_tolerant(&mut self) -> (Option<Program>, Vec<ParserError>) {
        let program = self.parse().ok();
        (program, self.errors.clone())
    }
}
```

**Acceptance:**
- Multiple errors collected in single parse
- Synchronization skips to next statement
- Parser continues after error (doesn't panic)
- All errors reported at end

---

### Task 9: Comprehensive Integration Tests

**File:** `polyglot-parser/tests/integration_tests.rs`

```rust
#[test]
fn test_complete_pipeline_composition_chain() {
    let source = include_str!("../fixtures/pipeline_composition_example.pg");
    let ast = parse(source).unwrap();

    assert_eq!(ast.pipelines.len(), 3);
    assert!(ast.pipelines[0].statements.iter().any(|s| matches!(s, Statement::PipelineComposition(_))));
}

#[test]
fn test_loop_with_nested_error_blocks() {
    let source = include_str!("../fixtures/loop_with_errors.pg");
    let ast = parse(source).unwrap();

    assert_matches!(
        ast.pipelines[0].statements[0],
        Statement::Loop(LoopStatement {
            body: vec![
                Statement::PipelineCall(_),
                Statement::ErrorBlock(_),
            ],
            ..
        })
    );
}

#[test]
fn test_enum_with_multiple_serial_loads() {
    let source = include_str!("../fixtures/enum_config_loads.pg");
    let ast = parse(source).unwrap();

    assert_eq!(ast.enums[0].serial_loads.len(), 3);
    assert!(ast.enums[0].error_handler.is_some());
}

#[test]
fn test_multi_file_compilation() {
    let files = vec![
        ("file1.pg", "[#] 1\n{|} Pipeline1\n{x}"),
        ("file2.pg", "[#] 2\n{|} Pipeline2\n[r] |Pipeline1\n{x}"),
    ];

    let program = compile_multi_file(files).unwrap();
    assert_eq!(program.pipelines.len(), 2);
}
```

**Acceptance:**
- All documentation examples parse correctly
- Complex nested structures handled
- Error recovery works
- Multi-file compilation successful

---

### Task 10: Performance Optimization

**File:** `polyglot-parser/benches/parser_bench.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_1000_line_file(c: &mut Criterion) {
    let source = generate_test_file(1000);
    let tokens = lex(&source);

    c.bench_function("parse 1000 lines", |b| {
        b.iter(|| {
            let parser = Parser::new(black_box(tokens.clone()));
            parser.parse().unwrap()
        })
    });
}

criterion_group!(benches, bench_1000_line_file);
criterion_main!(benches);
```

**Acceptance:**
- 1,000-line file parses in <500ms
- 10,000-line file parses in <5s
- Memory usage <20MB for 10,000-line file

---

## Definition of Done

- [ ] All AST node types defined and tested
- [ ] Pipeline composition parsing works
- [ ] Loop statement parsing with indentation works
- [ ] Enum block parsing with serial loads works
- [ ] Error block parsing with pattern matching works
- [ ] Type validation with nested `pg.` prefix works
- [ ] Three-phase pipeline resolution works
- [ ] Error recovery with synchronization works
- [ ] Unit tests pass (>95% code coverage)
- [ ] Integration tests pass (all documentation examples)
- [ ] Performance benchmarks meet targets
- [ ] Code reviewed and approved
- [ ] Documentation updated

---

## Testing Requirements

### Unit Tests
- [ ] All AST node types
- [ ] Pipeline composition parsing
- [ ] Loop parsing with indentation
- [ ] Enum parsing with serial loads
- [ ] Error block pattern matching
- [ ] Type validation (nested prefix)
- [ ] Three-phase resolution
- [ ] Error recovery

### Integration Tests
- [ ] Complete pipeline composition chains
- [ ] Loops with nested error blocks
- [ ] Enums with multiple serial loads
- [ ] Multi-file compilation
- [ ] Trigger I/O wiring examples

### Performance Tests
- [ ] 1,000-line file benchmark
- [ ] 10,000-line file stress test
- [ ] Memory profiling

---

## Technical Notes

- **Recursive Descent:** Hand-written for clarity and error control
- **Error Recovery:** Use synchronization points (statement starts, block ends)
- **Type Validation:** CRITICAL - check double `pg.` prefix for arrays
- **Three-Phase Resolution:** Check current file first, then same package, then registry
- **No Backtracking:** Lookahead sufficient (lexer handles disambiguation)

---

## References

- [Parser Architecture v0.0.4](../architecture/parser-architecture-v0.0.4.md)
- [Lexer Architecture v0.0.4](../architecture/lexer-architecture-v0.0.4.md)
- [v0.0.4 Grammar](../../User/reference/grammar.md)
- [Polly Examples](../../../bmad-polly/data/memory/)

---

**Status:** Ready for Implementation
**Next Story:** Story 13.3 - IR Generation v0.0.4 Updates
