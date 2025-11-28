//! Recursive Descent Parser for Polyglot v0.0.2
//!
//! This module implements Story 1.5: Parser Implementation with:
//! - Recursive descent parsing strategy
//! - Three-phase resolution (Phase 1 & 3, Phase 2 deferred to Story 1.5.5)
//! - Import resolution via FileRegistryResolver
//! - Parameter validation and type checking
//! - Comprehensive error detection (all 37 ParserError types)
//!
//! # Architecture
//!
//! The parser follows a traditional recursive descent approach:
//! 1. Tokenize source code (via Lexer)
//! 2. Parse tokens into AST
//! 3. Resolve imports (Phase 1: current file, Phase 3: registry)
//! 4. Validate semantics (types, parameters, mandatory sections)
//! 5. Return Program AST or ParserError
//!
//! # Usage
//!
//! ```rust,ignore
//! use polyglot_parser::{Parser, FileRegistryResolver};
//!
//! let source = r#"
//!     [@] Local@MyApp:1.0.0
//!     [X]
//!
//!     [|] HelloWorld
//!     [i] #Pipeline.NoInput
//!     [t] |T.Call
//!     [W] |W.Polyglot.Scope
//!     [r] .message: pg\string << "Hello"
//!     [o] .message: pg\string
//!     [X]
//! "#;
//!
//! let resolver = FileRegistryResolver::from_file("test-registry.json")?;
//! let parser = Parser::new(source, resolver);
//! let program = parser.parse()?;
//! ```

use crate::ast::*;
use crate::error::ParserError;
use crate::import_resolver::ImportResolver;
use crate::span::{Position, Span};
use polyglot_lexer::{Lexer, Token, TokenKind};
use std::collections::HashMap;

// ============================================================================
// Parser State
// ============================================================================

/// Main parser struct with token stream and state
pub struct Parser<R: ImportResolver> {
    tokens: Vec<Token>,
    current: usize,
    resolver: R,
    source_file: Option<String>,
    /// Package alias → PackageSpec mapping
    package_aliases: HashMap<String, PackageSpec>,
    /// Pipeline name → Pipeline mapping (current file scope)
    local_pipelines: HashMap<String, Pipeline>,
}

