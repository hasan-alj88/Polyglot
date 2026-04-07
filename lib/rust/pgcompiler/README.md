# Rust — PGCompiler

The Polyglot compiler implemented in Rust. Handles lexing, parsing, and compile rule enforcement for `.pg` files.

## Components

| Component | Description |
|-----------|-------------|
| Lexer | Tokenizes `.pg` source files into a token stream |
| Parser | Builds AST from token stream following EBNF grammar |
| Compile Rules | Enforces PGE (error) and PGW (warning) rules against the AST |
| Registry Builder | Collects `{N}` definitions and emits the native registry |

## Not a Runtime Dispatcher

The PGCompiler does not dispatch native operations at runtime. It reads `{N}` definitions at compile time to:
- Validate mutual exclusion (PGE01028)
- Validate language bindings against config
- Build the native registry consumed by TM, QH, and Runner at startup

See `docs/technical/spec/native-dispatch.md#Subsystem Architecture` for details.
