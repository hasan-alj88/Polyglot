# AST Gap Analysis: Story 1.4 - Missing Semantic Analysis Support

**Date:** 2025-11-28
**Analyst:** Bob (SM) via User Feedback
**Status:** Critical Gap Identified

---

## Executive Summary

Story 1.4 successfully implemented AST node types for **single pipeline parsing**, but the implementation is **incomplete for full .pg file compilation** and **semantic analysis** requirements. The current AST cannot:

1. ✗ Represent complete .pg file structure (package + imports + multiple definitions)
2. ✗ Support multi-file compilation (combining .pg files from same package)
3. ✗ Enable semantic analysis (undeclared variable/pipeline/enum/error detection)
4. ✗ Track symbol declarations for import resolution

**User Expectation (from feedback):**
> "The expectation now is that AST definition is sufficient enough to be able to detect syntax errors, undeclared {variables, Pipelines, Enumerations, Errors} and importing packages, combine pg file of same package in order of provided in [#]"

**Current Reality:**
The AST can parse individual pipelines but **cannot represent the full program structure** required for semantic analysis.

---

## Gap #1: Missing Top-Level Program Structure

### BNF Grammar Requirement

Per `docs/user/language/12-bnf-grammar.md`:

```bnf
<program> ::= <package-declaration-block>
              { <top-level-element> }

<top-level-element> ::= <pipeline-definition>
                      | <enumeration-definition>
                      | <error-definition>
                      | <comment>
```

**Every .pg file MUST:**
1. Start with exactly ONE `[@]` package declaration block
2. Contain zero or more top-level definitions (pipelines, enums, errors)

### Current AST Implementation

```rust
// polyglot-parser/src/ast.rs
pub struct Pipeline { ... }  // Only represents ONE pipeline
```

**Missing:**
- `Program` or `CompilationUnit` struct to hold:
  - Package declaration
  - List of top-level definitions
  - Source file metadata

### Impact

- ❌ Parser cannot build AST for complete .pg file
- ❌ Cannot combine multiple .pg files from same package
- ❌ Cannot track which definitions belong to which package
- ❌ No way to represent file-level imports

### Recommendation

Add to `polyglot-parser/src/ast.rs`:

```rust
/// Top-level compilation unit representing a complete .pg file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// Package declaration (required - every .pg file has one)
    pub package: PackageDeclaration,
    /// Top-level definitions (pipelines, enums, errors)
    pub definitions: Vec<Definition>,
    /// Source file path (for multi-file compilation)
    pub source_file: Option<String>,
    /// Source location (entire file span)
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Definition {
    Pipeline(Pipeline),
    Enumeration(EnumerationDefinition),
    Error(ErrorDefinition),
}
```

---

## Gap #2: Missing Package Declaration AST

### BNF Grammar Requirement

```bnf
<package-declaration-block> ::= "[@]" <package-spec>
                                [ <alias-declaration> ]
                                { <import-declaration> }
                                "[X]"

<package-spec> ::= <registry-path> ":" <version>

<registry-path> ::= <registry-tier> "@" <package-path>

<import-declaration> ::= "[<]" "@" <import-alias> "<<" <package-spec>
```

**Example:**
```polyglot
[@] Local@MyApp.Example:1.0.0
[A] MyAlias
[<] @utils << Community@DataHelpers:2.3.1
[<] @db << Local@DatabaseLib:1.0.0
[X]
```

### Current AST Implementation

- ✓ Has `Identifier::Package(String)` for **using** packages
- ✗ NO AST node for **declaring** packages
- ✗ NO AST node for **importing** packages

### Impact

- ❌ Cannot represent package metadata (name, version, registry)
- ❌ Cannot track import aliases for resolution
- ❌ Cannot validate import versions
- ❌ Cannot detect circular dependencies

### Recommendation

Add to `polyglot-parser/src/ast.rs`:

```rust
/// Package declaration block ([@] ... [X])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageDeclaration {
    /// Package specification (e.g., "Local@MyApp.Example:1.0.0")
    pub spec: PackageSpec,
    /// Optional package alias ([A] MyAlias)
    pub alias: Option<String>,
    /// Import declarations ([<] @alias << package:version)
    pub imports: Vec<ImportDeclaration>,
    /// Source location
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageSpec {
    /// Registry tier (Local, Community, Enterprise)
    pub registry: String,
    /// Package path (e.g., "MyApp.Example")
    pub path: Vec<String>,
    /// Semantic version (major.minor.patch)
    pub version: Version,
    /// Source location
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDeclaration {
    /// Import alias (e.g., "utils")
    pub alias: String,
    /// Imported package specification
    pub package: PackageSpec,
    /// Source location
    pub span: Span,
}
```

