---
issue: 147
group: 5
group_name: "jm3lib Classification"
priority: P3-medium
status: brief-ready
---

# Issue #147: ForEach.Level uses ~~ double-prefix, violating single ~ rule

## Inconsistency
The EBNF grammar defines expand invocations as using a single `~` prefix: `expand_invocation ::= '~' expand_operator`. All other ForEach operators follow this rule cleanly -- `~ForEach.Array`, `~ForEach.Map`, `~ForEach.Serial` each use one `~` and their IO lines use standard input paths. However, `~ForEach.Level` introduces a second `~` as a suffix on the input path: `<level << #SomeData.SubField.~`. This means the operator's usage contains two tilde characters (`~ForEach.Level` + `.~`), creating an implicit "double-tilde" pattern that no other expand operator exhibits. The `.~` suffix has no EBNF production rule -- it is described only in prose as "the `~` suffix on the input path marks the iteration point" but is not formalized in the grammar.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/ebnf/12-collections.md` | Defines `expand_invocation ::= '~' expand_operator` (single prefix) but describes `.~` suffix in prose only (line ~36) with no grammar production |
| `docs/user/jm3lib/expanders/ForEach/Level.md` | Documents `<level << #SomeData.SubField.~` with `.~` suffix but does not explain the grammatical basis |
| `docs/user/concepts/collections/expand.md` | Lists `~ForEach.Level` in the operator table with `<level` input but does not mention the `.~` suffix in the table; only describes it in the subsection below |
| `docs/technical/edge-cases/12-collections.md` | EC-12.4 tests the tilde suffix but calls it "special input syntax" without EBNF formalization |

## Example
**Source A** (`docs/technical/ebnf/12-collections.md`, line ~18):
> expand_invocation   ::= '~' expand_operator ;

**Source B** (`docs/technical/ebnf/12-collections.md`, line ~36):
> **`~ForEach.Level` special input syntax:** The `~` suffix on input marks the iteration point: `<level << #SomeData.SubField.~`

**Source C** (`docs/user/jm3lib/expanders/ForEach/Level.md`, lines ~25-26):
> ```polyglot
> [r] ~ForEach.Level
>    [~] <level << #SomeData.SubField.~

**Source D** (`docs/user/concepts/collections/expand.md`, line ~24):
> | `~ForEach.Level` | Siblings at a specified level only | `<level`, `>key`, `>item` |

(Note: the table shows `<level` without the `.~` suffix, while all code examples include it)

## Prior Related Work
- Issue #89 -- Added `~ForEach.Map` and `*Into.Map`; established the expand/collect operator naming pattern with single `~` prefix
- Issue #94 -- Macro-for-generics redesign that removed `~ForEach.Dataframe.Column`; clarified operator classification boundaries
- Issues #99-#106 -- EBNF edge cases batch; formalized syntax rules for previously prose-only features

## Recommendation
Determine whether the `.~` suffix on the input path is (a) a distinct syntactic element (a "level iteration marker" on paths) that needs its own EBNF production rule, or (b) a typo/design artifact that should be replaced with a different mechanism. If intentional, add a grammar production like `level_input_path ::= type_path '.' '~'` to the EBNF and document it as a path syntax feature separate from the expand operator prefix. If unintentional, redesign the level specification mechanism -- for example, using a numeric depth parameter or a named IO input instead of a path suffix.

## Discussion Prompts
1. Is the `.~` suffix a path syntax element (like `.` and `:` separators) or an operator-level construct that belongs in the expand grammar?
2. Should the expand operator table in `expand.md` include the `.~` suffix in the IO column for `~ForEach.Level`, or does the current `<level` notation correctly abstract it away?
3. Does the absence of an EBNF production for `.~` mean the compiler has no formal rule to parse it, or is it handled as a special case within `expand_io_line`?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 147*
