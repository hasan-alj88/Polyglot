---
issue: 155
group: 2
group_name: "Syntax Documentation Gaps"
priority: P2-high
status: brief-ready
---

# Issue #155: Metadata path syntax gap between user and technical docs

## Inconsistency
User docs (metadata.md, identifiers.md) show simple shorthand accessors like `$name%state` and `=Pipeline%status` with no explanation of the underlying path grammar. The technical spec (metadata-tree/path-grammar.md) defines a complex formal grammar with instance addressing (`%{type_prefix}:{ref}:{instance}.{fields}`), but this full syntax is never surfaced in user docs. The path-grammar.md does have a "Shorthand in User Code" section showing the resolution (e.g., `=MyPipeline%status` resolves to `%=:MyPipeline:<current>.status`), but this bridging information exists only in the technical spec that pg-coder audience docs never reference directly. A user reading only the SPEC-INDEX learning path would have no idea that `%=:MyPipeline:0.status` is a valid path or what `:<current>` means.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/user/concepts/metadata.md` | Shows `=Name%status`, `$name%state` shorthand only; no mention of full instance-qualified paths or the `:{instance}` segment |
| `docs/user/syntax/identifiers.md` | Lists `%` as "Metadata accessor" with example `=Pipeline%status` but no link to the path grammar |
| `docs/technical/spec/metadata-tree/path-grammar.md` | Full grammar exists here but is audience-gated to `[architect, designer]`; user docs never bridge to it |
| `docs/user/concepts/data-is-trees.md` | Shows tree branches like `%=:ProcessData:0` with instance numbers but never explains the `:0` addressing syntax |

## Example
**Source A** (`docs/user/concepts/metadata.md`, line ~68):
> | `=Name%status` | `#live.#PipelineStatus` | AwaitTrigger, Disabled, Running, Failed |

**Source B** (`docs/technical/spec/metadata-tree/path-grammar.md`, line ~17):
> instance_path   ::= "%" type_prefix ":" ref ":" instance { "." field }

**Source C** (`docs/technical/spec/metadata-tree/path-grammar.md`, line ~51):
> | `=MyPipeline%status` | `%=:MyPipeline:<current>.status` |

**Source D** (`docs/user/concepts/data-is-trees.md`, line ~67):
> | Struct types | [[syntax/types/structs#Struct Types]] | `%#` | `%#:UserRecord:0` |

## Prior Related Work
- Issue #131 -- Permission branch `%_` path grammar exception fix (closed 2026-04-05). Split `type_prefix` into standard and exception branches; established that `%_`, `%!`, `%@` have their own grammar rules.
- Issue #82 -- Added `%_` metadata tree branch for permissions. Extended path grammar with permission paths.

## Recommendation
Add an "Advanced Metadata Paths" section to `docs/user/concepts/metadata.md` (or as a subsection of data-is-trees.md) that bridges the simple shorthand to the full grammar. This section should: (1) show the full path pattern `%{type_prefix}:{ref}:{instance}.{field}`, (2) explain the `:<current>` implicit resolution, (3) give examples for each prefix, and (4) link to `technical/spec/metadata-tree/path-grammar.md` for the formal EBNF. This fills the gap without duplicating the technical spec -- users get enough to understand instance-qualified paths when they encounter them in error messages or debugging output.

## Discussion Prompts
1. Should the full path grammar be presented in user docs, or just the shorthand-to-full mapping table from path-grammar.md?
2. Does data-is-trees.md's use of `%#:UserRecord:0` without explanation create confusion for new users, or is it acceptable as a "preview" of the tree structure?
3. Is instance addressing (`:<current>`, `:0`, `:1`) a concept users ever need to write explicitly, or is it always implicit/compiler-managed?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 155*