---

## Gap #3: Missing Enumeration Definition AST

### BNF Grammar Requirement

```bnf
<enumeration-definition> ::= "[#]" <enumeration-name>
                             { <enumeration-element> }
                             "[X]"

<enumeration-element> ::= <field-definition>
                        | <alias-definition>
                        | <comment>

<field-definition> ::= "[<]" <field-name> ":" <type> "<<" <expression>
```

**Example:**
```polyglot
[#] Status
[<] .pending: pg\string << "PENDING"
[<] .active: pg\string << "ACTIVE"
[<] .completed: pg\string << "COMPLETED"
[A] St
[X]
```

### Current AST Implementation

- ✓ Has `Identifier::Enum(String)` for **using** enums
- ✗ NO AST node for **defining** enums at top level

### Impact

- ❌ Cannot parse enumeration definitions
- ❌ Cannot validate enumeration field types
- ❌ Cannot detect undeclared enumerations
- ❌ Cannot enforce reserved enumeration schemas

### Recommendation

Add to `polyglot-parser/src/ast.rs`:

```rust
/// Top-level enumeration definition ([#] ... [X])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumerationDefinition {
    /// Enumeration name (e.g., "Status" or "Config.Database")
    pub name: Vec<String>,
    /// Field definitions
    pub fields: Vec<EnumField>,
    /// Optional alias ([A] ...)
    pub alias: Option<String>,
    /// Source location
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumField {
    /// Field name (e.g., ".pending")
    pub name: String,
    /// Field type annotation
    pub field_type: TypeAnnotation,
    /// Field value (constant expression)
    pub value: Expression,
    /// Source location
    pub span: Span,
}
```

---

## Gap #4: Missing Error Definition AST

### BNF Grammar Requirement

```bnf
<error-definition> ::= "[!]" "!" <error-name>
                       <message-field>
                       <code-field>
                       <trace-field>
                       { <custom-field> }
                       "[X]"

<message-field> ::= "[<]" ".message" ":" "pg\string" "<<" <string-literal>
<code-field> ::= "[<]" ".code" ":" "pg\int" "<<" <integer-literal>
<trace-field> ::= "[<]" ".trace" ":" "pg\string" "<<" <string-literal>
```

**Example:**
```polyglot
[!] !NetworkError
[<] .message: pg\string << "Network request failed"
[<] .code: pg\int << 1001
[<] .trace: pg\string << ""
[<] .retryable: pg\bool << #True
[X]
```

### Current AST Implementation

- ✓ Has `Identifier::Error(String)` for **using** errors
- ✗ NO AST node for **defining** errors at top level

### Impact

- ❌ Cannot parse error definitions
- ❌ Cannot validate required error fields (message, code, trace)
- ❌ Cannot detect undeclared errors
- ❌ Cannot validate custom error fields

### Recommendation

Add to `polyglot-parser/src/ast.rs`:

```rust
/// Top-level error definition ([!] ! ... [X])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    /// Error name (e.g., "NetworkError" or "Http.TimeoutError")
    pub name: Vec<String>,
    /// Required message field
    pub message: ErrorField,
    /// Required code field
    pub code: ErrorField,
    /// Required trace field
    pub trace: ErrorField,
    /// Custom fields (optional)
    pub custom_fields: Vec<ErrorField>,
    /// Source location
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorField {
    /// Field name (e.g., ".message", ".retryable")
    pub name: String,
    /// Field type
    pub field_type: TypeAnnotation,
    /// Field default value
    pub value: Expression,
    /// Source location
    pub span: Span,
}
```

---

## Gap #5: Missing Semantic Error Types

### Requirements

To enable semantic analysis (undeclared symbol detection), we need additional `ParserError` variants:

### Current Error Coverage

From `polyglot-parser/src/error.rs` (27 variants):
- ✓ Syntax errors (UnexpectedToken, UnexpectedEof, UnclosedDelimiter)
- ✓ Block hierarchy errors (BlockOrderViolation, DuplicateBlock)
- ✓ Type errors (UnknownType, InvalidTypeAnnotation)
- ✗ **NO semantic analysis errors**

### Missing Error Types

Add to `polyglot-parser/src/error.rs`:

