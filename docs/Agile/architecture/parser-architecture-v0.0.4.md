# Parser Architecture - Polyglot v0.0.4

**Author:** Winston (Architect)
**Date:** 2025-12-29
**Version:** v0.0.4
**Status:** Architecture Complete - Ready for Implementation

---

## Executive Summary

This document defines the complete parser architecture for Polyglot v0.0.4, transforming token streams into Abstract Syntax Trees (AST). The parser uses a recursive descent approach with explicit error recovery, supporting all v0.0.4 language features including pipeline composition, loops, error blocks, enum definitions, and trigger I/O wiring.

**Key Architecture Decisions:**
- **Recursive Descent Parser** - Hand-written for clarity and error control
- **Complete AST Design** - All v0.0.4 constructs represented
- **Three-Phase Resolution** - Current file → Same package → External registry
- **Explicit Error Recovery** - Synchronization points for multiple error reporting
- **Type Validation** - Nested type annotations with double `pg.` prefix checking

---

## AST Design

### Root Structure

```rust
pub struct Program {
    pub package: Option<PackageBlock>,
    pub enums: Vec<EnumBlock>,
    pub pipelines: Vec<PipelineBlock>,
    pub span: Span,
}

pub struct Span {
    pub start: Position,
    pub end: Position,
}

pub struct Position {
    pub line: usize,
    pub column: usize,
}
```

### Block Nodes

#### Package Block

```rust
pub struct PackageBlock {
    pub registry: String,          // e.g., "local", "github.com/user/repo"
    pub path: String,               // e.g., "utilities/math"
    pub version: String,            // e.g., "0.1.0"
    pub span: Span,
}
```

#### Pipeline Block

```rust
pub struct PipelineBlock {
    pub name: String,
    pub trigger: Option<TriggerDef>,
    pub inputs: Vec<InputParameter>,
    pub outputs: Vec<OutputParameter>,
    pub statements: Vec<Statement>,
    pub span: Span,
}

pub struct TriggerDef {
    pub trigger_type: TriggerType,
    pub config: TriggerConfig,
    pub span: Span,
}

pub enum TriggerType {
    CLI,
    FolderNewFiles { folder: String },
    HTTPRequest { endpoint: String },
    ScheduleCron { schedule: String },
}

pub struct InputParameter {
    pub name: String,
    pub type_annotation: Type,
    pub span: Span,
}

pub struct OutputParameter {
    pub name: String,
    pub type_annotation: Type,
    pub wiring: Option<WiringTarget>,  // For >> operator
    pub span: Span,
}

pub struct WiringTarget {
    pub target_pipeline: Option<String>,  // For |> composition
    pub target_input: String,              // Parameter name after >>
    pub span: Span,
}
```

#### Enum Block

```rust
pub struct EnumBlock {
    pub name: String,
    pub entries: Vec<EnumEntry>,
    pub aliases: Vec<AliasDeclaration>,
    pub serial_loads: Vec<SerialLoad>,
    pub error_handler: Option<ErrorBlock>,  // Scope-wide [s][!]
    pub span: Span,
}

pub struct EnumEntry {
    pub value_name: String,
    pub fields: Vec<FieldDefinition>,
    pub span: Span,
}

pub struct SerialLoad {
    pub format: SerialFormat,      // YAML, JSON, TOML
    pub file_path: Expression,     // Can be variable or literal
    pub field_accessor: Option<FieldAccessor>,  // [.] syntax
    pub span: Span,
}

pub enum SerialFormat {
    YAML,
    JSON,
    TOML,
}

pub struct FieldAccessor {
    pub path: Vec<String>,  // Dot-separated field path
    pub span: Span,
}

pub struct AliasDeclaration {
    pub alias_name: String,
    pub full_type: String,
    pub full_value: String,
    pub span: Span,
}
```

### Statement Nodes

