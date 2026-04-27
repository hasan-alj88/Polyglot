---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: Validation CLI Integration

## Description
Integrate the EBNF Compiler Validation phase into the `polyglot` CLI application to allow users to invoke the validator on `.pg` files.

## Instructions
1. Update `lib/polyglot/src/main.rs` to accept a new CLI argument: `--validate`.
2. When `--validate` is passed, the CLI should:
   - Run the existing lexer to generate the `PolyglotToken` stream.
   - Pass the token stream directly into the EBNF validation engine.
   - Evaluate the returned `ValidationReport`.
   - If `total_errors == 0`, print a success message (`"Validation Passed"`).
   - If `total_errors > 0`, invoke the Error Formatting Display (TASK011) to print the errors to the console.
   - Exit with a non-zero exit code (`std::process::exit(1)`) if validation fails.

## Acceptance Criteria
- [ ] `polyglot --validate -c <file.pg>` successfully runs the entire pipeline (Lexer -> EBNF -> Report -> Display).
- [ ] Passing a perfectly valid file exits with status code `0` and a success message.
- [ ] Passing an invalid file triggers the formatted PGE error output and exits with status code `1`.
