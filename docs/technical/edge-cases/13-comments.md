---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 13. Comments (S13)

### EC-13.1: Single-line square bracket comment

**EBNF:** `comment_line ::= "[ ]" comment_text`

```polyglot
[ ] This is a comment
```

### EC-13.2: Definition-level curly bracket comment

**EBNF:** `comment_curly ::= "{ }" comment_text`

```polyglot
{ } This is a top-level comment between definitions
```

### EC-13.3: Multiline comment block

**EBNF:** `multiline_comment ::= "[ ]<" NEWLINE { any_text NEWLINE } "[ ]>"`

```polyglot
[ ]<
This is a multiline comment.
It can span multiple lines.
No bracket prefix needed inside.
[ ]>
```