```rust
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    PipelineCall(PipelineCall),
    PipelineComposition(PipelineComposition),
    Fork(ForkStatement),
    Loop(LoopStatement),
    ErrorBlock(ErrorBlock),
    SerialLoad(SerialLoad),
}

pub struct VariableDeclaration {
    pub marker: Marker,            // [r], [p], etc.
    pub name: String,
    pub type_annotation: Option<Type>,
    pub assignment: Assignment,
    pub span: Span,
}

pub struct Assignment {
    pub operator: AssignmentOperator,  // <<, <~, >>, ~>
    pub value: Expression,
    pub span: Span,
}

pub enum AssignmentOperator {
    PullFinal,      // <<
    PullDefault,    // <~
    PushFinal,      // >>
    PushDefault,    // ~>
}

pub struct PipelineCall {
    pub marker: Marker,
    pub pipeline_name: String,
    pub inputs: Vec<InputBinding>,
    pub outputs: Vec<OutputBinding>,
    pub span: Span,
}

pub struct InputBinding {
    pub parameter_name: String,
    pub operator: AssignmentOperator,
    pub value: Expression,
    pub span: Span,
}

pub struct OutputBinding {
    pub parameter_name: String,
    pub operator: AssignmentOperator,
    pub target: Variable,
    pub span: Span,
}

pub struct PipelineComposition {
    pub operator: CompositionOperator,  // |>
    pub next_pipeline: Option<String>,  // None for final in chain
    pub wirings: Vec<OutputToInputWiring>,
    pub span: Span,
}

pub struct OutputToInputWiring {
    pub output_param: String,
    pub input_param: String,
    pub span: Span,
}
```

### Loop Statement

```rust
pub struct LoopStatement {
    pub marker: Marker,               // [p] (parallel) or [r] (sequential)
    pub unpack_operator: UnpackCall,  // ~ForEach.Array
    pub body: Vec<Statement>,         // Indentation-based
    pub pack_operator: PackCall,      // *Into.Array
    pub span: Span,
}

pub struct UnpackCall {
    pub stdlib_name: String,       // e.g., "ForEach.Array"
    pub params: LoopParams,        // (~) parameters
    pub span: Span,
}

pub struct PackCall {
    pub stdlib_name: String,       // e.g., "Into.Array"
    pub params: PackParams,        // (*) parameters
    pub span: Span,
}

pub struct LoopParams {
    pub source: Expression,        // Collection to iterate
    pub item_var: Option<String>,  // Item variable name
    pub index_var: Option<String>, // Index variable name (optional)
    pub span: Span,
}

pub struct PackParams {
    pub target: Variable,          // Collection to build
    pub span: Span,
}
```

### Fork Statement

```rust
pub struct ForkStatement {
    pub condition_var: String,     // Variable to check
    pub branches: Vec<ForkBranch>,
    pub wildcard: Option<WildcardBranch>,  // [f] *?
    pub span: Span,
}

pub struct ForkBranch {
    pub pattern: Pattern,
    pub body: Vec<Statement>,
    pub span: Span,
}

pub enum Pattern {
    EnumValue { type_name: String, value_name: String },
    ErrorType { error_type: String },
    Comparison { operator: ComparisonOp, value: Expression },
    Wildcard,  // *?
}

pub struct WildcardBranch {
    pub body: Vec<Statement>,
    pub span: Span,
}
```

### Error Block

```rust
pub struct ErrorBlock {
    pub marker: Marker,            // [!]
    pub handlers: Vec<ErrorHandler>,
    pub span: Span,
}

pub struct ErrorHandler {
    pub pattern: ErrorPattern,
    pub body: Vec<Statement>,
    pub span: Span,
}

pub enum ErrorPattern {
    Specific { error_type: String },        // [?] #Error.FileNotFound
    Category { prefix: String },            // [?] #Error.File*
    Conversion { from: String, to: String }, // [?] #Error.* #Warning.*
    Wildcard,                                // [?] *?
}
```

### Expression Nodes

