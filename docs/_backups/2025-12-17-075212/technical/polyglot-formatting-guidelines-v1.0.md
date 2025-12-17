# Polyglot Formatting Guidelines (PFG) v1.0

**Version:** 1.0.0
**Status:** Draft
**Date:** 2025-11-21
**Authors:** Polyglot Core Team

---

## Table of Contents

1. [Introduction](#introduction)
2. [PFG-001: Code Layout & Style](#pfg-001-code-layout--style)
3. [PFG-002: Naming Conventions](#pfg-002-naming-conventions)
4. [PFG-003: Syntax Highlighting](#pfg-003-syntax-highlighting)
5. [PFG-004: Editor Integration](#pfg-004-editor-integration)
6. [PFG-005: Tooling Standards](#pfg-005-tooling-standards)
7. [PFG-006: Documentation Generation](#pfg-006-documentation-generation)
8. [PFG-007: Code Scaffolding](#pfg-007-code-scaffolding)
9. [Future Enhancements](#future-enhancements)
10. [Appendices](#appendices)

---

## Introduction

### Purpose

The Polyglot Formatting Guidelines (PFG) establish a comprehensive, consistent style guide for writing Polyglot code. These guidelines serve three primary purposes:

1. **Readability** - Enable developers to quickly understand code structure
2. **Consistency** - Provide shared conventions for teams and the community
3. **Toolability** - Define clear rules that enable automated formatting and linting

### Scope

PFG covers:
- Code layout and visual structure
- Naming conventions for all language elements
- Syntax highlighting specifications
- Editor and IDE integration standards
- Tooling requirements and behavior
- Documentation generation formats
- Code scaffolding templates

### Design Philosophy

Polyglot's formatting guidelines are built on these fundamental principles:

1. **Block markers are the structure** - Polyglot uses explicit 3-character block markers (`[x]`) instead of indentation or braces
2. **Visual clarity over brevity** - Spacing and structure compensate for lack of indentation
3. **Familiarity where possible** - Adopt proven conventions (PEP 8, Rust) unless Polyglot's nature demands otherwise
4. **Explicit over implicit** - Clear, unambiguous syntax that's both human and machine readable
5. **Automation-first** - Design for asynchronous, parallel execution requires precise semantics

### Audience

- Polyglot developers (all skill levels)
- Tool developers (editor plugins, linters, formatters)
- Language implementers
- Documentation writers

### Compliance Levels

- **MUST** / **REQUIRED** - Mandatory rules that affect compilation or correctness
- **SHOULD** / **RECOMMENDED** - Strong recommendations for consistency
- **MAY** / **OPTIONAL** - Allowed variations or project-specific choices

---

## PFG-001: Code Layout & Style

### The 3-Character Rule

**REQUIRED:** All Polyglot code MUST use block markers at the start of each line.

Block markers follow the pattern: `[x]` where `x` is a single character.

**Examples:**
```polyglot
[|] PipelineName     ✓ Valid
[@] PackageDecl      ✓ Valid
[ r ] Operation      ✗ Invalid (spaces inside brackets)
[rr] Operation       ✗ Invalid (two characters)
```

### No Indentation

**REQUIRED:** Polyglot code MUST NOT use indentation to indicate scope.

**Rationale:** Block markers explicitly declare both scope AND execution semantics. Indentation would be redundant and could conflict with marker-defined structure.

**Example:**
```polyglot
[|] Pipeline
[i] .input: pg\string
[r] .x: pg\int << 1
[?] .x > 0
[~][r] .y: pg\int << 2
[X]
```

All lines start at column 0 (no leading whitespace). The `[~]` marker indicates nesting level.

### Vertical Spacing

#### Between File-Scope Definitions

**REQUIRED:** Three (3) blank lines MUST precede each file-scope definition.

File-scope definitions are:
- `[@]` Package declarations
- `[|]` Pipeline definitions
- `[#]` Enumeration definitions
- `[!]` Error definitions
- `[M]` Macro definitions

**Example:**
```polyglot
[@] Local@MyProject::1.0.0
[X]


[#] Config.Database
[<] .host: pg\string << "localhost"
[X]


[|] ProcessData
[i] .input: pg\string
[X]
```

**Rationale:** Creates clear visual "chapters" in the file, making structure scannable at a glance.

#### Before Branch Points

**RECOMMENDED:** One (1) blank line SHOULD precede branch points.

Branch points are:
- `[p]` Parallel execution blocks
- `[?]` Conditional blocks
- `[!]` Error handling blocks

**Example:**
```polyglot
[r] .x: pg\int << 1
[r] .y: pg\int << 2

[?] .x > .y
[~][r] .result: pg\string << "x is greater"
```

**Rationale:** Visually separates different execution flows.

#### Within Sequential Operations

**RECOMMENDED:** No blank lines between sequential `[r]` operations unless grouping related steps.

**Example:**
```polyglot
// Tight grouping (preferred for short sequences)
[r] .step1: pg\string << "a"
[r] .step2: pg\string << "b"
[r] .step3: pg\string << "c"

// Grouped with comments (acceptable for clarity)
// Phase 1: Initialize
[r] .init_a: pg\string << "x"
[r] .init_b: pg\string << "y"

// Phase 2: Process
[r] .process_a: pg\string << "z"
```

### Horizontal Spacing

#### Around Operators

**REQUIRED:** Single space MUST surround assignment operators.

```polyglot
[r] .x: pg\int << 5        ✓ Correct
[r] .x:pg\int<<5           ✗ Incorrect (no spaces)
[r] .x : pg\int  <<  5     ✗ Incorrect (excessive spaces)
```

#### After Type Separator

**REQUIRED:** Single space MUST follow the colon (`:`) in type annotations.

```polyglot
.variable: pg\string       ✓ Correct
.variable:pg\string        ✗ Incorrect (no space)
.variable :pg\string       ✗ Incorrect (space before colon)
```

#### Inside Block Markers

**REQUIRED:** NO spaces allowed inside block markers.

```polyglot
[r]                        ✓ Correct
[ r ]                      ✗ Incorrect
[r ]                       ✗ Incorrect
```

### Line Length

**RECOMMENDED:** Context-aware line length limits SHOULD be enforced.

| Content Type | Limit | Rationale |
|--------------|-------|-----------|
| Comments | 79 characters | Easier to read prose in narrower columns |
| Code | 99 characters | Balance between readability and expression space |
| String literals | 120 characters | Avoid breaking long paths, URLs, or messages |

**Line Continuation:**

Use `[*]` marker for logical line continuation:
```polyglot
[r] .long_expression: pg\string <<
[*]   "This is a very long string that needs to be " +
[*]   "split across multiple lines for readability"
```

**Exception:** String concatenation uses `+"` prefix (multiline string):
```polyglot
[r] .message: pg\string << "First part"
[*] +"Second part continues"
[*] +"Third part continues"
```

### Comments

#### Comment Syntax

**REQUIRED:** Polyglot supports two comment styles:

1. **Single-line:** `// comment text`
2. **Multi-line:** `/* comment text */`

**Note:** Comments use forward slash `/`, while type separators use backslash `\`.

#### Comment Style (PEP 8 Adapted)

**RECOMMENDED:** Follow these comment conventions:

1. **Block comments:**
   - Appear at the same level as the code they describe
   - Each line starts with `//` and a single space
   - Paragraphs separated by single `//` line

```polyglot
// This is a block comment explaining the following
// pipeline's purpose and behavior.
//
// It can span multiple paragraphs.
[|] Pipeline
```

2. **Inline comments:**
   - Separated from code by at least two spaces
   - Start with `//` and a single space
   - Use sparingly

```polyglot
[r] .x: pg\int << 42  // The answer to everything
```

3. **Complete sentences:**
   - Comments should be complete sentences
   - First word capitalized (unless it's an identifier)
   - End with a period

---

## PFG-002: Naming Conventions

Polyglot uses **dot-based hierarchical naming** for serial data organization. All identifiers use dots (`.`) to create namespaces.

### Variables

**REQUIRED:** Variable names MUST follow these rules:

1. **Start with dot:** `.variable_name`
2. **Lowercase only:** `[a-z0-9_.]`
3. **Cannot end with dot:** `.name.` is invalid
4. **No consecutive dots:** `.name..field` is invalid
5. **Use snake_case:** `.my_variable_name`

**Examples:**
```polyglot
.input                     ✓ Valid
.file_path                 ✓ Valid
.my.nested.var             ✓ Valid
input                      ✗ Invalid (missing dot)
.path.                     ✗ Invalid (ends with dot)
.bad..name                 ✗ Invalid (consecutive dots)
.CamelCase                 ✗ Invalid (uppercase)
```

**Rationale:** Dots enable clear string interpolation: `"Hello {.name}"` and distinguish variables from other identifiers.

### Pipelines

**REQUIRED:** Pipeline names MUST follow these rules:

1. **Start with pipe:** `|PipelineName`
2. **Use CamelCase:** `|MyPipeline`
3. **Can include dots for hierarchy:** `|Utils.String.Format`
4. **Cannot start with number:** `|2Process` is invalid

**RECOMMENDED:** Use verb-based names for clarity:
```polyglot
|ProcessData               ✓ Verb-based, clear
|ValidateInput             ✓ Verb-based, clear
|DataProcessor             ⚠ Noun-based, acceptable but less clear
```

**Reserved Namespaces:**
- `|U.*` - Standard library utilities
- `|W.*` - Built-in runtime wrappers
- `|T.*` - Trigger types
- `|Q.*` - Queue operations
- `|Y.*` - Join operations
- `|M.*` - User-defined macros (RECOMMENDED for user macros)

### Enumerations

**REQUIRED:** Enumeration names MUST follow these rules:

1. **Start with hash:** `#EnumName`
2. **Use CamelCase:** `#MyEnum`
3. **Can include dots for nesting:** `#Config.Database.Host`

**Examples:**
```polyglot
#Config                    ✓ Valid
#Path.Identifiers.Home     ✓ Valid nested
#myenum                    ✗ Invalid (not CamelCase)
```

**Aliases:**
```polyglot
[#] Boolean.True
[A] True                   // Creates #True alias

// Later usage:
.flag: pg\bool << #True    // Alias
.flag: pg\bool << #Boolean.True  // Full path
```

### Errors

**REQUIRED:** Error names MUST follow these rules:

1. **Start with exclamation:** `!ErrorName`
2. **Use CamelCase:** `!MyError`
3. **Can include dots for hierarchy:** `!Network.Timeout`

**RECOMMENDED:** Error name patterns:
```polyglot
!ValidationError           ✓ Descriptive with "Error" suffix
!ValidationFailed          ✓ Descriptive action-based
!NetworkTimeout            ✓ Specific error condition
!Error                     ⚠ Too generic (avoid)
```

### Files and Packages

**REQUIRED:** File naming conventions:

1. **Extension:** `.pg` (all Polyglot files)
2. **Name format:** `CamelCase.pg`

**Examples:**
```
ProcessData.pg             ✓ Valid
MyUtilities.pg             ✓ Valid
process_data.pg            ⚠ Discouraged (prefer CamelCase)
my-file.pg                 ✗ Invalid (hyphens not allowed)
```

**Package paths:**
```polyglot
[@] Local@MyProject.DataProcessing::1.0.0
[@] Community.hasan@StringUtils::2.1.0
[@] Company.acme@InternalLib::3.0.0
```

**Registry names:** CamelCase identifiers (`Local`, `Community`, `Company`)

### Constants

**REQUIRED:** Constants are distinguished by the `[i]` marker, NOT naming convention.

Use `.snake_case` like regular variables:
```polyglot
[i] .max_connections: pg\int << 100
[i] .default_timeout: pg\int << 30
```

**Rationale:** Marker-based distinction keeps naming consistent with variables.

---

## PFG-003: Syntax Highlighting

### Color Scheme Philosophy

Polyglot syntax highlighting is designed to:
1. Distinguish block marker categories visually
2. Use familiar colors (based on VS Code Dark+ Python theme)
3. Support multiple themes while maintaining category grouping
4. Provide both dark and light mode variants

### Default Theme: VS Code Dark+ (Dark Mode)

#### Color Categories

**Category A: Structure** - Blue `#569CD6`
- `[@]` Package declaration
- `[|]` Pipeline definition
- `[M]` Macro definition
- `[X]` Block terminator

**Category B: Enumeration** - Yellow `#DCDCAA`
- `[#]` Enumeration definition
- `#EnumName` - Enumeration identifiers

**Category C: Errors** - Light Red `#F48771`
- `[!]` Error definition/handling
- `!ErrorName` - Error identifiers

**Category D: Flow Control** - Purple `#C586C0`
- `[r]` Sequential execution
- `[p]` Parallel execution
- `[Y]` Join operation
- `[b]` Break/Continue

**Category E: Data Flow** - Teal `#4EC9B0`
- `[<]` Input assignment
- `[>]` Output extraction
- `[i]` Input declaration
- `[o]` Output declaration
- `[i]` Constant declaration

**Category F: Conditionals** - Orange `#CE9178`
- `[?]` Conditional block
- `[t]` Trigger declaration
- `[Q]` Queue control

**Category G: Boolean Logic** - Bright Cyan `#4FC1FF`
- `[&]` AND logic
- `[+]` OR logic
- `[-]` NOT logic
- `[^]` XOR logic
- `[.]` Continuation

**Category H: Macro Special** - Purple `#C586C0`
- `[W]` Wrapper declaration
- `[{]` Macro code start
- `[}]` Macro code end

**Category I: Nesting** - Gray `#808080`
- `[~]` Nesting indicator

#### Additional Token Colors

- **Variables** `.name` - Light cyan-blue `#9CDCFE`
- **Operators** `<<` `>>` `:` - Light gray `#D4D4D4`
- **Types** `pg\string` - Teal `#4EC9B0`
- **Strings** `"content"` - Brownish-orange `#CE9178`
- **Numbers** `42` `3.14` - Light green `#B5CEA8`
- **Comments** `//` `/* */` - Green `#6A9955`
- **Pipeline calls** `|PipelineName` - Yellow `#DCDCAA`
- **Boolean literals** `True` `False` - Blue `#569CD6`

### Grammar Implementation

#### TextMate Grammar (Initial Implementation)

**REQUIRED:** Polyglot editors MUST support TextMate-style syntax highlighting for broad compatibility.

**Scope Naming Convention:**
```
keyword.control.polyglot         # Flow control markers
keyword.other.polyglot           # Structure markers
entity.name.function.polyglot    # Pipeline names
entity.name.type.polyglot        # Enumeration/type names
variable.other.polyglot          # Variables
string.quoted.double.polyglot    # Strings
comment.line.polyglot            # Comments
```

#### Tree-sitter Grammar (Future Enhancement)

**RECOMMENDED:** Advanced editors SHOULD support Tree-sitter for:
- Context-aware highlighting
- Better performance on large files
- Integration with other editor features (folding, navigation)

**Timeline:** Tree-sitter support planned for PFG v1.1+

### Multi-Theme Support

**ALLOWED:** Editor plugins MAY provide alternative color schemes (Monokai, Solarized, etc.)

**REQUIRED:** All themes MUST maintain the category grouping (similar blocks → similar colors)

**Example Alternative Themes:**
- Monokai
- Solarized Dark/Light
- GitHub Dark/Light
- Dracula
- Nord
- One Dark Pro

### Light Mode Variant

**RECOMMENDED:** Dark mode colors provided above. Light mode variants SHOULD maintain similar hue relationships with adjusted brightness for readability on light backgrounds.

---

## PFG-004: Editor Integration

### Language Server Protocol (LSP)

Polyglot editor support is built on the Language Server Protocol for cross-editor compatibility.

#### Tier 1: MVP Features (v1.0)

**REQUIRED for basic editor support:**

1. **Diagnostics**
   - Syntax errors (malformed block markers)
   - Block matching validation (every `[|]` has matching `[X]`)
   - Missing required fields (trigger `[t]`, error `.message`/`.code`/`.trace`)
   - Variable naming rule violations
   - Type validation

2. **Auto-completion**
   - Block markers: `[|]`, `[r]`, `[<]`, etc.
   - Variable names (triggered by `.`)
   - Pipeline names (triggered by `|`)
   - Enumeration/Error references (triggered by `#` and `!`)
   - Type names (`pg\`, `py\`, etc.)

3. **Document Symbols (Outline View)**
   - Show all pipelines, enumerations, errors, macros
   - Hierarchical structure (file → definitions → inputs/outputs)

4. **Formatting**
   - Apply spacing rules (3 blank lines, 1 before branches)
   - Validate no indentation
   - Line length warnings
   - Spacing around operators

5. **Block Marker Visualization**
   - Highlight matching block pairs (click `[|]` highlights its `[X]`)
   - Visual nesting indicators
   - `[~]` depth calculation

#### Tier 2: Enhanced Features (v1.1+)

**RECOMMENDED for good developer experience:**

6. **Hover Information**
   - Pipeline signatures (show inputs/outputs)
   - Enumeration field values
   - Error structure
   - Type information

7. **Go-to-Definition**
   - Jump from pipeline call → pipeline definition
   - Jump from enum reference → enum definition
   - Jump from variable usage → declaration

8. **Polyglot-Specific Block Helpers**
   - Bracket matching-style for block markers
   - Subtle vertical guides showing block scope
   - Minimap markers for major definitions

#### Tier 3: Advanced Features (v2.0+)

**OPTIONAL for comprehensive IDE experience:**

9. **Find References**
10. **Rename Refactoring**
11. **Code Actions (Quick Fixes)**
12. **Debugger Integration** (future)
13. **Visual Pipeline Editor** (future)

### Code Snippets

**REQUIRED:** Editor plugins MUST provide these standard snippets:

1. **`pipe`** - Full pipeline template
```polyglot
[|] ${1:PipelineName}
[i] ${2:#None}
[t] |T.Call
[W] |W.Polyglot.Scope

$0
[X]
```

2. **`enum`** - Enumeration definition
```polyglot
[#] ${1:EnumName}
[<] .${2:field}: ${3:pg\string} << ${4:"value"}
[X]
```

3. **`err`** - Error definition
```polyglot
[!] !${1:ErrorName}
[<] .message: pg\string << "${2:Error message}"
[<] .code: pg\int << ${3:1000}
[<] .trace: pg\string << ""
[X]
```

4. **`if`** - Conditional block
```polyglot
[?] ${1:.condition} ?> ${2:value}
[~]
[~][r] $0
```

5. **`par`** - Parallel execution
```polyglot
[p] |${1:PipelineName}
[<] .${2:input}: ${3:type} << ${4:value}
[>] .${5:output} >> ${6:variable}
```

### File Association

**Language ID:** `polyglot`
**File Extension:** `.pg`
**MIME Type:** `text/x-polyglot`

**File Icon:** Use logo from `Polyglot Logo/` directory in repository

### Editor-Specific Integration

Editors MAY provide additional features beyond LSP:
- Folding providers (collapse/expand blocks)
- Breadcrumb trail (show current hierarchy)
- Status bar indicators (nesting depth, current pipeline)
- Custom color pickers for theme editing

---

## PFG-005: Tooling Standards

### Unified Tooling: `polyglot-tools`

**REQUIRED:** The official Polyglot tooling suite MUST be provided as a single binary with subcommands.

**Tool Name:** `polyglot-tools`

**Subcommands:**
```bash
polyglot-tools fmt check <file>     # Lint only (no changes)
polyglot-tools fmt format <file>    # Format in-place
polyglot-tools lsp                  # Start LSP server
polyglot-tools doc <file>           # Generate documentation
```

**Rationale:**
- Single installation for users
- Shared parsing/analysis code
- Consistent versioning
- Written in Rust (native Polyglot implementation language)

### Configuration: `polyglot.toml`

**REQUIRED:** Configuration MUST use TOML format in a file named `polyglot.toml`

**Location:** Project root directory

**Example Configuration:**
```toml
[format]
line_length = 99              # Max line length (default: 99, range: 79-120)
check_spacing = true          # Enforce spacing rules
context_aware_length = true   # Different limits for comments/code/strings

[lint]
block_matching = "error"      # Ensure [|] has matching [X]
trigger_required = "error"    # All pipelines must have [t]
unused_variables = "warn"     # Warn on declared but unused variables
missing_types = "error"       # All variables must have type annotations
max_nesting_depth = 4         # Warning if deeper than 4 levels

[lint.rules]
E001.severity = "error"       # Block matching errors always fail
W001.severity = "warn"        # Spacing warnings in dev
W101.severity = "ignore"      # Ignore unused variable warnings

[editor]
format_on_save = true
```

### Error Codes and Severity

#### Error Code Structure

Errors use **numbered codes** (Rust-style):
- **E001-E099:** Structural errors
- **E100-E199:** Syntax errors
- **E200-E299:** Semantic errors
- **W001-W099:** Style warnings
- **W100-W199:** Best practice warnings

#### Category 1: Structural Errors (E001-E099)

- **E001:** Missing closing `[X]` for block
- **E002:** Mismatched block markers
- **E003:** Block marker not at line start
- **E004:** Invalid block marker character

#### Category 2: Syntax Errors (E100-E199)

- **E101:** Invalid variable name (must start with `.`)
- **E102:** Invalid variable characters (only `a-z`, `_`, `.`)
- **E103:** Invalid pipeline name (must start with `|`)
- **E104:** Consecutive dots in variable name (`..`)

#### Category 3: Semantic Errors (E200-E299)

- **E201:** Missing required trigger `[t]`
- **E202:** Missing required error fields (`.message`, `.code`, `.trace`)
- **E203:** Undefined variable reference
- **E204:** Undefined pipeline reference

#### Category 4: Style Warnings (W001-W099)

- **W001:** Missing 3 blank lines between definitions
- **W002:** Missing 1 blank line before branch
- **W003:** Line too long (exceeds configured limit)
- **W004:** Inconsistent spacing around operators

#### Category 5: Best Practices (W100-W199)

- **W101:** Unused variable declaration
- **W102:** Variable shadowing
- **W103:** Missing output declaration `[o]`
- **W201:** Nesting depth exceeds recommended maximum

### Auto-Fix Capabilities

#### Safe Auto-Fixes (Default)

**REQUIRED:** `polyglot-tools fmt format` MUST apply these fixes automatically:

- ✅ W001: Add missing blank lines between definitions
- ✅ W002: Add missing blank line before branches
- ✅ W004: Fix spacing around operators (`<<`, `>>`, `:`)
- ✅ W003: Break long lines at logical points
- ✅ Remove trailing whitespace

#### Unsafe Fixes (Require Flag)

**OPTIONAL:** `polyglot-tools fmt format --unsafe` MAY apply these fixes:

- ⚠️ E201: Add missing `[t] |T.Call` (might not be correct trigger type)
- ⚠️ E202: Add skeleton error fields (might not be correct values)
- ⚠️ W101: Remove unused variables (might break future code)

#### Manual Fixes Required

**NEVER auto-fix:**

- ❌ E001: Missing `[X]` (tool can't determine correct location)
- ❌ E203: Undefined variable (can't guess intent)
- ❌ E204: Undefined pipeline (needs manual implementation)

### Format Skip Annotation

**ALLOWED:** Developers MAY skip formatting for specific blocks using:

```polyglot
// @polyglot-fmt-ignore
[|] SpecialFormattedPipeline
[r] .x: pg\int << 1  // Intentional custom formatting
[X]
```

**IMPORTANT:** Skip annotation only applies to **valid** Polyglot code. Invalid syntax will still be flagged as errors.

### Output Formats

**REQUIRED:** `polyglot-tools fmt check` MUST support multiple output formats:

1. **Pretty (default)** - Human-readable terminal output
```
Error[E001]: Missing closing [X] for pipeline block
  --> file.pg:42:5
   |
42 | [|] ProcessData
   | ^^^ block opened here, but never closed
   |
   = help: Add [X] at the end of the pipeline
```

2. **GCC-style** - For CI/tooling integration
```
file.pg:42:5: error[E001]: Missing closing [X] for pipeline block
file.pg:15:1: warning[W003]: Line exceeds 99 characters (105 chars)
```

3. **JSON** - For programmatic use
```json
{
  "errors": [
    {
      "code": "E001",
      "severity": "error",
      "message": "Missing closing [X] for pipeline block",
      "file": "file.pg",
      "line": 42,
      "column": 5
    }
  ]
}
```

**Flag:** `--format=<pretty|gcc|json>`

### Performance: Caching

**RECOMMENDED:** `polyglot-tools` SHOULD cache lint results for performance.

**Implementation:**
- Cache location: `.polyglot-cache/` (added to `.gitignore`)
- Cache key: File content hash + tool version
- Invalidate on: File change or config change
- CLI flag: `--no-cache` to disable

**Expected Performance:**
```
First run:  Checked 1000 files in 2.5s
Cached run: Checked 1000 files in 0.3s (998 cached)
```

### CI/CD Integration

#### Pre-commit Hooks

**PROVIDED:** Polyglot tools MUST include `.pre-commit-hooks.yaml`:

```yaml
- id: polyglot-fmt-check
  name: Polyglot Format Check
  entry: polyglot-tools fmt check
  language: system
  files: \.pg$

- id: polyglot-fmt-format
  name: Polyglot Auto-format
  entry: polyglot-tools fmt format
  language: system
  files: \.pg$
```

#### GitHub Actions Example

```yaml
name: Polyglot Lint

on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install polyglot-tools
        run: cargo install polyglot-tools
      - name: Check formatting
        run: polyglot-tools fmt --check --strict .
```

**Strict Mode:** `--strict` flag treats warnings as errors in CI

---

## PFG-006: Documentation Generation

### Markdown Format

**RECOMMENDED:** Documentation generators SHOULD produce Markdown format.

### Structure-Based Generation

Documentation is generated from block markers + comments:

**Source Code:**
```polyglot
// Processes input data and returns formatted result.
// This pipeline handles file validation and transformation.
[|] ProcessData
[i] .input_file: pg\path
[i] Default .timeout: pg\int << 30
[t] |T.File.Modified
[o] .result: pg\string
[X]
```

**Generated Markdown:**
```markdown
## Pipeline: ProcessData

Processes input data and returns formatted result.
This pipeline handles file validation and transformation.

**Inputs:**
- `.input_file: pg\path` - Path to input file
- `.timeout: pg\int` (default: 30) - Timeout in seconds

**Trigger:** File.Modified

**Outputs:**
- `.result: pg\string` - Processed result
```

### Auto-Doc Command

```bash
polyglot-tools doc file.pg > docs/api.md
polyglot-tools doc --format html file.pg > docs/api.html
```

**Status:** Future enhancement - format defined, implementation TBD

---

## PFG-007: Code Scaffolding

### Standard Templates

**DEFINED:** PFG specifies standard templates for common patterns.

**Implementation:** CLI tools are OPTIONAL, but templates are the specification.

### Template: Basic Pipeline

```polyglot
[|] ${PipelineName}
[i] ${InputDeclaration}
[t] |T.Call
[W] |W.Polyglot.Scope

[r] ${Operation}
[o] ${OutputDeclaration}
[X]
```

### Template: Error Handler

```polyglot
[|] ${PipelineName}
[i] ${Inputs}
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |${OperationThatMightFail}
[<] ${InputParams}
[>] ${Outputs}
[~]
[~][!] !${ErrorType}
[~][>] .message: pg\string >> ${ErrorVariable}
[~]
[~]// Recovery logic
[~][r] ${FallbackOperation}

[o] ${Outputs}
[X]
```

### Template: Parallel Execution

```polyglot
[|] ${PipelineName}
[i] ${Inputs}
[t] |T.Call
[W] |W.Polyglot.Scope

[r] ${ResultVariable}: ${Type} << ${DefaultValue}

[p] |${Pipeline1}
[<] ${Input1}
[>] ${Output1} >> ${ResultVariable1}

[~]

[p] |${Pipeline2}
[<] ${Input2}
[>] ${Output2} >> ${ResultVariable2}

[Y] |Y.Join
[>] ${ResultVariable1}
[>] ${ResultVariable2}

[o] ${FinalOutput}
[X]
```

---

## Future Enhancements

These features are planned for future PFG versions but not required for v1.0:

### Theme Marketplace (PFG v1.2+)

- Community-contributed color schemes
- Rating/voting system
- Integration with editor plugin managers

### Code Metrics Dashboard (PFG v2.0+)

- Track code complexity over time
- Show "code health" score
- Identify refactoring candidates
- Pipeline dependency graphs

### Advanced IDE Features (PFG v2.0+)

- Debugger integration (breakpoints at block markers)
- Visual pipeline flow editor
- Interactive execution visualization
- Performance profiling integration

### AI-Assisted Features (Future)

- Suggest refactorings
- Auto-generate documentation descriptions
- Detect code smells
- Pipeline optimization suggestions

---

## Appendices

### Appendix A: Complete Color Reference

See [Syntax Highlighting Preview](../polyglot-syntax-darkmode-preview.html) for visual examples.

#### Dark Mode Complete Palette

| Element | Category | Color | Hex Code |
|---------|----------|-------|----------|
| `[@]` `[|]` `[M]` `[X]` | Structure | Blue | #569CD6 |
| `[#]` Marker | Enumeration | Yellow | #DCDCAA |
| `#EnumName` | Enumeration ID | Yellow | #DCDCAA |
| `[!]` Marker | Error | Light Red | #F48771 |
| `!ErrorName` | Error ID | Light Red | #F48771 |
| `[r]` `[p]` `[Y]` | Flow Control | Purple | #C586C0 |
| `[<]` `[>]` `[i]` `[o]` `[i]` | Data Flow | Teal | #4EC9B0 |
| `[?]` `[t]` `[Q]` | Conditionals | Orange | #CE9178 |
| `[&]` `[+]` `[-]` `[^]` | Boolean Logic | Cyan | #4FC1FF |
| `[W]` `[{]` `[}]` | Macro Special | Purple | #C586C0 |
| `[~]` | Nesting | Gray | #808080 |
| `.variable` | Variables | Light Cyan | #9CDCFE |
| `<<` `>>` `:` | Operators | Light Gray | #D4D4D4 |
| `pg\string` | Types | Teal | #4EC9B0 |
| `"string"` | Strings | Orange | #CE9178 |
| `42` | Numbers | Light Green | #B5CEA8 |
| `// comment` | Comments | Green | #6A9955 |
| `|Pipeline` | Pipeline Calls | Yellow | #DCDCAA |
| `True` `False` | Booleans | Blue | #569CD6 |

### Appendix B: Error Code Quick Reference

#### Errors (E001-E299)

**Structural (E001-E099):**
- E001: Missing closing [X]
- E002: Mismatched block markers
- E003: Block marker not at line start
- E004: Invalid block marker character

**Syntax (E100-E199):**
- E101: Invalid variable name
- E102: Invalid variable characters
- E103: Invalid pipeline name
- E104: Consecutive dots in variable

**Semantic (E200-E299):**
- E201: Missing required trigger
- E202: Missing required error fields
- E203: Undefined variable reference
- E204: Undefined pipeline reference

#### Warnings (W001-W199)

**Style (W001-W099):**
- W001: Missing 3 blank lines
- W002: Missing 1 blank line before branch
- W003: Line too long
- W004: Inconsistent spacing

**Best Practices (W100-W199):**
- W101: Unused variable
- W102: Variable shadowing
- W103: Missing output declaration
- W201: Excessive nesting depth

### Appendix C: Example Code

See `docs/user/examples/` directory for comprehensive examples demonstrating:
- [Hello World Examples](../user/examples/hello-world.md)
- [Approved Examples](../user/examples/approved-examples.md)

All examples follow PFG v1.0 specifications.

---

## Version History

- **v1.0.0** (2025-11-21) - Initial draft specification

---

## Contributing

To propose changes to PFG:

1. Open an issue in the Polyglot repository
2. Discuss rationale and impact
3. Submit pull request with specification changes
4. Update appendices and examples to match

PFG changes require consensus from Polyglot core team and community review.

---

## License

This specification is part of the Polyglot project and follows the same license terms.

---

**End of Polyglot Formatting Guidelines v1.0**
