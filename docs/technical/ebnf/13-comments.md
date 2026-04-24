---
audience: design
type: spec
updated: 2026-04-24
---

<!-- @ebnf/INDEX -->

## 13. Comments

```ebnf
comment_line        ::= "[ ]" comment_text ;
comment_curly       ::= "{ }" comment_text ;

comment_text        ::= { any_char } ;
```

**Rule:** A bracket containing only whitespace is always a comment: `[ ]` or `{ }`.

---
