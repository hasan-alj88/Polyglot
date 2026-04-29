---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: Validation CLI Integration

## Description
Integrate the EBNF Compiler Validation phase into the `aljam3` CLI application to allow users to invoke the validator on `.aj3` files.

## Instructions
1. Update `lib/aljam3/src/main.rs` to accept a new CLI argument: `--validate`.
2. When `--validate` is passed, the CLI should:
   - Run the existing lexer to generate the `Aljam3Token` stream.
   - Pass the token stream directly into the EBNF validation engine.
   - Evaluate the returned `ValidationReport`.
   - If `total_errors == 0`, print a success message (`"Validation Passed"`).
   - If `total_errors > 0`, invoke the Error Formatting Display (TASK011) to print the errors to the console.
   - Exit with a non-zero exit code (`std::process::exit(1)`) if validation fails.

## Acceptance Criteria
- [ ] `aljam3 --validate -c <file.aj3>` successfully runs the entire pipeline (Lexer -> EBNF -> Report -> Display).
- [ ] Passing a perfectly valid file exits with status code `0` and a success message.
- [ ] Passing an invalid file triggers the formatted PGE error output and exits with status code `1`.
