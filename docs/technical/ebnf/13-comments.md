---
audience: design
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 13. Comments

```ebnf
comment_line        ::= "[ ]" comment_text ;
comment_curly       ::= "{ }" comment_text ;

multiline_comment   ::= "[ ]<" NEWLINE
                         { any_text NEWLINE }
                         "[ ]>" ;

comment_text        ::= { any_char } ;
```

**Rule:** A bracket containing only whitespace is always a comment: `[ ]` or `{ }`.

---