```rust
pub enum Expression {
    Variable(Variable),
    Literal(Literal),
    StringInterpolation(StringInterpolation),
    PipelineInline(PipelineInline),
    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>),
    FieldAccess(Box<FieldAccess>),
}

pub struct Variable {
    pub name: String,
    pub span: Span,
}

pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    EnumValue { type_name: String, value_name: String },
    Path(String),
}

pub struct StringInterpolation {
    pub template: String,
    pub interpolations: Vec<Interpolation>,
    pub span: Span,
}

pub struct Interpolation {
    pub variable: String,
    pub position: usize,
}

pub struct PipelineInline {
    pub pipeline_name: String,
    pub arguments: String,  // Formatted string template
    pub span: Span,
}

pub struct BinaryOp {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
    pub span: Span,
}

pub enum BinaryOperator {
    Equal,       // =?
    NotEqual,    // =!?
    Greater,     // >?
    Less,        // <?
    GreaterEq,   // >=?
    LessEq,      // <=?
    LogicalAnd,  // &?
    LogicalOr,   // |?
    In,          // in?
    NotIn,       // in!?
    Contains,    // *?
    RegexMatch,  // re?
}
```

### Type System

```rust
pub enum Type {
    Primitive(PrimitiveType),
    Array(Box<ArrayType>),
    Enum(String),
    Serial,
    Path,
    Foreign(String),
}

pub enum PrimitiveType {
    String,
    Int,
    Float,
    Boolean,
}

pub struct ArrayType {
    pub element_type: Type,  // MUST have pg. prefix
    pub span: Span,
}
```

---

## Parsing Algorithms

### Top-Level Program Parsing

```rust
impl Parser {
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut package = None;
        let mut enums = Vec::new();
        let mut pipelines = Vec::new();

        while !self.is_at_end() {
            match self.current_token()?.kind {
                TokenKind::BlockPackageStart => {
                    package = Some(self.parse_package_block()?);
                }
                TokenKind::BlockEnumStart => {
                    enums.push(self.parse_enum_block()?);
                }
                TokenKind::BlockPipelineStart => {
                    pipelines.push(self.parse_pipeline_block()?);
                }
                _ => {
                    return Err(self.error("Expected block start"));
                }
            }
        }

        Ok(Program {
            package,
            enums,
            pipelines,
            span: self.span(),
        })
    }
}
```

### Pipeline Block Parsing

```rust
fn parse_pipeline_block(&mut self) -> Result<PipelineBlock, ParserError> {
    self.expect(TokenKind::BlockPipelineStart)?;  // {|}

    let name = self.expect_identifier()?;

    let mut trigger = None;
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut statements = Vec::new();

    while !self.check(TokenKind::BlockEnd) {
        if self.check(TokenKind::MarkerTrigger) {
            trigger = Some(self.parse_trigger()?);
        } else if self.check(TokenKind::MarkerPipelineDef) {
            // [|] <input or >output
            if self.peek_ahead(1)?.kind == TokenKind::Input {
                inputs.push(self.parse_input_parameter()?);
            } else if self.peek_ahead(1)?.kind == TokenKind::Output {
                outputs.push(self.parse_output_parameter()?);
            }
        } else {
            statements.push(self.parse_statement()?);
        }
    }

    self.expect(TokenKind::BlockEnd)?;  // {x}

    Ok(PipelineBlock {
        name,
        trigger,
        inputs,
        outputs,
        statements,
        span: self.span(),
    })
}
```

### Statement Parsing

```rust
fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    let marker = self.parse_marker()?;

    match self.current_token()?.kind {
        TokenKind::PrefixVariable => {
            // Variable declaration: [r] $var :type << value
            Ok(Statement::VariableDeclaration(
                self.parse_variable_declaration(marker)?
            ))
        }
        TokenKind::OpPipe => {
            // Pipeline call: [r] |Pipeline
            Ok(Statement::PipelineCall(
                self.parse_pipeline_call(marker)?
            ))
        }
        TokenKind::PipelineComposition => {
            // Pipeline composition: [|] |>
            Ok(Statement::PipelineComposition(
                self.parse_pipeline_composition()?
            ))
        }
        TokenKind::MarkerFork => {
            // Fork statement: [f] $var
            Ok(Statement::Fork(
                self.parse_fork_statement()?
            ))
        }
        TokenKind::MarkerLoop => {
            // Loop statement: [p] ~ForEach.Array
            Ok(Statement::Loop(
                self.parse_loop_statement(marker)?
            ))
        }
        TokenKind::MarkerError => {
            // Error block: [!]
            Ok(Statement::ErrorBlock(
                self.parse_error_block()?
            ))
        }
        TokenKind::MarkerSerialLoad => {
            // Serial load: [s] yaml|json|toml path
            Ok(Statement::SerialLoad(
                self.parse_serial_load()?
            ))
        }
        _ => Err(self.error("Expected statement"))
    }
}
```

