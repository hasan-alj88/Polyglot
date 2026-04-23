---
user-story: "US001-Lexer-Line-Parsing"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: Implement Bracket Marker & Expression Isolation

## Instructions
1. After removing the leading 3-space multiple indentation, extract the starting marker.
2. Identify syntax `{}`, `[]`, or `()`. If the content is totally empty, discard the entire line as a comment.
3. Extract the inner string (`X`) as the `MarkerToken`.
4. Capture the remaining trailing string on the line strictly as the `ExpressionToken`.
5. Attach the line number and start/end column numbers to these emitted tokens for source mapping.
