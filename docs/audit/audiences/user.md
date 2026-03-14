---
type: audience-rules
audience: user
updated: 2026-03-14
---

# Audience: Users

<!-- @vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

End users who run Polyglot services and consume the outputs of automated workflows. They may not be programmers. They interact with Polyglot through its services, not its code.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Approachable, jargon-free, encouraging |
| Voice | Active voice, present tense, second person ("you") |
| Format | Short paragraphs, task-oriented, examples first |
| Structure | How-to focus: goal -> steps -> result |
| Vocabulary | Plain language. Define any technical term on first use via [[glossary]] |
| Code | Only show what users type/run. No internals |

## NEVER

- Expose compiler internals or AST details
- Assume programming language knowledge
- Use developer jargon without definition
- Show raw error traces — explain what went wrong and how to fix it
- Reference .pg file syntax (that's developer audience)

## ALWAYS

- Lead with what the user will achieve
- Include expected output/result for every example
- Provide troubleshooting for common failures
- Link to developer docs when deeper detail is needed (don't duplicate)
