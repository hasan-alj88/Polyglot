---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: Error Formatting Display

## Description
Implement a Rust-style visual error formatter that consumes the `ValidationReport` data payload and renders developer-friendly, actionable console output.

## Instructions
1. Parse the `ValidationReport` and iterate through the `violations`.
2. Implement a visual printer that generates the following structure for each error:
   - Header line displaying `error[PGE_CODE]: PGE_NAME`.
   - File path and location pointer ` --> filepath:line:col`.
   - A contextual ASCII art code block showing the `snippet` string.
   - An ASCII caret (`^`) pointing specifically to the `col` index underneath the snippet.
   - The detailed `message` appended to the caret line.
3. Ensure color-coding is applied if possible (e.g., Red for `error`, Blue for paths).

## Acceptance Criteria
- [ ] The CLI outputs errors in the exact visual structure approved in the research phase.
- [ ] Carets (`^`) correctly align with the specified `col` within the printed `snippet`.
- [ ] Multiple errors are printed sequentially with clear separation.