### Loop Parsing with Indentation

```rust
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
```

### Pipeline Composition Parsing

```rust
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
```

### Type Parsing with Nested Prefix Validation

```rust
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
```

### Enum Block Parsing

```rust
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
            // [s][!] - Scope-wide error handler for ALL serial loads
            error_handler = Some(self.parse_error_block()?);
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
```

---

### Enum Block Indentation Parsing

**Context:** v0.0.4 introduces indented field mappings under `[s]` serial load blocks within enum definitions. This section documents the complete indentation parsing algorithm.

**Reference:** See `docs/Tech/implementation/parser/canonical-enum-example-v0.0.4.md` for complete working example.

#### Field Mapping Syntax

Field mappings use the `[.]` marker and **must be indented** under their parent `[s]` block:

```polyglot
{#} #App.Config;DatabaseConfig
[A] #AppConfig

[s] |YAML.Load"\\FileDir\\config\\database.yaml"
   [.] .db.host:pg.string << .connection.host
   [.] .db.credentials:pg.serial << .connection.credentials
      [.] .username:pg.string << .username
      [.] .password:pg.string << .password

[s][!] !*
{x}
```

**Indentation Rules:**
1. Field mappings (`[.]`) at 3 spaces under `[s]`
2. Nested subfields at 6 spaces (additional 3 spaces per level)
3. Maximum nesting depth: Unlimited (recursive structure)
4. Consistency required within same level

#### Parser Implementation

**Updated Serial Load Parsing:**

```rust
fn parse_serial_load_in_enum(&mut self) -> Result<SerialLoad, ParserError> {
    // [s] |Format.Load"path"
    self.expect(TokenKind::MarkerSerialLoad)?;

    let format = self.parse_serial_format()?;
    let file_path = self.parse_expression()?;

    // Expect indentation for field mappings
    let mut field_mappings = Vec::new();

    if self.check(TokenKind::Indent) {
        self.advance();

        // Parse all field mappings at this indentation level
        while !self.check(TokenKind::Dedent) && !self.at_end_of_block() {
            field_mappings.push(self.parse_field_mapping()?);
        }

        if self.check(TokenKind::Dedent) {
            self.advance();
        }
    }

    Ok(SerialLoad {
        format,
        file_path,
        field_mappings,
        span: self.span(),
    })
}
```

**Field Mapping Parser:**

```rust
fn parse_field_mapping(&mut self) -> Result<FieldMapping, ParserError> {
    // [.] .field:type << .source.path
    // or
    // [.] .field:type << #EnumType.Value

    self.expect(TokenKind::MarkerSubfield)?;  // [.]

    // Parse dotted field name: .db.host
    let field_name = self.parse_dotted_field_name()?;

    self.expect(TokenKind::Colon)?;
    let field_type = self.parse_type()?;

    self.expect(TokenKind::OpPullFinal)?;  // <<

    // Parse RHS - could be field accessor OR enum constant
    let source = if self.check(TokenKind::Hash) {
        // Enum constant: #DB.RDBMS.Posgresql
        FieldSource::EnumConstant(self.parse_enum_constant()?)
    } else {
        // Field accessor: .connection.host
        FieldSource::FieldAccessor(self.parse_field_accessor()?)
    };

    // Check for nested subfields (additional indentation)
    let mut subfields = Vec::new();
    if self.check(TokenKind::Indent) {
        self.advance();

        while !self.check(TokenKind::Dedent) {
            subfields.push(self.parse_field_mapping()?);  // Recursive
        }

        if self.check(TokenKind::Dedent) {
            self.advance();
        }
    }

    Ok(FieldMapping {
        field_name,
        field_type,
        source,
        subfields,
        span: self.span(),
    })
}
```

**Dotted Field Name Parser:**

