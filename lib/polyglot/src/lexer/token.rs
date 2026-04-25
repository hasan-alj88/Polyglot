#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub value: T,
    pub line: usize,
    pub col: usize,
}

impl<T> Spanned<T> {
    pub fn new(value: T, line: usize, col: usize) -> Self {
        Self { value, line, col }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolyglotToken {
    // Structural
    Scope(usize),
    TokNewline,
    TokSpace,

    // Definitions
    DefPackage,
    DefData,
    DefPipeline,
    DefTrigger,
    DefWrapper,
    DefNative,
    DefQueue,
    DefError,
    DefPermission,
    DefCollector,
    DefConstructor,
    DefComment,

    // Actions
    ActionRegistry,
    ActionExecSeq,
    ActionExecPar,
    ActionExecBg,
    ActionDataLoad,
    ActionTypeBind,
    ActionCondSwitch,
    ActionError,
    ActionTrigger,
    ActionQueue,
    ActionWrapper,
    ActionScopeIn,
    ActionScopeOut,
    ActionDataAccessFixed,
    ActionDataAccessFlex,
    ActionLogicalAnd,
    ActionLogicalOr,
    ActionLogicalXor,
    ActionContinuation,
    ActionForeignCode,
    ActionMetadata,
    ActionComment,

    // Identifiers (Data-Carrying)
    Variable(String),
    Data(String),
    DataType(String),
    Pipeline(String),
    Error(String),
    InputParameter(String),
    TerminalData(String),
    FixedSubField(String),
    FlexibleSubField(String),
    CommentText(String),
    // (Other identifiers can be added here)

    // Operators
    OpPushLeft,
    OpPushRight,
    OpFallbackPushLeft,
    OpFallbackPushRight,

    // IO Brackets
    IoPipeline,
    IoParamOutFallback,
    IoParamInFallback,
    IoComment,

    // Fallback/Error
    MissingMarker,
    IncorrectIndent(String),
    TokUnrecognized(char),
}
