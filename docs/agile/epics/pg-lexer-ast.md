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

## Features Breakdown
*Note: Due to the size of the lexer components, development will be split into manageable features. User stories and specific execution tasks will be determined and assigned once these features are finalized.*

### Feature 1: Core Line Reader & Indentation Tracker
- Scan `*.pg` source code files line by line.
- Extract leading whitespace/indentation perfectly to track scope.
- Manage an internal stack to properly open and close parent nodes in the AST structure based on indent increases/decreases.

### Feature 2: Tokenizer (Marker & Prefix Isolation)
- Identify and isolate the `{marker}` segment of the line.
- Extract the remaining `{1 expression}`.
- Identify known prefixes for Polyglot objects within the expression to rapidly tag their types without needing complex context.

### Feature 3: Expression Parser
- Evaluate the `{1 expression}` dynamically based on the preceding marker and token type.
- Handle literals, string captures, object references, and arithmetic/logic primitives, ensuring the predictable single-expression rule is upheld.

### Feature 4: AST JSON Serialization
- Accumulate the hierarchical nodes parsed from Features 1-3.
- Map the syntax constructs (Indent -> Hierarchy, Marker -> Node Type, Expression -> Values/Children) to an optimized JSON schema.
- Export to `.json` files.

### Feature 5: Error Handling & Source Mapping
- Provide meaningful syntax error messages pointing to the exact line number, column, and unexpected token.
- Handle corrupted indentation, invalid prefixes, or multiple expressions per line gracefully by failing fast to assist developers.
