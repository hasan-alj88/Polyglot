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
use std::path::PathBuf;

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

    // Multi-file compilation support (Story 1.5.5)
    /// Cache of parsed files (PathBuf → Program) to avoid re-parsing
    file_cache: HashMap<PathBuf, Program>,
    /// Current package specification for same-package identification
    current_package: Option<PackageSpec>,
    /// File ordering ([#] marker values): PathBuf → Optional<order_number>
    /// None means no [#] marker (processed after numbered files)
    file_ordering: HashMap<PathBuf, Option<usize>>,
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
            // Initialize multi-file fields (Story 1.5.5)
            file_cache: HashMap::new(),
            current_package: None,
            file_ordering: HashMap::new(),
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

        // Parse optional [#] file ordering marker (Story 1.5.5)
        let file_order = self.parse_file_ordering_marker()?;

        // Store file ordering in parser state if present
        if let Some(order_num) = file_order {
            if let Some(ref file_path) = self.source_file {
                let path = PathBuf::from(file_path);
                self.file_ordering.insert(path, Some(order_num));
            }
        } else if let Some(ref file_path) = self.source_file {
            // No [#] marker - file is processed after numbered files
            let path = PathBuf::from(file_path);
            self.file_ordering.insert(path, None);
        }

        self.skip_newlines();

        // Parse package declaration (mandatory)
        let package = self.parse_package_declaration()?;

        // Store current package for Phase 2 resolution (Story 1.5.5)
        self.current_package = Some(package.spec.clone());

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
    // Multi-File Compilation Support (Story 1.5.5)
    // ========================================================================

    /// Discover all .pg files in the given directory that belong to the same package
    ///
    /// This is a placeholder for multi-file compilation. Full implementation will be
    /// done when we have a higher-level compilation context that can manage multiple
    /// parser instances.
    ///
    /// # Arguments
    /// * `base_dir` - Directory to search for .pg files
    /// * `current_package` - Package spec to match against
    ///
    /// # Returns
    /// * `Ok(Vec<PathBuf>)` - List of file paths with matching package declarations
    /// * `Err(ParserError)` - If file access or parsing fails
    #[allow(dead_code)]
    fn discover_same_package_files(
        &self,
        base_dir: &std::path::Path,
        current_package: &PackageSpec,
    ) -> Result<Vec<PathBuf>, ParserError> {
        use std::fs;

        let mut same_package_files = Vec::new();

        // Read directory entries
        let entries = fs::read_dir(base_dir).map_err(|e| ParserError::UnexpectedEof {
            context: format!("Failed to read directory: {}", e),
            started_at: Span::start(),
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| ParserError::UnexpectedEof {
                context: format!("Failed to read directory entry: {}", e),
                started_at: Span::start(),
            })?;

            let path = entry.path();

            // Skip non-.pg files
            if !path.extension().map_or(false, |ext| ext == "pg") {
                continue;
            }

            // Skip the current file if source_file is set
            if let Some(ref current_file) = self.source_file {
                if path.ends_with(current_file) {
                    continue;
                }
            }

            // Try to extract package spec from file without full parsing
            let source = fs::read_to_string(&path).map_err(|e| ParserError::UnexpectedEof {
                context: format!("Failed to read file {:?}: {}", path, e),
                started_at: Span::start(),
            })?;

            // Quick check: extract package declaration by parsing just the header
            if let Ok(file_package_spec) = Self::extract_package_spec(&source) {
                // Check if package specs match
                if Self::package_specs_match(&file_package_spec, current_package) {
                    same_package_files.push(path);
                }
            }
        }

        Ok(same_package_files)
    }

    /// Extract package spec from source without full parsing
    ///
    /// This is a lightweight operation that only parses the package declaration
    fn extract_package_spec(source: &str) -> Result<PackageSpec, ParserError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|e| ParserError::LexerError {
            source: e,
            span: Span::start(),
        })?;

        // Create a minimal parser just for package extraction
        let mut parser = Parser {
            tokens,
            current: 0,
            resolver: crate::import_resolver::StubImportResolver::new(),
            source_file: None,
            package_aliases: HashMap::new(),
            local_pipelines: HashMap::new(),
            file_cache: HashMap::new(),
            current_package: None,
            file_ordering: HashMap::new(),
        };

        parser.skip_newlines();
        let package = parser.parse_package_declaration()?;
        Ok(package.spec)
    }

    /// Check if two PackageSpecs represent the same package
    ///
    /// Packages match if registry, path, and version are identical
    fn package_specs_match(spec1: &PackageSpec, spec2: &PackageSpec) -> bool {
        // Compare registry names (String)
        let registry_match = spec1.registry == spec2.registry;

        // Compare paths (Vec<String>)
        let path_match = spec1.path == spec2.path;

        // Compare versions
        let version_match = spec1.version.major == spec2.version.major
            && spec1.version.minor == spec2.version.minor
            && spec1.version.patch == spec2.version.patch;

        registry_match && path_match && version_match
    }

    /// Parse optional [#] file ordering marker
    ///
    /// Syntax: `[#] N` where N is an integer
    /// If present, returns Some(N), otherwise returns None
    ///
    /// # Returns
    /// * `Ok(Some(usize))` - File ordering number if [#] marker present
    /// * `Ok(None)` - No [#] marker found
    /// * `Err(ParserError)` - Invalid [#] marker syntax
    fn parse_file_ordering_marker(&mut self) -> Result<Option<usize>, ParserError> {
        // Check if next token is [#] (BlockVersionEnum)
        if !self.check(&TokenKind::BlockVersionEnum) {
            return Ok(None);
        }

        // Consume [#] marker
        let _marker_token = self.advance();

        // Expect an integer literal for the ordering number
        if !self.check(&TokenKind::LiteralInteger) {
            let error_token = self.peek();
            let error_span = Span::new(
                Position::new(error_token.line, error_token.column, 0),
                Position::new(error_token.line, error_token.column + error_token.lexeme.len(), 0),
            );
            return Err(ParserError::UnexpectedToken {
                expected: "integer literal".to_string(),
                found: format!("{:?}", error_token.kind),
                context: "parsing file ordering marker [#]".to_string(),
                span: error_span,
            });
        }

        let order_token = self.advance();
        let order_num = order_token.lexeme.parse::<usize>().map_err(|_| {
            let span = Span::new(
                Position::new(order_token.line, order_token.column, 0),
                Position::new(order_token.line, order_token.column + order_token.lexeme.len(), 0),
            );
            ParserError::UnexpectedToken {
                expected: "valid integer".to_string(),
                found: order_token.lexeme.clone(),
                context: "parsing file ordering number after [#]".to_string(),
                span,
            }
        })?;

        // Store in parser state and return
        Ok(Some(order_num))
    }

    // ========================================================================
    // Pipeline Resolution (Story 1.5.5 - Task 2)
    // ========================================================================

    /// Resolve a pipeline reference using three-phase resolution strategy
    ///
    /// Resolution phases:
    /// 1. **Phase 1**: Current file namespace (local_pipelines)
    /// 2. **Phase 2**: Same package, different files (by [#] order)
    /// 3. **Phase 3**: External packages (via ImportResolver/registry)
    ///
    /// # Arguments
    /// * `alias` - Package alias (e.g., "utils" from "@utils|Transform")
    /// * `pipeline_name` - Pipeline name (e.g., "Transform")
    ///
    /// # Returns
    /// * `Ok(true)` - Pipeline found and valid
    /// * `Ok(false)` - Pipeline not found
    /// * `Err(ParserError)` - Resolution error
    ///
    /// # Example
    /// ```ignore
    /// // Resolves @utils|Transform
    /// parser.resolve_pipeline_reference("utils", "Transform")?;
    /// ```
    #[allow(dead_code)]
    fn resolve_pipeline_reference(
        &mut self,
        alias: &str,
        pipeline_name: &str,
    ) -> Result<bool, ParserError> {
        // PHASE 1: Current file namespace
        // Check if pipeline is defined locally (no alias means current file)
        if alias == "self" || alias.is_empty() {
            if self.local_pipelines.contains_key(pipeline_name) {
                return Ok(true);
            }
        }

        // PHASE 2: Same package, different files
        // Check if alias resolves to same package as current package
        if let Some(alias_package) = self.package_aliases.get(alias) {
            if let Some(ref current_pkg) = self.current_package {
                if Self::package_specs_match(alias_package, current_pkg) {
                    // Same package - search same-package files
                    if let Some(base_dir) = self.get_base_directory() {
                        let same_package_files = self.discover_same_package_files(&base_dir, current_pkg)?;

                        // Validate file ordering (Task 3.1, 3.3)
                        self.validate_file_ordering(&same_package_files)?;

                        // Sort files by [#] ordering
                        let ordered_files = self.sort_files_by_ordering(&same_package_files)?;

                        // Search each file for the pipeline
                        for file_path in ordered_files {
                            if self.search_file_for_pipeline(&file_path, pipeline_name)? {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        // PHASE 3: External packages (registry)
        // Use the ImportResolver to check external packages
        if let Some(package_spec) = self.package_aliases.get(alias) {
            // Try to list available pipelines from the package
            // For now, we don't have a direct "does pipeline exist" method,
            // so we'll return false and let validation happen elsewhere
            // TODO: Implement proper Phase 3 validation when FileRegistryResolver
            // exposes pipeline existence checking
            let _ = package_spec; // Use the variable
        }

        // Pipeline not found in any phase
        Ok(false)
    }

    /// Get the base directory for file discovery
    ///
    /// Extracts the directory containing the current source file
    fn get_base_directory(&self) -> Option<PathBuf> {
        self.source_file.as_ref().and_then(|path| {
            PathBuf::from(path).parent().map(|p| p.to_path_buf())
        })
    }

    /// Validate file ordering markers for duplicates and gaps
    ///
    /// Checks for duplicate [#] markers across package files and warns about gaps
    /// in sequential ordering (Story 1.5.5 - Tasks 3.1, 3.3)
    ///
    /// # Arguments
    /// * `files` - List of file paths to validate
    ///
    /// # Errors
    /// Returns `ParserError::DuplicateFileOrder` if duplicate [#] numbers found
    ///
    /// # Side Effects
    /// Prints warnings to stderr if gaps detected in sequential ordering
    fn validate_file_ordering(&mut self, files: &[PathBuf]) -> Result<(), ParserError> {
        use std::collections::HashMap;
        use std::fs;

        // Map of order_num -> file_path for duplicate detection
        let mut order_map: HashMap<usize, PathBuf> = HashMap::new();
        let mut numbered_files: Vec<usize> = Vec::new();

        // Parse [#] markers from each file and check for duplicates
        for file_path in files {
            // Read file to extract [#] marker
            let source = fs::read_to_string(file_path).map_err(|e| ParserError::UnexpectedEof {
                context: format!("Failed to read file {:?}: {}", file_path, e),
                started_at: Span::start(),
            })?;

            // Quick check for [#] marker at beginning of file
            let order_num = Self::extract_file_order_marker(&source)?;

            if let Some(num) = order_num {
                // Check for duplicate
                if let Some(existing_file) = order_map.get(&num) {
                    return Err(ParserError::DuplicateFileOrder {
                        file1: existing_file.display().to_string(),
                        file2: file_path.display().to_string(),
                        order_num: num,
                        span: Span::start(), // Could be improved with actual file span
                    });
                }

                order_map.insert(num, file_path.clone());
                numbered_files.push(num);

                // Store in file_ordering for later use
                self.file_ordering.insert(file_path.clone(), Some(num));
            } else {
                // No [#] marker - store as None
                self.file_ordering.insert(file_path.clone(), None);
            }
        }

        // Validate sequential ordering (Task 3.3)
        if !numbered_files.is_empty() {
            numbered_files.sort_unstable();

            // Check for gaps in sequential ordering
            for i in 0..numbered_files.len() - 1 {
                let current = numbered_files[i];
                let next = numbered_files[i + 1];

                if next != current + 1 {
                    eprintln!(
                        "Warning: Gap in file ordering sequence: found [#] {} followed by [#] {} \
                        (expected sequential numbering)",
                        current, next
                    );
                }
            }
        }

        Ok(())
    }

    /// Extract [#] file order marker from source without full parsing
    ///
    /// Quick extraction for validation purposes
    ///
    /// # Arguments
    /// * `source` - Source code string
    ///
    /// # Returns
    /// * `Ok(Some(num))` - File has [#] num marker
    /// * `Ok(None)` - No [#] marker
    /// * `Err` - Parse error
    fn extract_file_order_marker(source: &str) -> Result<Option<usize>, ParserError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|e| ParserError::LexerError {
            source: e,
            span: Span::start(),
        })?;

        // Look for [#] token followed by integer
        for (i, token) in tokens.iter().enumerate() {
            if matches!(token.kind, TokenKind::BlockVersionEnum) {
                // Found [#], check next token
                if let Some(next_token) = tokens.get(i + 1) {
                    if matches!(next_token.kind, TokenKind::LiteralInteger) {
                        if let Ok(num) = next_token.lexeme.parse::<usize>() {
                            return Ok(Some(num));
                        }
                    }
                }
                // [#] found but no valid integer - return None (handled elsewhere)
                return Ok(None);
            }

            // Stop at package declaration ([@]) - no [#] marker before it
            if matches!(token.kind, TokenKind::BlockPackageStart) {
                return Ok(None);
            }
        }

        Ok(None)
    }

    /// Sort files by [#] ordering markers
    ///
    /// Files with [#] markers come first (sorted by number),
    /// files without markers come last (in arbitrary order)
    ///
    /// # Arguments
    /// * `files` - List of file paths to sort
    ///
    /// # Returns
    /// Sorted vector of file paths
    fn sort_files_by_ordering(&self, files: &[PathBuf]) -> Result<Vec<PathBuf>, ParserError> {
        let mut ordered_files: Vec<(PathBuf, Option<usize>)> = files
            .iter()
            .map(|path| {
                let ordering = self.file_ordering.get(path).copied().flatten();
                (path.clone(), ordering)
            })
            .collect();

        // Sort: numbered files first (by number), then unnumbered files
        ordered_files.sort_by(|a, b| match (a.1, b.1) {
            (Some(n1), Some(n2)) => n1.cmp(&n2),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        Ok(ordered_files.into_iter().map(|(path, _)| path).collect())
    }

    /// Search a file for a pipeline definition
    ///
    /// Parses the file (using cache if available) and checks if it contains
    /// the specified pipeline.
    ///
    /// # Arguments
    /// * `file_path` - Path to the .pg file
    /// * `pipeline_name` - Name of pipeline to search for
    ///
    /// # Returns
    /// * `Ok(true)` - Pipeline found in file
    /// * `Ok(false)` - Pipeline not found
    /// * `Err(ParserError)` - Parse error
    fn search_file_for_pipeline(
        &mut self,
        file_path: &PathBuf,
        pipeline_name: &str,
    ) -> Result<bool, ParserError> {
        use std::fs;

        // Check cache first
        if let Some(program) = self.file_cache.get(file_path) {
            return Ok(Self::program_contains_pipeline(program, pipeline_name));
        }

        // Not in cache - parse the file
        let source = fs::read_to_string(file_path).map_err(|e| ParserError::UnexpectedEof {
            context: format!("Failed to read file {:?}: {}", file_path, e),
            started_at: Span::start(),
        })?;

        // Parse the file
        let parsed_program = Self::parse_file(&source)?;

        // Cache the parsed program
        let found = Self::program_contains_pipeline(&parsed_program, pipeline_name);
        self.file_cache.insert(file_path.clone(), parsed_program);

        Ok(found)
    }

    /// Check if a Program contains a pipeline definition
    ///
    /// # Arguments
    /// * `program` - Parsed program to search
    /// * `pipeline_name` - Name of pipeline to find
    ///
    /// # Returns
    /// `true` if pipeline is defined in the program
    fn program_contains_pipeline(program: &Program, pipeline_name: &str) -> bool {
        program.definitions.iter().any(|def| match def {
            Definition::Pipeline(pipeline) => pipeline.name == pipeline_name,
            _ => false,
        })
    }

    /// Parse a file without consuming self
    ///
    /// Creates a minimal parser to parse a file for pipeline lookup
    ///
    /// # Arguments
    /// * `source` - Source code to parse
    ///
    /// # Returns
    /// Parsed Program or ParserError
    fn parse_file(source: &str) -> Result<Program, ParserError> {
        use crate::import_resolver::StubImportResolver;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|e| ParserError::LexerError {
            source: e,
            span: Span::start(),
        })?;

        let parser = Parser {
            tokens,
            current: 0,
            resolver: StubImportResolver::new(),
            source_file: None,
            package_aliases: HashMap::new(),
            local_pipelines: HashMap::new(),
            file_cache: HashMap::new(),
            current_package: None,
            file_ordering: HashMap::new(),
        };

        parser.parse()
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

    // ========================================================================
    // File Ordering Marker Tests (Story 1.5.5 - Task 1.3)
    // ========================================================================

    #[test]
    fn test_parse_file_ordering_marker_with_number() {
        let source = r#"
[#] 1
[@] Local@MyApp:1.0.0
[X]

[|] Test
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
[X]
        "#;

        let resolver = FileRegistryResolver::empty();
        let parser = Parser::new(source, resolver).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Failed to parse file with [#] 1 marker: {:?}", result.err());
    }

    #[test]
    fn test_parse_file_ordering_marker_large_number() {
        let source = r#"
[#] 42
[@] Local@MyApp:1.0.0
[X]

[|] Test
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
[X]
        "#;

        let resolver = FileRegistryResolver::empty();
        let parser = Parser::new(source, resolver).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Failed to parse file with [#] 42 marker: {:?}", result.err());
    }

    #[test]
    fn test_parse_file_without_ordering_marker() {
        let source = r#"
[@] Local@MyApp:1.0.0
[X]

[|] Test
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
[X]
        "#;

        let resolver = FileRegistryResolver::empty();
        let parser = Parser::new(source, resolver).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Failed to parse file without [#] marker: {:?}", result.err());
    }

    #[test]
    fn test_parse_file_ordering_marker_missing_number() {
        let source = r#"
[#]
[@] Local@MyApp:1.0.0
[X]
        "#;

        let resolver = FileRegistryResolver::empty();
        let parser = Parser::new(source, resolver).unwrap();
        let result = parser.parse();

        assert!(result.is_err(), "Expected error for [#] without number");
        match result {
            Err(ParserError::UnexpectedToken { expected, context, .. }) => {
                assert_eq!(expected, "integer literal");
                assert_eq!(context, "parsing file ordering marker [#]");
            }
            _ => panic!("Expected UnexpectedToken error, got: {:?}", result),
        }
    }

    #[test]
    fn test_parse_file_ordering_marker_zero() {
        let source = r#"
[#] 0
[@] Local@MyApp:1.0.0
[X]

[|] Test
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
[X]
        "#;

        let resolver = FileRegistryResolver::empty();
        let parser = Parser::new(source, resolver).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Failed to parse file with [#] 0 marker: {:?}", result.err());
    }
}