```rust
fn parse_dotted_field_name(&mut self) -> Result<DottedFieldName, ParserError> {
    // .db.host.port -> DottedFieldName { parts: ["db", "host", "port"] }
    let mut parts = Vec::new();

    self.expect(TokenKind::Dot)?;
    parts.push(self.expect_identifier()?);

    // Continue parsing dots followed by identifiers
    while self.check(TokenKind::Dot) && self.peek_ahead_is_identifier() {
        self.advance();  // consume dot
        parts.push(self.expect_identifier()?);
    }

    Ok(DottedFieldName { parts, span: self.span() })
}

fn peek_ahead_is_identifier(&self) -> bool {
    // Look ahead to see if next token after dot is an identifier
    // This prevents treating `.source` in `<< .source` as part of field name
    self.peek_ahead(1).map(|t| t.kind == TokenKind::Identifier).unwrap_or(false)
}
```

**Enum Constant Parser:**

```rust
fn parse_enum_constant(&mut self) -> Result<EnumConstant, ParserError> {
    // #DB.RDBMS.Posgresql
    self.expect(TokenKind::Hash)?;

    let enum_type = self.expect_identifier()?;
    self.expect(TokenKind::Dot)?;
    let enum_name = self.expect_identifier()?;
    self.expect(TokenKind::Dot)?;
    let value = self.expect_identifier()?;

    Ok(EnumConstant {
        enum_type,
        enum_name,
        value,
        span: self.span(),
    })
}
```

#### AST Representation

**Updated SerialLoad Node:**

```rust
pub struct SerialLoad {
    pub format: SerialFormat,
    pub file_path: Expression,
    pub field_mappings: Vec<FieldMapping>,  // v0.0.4: Indented field mappings
    pub span: Span,
}

pub enum SerialFormat {
    Yaml,
    Json,
    Toml,
    Xml,
}
```

**New FieldMapping Node:**

```rust
pub struct FieldMapping {
    pub field_name: DottedFieldName,
    pub field_type: TypeAnnotation,
    pub source: FieldSource,
    pub subfields: Vec<FieldMapping>,  // Recursive nesting
    pub span: Span,
}

pub struct DottedFieldName {
    pub parts: Vec<String>,  // ["db", "host"] for .db.host
    pub span: Span,
}

pub enum FieldSource {
    FieldAccessor(FieldAccessor),  // .connection.host
    EnumConstant(EnumConstant),    // #DB.RDBMS.Posgresql
}

pub struct FieldAccessor {
    pub path: Vec<String>,  // ["connection", "host"] for .connection.host
    pub span: Span,
}

pub struct EnumConstant {
    pub enum_type: String,   // "DB"
    pub enum_name: String,   // "RDBMS"
    pub value: String,       // "Posgresql"
    pub span: Span,
}
```

#### Canonical Example

**Complete enum with all features:**

```polyglot
{#} #App.Config;DatabaseConfig
[A] #AppConfig

[s] |YAML.Load"\\FileDir\\config\\database.yaml"
   [.] .db.RDBMS:pg.string << #DB.RDBMS.Posgresql
   [.] .db.host:pg.string << .connection.host
   [.] .db.port:pg.int << .connection.port
   [.] .db.credentials:pg.serial << .connection.credentials
      [.] .username:pg.string << .username
      [.] .password:pg.string << .password
   [.] .db.pool:pg.serial << .connection.pool
      [.] .connections.min:pg.int << .min_connections
      [.] .connections.max:pg.int << .max_connections
      [.] .timeout_seconds:pg.int << .timeout_seconds

[s] |YAML.Load"\\FileDir\\config\\cache.yaml"
   [.] .cache.enabled:pg.bool << .cache.enabled
   [.] .cache.ttl_seconds:pg.int << .cache.ttl

[s][!] !*
{x}
```

**Corresponding YAML (`database.yaml`):**

```yaml
connection:
  host: "localhost"
  port: 5432
  credentials:
    username: "app_user"
    password: "secure_password"
  pool:
    min_connections: 5
    max_connections: 20
    timeout_seconds: 30
```

#### Field Accessor Patterns

