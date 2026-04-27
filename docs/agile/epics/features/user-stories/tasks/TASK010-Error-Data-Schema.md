---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: Error Data Schema & Aggregation

## Description
Implement the core data structures (`ValidationError` and `ValidationReport`) to capture and aggregate EBNF compliance violations in a standardized format.

## Instructions
1. Define the `ValidationError` struct containing `code`, `name`, `message`, `line`, `col`, and `snippet`.
2. Define the `ValidationReport` struct containing `status`, `total_errors`, `file`, and a `Vec<ValidationError>`.
3. Ensure the validator performs a linear pass over the token stream, accumulating all discovered errors into the `ValidationReport` rather than halting on the first error.
4. Export the data as a structured JSON object or native Rust struct for downstream CLI display formatters.

## Acceptance Criteria
- [ ] Multiple errors across different lines are successfully aggregated into a single report.
- [ ] The schema matches the agreed upon JSON structure.
- [ ] The compiler gracefully continues checking subsequent lines after catching an error.
