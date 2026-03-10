// Token types for Polyglot v0.0.4
// Complete enumeration of all token types

use serde::{Deserialize, Serialize};

/// Represents a single token in the Polyglot language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            kind,
            lexeme,
            line,
            column,
        }
    }
}

/// All token types in Polyglot v0.0.4 (123 types total)
/// Recently added: Indent, Dedent (for loop body indentation tracking)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenKind {
    // ========================================
    // Block Markers (30 tokens)
    // ========================================
    BlockPackageStart,     // [@]
    BlockVersionEnum,      // [#]
    BlockEnd,              // [X]
    BlockPipelineStart,    // [|]
    BlockInput,            // [i]
    BlockTrigger,          // [t]
    BlockQueue,            // [Q]
    BlockWrapper,          // [W]
    BlockSetup,            // [\]
    BlockCleanup,          // [/]
    BlockOutput,           // [o]
    BlockSequential,       // [r]
    BlockInputBinding,     // [<]
    BlockOutputBinding,    // [>]
    BlockParallel,         // [p]
    BlockFork,             // [f] - Fork/Conditional
    BlockJoin,             // [v] - Join/Synchronize
    BlockBackground,       // [b]
    BlockSerialLoad,       // [s] - Loading Serial files
    BlockErrorCatch,       // [!]
    BlockConditional,      // [?]
    BlockBody,             // [~]
    BlockBoolOr,           // [+]
    BlockBoolAnd,          // [&]
    BlockBoolNot,          // [-] - Boolean NOT
    BlockBoolXor,          // [^] - Boolean XOR
    BlockSubfield,         // [.] - Subfield definition marker
    BlockLineContinuation, // [*]
    BlockMacroDefinition,  // [M]
    BlockScopeInput,       // [{]
    BlockScopeOutput,      // [}]
    BlockAliasDefinition,  // [A]

    // ========================================
    // Assignment Operators (6 tokens)
    // ========================================
    OpPushLeft,           // << - Push to left
    OpPushRight,          // >> - Push to right
    OpDefaultPushLeft,    // <~ - Default push to left
    OpDefaultPushRight,   // ~> - Default push to right
    OpVariadicPushLeft,   // <<< - Variadic push to left
    OpVariadicPushRight,  // >>> - Variadic push to right

    // ========================================
    // String Operators (1 token)
    // ========================================
    OpStringConcat, // +"

    // ========================================
    // Comparison Operators (11 tokens)
    // ========================================
    OpEqual,           // =?
    OpNotEqual,        // =!?
    OpGreater,         // >?
    OpNotGreater,      // >!?
    OpLess,            // <?
    OpNotLess,         // <!?
    OpGreaterEqual,    // >=?
    OpNotGreaterEqual, // >=!?
    OpLessEqual,       // <=?
    OpNotLessEqual,    // <=!?

    // ========================================
    // Pattern Operators (5 tokens)
    // ========================================
    OpWildcard,      // *?
    OpRegex,         // re?
    OpNotRegex,      // re!?
    OpInCollection,  // in?
    OpNotInCollection, // in!?

    // ========================================
    // Range Operators (4 tokens)
    // ========================================
    OpRangeClosed,    // ?[
    OpRangeOpen,      // ?(
    OpRangeHalfRight, // ?]
    OpRangeHalfLeft,  // ?)

    // ========================================
    // Composition Operators (1 token)
    // ========================================
    OpPipelineCompose, // |>

    // ========================================
    // Delimiters (14 tokens)
    // ========================================
    DelimiterBraceOpen,          // {
    DelimiterBraceClose,         // }
    DelimiterParenOpen,          // (
    DelimiterParenClose,         // )
    DelimiterSquareBracketClose, // ] (for range operators, NOT block markers)
    DelimiterQuote,              // " (for reference, actual strings use STRING_* tokens)
    DelimiterComma,              // ,
    DelimiterColon,              // :
    DelimiterSemicolon,          // ; (for reserved indication: #;Boolean;True)
    DelimiterAt,                 // @
    DelimiterBackslash,          // \
    DelimiterPipe,               // |
    DelimiterDot,                // .
    DelimiterInputPrefix,        // < (standalone, for input arguments)
    DelimiterOutputPrefix,       // > (standalone, for output arguments)

    // ========================================
    // String Literal Tokens (6 tokens) - NEW
    // ========================================
    StringStart,        // " (opening quote)
    StringContent,      // Static text between interpolations
    StringEnd,          // " (closing quote)
    InterpolationStart, // { (opening brace in string)
    InterpolationEnd,   // } (closing brace in string)
    FormatIdentifier,   // Format specifier (e.g., Hex, Currency)

    // ========================================
    // Identifiers (10 categories)
    // ========================================
    IdentifierVariable,       // .identifier (can have dot hierarchy: .var.name)
    IdentifierEnum,           // #identifier (can have dot hierarchy: #Config.Timeout)
    IdentifierPipeline,       // |identifier (can have dot hierarchy: |U.Log.Info)
    IdentifierError,          // !identifier (can have dot hierarchy: !Network.Timeout)
    IdentifierUnpack,         // ~identifier (e.g., ~ForEach, ~Enumerate, ~Zip)
    IdentifierJoin,           // ~Y.identifier
    IdentifierInputArgument,  // <identifier (e.g., <input1, <data)
    IdentifierOutputArgument, // >identifier (e.g., >output1, >result)
    IdentifierDataType,       // :type.path (e.g., :pg.string, :pg.array.pg.int)
    IdentifierPackageSpec,    // @Registry::Package:Version (e.g., @Local::MyApp:1.0.0.0)
    IdentifierMetadata,       // %identifier (e.g., %Doc, %Author, %Deprecated)
    Identifier,               // Plain identifier (DEPRECATED - should not exist in valid code)

    // ========================================
    // Reserved Enumerations (10 tokens)
    // ========================================
    ReservedPgVarDeclared,     // #PgVar.States.Declared
    ReservedPgVarDefaultReady, // #PgVar.States.DefaultReady
    ReservedPgVarPending,      // #PgVar.States.Pending
    ReservedPgVarReady,        // #PgVar.States.Ready
    ReservedPgVarFaulted,      // #PgVar.States.Faulted
    ReservedBooleanTrue,       // #Boolean.True
    ReservedBooleanFalse,      // #Boolean.False
    ReservedNone,              // #None
    ReservedPipelineNoInput,   // #Pipeline.NoInput
    ReservedNoError,           // !NoError

    // ========================================
    // Literals (6 types)
    // ========================================
    LiteralInteger,           // 42, -10
    LiteralFloat,             // 3.14, -0.5
    LiteralDatetime,          // DT"2024-01-15T14:30:00Z"
    LiteralDuration,          // DT.Minutes"5"
    LiteralCollection,        // {1, 2, 3}
    LiteralPipelineFormatted, // |Pipeline"formatted {.string}"

    // ========================================
    // Type Tokens (10 tokens) - DEPRECATED
    // These should NOT appear as standalone tokens in valid v0.0.4 code.
    // Types should only appear within IdentifierDataType tokens (e.g., :pg.string)
    // Keeping for backward compatibility during transition.
    // ========================================
    TypeNamespace, // DEPRECATED: pg, py, rs, go, js, node
    TypeString,    // DEPRECATED: string
    TypeInt,       // DEPRECATED: int
    TypeFloat,     // DEPRECATED: float
    TypeBool,      // DEPRECATED: bool
    TypeDatetime,  // DEPRECATED: dt
    TypePath,      // DEPRECATED: path
    TypeSerial,    // DEPRECATED: serial
    TypeArray,     // DEPRECATED: array
    TypeSet,       // DEPRECATED: set

    // ========================================
    // Special Identifiers (5 categories)
    // ========================================
    SpecialDatetime,    // DT.Operation
    SpecialRuntime,     // RT.Language
    SpecialTrigger,     // TG.Type
    SpecialTriggerType, // |T.Call, |T.String.Call
    SpecialWrapper,     // |W.Polyglot.Scope

    // ========================================
    // Comments (2 tokens)
    // ========================================
    CommentSingle, // // ...
    CommentMulti,  // /* ... */

    // ========================================
    // Whitespace (6 tokens)
    // ========================================
    Newline,    // \n
    Whitespace, // Space, tab (usually skipped)
    Indent,     // Indentation increase (loop bodies only)
    Dedent,     // Indentation decrease (loop bodies only)

    // ========================================
    // Version (1 token)
    // ========================================
    Version, // 1.0.0

    // ========================================
    // End of File (1 token)
    // ========================================
    Eof,
}

