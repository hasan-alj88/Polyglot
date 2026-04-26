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
    ActionImport,
    ActionCollector,
    ContinueActionLine,

    // Identifiers (Data-Carrying)
    Variable(String),
    Data(String),
    DataType(String),
    Pipeline(String),
    Error(String),
    Package(String),
    InputParameter(String),
    OutputParameter(String),
    TerminalData(String),
    FixedSubField(String),
    FlexibleSubField(String),
    CommentText(String),
    InvalidIdentifier(String),
    Registry(String),
    PackageName(String),
    Environment(String),
    Trigger(String),
    QueueConfig(String),
    Wrapper(String),
    Collector(String),
    StringLiteral(String),
    SubstituteVariable(String),
    MetaData(String),
    NoVersion,
    InlineInstruction(String),
    Constructor(String),
    InlineString(String),
    RangeFrom(String),
    RangeTo(String),
    // (Other identifiers can be added here)

    // Operators
    PullFrom,
    PushInto,
    DefaultPullFrom,
    DefaultPushInto,
    FallBackPullFrom,
    FallBackPushInto,

    // Compression Operators
    IsItEqual,
    IsItNotEqual,
    IsItGreaterThan,
    IsItNotGreaterThan,
    IsItLessThan,
    IsItNotLessThan,
    IsItOtherwise,

    // Range Operators
    IsItInRangeInclusiveFrom,
    IsItInRangeExclusiveFrom,
    IsItInRangeInclusiveTo,
    IsItInRangeExclusiveTo,
    RangeSeparator,

    // IO Brackets
    PipelineIO,
    DataInput,
    ExpanderIO,
    CollectorIO,
    ContinueIOLine,
    IoParamOutFallback,
    IoParamInFallback,
    IoComment,

    // Fallback/Error
    MissingMarker,
    IncorrectIndent(String),
    InvalidPattern(String),
}
