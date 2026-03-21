# Decision: Numeric RE Patterns

**Date:** 2026-03-20
**Status:** Decided

## Context

All Polyglot data is serialized strings. `int` and `float` types are described as "matched via regular expression patterns" but no formal patterns were specified.

## Decision

### RE Patterns

| Type | RE Pattern |
|------|-----------|
| `int` | `^-?[0-9]+$` |
| `float` | `^-?[0-9]+\.[0-9]+$` |

### Leading Zeros: Allowed (Permissive)

Leading zeros are accepted for both `int` and `float` (e.g., `007`, `00`, `003.14` are all valid). The simpler, more lenient pattern was chosen over strict no-leading-zeros (`^-?(0|[1-9][0-9]*)$`).

## Updated Files

- `docs/user/syntax/types.md` — Basic Types section
- `docs/technical/EBNF.md` — comments after `int_literal`/`float_literal`