impl TokenKind {
    /// Returns true if this token type should be skipped (not emitted)
    pub fn should_skip(&self) -> bool {
        matches!(
            self,
            TokenKind::Whitespace | TokenKind::CommentSingle | TokenKind::CommentMulti
        )
    }

    /// Returns a human-readable description of the token kind
    pub fn description(&self) -> &'static str {
        match self {
            // Block markers
            TokenKind::BlockPackageStart => "package start marker [@]",
            TokenKind::BlockVersionEnum => "version/enum marker [#]",
            TokenKind::BlockEnd => "block end marker [X]",
            TokenKind::BlockPipelineStart => "pipeline start marker [|]",
            TokenKind::BlockInput => "input marker [i]",
            TokenKind::BlockTrigger => "trigger marker [t]",
            TokenKind::BlockQueue => "queue marker [Q]",
            TokenKind::BlockWrapper => "wrapper marker [W]",
            TokenKind::BlockSetup => "setup marker [\\]",
            TokenKind::BlockCleanup => "cleanup marker [/]",
            TokenKind::BlockOutput => "output marker [o]",
            TokenKind::BlockSequential => "sequential marker [r]",
            TokenKind::BlockInputBinding => "input binding marker [<]",
            TokenKind::BlockOutputBinding => "output binding marker [>]",
            TokenKind::BlockParallel => "parallel marker [p]",
            TokenKind::BlockFork => "fork marker [f]",
            TokenKind::BlockJoin => "join marker [v]",
            TokenKind::BlockBackground => "background marker [b]",
            TokenKind::BlockSerialLoad => "serial load marker [s]",
            TokenKind::BlockErrorCatch => "error catch marker [!]",
            TokenKind::BlockConditional => "conditional marker [?]",
            TokenKind::BlockBody => "body marker [~]",
            TokenKind::BlockBoolOr => "boolean OR marker [+]",
            TokenKind::BlockBoolAnd => "boolean AND marker [&]",
            TokenKind::BlockBoolNot => "boolean NOT marker [-]",
            TokenKind::BlockBoolXor => "boolean XOR marker [^]",
            TokenKind::BlockSubfield => "subfield definition marker [.]",
            TokenKind::BlockLineContinuation => "line continuation marker [*]",
            TokenKind::BlockMacroDefinition => "macro definition marker [M]",
            TokenKind::BlockScopeInput => "scope input marker [{]",
            TokenKind::BlockScopeOutput => "scope output marker [}]",
            TokenKind::BlockAliasDefinition => "alias definition marker [A]",

            // Operators
            TokenKind::OpPushLeft => "push left operator <<",
            TokenKind::OpPushRight => "push right operator >>",
            TokenKind::OpDefaultPushLeft => "default push left operator <~",
            TokenKind::OpDefaultPushRight => "default push right operator ~>",
            TokenKind::OpVariadicPushLeft => "variadic push left operator <<<",
            TokenKind::OpVariadicPushRight => "variadic push right operator >>>",
            TokenKind::OpStringConcat => "string concatenation operator +\"",
            TokenKind::OpEqual => "equal operator =?",
            TokenKind::OpNotEqual => "not equal operator =!?",
            TokenKind::OpGreater => "greater than operator >?",
            TokenKind::OpNotGreater => "not greater than operator >!?",
            TokenKind::OpLess => "less than operator <?",
            TokenKind::OpNotLess => "not less than operator <!?",
            TokenKind::OpGreaterEqual => "greater or equal operator >=?",
            TokenKind::OpNotGreaterEqual => "not greater or equal operator >=!?",
            TokenKind::OpLessEqual => "less or equal operator <=?",
            TokenKind::OpNotLessEqual => "not less or equal operator <=!?",
            TokenKind::OpWildcard => "wildcard operator *?",
            TokenKind::OpRegex => "regex operator re?",
            TokenKind::OpNotRegex => "not regex operator re!?",
            TokenKind::OpInCollection => "in collection operator in?",
            TokenKind::OpNotInCollection => "not in collection operator in!?",
            TokenKind::OpPipelineCompose => "pipeline composition operator |>",
            TokenKind::OpRangeClosed => "closed range operator ?[",
            TokenKind::OpRangeOpen => "open range operator ?(",
            TokenKind::OpRangeHalfRight => "half-right range operator ?]",
            TokenKind::OpRangeHalfLeft => "half-left range operator ?)",

            // Delimiters
            TokenKind::DelimiterBraceOpen => "opening brace {",
            TokenKind::DelimiterBraceClose => "closing brace }",
            TokenKind::DelimiterParenOpen => "opening parenthesis (",
            TokenKind::DelimiterParenClose => "closing parenthesis )",
            TokenKind::DelimiterSquareBracketClose => "closing square bracket ]",
            TokenKind::DelimiterQuote => "quote \"",
            TokenKind::DelimiterComma => "comma ,",
            TokenKind::DelimiterColon => "colon :",
            TokenKind::DelimiterSemicolon => "semicolon ;",
            TokenKind::DelimiterAt => "at sign @",
            TokenKind::DelimiterBackslash => "backslash \\",
            TokenKind::DelimiterPipe => "pipe |",
            TokenKind::DelimiterDot => "dot .",
            TokenKind::DelimiterInputPrefix => "input prefix <",
            TokenKind::DelimiterOutputPrefix => "output prefix >",

            // String tokens
            TokenKind::StringStart => "string start",
            TokenKind::StringContent => "string content",
            TokenKind::StringEnd => "string end",
            TokenKind::InterpolationStart => "interpolation start",
            TokenKind::InterpolationEnd => "interpolation end",
            TokenKind::FormatIdentifier => "format identifier",

            // Identifiers
            TokenKind::IdentifierVariable => "variable identifier",
            TokenKind::IdentifierEnum => "enum identifier",
            TokenKind::IdentifierPipeline => "pipeline identifier",
            TokenKind::IdentifierError => "error identifier",
            TokenKind::IdentifierUnpack => "unpack identifier",
            TokenKind::IdentifierJoin => "join identifier",
            TokenKind::IdentifierInputArgument => "input argument identifier",
            TokenKind::IdentifierOutputArgument => "output argument identifier",
            TokenKind::IdentifierDataType => "data type identifier",
            TokenKind::IdentifierPackageSpec => "package spec identifier",
            TokenKind::IdentifierMetadata => "metadata identifier",
            TokenKind::Identifier => "plain identifier",

            // Reserved
            TokenKind::ReservedPgVarDeclared => "reserved #PgVar.States.Declared",
            TokenKind::ReservedPgVarDefaultReady => "reserved #PgVar.States.DefaultReady",
            TokenKind::ReservedPgVarPending => "reserved #PgVar.States.Pending",
            TokenKind::ReservedPgVarReady => "reserved #PgVar.States.Ready",
            TokenKind::ReservedPgVarFaulted => "reserved #PgVar.States.Faulted",
            TokenKind::ReservedBooleanTrue => "reserved #Boolean.True",
            TokenKind::ReservedBooleanFalse => "reserved #Boolean.False",
            TokenKind::ReservedNone => "reserved #None",
            TokenKind::ReservedPipelineNoInput => "reserved #Pipeline.NoInput",
            TokenKind::ReservedNoError => "reserved !NoError",

            // Literals
            TokenKind::LiteralInteger => "integer literal",
            TokenKind::LiteralFloat => "float literal",
            TokenKind::LiteralDatetime => "datetime literal",
            TokenKind::LiteralDuration => "duration literal",
            TokenKind::LiteralCollection => "collection literal",
            TokenKind::LiteralPipelineFormatted => "pipeline formatted string literal",

            // Types
            TokenKind::TypeNamespace => "type namespace",
            TokenKind::TypeString => "string type",
            TokenKind::TypeInt => "int type",
            TokenKind::TypeFloat => "float type",
            TokenKind::TypeBool => "bool type",
            TokenKind::TypeDatetime => "datetime type",
            TokenKind::TypePath => "path type",
            TokenKind::TypeSerial => "serial type",
            TokenKind::TypeArray => "array type",
            TokenKind::TypeSet => "set type",

            // Special
            TokenKind::SpecialDatetime => "datetime pipeline",
            TokenKind::SpecialRuntime => "runtime wrapper",
            TokenKind::SpecialTrigger => "trigger",
            TokenKind::SpecialTriggerType => "trigger type",
            TokenKind::SpecialWrapper => "wrapper",

            // Comments
            TokenKind::CommentSingle => "single-line comment",
            TokenKind::CommentMulti => "multi-line comment",

            // Whitespace
            TokenKind::Newline => "newline",
            TokenKind::Whitespace => "whitespace",
            TokenKind::Indent => "indentation increase",
            TokenKind::Dedent => "indentation decrease",

            // Version
            TokenKind::Version => "version number",

            // EOF
            TokenKind::Eof => "end of file",
        }
    }
}