```rust
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParserError {
    // ... existing 27 variants ...

    // === SEMANTIC ANALYSIS ERRORS ===

    /// Undeclared variable reference
    #[error("Undeclared variable at {span}: '{name}' has not been declared")]
    UndeclaredVariable {
        name: String,
        span: Span,
    },

    /// Undeclared pipeline reference
    #[error("Undeclared pipeline at {span}: '{name}' is not defined or imported")]
    UndeclaredPipeline {
        name: String,
        available: Vec<String>,  // Suggestion list
        span: Span,
    },

    /// Undeclared enumeration reference
    #[error("Undeclared enumeration at {span}: '{name}' is not defined or imported")]
    UndeclaredEnum {
        name: String,
        available: Vec<String>,
        span: Span,
    },

    /// Undeclared error type reference
    #[error("Undeclared error at {span}: '{name}' is not defined or imported")]
    UndeclaredError {
        name: String,
        available: Vec<String>,
        span: Span,
    },

    /// Unresolved import
    #[error("Unresolved import at {span}: package '{package}' not found")]
    UnresolvedImport {
        package: String,
        span: Span,
    },

    /// Duplicate definition
    #[error("Duplicate definition at {span}: '{name}' already defined at {original_span}")]
    DuplicateDefinition {
        name: String,
        original_span: Span,
        span: Span,
    },

    /// Circular import dependency
    #[error("Circular import at {span}: package '{package}' creates import cycle")]
    CircularImport {
        package: String,
        cycle: Vec<String>,
        span: Span,
    },

    /// Variable used before declaration
    #[error("Variable used before declaration at {span}: '{name}' used at line {use_line}, declared at line {decl_line}")]
    UseBeforeDeclaration {
        name: String,
        use_line: usize,
        decl_line: usize,
        span: Span,
    },

    /// Type mismatch in assignment
    #[error("Type mismatch at {span}: cannot assign {source_type} to {target_type}")]
    TypeMismatch {
        target_type: String,
        source_type: String,
        span: Span,
    },

    /// Invalid package version
    #[error("Invalid package version at {span}: '{version}' does not match semver format")]
    InvalidPackageVersion {
        version: String,
        span: Span,
    },
}
```

### Impact of Missing Semantic Errors

Without these error types:
- ❌ Cannot report undeclared symbol errors during semantic analysis
- ❌ Cannot provide helpful "did you mean?" suggestions
- ❌ No validation of package imports
- ❌ No detection of duplicate definitions
- ❌ Poor developer experience (silent failures or generic errors)

---

## Gap #6: Multi-File Compilation Support

### User Requirement

> "combine pg file of same package in order of provided in [#]"

### BNF Context

The `[#]` in this context likely refers to the **package declaration order** across multiple .pg files that belong to the same package.

### Current AST Limitation

