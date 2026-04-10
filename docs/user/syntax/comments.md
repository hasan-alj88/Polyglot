---
audience: pg-coder
type: specification
updated: 2026-03-18
status: draft
---

# Comments

<!-- @c:blocks -->
<!-- @u:line-structure -->
Bracket comments replace `//` entirely. Comment brackets follow the same [[line-structure]] rules (3-space indentation, one per line). See [[blocks#Comments]] for the `[ ]` block element and [[blocks]] `{ }` for definition-level comments.

| Syntax | Type | Context |
|--------|------|---------|
| `[ ] text` | Single-line comment (square bracket) | Inside any block |
| `{ } text` | Single-line comment (curly bracket) | Between definitions (top level) |
| `[ ]<` ... `[ ]>` | Multi-line comment block | Inside any block |
| `( ) text` | IO comment (round bracket) | Inside `(X)` IO blocks only |

A bracket with only whitespace inside is always a comment.

## Warning Suppression

Place `[ ] Ignore PGW-NNN` on the line immediately above a statement that produces a compiler warning. The compiler will suppress that specific warning for the next line only.

- Only works for **warnings** (`PGW-NNN` codes). Cannot suppress **errors** (`PGE-NNN`).
- One `Ignore` per warning. Multiple warnings on the same line need multiple `Ignore` comments.

```polyglot
[ ] Ignore PGW03001
[b] -Audit.Log
   (-) <event << $event            [ ] no warning — suppressed

[ ] Ignore PGW02001
[-] -LogStatus
   (-) <status << >label           [ ] PGW02001 suppressed
```
