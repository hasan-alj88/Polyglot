# Brainstorm: String RE Subfield System

**Status:** Decided (2026-03-20)
**Decision:** See [plan/decisions/string-re-subfields.md](../plan/decisions/string-re-subfields.md)

## Summary of Decisions

1. **RawString** is the true primitive (compiler intrinsic) — literal raw chars, no interpolation
2. **#String** is a struct: `.string;RawString` + `.re;RawString`
3. **int/float** are flexible subtypes of `#String` with pre-set `.re` patterns
4. **bool** (`#Boolean`) is a separate enum struct, NOT a `#String` subtype
5. **RE declaration** uses standard subfield assignment — no new grammar
6. **Validation** — compile-time for literals (PGE-410), runtime for dynamic values
7. **RawString literals** — `=RawString"..."` or `=rs"..."`
8. **Custom string subtypes** — users define `{#} #String.customName` with their own `.re`

## Deferred

- ~~Metadata Data Tree~~ — **Decided**: full `%#:String:*` path documentation. See [plan/decisions/metadata-data-tree.md](../plan/decisions/metadata-data-tree.md)
- RE composition across structs/arrays
- `eng`/`sci` RE patterns (proposed, not decided)
- `WindowsPath`/`UnixPath` RE patterns
