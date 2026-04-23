---
github-issue-link: "#"
status: "planning"
assignee: "@product_owner"
dependencies: []
---
# Epic: Polyglot Lexer & AST Generator

**Audience**: Internal Agile Personas (Product Owner, Scrum Master, Development Team)

## Objective
Build a robust and efficient lexer that parses `*.pg` (Polyglot) code syntax and translates it into well-structured AST (Abstract Syntax Tree) JSONs. This forms the foundational layer for all subsequent compilation, analysis, and execution of Polyglot code.

## Research Findings: Impact of Strict Syntax Rules
**User Observation:** *All polyglot objects have identifiable prefixes, and all polyglot lines are in the form `{indent}{marker}{1 expression}`.*

**Analysis:** Do these constraints help the lexer and make its job easier? **Yes, significantly.** 

These intentional language design choices dramatically simplify lexical analysis and parsing compared to traditional language compilation:
1. **Line-Based Predictability:** The strict `{indent}{marker}{1 expression}` structure means the lexer can operate primarily line-by-line. Instead of scanning characters to find arbitrary end-of-statement delimiters (like `;`) or block closures (`}`), the line itself is the fundamental unit of operation.
2. **Trivial Scope Management:** The `{indent}` level directly dictates the AST hierarchy. Generating the nested AST JSON becomes a simple stack-push/pop operation governed completely by indentation count.
3. **Elimination of Backtracking:** Because "all polyglot objects have identifiable prefixes," the parser never has to guess what sort of object it is processing or backtrack if a guess is wrong. The prefix perfectly identifies the semantic category (type, variable, function, etc.) at the token level, eliminating the need for complex symbol table lookups during early tokenization.
4. **Isolated Complexity:** The only complex parsing required is constrained to the `{1 expression}` slot. The structural constraints (`{indent}{marker}`) ensure that the parser is already in exactly the right context when it evaluates that single expression.
5. **Linear Time Complexity $O(N)$:** This structure allows for a highly efficient single-pass recursive descent parser that can map directly to a JSON representation with minimal memory overhead.

## Scope
- **In Scope:**
  - Tokenizing `{indent}`, `{marker}`, and `{expression}` from `*.pg` files.
  - Resolving object classes based on their defined prefixes.
  - Generating and exporting structured, nested JSON AST files.
  - Returning informative lexical errors (line/column tracking).
- **Out of Scope:**
  - Code generation or compilation into target languages (handled downstream).
  - Semantic validation (e.g., type checking, checking if referenced variables exist) beyond lexical syntax.

## Features Breakdown & Pipeline Architecture
To ensure strict separation of concerns, the transformation of `*.pg` files follows an explicit timeline. This Epic is divided into features mapping directly to the 5 distinct architectural steps. Each feature will serve as a parent for executing specific development tasks.

### Feature 1: Lexer (Token Stream Generator)
- Scans `*.pg` source code files line by line (ignore entirely blank lines).
- Evaluates scope: Indentation is **strictly 3 spaces per level**. If an indentation is not a multiple of 3, immediately trigger a compile syntax error.
- Enforces syntactic boundaries: Explicitly isolates markers bound by `{X}`, `[X]`, or `(X)`.
- Handles Comments: If the bracket is empty (`{}`, `[]`, or `()`), treats the entire line as a comment and ignores it.
- Extracts the remaining `{1 expression}` safely.
- Emits a linear, predictable stream of primitive syntax tokens (Indentation changes, specific Marker tags, and raw Expression strings).
- Attaches source maps for meaningful syntax error messages (line number and column precision).

### Feature 2: AST Parser
- Consumes the Token Stream to build a hierarchical *Abstract Syntax Tree*.
- Represents structural parent-child relationships solely based on indentations and marker scopes.
- Contains no domain logic—only structural integrity (`[{scope_level, marker_type, expression_ast_node}]`).

### Feature 3: Compiler
- Traverses the raw structural AST layer.
- Applies domain-specific evaluation and resolves complex relationships between the structural markers.
- Evaluates the expressions deeper based on marker types and identifiable object prefixes.

### Feature 4: Behavioral Contract JSON Export
- Outputs the final formatted domain functionality.
- Maps the Compiler's analyzed objects into the strict JSON schema.
- For a Pipeline object, enforces the mapping of logical blocks into `Inputs`, `Outputs`, `Triggers`, `QueueJobRules`, `Setup`, `Execution`, and `Cleanup`.