The `Program` struct (Gap #1) needs to support:
1. Tracking source file path
2. Combining multiple `Program` ASTs from same package
3. Preserving definition order

### Recommendation

Extend `Program` struct:

```rust
pub struct Program {
    pub package: PackageDeclaration,
    pub definitions: Vec<Definition>,
    pub source_file: Option<String>,  // ✓ Already planned
    pub file_order: Option<usize>,    // NEW: For multi-file ordering
    pub span: Span,
}

/// Multi-file compilation unit (combines multiple .pg files from same package)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageCompilationUnit {
    /// Package specification (must match across all files)
    pub package: PackageSpec,
    /// Combined imports from all files
    pub imports: Vec<ImportDeclaration>,
    /// All definitions from all files (ordered by file_order)
    pub definitions: Vec<Definition>,
    /// Source files included in this compilation
    pub source_files: Vec<String>,
}
```

---

## Impact Assessment

### Current Capabilities (Story 1.4 Implementation)

✅ **What Works:**
- Parse individual pipeline structures
- Validate AST node construction
- Track source locations (Span)
- Visitor pattern for AST traversal

### Critical Limitations

❌ **What Doesn't Work:**
- Cannot parse complete .pg files (missing `[@]` block)
- Cannot parse enumeration definitions (`[#]` blocks)
- Cannot parse error definitions (`[!]` blocks)
- Cannot represent imports (`[<]` declarations)
- Cannot detect undeclared symbols (missing semantic errors)
- Cannot combine multiple files from same package
- Cannot perform semantic analysis (only syntax validation)

### Downstream Impact

**Epic 2: IR Generation & Validation** (blocked):
- IR generator expects semantic analysis to be done
- Cannot generate IR for enumerations/errors without AST nodes
- Cannot resolve imports for cross-package references

**Epic 3: Database Schema** (partially blocked):
- Cannot store package metadata
- Cannot track import dependencies

**Epic 11: Documentation & Examples** (blocked):
- Cannot compile example .pg files (all start with `[@]` blocks)
- Cannot demonstrate package imports

---

## Root Cause Analysis

### Why Was This Missed?

1. **Story 1.4 scope was unclear** about "complete .pg file" vs "single pipeline"
2. **Acceptance criteria focused on Pipeline AST**, not Program-level structure
3. **BNF grammar not consulted** during story definition
4. **Test cases only covered single pipeline**, not full file structure

### Lessons Learned

- ✓ Always validate story scope against BNF grammar
- ✓ Include "complete file parsing" test case
- ✓ Cross-reference with downstream dependencies (IR generation needs this)

---

## Recommendations

### Option 1: Extend Story 1.4 (Recommended)

**Approach:** Reopen Story 1.4 and add missing AST nodes before moving to Story 1.5

**Rationale:**
- Story 1.4 is titled "Parser AST Definitions" (should include ALL definitions)
- Parser implementation (Story 1.5) needs complete AST to work against
- Cleaner to have all AST types in one story

**New Acceptance Criteria:**
- AC6: Program structure (Program, PackageDeclaration, ImportDeclaration)
- AC7: Top-level definitions (EnumerationDefinition, ErrorDefinition)
- AC8: Semantic error types (10 additional ParserError variants)
- AC9: Multi-file compilation support (PackageCompilationUnit)

**Effort Estimate:**
- 2-3 days additional work
- ~500 lines of AST code
- ~30 additional unit tests

### Option 2: Create New Story 1.4.5 "Semantic Analysis AST Extensions"

**Approach:** Create a new story between 1.4 and 1.5

**Rationale:**
- Story 1.4 is "done" and in review
- Avoid scope creep on completed story
- Explicit story for semantic analysis preparation

**Drawback:**
- Story 1.5 (Parser Implementation) would need to be blocked until 1.4.5 completes
- Creates confusion about what "Parser AST Definitions" includes

### Option 3: Defer to Epic 2 "IR Generation"

**Approach:** Add semantic analysis as part of IR validation

**Rationale:**
- Semantic analysis is often considered part of IR generation phase

**Drawback:**
- IR generator becomes more complex (doing both IR gen + semantic analysis)
- Parser tests in Story 1.5 cannot validate semantic errors
- Violates separation of concerns (parser should detect semantic errors)

---

## Recommended Action Plan

**Immediate Next Steps:**

1. **User Decision Required:**
   - Should we extend Story 1.4 (Option 1)?
   - Or create Story 1.4.5 (Option 2)?

2. **If Option 1 (Extend Story 1.4):**
   - Move story status: `review` → `in-progress`
   - Add new acceptance criteria (AC6-AC9)
   - Implement missing AST nodes (~2 days)
   - Update tests and documentation
   - Re-submit for review

3. **If Option 2 (New Story 1.4.5):**
   - Draft Story 1.4.5 with complete scope
   - Update Epic 1 timeline
   - Block Story 1.5 until 1.4.5 completes

4. **Documentation Updates:**
   - Update `docs/project/epics.md` with clarification
   - Update test design document to include semantic analysis tests

---

## Conclusion

Story 1.4's current implementation is **technically correct but functionally incomplete**. While it successfully defines AST nodes for pipeline structures, it **does not provide the full program-level AST** required for:
- Complete .pg file parsing
- Semantic analysis
- Import resolution
- Multi-file compilation

**This gap must be addressed before Story 1.5 (Parser Implementation)**, as the parser needs a complete AST to parse against.

**Recommended Action:** **Option 1 - Extend Story 1.4** to include complete AST definitions (Program, PackageDeclaration, Enumeration, Error, semantic errors) before proceeding to parser implementation.

---

## Appendix: Complete AST Structure (Proposed)

```rust
// polyglot-parser/src/ast.rs (proposed complete structure)

/// Top-level compilation unit (complete .pg file)
pub struct Program {
    pub package: PackageDeclaration,
    pub definitions: Vec<Definition>,
    pub source_file: Option<String>,
    pub span: Span,
}

pub enum Definition {
    Pipeline(Pipeline),
    Enumeration(EnumerationDefinition),
    Error(ErrorDefinition),
}

pub struct PackageDeclaration { /* See Gap #2 */ }
pub struct ImportDeclaration { /* See Gap #2 */ }
pub struct EnumerationDefinition { /* See Gap #3 */ }
pub struct ErrorDefinition { /* See Gap #4 */ }

// Existing AST nodes (already implemented)
pub struct Pipeline { /* ✓ Already complete */ }
pub struct Block { /* ✓ Already complete */ }
pub enum Statement { /* ✓ Already complete */ }
pub enum Expression { /* ✓ Already complete */ }
```

Total additional code:
- ~500 lines AST definitions
- ~300 lines unit tests
- ~50 lines error types

---

**Gap Analysis Complete**
**Status:** Awaiting user decision on remediation approach
