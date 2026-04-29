use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref RE_TYPED_VAR: Regex = Regex::new(r"^\$(?P<var>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z0-9]+)*)#(?P<type>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z0-9]+)*)").unwrap();
    pub static ref RE_STANDALONE_VAR: Regex = Regex::new(r"^\$(?P<var>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z0-9]+)*)").unwrap();

    // String literals with potential variable substitutions
    pub static ref RE_STRING: Regex = Regex::new(r#"^"(?P<inner>.*?)""#).unwrap();

    // Registry and Package definitions with optional version
    pub static ref RE_REGISTRY_PKG: Regex = Regex::new(r"^@(?P<reg>[a-zA-Z0-9]+:[a-zA-Z0-9]+)<(?P<pkg>[a-zA-Z0-9.]+)(?::(?P<ver>[a-zA-Z0-9.]+))?").unwrap();

    // Generic packages
    pub static ref RE_PACKAGE: Regex = Regex::new(r"^@(?P<pkg>[a-zA-Z0-9.]+)").unwrap();

    // Environment
    pub static ref RE_ENVIRONMENT: Regex = Regex::new(r"^;(?P<env>[a-zA-Z0-9.]+)").unwrap();

    // Actions & Identifiers: pipelines, triggers, queues, wrappers
    pub static ref RE_ACTION_CALL: Regex = Regex::new(r#"^-(?P<ident>[a-zA-Z0-9.]+)(?P<has_string>"(?P<str>.*?)")?"#).unwrap();

    // Constructors
    // Also matched for standalone variables if they don't have `#type` or `""` attached
    pub static ref RE_CONSTRUCTOR: Regex = Regex::new(r#"^\$(?P<ident>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z0-9]+)*)(?P<has_string>"(?P<str>.*?)")?"#).unwrap();

    // IO Parameters
    pub static ref RE_INPUT_PARAM: Regex = Regex::new(r"^<(?P<param>[a-zA-Z0-9.]+)").unwrap();
    pub static ref RE_OUTPUT_PARAM: Regex = Regex::new(r"^>(?P<param>[a-zA-Z0-9.]+)").unwrap();

    // Operators
    pub static ref RE_PULL: Regex = Regex::new(r"^<<").unwrap();
    pub static ref RE_PUSH: Regex = Regex::new(r"^>>").unwrap();
    pub static ref RE_DEFAULT_PULL: Regex = Regex::new(r"^<~").unwrap();
    pub static ref RE_DEFAULT_PUSH: Regex = Regex::new(r"^~>").unwrap();
    pub static ref RE_FALLBACK_PULL: Regex = Regex::new(r"^<!").unwrap();
    pub static ref RE_FALLBACK_PUSH: Regex = Regex::new(r"^>!").unwrap();

    // Compression Operators
    pub static ref RE_COMPRESSION: Regex = Regex::new(r"^(?P<op>=\?|=!\?|>\?|>\!\?|<\?|<\!\?|\*\?)").unwrap();

    // Range Operations
    pub static ref RE_RANGE: Regex = Regex::new(r#"^\?(?P<open>\[|\()(?P<from>[^,]+),(?P<to>[^\]\)]+)(?P<close>\]|\))"#).unwrap();

    // Collectors
    pub static ref RE_COLLECTOR: Regex = Regex::new(r"^\*(?P<coll>[a-zA-Z0-9.]+)").unwrap();

    // Data and Metadata
    pub static ref RE_DATA_TYPE: Regex = Regex::new(r"^#(?P<type>[a-z][a-zA-Z0-9]*(?:[.:][a-zA-Z0-9]+)*)").unwrap();
    pub static ref RE_ISOLATED_DATA: Regex = Regex::new(r"^#(?P<data>[A-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z0-9]+)*)").unwrap();
    pub static ref RE_DATA_FIELD: Regex = Regex::new(r"^\.(?P<ident>[a-zA-Z0-9_]+)").unwrap();
    pub static ref RE_METADATA: Regex = Regex::new(r"^%(?P<meta>[a-zA-Z0-9.]+)").unwrap();

    // Boolean Predicates
    pub static ref RE_PREDICATE: Regex = Regex::new(r#"^\?(?P<ident>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z][a-zA-Z0-9]*)*)(?P<has_string>"(?P<str>.*?)")?"#).unwrap();

    // Comments
    pub static ref RE_COMMENT_ACTION: Regex = Regex::new(r"^\[ \] *(?P<text>.*)").unwrap();
    pub static ref RE_COMMENT_DEF: Regex = Regex::new(r"^\{ \} *(?P<text>.*)").unwrap();
    pub static ref RE_COMMENT_IO: Regex = Regex::new(r"^\( \) *(?P<text>.*)").unwrap();

    // Invalid constructs
    pub static ref RE_INVALID_FIELD: Regex = Regex::new(r"^[.:]_[a-zA-Z0-9_]*").unwrap();
}