**1. Simple Field Accessor:**
```polyglot
[.] .db.host:pg.string << .connection.host
```
Maps YAML path `connection.host` to enum field `.db.host`.

**2. Enum Constant Assignment:**
```polyglot
[.] .db.RDBMS:pg.string << #DB.RDBMS.Posgresql
```
Assigns enum constant value (not from YAML source).

**3. Nested Serial Field:**
```polyglot
[.] .db.credentials:pg.serial << .connection.credentials
   [.] .username:pg.string << .username
   [.] .password:pg.string << .password
```
Maps nested YAML object to nested serial fields.

**4. Dotted Nested Paths:**
```polyglot
[.] .db.pool:pg.serial << .connection.pool
   [.] .connections.min:pg.int << .min_connections
   [.] .connections.max:pg.int << .max_connections
```
Uses dotted notation (`.connections.min`) within nested structure.

#### Critical Implementation Notes

**⚠️ Dotted Field Names:**
- Field names use **dots** (`.db.host`) not underscores (`.db_host`)
- Parser must treat `.db.host` as a single field identifier with multiple parts
- Lexer already handles this via `DottedFieldName` token type

**⚠️ Enum Constants on RHS:**
- RHS of `<<` can be **either** field accessor OR enum constant
- Parser must check for `#` token to distinguish
- Example: `[.] .db.RDBMS:pg.string << #DB.RDBMS.Posgresql`

**⚠️ Recursive Nesting:**
- Field mappings can nest arbitrarily deep
- Each nesting level requires `INDENT` token
- Parser must maintain indentation stack

**⚠️ Indentation vs Scope:**
- Indentation defines **structure**, not just formatting
- `DEDENT` token signals end of nested block
- Missing indentation is a syntax error

#### Comparison with Loop Indentation

Enum indentation parsing follows the **same pattern** as loop indentation (documented lines 532-574):

**Loop Indentation (Reference Pattern):**
```rust
fn parse_loop_statement(&mut self, marker: Marker) -> Result<LoopStatement, ParserError> {
    // [p] ~ForEach.Array
    self.expect(TokenKind::MarkerLoop)?;
    let unpack_operator = self.parse_unpack_call()?;

    // Expect INDENT ← Explicit indentation requirement
    self.expect(TokenKind::Indent)?;

    // Parse loop body
    let mut body = Vec::new();
    while !self.check(TokenKind::Dedent) {
        body.push(self.parse_statement()?);
    }

    // Expect DEDENT ← Explicit dedentation requirement
    if self.check(TokenKind::Dedent) {
        self.advance();
    }
    // ...
}
```

**Enum Field Mapping Indentation (New Pattern):**
```rust
fn parse_serial_load_in_enum(&mut self) -> Result<SerialLoad, ParserError> {
    // [s] |Format.Load"path"
    self.expect(TokenKind::MarkerSerialLoad)?;
    let format = self.parse_serial_format()?;

    // Expect INDENT ← Same explicit requirement
    if self.check(TokenKind::Indent) {
        self.advance();

        // Parse field mappings
        let mut field_mappings = Vec::new();
        while !self.check(TokenKind::Dedent) {
            field_mappings.push(self.parse_field_mapping()?);
        }

        // Expect DEDENT ← Same explicit requirement
        if self.check(TokenKind::Dedent) {
            self.advance();
        }
    }
    // ...
}
```

**Pattern Consistency:** Both use `INDENT`/`DEDENT` tokens to manage scope, maintain indentation stack, and support recursive nesting.

---

## Error Recovery Strategy

### Synchronization Points

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
}
```

### Multiple Error Collection

```rust
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParserError>,  // Collect all errors
}

