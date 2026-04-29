---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.1 Package Declaration

```ebnf
package_block       ::= "{@}" package_id NEWLINE
                         { indent import_line NEWLINE }
                         { indent comment_line NEWLINE } ;

import_line         ::= "[@]" '@' name push_left package_id ;
```

**Rule:** `{@}` must be the first block in every `.aj3` file. Exactly one `{@}` per file — multiple `{@}` blocks are not allowed. Multiple `{#}` and `{-}` definitions are allowed.

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.1 `{@}` Package | [[syntax/packages\|packages]] |
