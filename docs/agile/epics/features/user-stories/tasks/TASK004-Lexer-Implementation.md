---
user-story: "US001-Lexer-Line-Parsing"
github-issue-link: "https://github.com/hasan-alj88/Polyglot/issues/365"
status: "todo"
assignee: "@developer"
---

# Task: Implement Rust Lexer Pipeline

## Description
Translate the lexical definitions, whitespace rules, prefix mapping, and token extraction regexes (from `TASK003`) into a functional Rust tokenization pipeline. The lexer should consume a raw `&str` script and output an ordered stream of tokens without applying semantic validation (validation belongs to the Compiler).

## Instructions

1. **Setup Token Enums**
   - In the `/lib/polyglot/src/lexer` module, create the `PolyglotToken` enumeration mapping out all components from the Token Catalog.
   - Use standard data-carrying Enums for tokens containing extracted values (e.g., `Data(String)`, `Variable(String)`, `LiteralInt(i64)`).

2. **Implement Tokenizer Core (Line-by-Line EBNF Engine)**
   - Abandon the standard infinite character-loop tokenization model. Instead, the lexer enforces Polyglot's rigid grammatical topology by executing a 4-phase sequential algorithm on every line until EOF:
     1. **Indent Phase:** Calculate leading spaces. Yield a `TOK_INDENT` for every 3 contiguous spaces.
     2. **Marker Phase:** Identify the primary line structural marker (e.g. `[-]`, `{#}`, `(>)`) bridging the indent to the expression. Yield the corresponding Marker token.
     3. **Expression Phase:** Pass the remaining substring into the EBNF Composite Regex matching engine (from `TASK003`). The matcher evaluates the expression as a pattern sequence and yields the remaining token(s) and identifiers synchronously.
     4. **Newline Phase:** Yield `TOK_NEWLINE` and advance to the next line.
3. **Handle Locational Metadata**
   - Wrap the resulting tokens in a `Spanned<T>` struct tracking the logical File Line and Column offsets, which are necessary for the AST and compiler error reporting.
   - Note: A raw byte offset is insufficient; line & column counting is strictly required.

4. **Implement Fallback/Error Recovery**
   - Ensure the lexer processes to `EOF` without panics or crashes.
   - Any character or sequence that does not match a valid Polyglot rule should be swallowed into a `TOK_UNRECOGNIZED(String)` variant and passed along the stream. The parser/compiler step will eventually halt and report it safely.

5. **Unit Tests**
   - Write comprehensive unit tests validating string payloads for Monolithic Identifiers without their prefixes.
   - Validate whitespace rule behavior surrounding `{`, `[`, and `#`.

## Acceptance Criteria
- [ ] `PolyglotToken` enum accurately matches the catalog in `TASK003`.
- [ ] Prefixes for monolithic identifiers are stripped during the extraction step.
- [ ] The lexer correctly tracks coordinates (`Line` / `Col`).
- [ ] Unrecognized garbage characters emit `TOK_UNRECOGNIZED` and don't panic the lexer thread.
- [ ] Core unit tests for extraction regex compliance are passing.