impl Parser {
    pub fn parse_tolerant(&mut self) -> (Option<Program>, Vec<ParserError>) {
        let program = self.parse().ok();
        (program, self.errors.clone())
    }
}
```

---

## Three-Phase Resolution

### Phase 1: Current File Namespace

```rust
fn resolve_pipeline_reference(&mut self, name: &str) -> Result<PipelineSignature, ParserError> {
    // PHASE 1: Search current file first
    if let Some(pipeline) = self.current_file_pipelines.get(name) {
        return Ok(pipeline.clone());
    }

    // Continue to Phase 2...
    self.resolve_same_package(name)
}
```

### Phase 2: Same Package Files (by `[#]` order)

```rust
fn resolve_same_package(&mut self, name: &str) -> Result<PipelineSignature, ParserError> {
    let current_package = self.get_current_package();

    // Get all files in same package, ordered by [#] markers
    let same_package_files = self.find_same_package_files(current_package)?;

    // Validate [#] ordering (no duplicates)
    self.validate_file_ordering(&same_package_files)?;

    // Search in [#] order
    for file in same_package_files {
        if let Some(pipeline) = file.pipelines.get(name) {
            return Ok(pipeline.clone());
        }
    }

    // Continue to Phase 3...
    self.resolve_external_registry(name)
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
                });
            }
        }
    }

    Ok(())
}
```

### Phase 3: External Registry

```rust
fn resolve_external_registry(&mut self, name: &str) -> Result<PipelineSignature, ParserError> {
    // Query database registry
    let pipeline = self.registry.lookup(name)?;

    Ok(pipeline)
}
```

---

## Testing Strategy

### Unit Tests (Per AST Node)
- Each parse function tested independently
- Valid syntax cases
- Invalid syntax error handling
- Edge cases (empty blocks, nested structures)

### Integration Tests (Complete Pipelines)
- Example pipelines from documentation
- Multi-file compilation with Phase 2 resolution
- Pipeline composition chains
- Loop with nested error blocks
- Enum definitions with serial loads

### Error Recovery Tests
- Multiple syntax errors in single file
- Synchronization to next statement
- Error reporting with line/column numbers

### Type Validation Tests
- Nested type annotations (`pg.array.pg.string`)
- Missing second `pg.` prefix (should error)
- Type compatibility checking

---

## Performance Targets

- **Parsing Speed:** <500ms for 1,000-line files
- **Memory:** <20MB for 10,000-line files
- **Error Recovery:** All errors collected (no early exit)

---

## Implementation Phases

### Phase 1: Basic Structure (Week 1)
- AST type definitions
- Parser skeleton with token stream
- Top-level program parsing (package, enums, pipelines)

### Phase 2: Statement Parsing (Week 2)
- Variable declarations
- Pipeline calls
- Basic expressions

### Phase 3: Advanced Features (Week 3)
- Pipeline composition
- Loop statements with indentation
- Fork statements
- Error blocks

### Phase 4: Enum System (Week 4)
- Enum block parsing
- Serial load statements
- Field accessor syntax
- Scope-wide error handling

### Phase 5: Error Recovery & Testing (Week 5)
- Synchronization points
- Multiple error collection
- Integration tests
- Performance optimization

---

## Dependencies

**Rust Crates:**
- `thiserror` 2.0.17 - Error types
- Lexer output (Token stream)

**Internal Dependencies:**
- `polyglot-lexer` - Token definitions
- `polyglot-db` - Registry lookup (Phase 3 resolution)

---

## References

### Architecture & Specification
- [Lexer Architecture](./lexer-architecture-v0.0.4.md) - Lexer implementation details
- [v0.0.4 EBNF Grammar](../../User/reference/grammar.md) - Formal grammar specification
- [Epic 13 - v0.0.4 Migration](../epics.md#epic-13-v004-syntax-migration) - Migration project overview

### Enum Block Documentation
- [User Guide - Enum Definitions](../../User/language/types/enum-definitions.md) - User-facing enum documentation
- [Canonical Enum Example](../../Tech/implementation/parser/canonical-enum-example-v0.0.4.md) - Complete reference implementation
- [Enum Syntax Migration Guide](../../Tech/implementation/parser/enum-syntax-migration-v0.0.3-to-v0.0.4.md) - v0.0.3 → v0.0.4 syntax changes
- [v0.0.4 Parsing Rules Assessment](../../Tech/implementation/parser/v0.0.4-parsing-rules-assessment.md) - Documentation gap analysis

---

**Status:** Architecture Complete - Ready for Implementation
**Next Step:** Create detailed implementation stories (Story 13.6-13.10)
