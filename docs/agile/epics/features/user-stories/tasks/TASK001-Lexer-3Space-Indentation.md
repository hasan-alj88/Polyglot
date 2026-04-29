---
user-story: "US001-Lexer-Line-Parsing"
github-issue-link: "https://github.com/hasan-alj88/Aljam3/issues/363"
status: "done"
assignee: "@developer"
---
# Task: Implement 3-Space Indentation Scope Tracker

## Instructions
1. Build the initial file reader/iterator.
2. For each line, count the leading whitespaces.
3. Assert that the count `X % 3 == 0`. Else throw a compile error indicating invalid spacing.
4. Manage an internal depth property (e.g. `X / 3`). Emit a "scope increase" or "scope decrease" token based on changes to this depth between lines.
