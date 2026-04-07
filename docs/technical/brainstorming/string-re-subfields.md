---
audience: developer
type: brainstorming
updated: 2026-04-05
---

# Brainstorm: String RE Subfield System

**Status:** Decided (2026-03-20)
**Decision:** See [plan/decisions/string-re-subfields.md](../plan/decisions/string-re-subfields.md)

## Summary of Decisions

1. **RawString** is the true primitive (compiler intrinsic) — literal raw chars, no interpolation
2. **#String** is a struct: `.string#RawString` + `.regex#RawString`
3. **int/float** are flexible subtypes of `#String` with pre-set `.regex` patterns
4. **bool** (`#Boolean`) is a ##Enum type, NOT a `#String` subtype
5. **RE declaration** uses standard subfield assignment — no new grammar
6. **Validation** — compile-time for literals (PGE04010), runtime for dynamic values
7. **RawString literals** — `=RawString"..."` or `=rs"..."`
8. **Custom string subtypes** — users define `{#} #String.customName` with their own `.regex`

## Deferred

- ~~Metadata Data Tree~~ — **Decided**: full `%#:String:*` path documentation. See [plan/decisions/metadata-data-tree.md](../plan/decisions/metadata-data-tree.md)
- RE composition across structs/arrays
- `eng`/`sci` RE patterns (proposed, not decided)
- `WindowsPath`/`UnixPath` RE patterns