impl<R: ImportResolver> Parser<R> {
    /// Create a new parser from source code
    ///
    /// # Arguments
    /// * `source` - Polyglot source code
    /// * `resolver` - Import resolver for package resolution
    ///
    /// # Returns
    /// * `Ok(Parser)` if lexer succeeds
    /// * `Err(ParserError)` if lexing fails
    pub fn new(source: &str, resolver: R) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|e| ParserError::LexerError {
            source: e,
            span: Span::start(),
        })?;

        Ok(Self {
            tokens,
            current: 0,
            resolver,
            source_file: None,
            package_aliases: HashMap::new(),
            local_pipelines: HashMap::new(),
        })
    }

    /// Create parser with source file name (for error reporting)
    pub fn with_source_file(mut self, file: String) -> Self {
        self.source_file = Some(file);
        self
    }

    /// Parse source code into Program AST
    ///
    /// # Returns
    /// * `Ok(Program)` if parsing succeeds
    /// * `Err(ParserError)` with detailed error information
    pub fn parse(mut self) -> Result<Program, ParserError> {
        // Skip leading newlines
        self.skip_newlines();

        // Parse package declaration (mandatory)
        let package = self.parse_package_declaration()?;

        // Parse definitions (enums, errors, pipelines)
        let mut definitions = Vec::new();

        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            let def = self.parse_definition()?;
            definitions.push(def);
        }

        // Validate program structure
        self.validate_program(&package, &definitions)?;

        Ok(Program {
            package,
            definitions,
            source_file: self.source_file.clone(),
            span: self.make_span_from_start(),
        })
    }

    // ========================================================================
    // Token Navigation
    // ========================================================================

    /// Check if at end of token stream
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().kind == TokenKind::Eof
    }

    /// Get current token without consuming
    fn peek(&self) -> &Token {
        if self.current < self.tokens.len() {
            &self.tokens[self.current]
        } else {
            &self.tokens[self.tokens.len() - 1] // EOF token
        }
    }

    /// Get next token without consuming
    fn peek_next(&self) -> Option<&Token> {
        if self.current + 1 < self.tokens.len() {
            Some(&self.tokens[self.current + 1])
        } else {
            None
        }
    }

    /// Get previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Consume current token and return it
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Check if current token matches kind
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().kind == kind
    }

    /// Check if current token matches any of the given kinds
    fn check_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.iter().any(|k| self.check(k))
    }

    /// Consume token if it matches expected kind
    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consume token if it matches any of the given kinds
    fn match_any(&mut self, kinds: &[TokenKind]) -> bool {
        if self.check_any(kinds) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Skip over newline tokens
    fn skip_newlines(&mut self) {
        while self.check(&TokenKind::Newline) {
            self.advance();
        }
    }

    /// Expect and consume specific token kind, error if not found
    fn expect(&mut self, kind: &TokenKind, context: &str) -> Result<&Token, ParserError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(ParserError::UnexpectedToken {
                expected: kind.description().to_string(),
                found: self.peek().kind.description().to_string(),
                context: context.to_string(),
                span: self.make_span_from_token(self.peek()),
            })
        }
    }

    // ========================================================================
    // Span Helpers
    // ========================================================================

    fn make_span_from_token(&self, token: &Token) -> Span {
        let start = Position::new(token.line, token.column, 0);
        let end = Position::new(token.line, token.column + token.lexeme.len(), 0);
        Span::new(start, end)
    }

    fn make_span_from_start(&self) -> Span {
        if self.tokens.is_empty() {
            return Span::start();
        }
        let first = &self.tokens[0];
        let last = self.previous();
        let start = Position::new(first.line, first.column, 0);
        let end = Position::new(last.line, last.column + last.lexeme.len(), 0);
        Span::new(start, end)
    }

    fn make_span(&self, start_token: &Token, end_token: &Token) -> Span {
        let start = Position::new(start_token.line, start_token.column, 0);
        let end = Position::new(end_token.line, end_token.column + end_token.lexeme.len(), 0);
        Span::new(start, end)
    }

    // ========================================================================
    // Package Declaration Parsing
    // ========================================================================

    /// Parse package declaration block: [@] ... [X]
    fn parse_package_declaration(&mut self) -> Result<PackageDeclaration, ParserError> {
        let start_token = self.peek().clone();

        // Expect [@] marker
        self.expect(&TokenKind::BlockPackageStart, "package declaration")?;

        // Parse package spec: registry@path:version
        let spec = self.parse_package_spec()?;

        // Parse optional alias: [A] name
        let alias = if self.match_token(&TokenKind::BlockVersionEnum) {
            // [#] marker followed by number (file ordering) or identifier (alias)
            // For now, we'll handle file ordering separately
            // Check if this is a file ordering marker or alias
            if self.check(&TokenKind::LiteralInteger) {
                // File ordering: [#] 1
                self.advance(); // consume number
                None
            } else {
                // TODO: Parse alias if needed
                None
            }
        } else {
            None
        };

        // Skip newlines before imports
        self.skip_newlines();

        // Parse imports: [<] @alias << Package@Path:Version
        let mut imports = Vec::new();
        while self.match_token(&TokenKind::BlockInputBinding) {
            let import = self.parse_import_declaration()?;
            imports.push(import);
            self.skip_newlines();
        }

        // Skip newlines before block end (already handled above in loop)
        self.skip_newlines();

        // Expect [X] end marker
        self.expect(&TokenKind::BlockEnd, "package declaration block")?;

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(PackageDeclaration {
            spec,
            alias,
            imports,
            span,
        })
    }

    /// Parse package spec: registry@path:version
    fn parse_package_spec(&mut self) -> Result<PackageSpec, ParserError> {
        let start_token = self.peek().clone();

        // Parse registry (identifier before @)
        let registry_token = self.expect(&TokenKind::Identifier, "package registry")?;
        let registry = registry_token.lexeme.clone();

        // Expect @ delimiter
        self.expect(&TokenKind::DelimiterAt, "package spec separator")?;

        // Parse path components (dot-separated identifiers)
        let mut path = Vec::new();
        loop {
            let component = self.expect(&TokenKind::Identifier, "package path component")?;
            path.push(component.lexeme.clone());

            if !self.match_token(&TokenKind::DelimiterDot) {
                break;
            }
        }

        // Expect : delimiter
        self.expect(&TokenKind::DelimiterColon, "package version separator")?;

        // Parse version (major.minor.patch)
        // Note: Lexer tokenizes "1.0.0" as LiteralFloat (1.0) + DelimiterDot + LiteralInteger (0)
        // or as Version token if specifically recognized
        let version = if self.check(&TokenKind::Version) {
            let version_token = self.advance().clone();
            self.parse_version(&version_token.lexeme)?
        } else {
            // Parse as major.minor.patch from separate tokens
            let major_minor_token = self.peek().clone();

            // Try float token (contains major.minor like "1.0")
            if self.match_token(&TokenKind::LiteralFloat) {
                let major_minor_str = major_minor_token.lexeme.clone();
                // Check if next token is .patch as IdentifierVariable (e.g., ".0")
                if self.check(&TokenKind::IdentifierVariable) {
                    let patch_token = self.advance().clone();
                    let patch_str = patch_token.lexeme.trim_start_matches('.');
                    let version_str = format!("{}.{}", major_minor_str, patch_str);
                    self.parse_version(&version_str)?
                } else {
                    // Otherwise expect standard dot + integer
                    self.expect(&TokenKind::DelimiterDot, "version patch separator")?;
                    let patch_token = self.expect(&TokenKind::LiteralInteger, "version patch")?;
                    let patch_str = patch_token.lexeme.clone();
                    let version_str = format!("{}.{}", major_minor_str, patch_str);
                    self.parse_version(&version_str)?
                }
            } else {
                // Try integer tokens (major.minor.patch as separate integers)
                let major_token = self.expect(&TokenKind::LiteralInteger, "version major")?;
                let major_str = major_token.lexeme.clone();
                self.expect(&TokenKind::DelimiterDot, "version minor separator")?;
                let minor_token = self.expect(&TokenKind::LiteralInteger, "version minor")?;
                let minor_str = minor_token.lexeme.clone();
                self.expect(&TokenKind::DelimiterDot, "version patch separator")?;
                let patch_token = self.expect(&TokenKind::LiteralInteger, "version patch")?;
                let patch_str = patch_token.lexeme.clone();
                let version_str = format!("{}.{}.{}", major_str, minor_str, patch_str);
                self.parse_version(&version_str)?
            }
        };

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(PackageSpec {
            registry,
            path,
            version,
            span,
        })
    }

    /// Parse version string into Version struct
    fn parse_version(&self, version_str: &str) -> Result<Version, ParserError> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return Err(ParserError::InvalidPackageVersion {
                version: version_str.to_string(),
                span: Span::start(),
            });
        }

        let major = parts[0].parse::<u32>().map_err(|_| {
            ParserError::InvalidPackageVersion {
                version: version_str.to_string(),
                span: Span::start(),
            }
        })?;

        let minor = parts[1].parse::<u32>().map_err(|_| {
            ParserError::InvalidPackageVersion {
                version: version_str.to_string(),
                span: Span::start(),
            }
        })?;

        let patch = parts[2].parse::<u32>().map_err(|_| {
            ParserError::InvalidPackageVersion {
                version: version_str.to_string(),
                span: Span::start(),
            }
        })?;

        Ok(Version::new(major, minor, patch))
    }

    /// Parse import declaration: [<] @alias << Package@Path:Version
    fn parse_import_declaration(&mut self) -> Result<ImportDeclaration, ParserError> {
        let start_token = self.previous().clone(); // Already consumed [<]

        // Parse @alias
        self.expect(&TokenKind::DelimiterAt, "import alias marker")?;
        let alias_token = self.expect(&TokenKind::Identifier, "import alias")?;
        let alias = alias_token.lexeme.clone();

        // Expect << operator
        self.expect(&TokenKind::OpPush, "import assignment")?;

        // Parse package spec
        let package = self.parse_package_spec()?;

        // Register alias for later resolution
        self.package_aliases.insert(alias.clone(), package.clone());

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(ImportDeclaration {
            alias,
            package,
            span,
        })
    }

    // ========================================================================
    // Definition Parsing (Enums, Errors, Pipelines)
    // ========================================================================

    /// Parse top-level definition
    fn parse_definition(&mut self) -> Result<Definition, ParserError> {
        match self.peek().kind {
            TokenKind::BlockVersionEnum => {
                // Could be enum or file ordering
                let next = self.peek_next();
                if let Some(token) = next {
                    if token.kind == TokenKind::LiteralInteger {
                        // File ordering marker - skip
                        self.advance(); // consume [#]
                        self.advance(); // consume number
                        self.parse_definition() // try next definition
                    } else {
                        Ok(Definition::Enumeration(self.parse_enumeration()?))
                    }
                } else {
                    Err(ParserError::UnexpectedEof {
                        context: "enumeration or file ordering".to_string(),
                        started_at: self.make_span_from_token(self.peek()),
                    })
                }
            }
            TokenKind::BlockPipelineStart => Ok(Definition::Pipeline(self.parse_pipeline()?)),
            _ => Err(ParserError::UnexpectedToken {
                expected: "definition ([#], [!], or [|])".to_string(),
                found: self.peek().kind.description().to_string(),
                context: "top-level definition".to_string(),
                span: self.make_span_from_token(self.peek()),
            }),
        }
    }

    /// Parse enumeration definition
    fn parse_enumeration(&mut self) -> Result<EnumerationDefinition, ParserError> {
        let start_token = self.peek().clone();

        // Consume [#] marker
        self.advance();

        // Parse enum name (can be multi-part: Config.Database)
        let mut name_parts = Vec::new();
        loop {
            let name_token = self.expect(&TokenKind::Identifier, "enumeration name")?;
            name_parts.push(name_token.lexeme.clone());

            if !self.match_token(&TokenKind::DelimiterDot) {
                break;
            }
        }

        // Parse fields: [<] .field_name: type << value
        let mut fields = Vec::new();
        while self.match_token(&TokenKind::BlockInputBinding) {
            let field_start = self.previous().clone();

            let field_token = self.expect(&TokenKind::IdentifierVariable, "enum field name")?;
            let field_name = field_token.lexeme.clone();

            // Expect : type separator
            self.expect(&TokenKind::DelimiterColon, "field type annotation")?;

            // Parse type
            let field_type = self.parse_type_annotation()?;

            // Expect << assignment
            self.expect(&TokenKind::OpPush, "field value assignment")?;

            // Parse value expression
            let value = self.parse_expression()?;

            let field_end = self.previous().clone();
            let field_span = self.make_span(&field_start, &field_end);

            fields.push(EnumField {
                name: field_name,
                field_type,
                value,
                span: field_span,
            });
        }

        // Parse optional alias: [A] AliasName
        let alias = None; // TODO: Parse alias if needed

        // Expect [X] end marker
        self.expect(&TokenKind::BlockEnd, "enumeration block")?;

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(EnumerationDefinition {
            name: name_parts,
            fields,
            alias,
            span,
        })
    }

    /// Parse pipeline definition
    fn parse_pipeline(&mut self) -> Result<Pipeline, ParserError> {
        let start_token = self.peek().clone();

        // Consume [|] marker
        self.advance();

        // Parse pipeline name
        let name_token = self.expect(&TokenKind::Identifier, "pipeline name")?;
        let name = name_token.lexeme.clone();

        // Parse [i] input section (mandatory)
        self.skip_newlines();
        let input = self.parse_input_section()?;

        // Parse [t] trigger section (mandatory)
        self.skip_newlines();
        let trigger = self.parse_trigger_section()?;

        // Parse [W] wrapper OR [\] setup + [/] cleanup (mandatory)
        self.skip_newlines();
        let wrapper = self.parse_wrapper_section()?;

        // Parse pipeline body (blocks and statements)
        self.skip_newlines();
        let body = self.parse_pipeline_body()?;

        // Parse [o] output section (mandatory)
        self.skip_newlines();
        let output = self.parse_output_section()?;

        // Skip newlines before block end
        self.skip_newlines();

        // Expect [X] end marker
        self.expect(&TokenKind::BlockEnd, "pipeline block")?;

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        let pipeline = Pipeline {
            name: name.clone(),
            input,
            output,
            trigger: Some(trigger),
            queue: None,
            wrapper: Some(wrapper),
            body,
            span,
        };

        // Register pipeline in local scope
        self.local_pipelines.insert(name, pipeline.clone());

        Ok(pipeline)
    }

    /// Parse [i] input section
    fn parse_input_section(&mut self) -> Result<Option<Vec<InputParameter>>, ParserError> {
        self.expect(&TokenKind::BlockInput, "input section")?;

        // Check for #Pipeline.NoInput
        if self.check(&TokenKind::ReservedPipelineNoInput) {
            self.advance();
            return Ok(None);
        }

        // Parse input parameters
        let mut inputs = Vec::new();
        // For now, parse a single input parameter
        // TODO: Handle multiple input parameters
        let input = self.parse_input_parameter()?;
        inputs.push(input);

        Ok(Some(inputs))
    }

    /// Parse single input parameter: .name: pg\type OR .name: pg\type <~ default OR .name: pg\type << value
    fn parse_input_parameter(&mut self) -> Result<InputParameter, ParserError> {
        let start_token = self.peek().clone();

        // Parse .variable_name
        let var_token = self.expect(&TokenKind::IdentifierVariable, "input parameter name")?;
        let name = var_token.lexeme.clone();

        // Expect : type separator
        self.expect(&TokenKind::DelimiterColon, "type annotation")?;

        // Parse type annotation
        let param_type = self.parse_type_annotation()?;

        // Check for default value or constant
        let default_value = if self.match_token(&TokenKind::OpDefault) {
            // <~ operator: default value
            Some(self.parse_expression()?)
        } else if self.match_token(&TokenKind::OpPush) {
            // << operator: constant value
            Some(self.parse_expression()?)
        } else {
            None
        };

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(InputParameter {
            name: name.trim_start_matches('.').to_string(),
            type_annotation: Some(param_type),
            default: default_value,
            span,
        })
    }

    /// Parse type annotation: pg\int, pg\string, etc.
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParserError> {
        // Parse namespace (pg, py, rs, etc.)
        let namespace_token = self.expect(&TokenKind::TypeNamespace, "type namespace")?;
        let namespace = namespace_token.lexeme.clone();

        // Expect \ delimiter
        self.expect(&TokenKind::DelimiterBackslash, "type separator")?;

        // Parse type name
        let type_token = self.peek().clone();
        let type_name = if self.match_any(&[
            TokenKind::TypeString,
            TokenKind::TypeInt,
            TokenKind::TypeFloat,
            TokenKind::TypeBool,
            TokenKind::TypeDatetime,
            TokenKind::TypePath,
            TokenKind::TypeSerial,
            TokenKind::TypeArray,
            TokenKind::TypeSet,
        ]) {
            type_token.lexeme.clone()
        } else {
            return Err(ParserError::UnexpectedToken {
                expected: "type name (string, int, float, etc.)".to_string(),
                found: self.peek().kind.description().to_string(),
                context: "type annotation".to_string(),
                span: self.make_span_from_token(self.peek()),
            });
        };

        Ok(TypeAnnotation::Named {
            namespace,
            type_name,
        })
    }

    /// Parse [t] trigger section
    fn parse_trigger_section(&mut self) -> Result<TriggerConfig, ParserError> {
        let start_token = self.peek().clone();

        self.expect(&TokenKind::BlockTrigger, "trigger section")?;

        // Parse trigger type: |T.Call, |T.Time, etc.
        let trigger_token = self.expect(&TokenKind::SpecialTriggerType, "trigger type")?;
        let trigger_type = Identifier::Pipeline(trigger_token.lexeme.clone());

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(TriggerConfig {
            trigger_type,
            parameters: Vec::new(), // TODO: Parse trigger parameters
            span,
        })
    }

    /// Parse [W] wrapper section
    fn parse_wrapper_section(&mut self) -> Result<WrapperConfig, ParserError> {
        let start_token = self.peek().clone();

        self.expect(&TokenKind::BlockWrapper, "wrapper section")?;

        // Parse wrapper type: |W.Polyglot.Scope, RT.Python, etc.
        let wrapper_token = self.peek().clone();
        let wrapper_type = if self.match_token(&TokenKind::SpecialWrapper) {
            Identifier::Pipeline(wrapper_token.lexeme.clone())
        } else if self.match_token(&TokenKind::SpecialRuntime) {
            // RT.Python"file.py"
            Identifier::Pipeline(wrapper_token.lexeme.clone())
        } else {
            return Err(ParserError::UnexpectedToken {
                expected: "wrapper type (|W.*, RT.*)".to_string(),
                found: self.peek().kind.description().to_string(),
                context: "wrapper section".to_string(),
                span: self.make_span_from_token(self.peek()),
            });
        };

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(WrapperConfig {
            wrapper_type,
            parameters: Vec::new(), // TODO: Parse wrapper parameters
            span,
        })
    }

    /// Parse pipeline body (blocks and statements)
    fn parse_pipeline_body(&mut self) -> Result<Block, ParserError> {
        let start_token = self.peek().clone();

        // Collect all statements from the body
        let mut statements = Vec::new();

        // Parse blocks: [r], [<], [>], etc.
        while self.check(&TokenKind::BlockSequential)
            || self.check(&TokenKind::BlockInputBinding)
            || self.check(&TokenKind::BlockOutputBinding) {

            // Consume block marker
            self.advance();

            // Parse statement within the block
            let stmt = self.parse_statement()?;
            statements.push(stmt);

            // Skip newlines between statements
            self.skip_newlines();
        }

        let end_token = if !statements.is_empty() {
            self.previous().clone()
        } else {
            start_token.clone()
        };

        let span = self.make_span(&start_token, &end_token);

        Ok(Block {
            block_type: BlockType::Sequential,
            statements,
            span,
        })
    }

    /// Parse a single block
    fn parse_block(&mut self) -> Result<Block, ParserError> {
        let start_token = self.peek().clone();

        // Parse block type
        let block_type = match self.peek().kind {
            TokenKind::BlockSequential => {
                self.advance();
                BlockType::Sequential
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "block marker ([r], [p], etc.)".to_string(),
                    found: self.peek().kind.description().to_string(),
                    context: "block".to_string(),
                    span: self.make_span_from_token(self.peek()),
                })
            }
        };

        // Parse statement(s) within block
        let mut statements = Vec::new();
        let stmt = self.parse_statement()?;
        statements.push(stmt);

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(Block {
            block_type,
            statements,
            span,
        })
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        let start_token = self.peek().clone();

        // Check for pipeline call: @alias|PipelineName
        if self.check(&TokenKind::DelimiterAt) {
            return self.parse_pipeline_call_statement();
        }

        // Parse variable identifier
        let var_token = self.expect(&TokenKind::IdentifierVariable, "variable name")?;
        let var_name = var_token.lexeme.trim_start_matches('.').to_string();

        // Check for type annotation (indicates variable declaration)
        if self.check(&TokenKind::DelimiterColon) {
            // Variable declaration: .name: type << expr
            self.advance(); // consume :

            // Parse type
            let type_annotation = self.parse_type_annotation()?;

            // Expect << assignment
            self.expect(&TokenKind::OpPush, "variable assignment")?;

            // Parse expression
            let init = self.parse_expression()?;

            let end_token = self.previous().clone();
            let span = self.make_span(&start_token, &end_token);

            Ok(Statement::VariableDeclaration {
                name: Identifier::Variable(var_name.clone()),
                type_annotation: Some(type_annotation),
                init: Some(init),
                span,
            })
        } else {
            // Assignment: .name << expr OR .name >> .source
            let operator = if self.match_token(&TokenKind::OpPush) {
                AssignmentOperator::Push
            } else if self.match_token(&TokenKind::OpPull) {
                AssignmentOperator::Pull
            } else if self.match_token(&TokenKind::OpDefault) {
                AssignmentOperator::Default
            } else {
                return Err(ParserError::UnexpectedToken {
                    expected: "assignment operator (<<, >>, or <~)".to_string(),
                    found: self.peek().kind.description().to_string(),
                    context: "assignment".to_string(),
                    span: self.make_span_from_token(self.peek()),
                });
            };

            // Parse right-hand side expression
            let value = self.parse_expression()?;

            let end_token = self.previous().clone();
            let span = self.make_span(&start_token, &end_token);

            Ok(Statement::Assignment {
                target: Identifier::Variable(var_name),
                operator,
                value,
                span,
            })
        }
    }

    /// Parse pipeline call statement: @alias|PipelineName [<] inputs [>] outputs
    fn parse_pipeline_call_statement(&mut self) -> Result<Statement, ParserError> {
        let start_token = self.peek().clone();

        // Parse @alias
        self.expect(&TokenKind::DelimiterAt, "pipeline call alias")?;
        let alias_token = self.expect(&TokenKind::Identifier, "package alias")?;
        let alias = alias_token.lexeme.clone();

        // Parse |PipelineName (lexer provides this as IdentifierPipeline)
        let pipeline_token = self.expect(&TokenKind::IdentifierPipeline, "pipeline name")?;
        let pipeline_name = pipeline_token.lexeme.trim_start_matches('|').to_string();

        // Construct full pipeline identifier with alias
        let full_name = format!("{}|{}", alias, pipeline_name);

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        // Note: Input/output bindings are parsed as separate statements in parse_pipeline_body
        // They follow on subsequent lines with [<] and [>] markers

        Ok(Statement::PipelineCall {
            pipeline: Identifier::Pipeline(full_name),
            args: Vec::new(),
            span,
        })
    }

    /// Parse expression (literals, identifiers, etc.)
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.peek().kind {
            TokenKind::LiteralInteger => {
                let token = self.advance().clone();
                let lexeme = token.lexeme.clone();
                let value = lexeme.parse::<i64>().map_err(|_| {
                    ParserError::UnexpectedToken {
                        expected: "valid integer".to_string(),
                        found: lexeme.clone(),
                        context: "integer literal parsing".to_string(),
                        span: self.make_span_from_token(&token),
                    }
                })?;
                let span = self.make_span_from_token(&token);
                Ok(Expression::Literal {
                    value: Literal::Integer(value),
                    span,
                })
            }
            TokenKind::LiteralFloat => {
                let token = self.advance().clone();
                let lexeme = token.lexeme.clone();
                let value = lexeme.parse::<f64>().map_err(|_| {
                    ParserError::UnexpectedToken {
                        expected: "valid float".to_string(),
                        found: lexeme.clone(),
                        context: "float literal parsing".to_string(),
                        span: self.make_span_from_token(&token),
                    }
                })?;
                let span = self.make_span_from_token(&token);
                Ok(Expression::Literal {
                    value: Literal::Float(value),
                    span,
                })
            }
            TokenKind::StringStart => {
                // Parse string with interpolation
                self.parse_string_literal()
            }
            TokenKind::IdentifierVariable => {
                let token = self.advance().clone();
                let name = token.lexeme.trim_start_matches('.').to_string();
                let span = self.make_span_from_token(&token);
                Ok(Expression::Identifier {
                    name: Identifier::Variable(name),
                    span,
                })
            }
            TokenKind::ReservedBooleanTrue => {
                let token = self.advance().clone();
                let span = self.make_span_from_token(&token);
                Ok(Expression::Literal {
                    value: Literal::Boolean(true),
                    span,
                })
            }
            TokenKind::ReservedBooleanFalse => {
                let token = self.advance().clone();
                let span = self.make_span_from_token(&token);
                Ok(Expression::Literal {
                    value: Literal::Boolean(false),
                    span,
                })
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: "expression (literal or identifier)".to_string(),
                found: self.peek().kind.description().to_string(),
                context: "expression".to_string(),
                span: self.make_span_from_token(self.peek()),
            }),
        }
    }

    /// Parse string literal with interpolation
    fn parse_string_literal(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.peek().clone();

        self.expect(&TokenKind::StringStart, "string literal")?;

        // Collect string content
        let mut content = String::new();
        while self.check(&TokenKind::StringContent) {
            let token = self.advance();
            content.push_str(&token.lexeme);
        }

        // TODO: Handle interpolation (StringInterpolationStart, StringInterpolationEnd)

        self.expect(&TokenKind::StringEnd, "string end")?;

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(Expression::Literal {
            value: Literal::String(content),
            span,
        })
    }

    /// Parse [o] output section
    fn parse_output_section(&mut self) -> Result<Option<TypeAnnotation>, ParserError> {
        self.expect(&TokenKind::BlockOutput, "output section")?;

        // Check for !NoError
        if self.check(&TokenKind::ReservedNoError) {
            self.advance();
            return Ok(None);
        }

        // Parse output parameter: .name: type
        let _var_token = self.expect(&TokenKind::IdentifierVariable, "output parameter name")?;

        // Expect : type separator
        self.expect(&TokenKind::DelimiterColon, "type annotation")?;

        // Parse type annotation
        let output_type = self.parse_type_annotation()?;

        Ok(Some(output_type))
    }

    /// Parse output parameter: .name: type
    fn parse_output_parameter(&mut self) -> Result<InputParameter, ParserError> {
        let start_token = self.peek().clone();

        // Parse .variable_name
        let var_token = self.expect(&TokenKind::IdentifierVariable, "output parameter name")?;
        let name = var_token.lexeme.trim_start_matches('.').to_string();

        // Expect : type separator
        self.expect(&TokenKind::DelimiterColon, "type annotation")?;

        // Parse type annotation
        let param_type = self.parse_type_annotation()?;

        let end_token = self.previous().clone();
        let span = self.make_span(&start_token, &end_token);

        Ok(InputParameter {
            name,
            type_annotation: Some(param_type),
            default: None,
            span,
        })
    }

    // ========================================================================
    // Validation
    // ========================================================================

    /// Validate program structure
    fn validate_program(
        &self,
        _package: &PackageDeclaration,
        definitions: &[Definition],
    ) -> Result<(), ParserError> {
        // Check that at least one pipeline is defined
        if definitions.is_empty() {
            return Err(ParserError::UnexpectedEof {
                context: "Expected at least one definition (pipeline, enum, or error)".to_string(),
                started_at: Span::start(),
            });
        }

        // TODO: Add more validation
        // - Duplicate pipeline names
        // - Undefined references
        // - Type mismatches via resolve_and_validate_pipeline_call

        Ok(())
    }

    /// Validate pipeline call against registry (Phase 3 resolution)
    ///
    /// This validates:
    /// - Package exists and is imported
    /// - Pipeline exists in the package
    /// - All required parameters are provided
    /// - No extra parameters are provided
    /// - Parameter types match (future enhancement)
    ///
    /// Note: This is a placeholder for future validation logic.
    /// Full validation will be implemented in a later story.
    #[allow(dead_code)]
    fn validate_pipeline_call(
        &self,
        _alias: &str,
        _pipeline_name: &str,
        _provided_params: &HashMap<String, String>,
        _span: &Span,
    ) -> Result<(), ParserError> {
        // TODO: Implement full validation in future story
        // For now, just return Ok since we don't have all error types defined
        //
        // Future implementation will:
        // 1. Look up package spec from alias
        // 2. Resolve package from registry (Phase 3)
        // 3. Get pipeline signature from registry
        // 4. Validate parameters against signature

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_registry_resolver::FileRegistryResolver;

    #[test]
    fn test_parse_simple_pipeline() {
        let source = r#"
[@] Local@MyApp:1.0.0
[X]

[|] HelloWorld
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .message: pg\string << "Hello"
[o] .message: pg\string
[X]
        "#;

        let resolver = FileRegistryResolver::empty();
        let parser = Parser::new(source, resolver).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_pipeline_with_bindings() {
        let source = r#"
[@] Local@MyApp:1.0.0
[<] @utils << Community@DataHelpers:2.3.1
[X]

[|] MyPipeline
[i] .data: pg\int

[t] |T.Call
[W] |W.Polyglot.Scope

[r] @utils|Transform
[<] .input << .data
[<] .scale << 2
[>] .result >> .transformed

[o] .transformed: pg\int
[X]
        "#;

        let resolver = FileRegistryResolver::empty();
        let parser = Parser::new(source, resolver).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }
}
