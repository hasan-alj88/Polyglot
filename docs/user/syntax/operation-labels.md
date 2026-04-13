---
audience: automation-builder
type: specification
updated: 2026-04-09
status: draft
---

# Operation Labels

<!-- @c:blocks -->
<!-- @u:io -->
Operation labels attach a reusable name to a pipeline call's IO, allowing downstream operations to reference outputs, inputs, errors, and permissions without intermediate variables. `($)` belongs to the `(X)` IO bracket family described in [[blocks#Data Flow]] and follows all [[io]] parameter rules.

## Overview

A `($)` line inside a call block declares an operation label. Once labeled, the operation's IO becomes accessible through `$Label>outputParam` syntax anywhere within the enclosing scope. This eliminates the need for temporary `$`-variables when one operation's output feeds directly into another.

## Syntax

`($)` is an IO line inside the call block, not on the execution line itself.

```ebnf
label_decl     = "($)" "$" label_name ;
label_name     = PascalCase | camelCase ;
label_accessor = "$" label_name accessor_sigil param_name ;
accessor_sigil = ">" | "<" | "!" | "_" ;
```

```polyglot
[-] -ReadFile
   ($) $Read
   (-) <path << "input.csv"
   (-) >content

[-] -ParseCSV
   ($) $Parse
   (-) <data << $Read>content
```

Key elements:

- `($)` declares that the operation's results carry a label.
- `$Read` names the label (follows `$` variable convention).
- `$Read>content` accesses the labeled operation's `>content` output.
- `(-) >content` declares an output without a `>>` target; the label captures it.

PascalCase is the stylistic recommendation. camelCase is valid but not preferred.

## Label Accessor Suite

Four accessors mirror existing IO prefix conventions:

| Accessor | Syntax | Reads |
|----------|--------|-------|
| Output | `$Label>outputParam` | Output parameters of the operation |
| Input | `$Label<inputParam` | Input parameters as consumed |
| Error | `$Label!errorName` | Errors produced by the operation |
| Permission | `$Label_permissionName` | Permission grants and constraints |

The `>`, `<`, `!`, `_` sigils reuse the same prefix conventions found throughout [[io]] and [[blocks]].

```polyglot
[-] -ReadFile
   ($) $Read
   (-) <path << "input.csv"
   (-) >content

[-] -Transform
   ($) $Tx
   (-) <data << $Read>content
   [!] $Read!File.NotFound
      [-] >data << ""

[?] $Read<path =? "input.csv"
```

Metadata access through `$Label%state` also works. The label IS the operation's instance IO, so the full metadata tree is reachable.

## Chain Step Labels

`($)` at chain level labels the whole chain. `(.)` labels individual steps within the chain, indented under `($)` and mapped by position.

```polyglot
[-] -ReadFile->-ParseCSV->-ValidateRows
   ($) $Pipeline
      (.) $Read
      (.) $Parse
      (.) $Validate
   (-) >$Read.path << "input.csv"
   (-) <$Parse.rows >> >result

[-] -NextStep
   (-) <data << $Pipeline>result
```

Rules for chain step labels:

- `(.)` lines indent under `($)` as children of the chain label.
- Position determines mapping: the first `(.)` maps to step 0, the second to step 1, and so on.
- Not all steps require labels. Unlabeled steps remain accessible through numeric or leaf-name references.
- `>$Read.path` replaces `>0.path`. The `.` here is chain step-field notation, not a variable field path (PGE05001 does not apply).

### Standard vs Chain Accessor Context

| Context | Accessor | Example |
|---------|----------|---------|
| Standard operation | `$Label>output` | `$Read>content` |
| Chain step IO | `>$Label.param` / `<$Label.param` | `>$Read.path`, `<$Parse.rows` |

See [[concepts/pipelines/chains]] for chain execution semantics.

## IO Comment

`( )` (parentheses with a space) introduces an inline comment within IO blocks.

```polyglot
[-] -Pipeline
   ($) $Op
   (-) <input << $value               ( ) describe this input
   (-) >output >> >result              ( ) describe this output
```

`( )` can appear at the end of an IO line (inline) or on its own line (block comment). It is valid only within `(X)` IO context. See [[comments]] for the full comment syntax.

## Semantic Rules

| Rule | Detail |
|------|--------|
| Final-on-complete | All `$Label>param` accessors become Final when the operation finishes. See [[variable-lifecycle]] |
| Read-only | `$Label>param << value` is a compile error (PGE02013) |
| Scope = enclosing block | The label releases when indentation returns to the parent block |
| No duplicates | Two `($) $Read` declarations in the same scope produce PGE02012 |
| Optional | `($)` is opt-in sugar; existing `(-) >output >> $var` wiring still works |
| Parallel-safe | `($)` on `[=]` operations restricts output access until collection completes (PGE03012) |
| Instance IO | `$Label%state` works because the label IS the operation's instance IO |

## Background Labels

`($)` on `[b]` is allowed only when the label's outputs are consumed within the same scope. A standalone fire-and-forget label with no consumer produces PGE02015.

```polyglot
( ) VALID -- label consumed in same scope
[b] -LogEvent
   ($) $Log
   (-) <msg << "started"
[-] -CheckStatus
   (-) <logId << $Log>id

( ) INVALID -- standalone fire-and-forget
[b] -LogEvent
   ($) $Log
   (-) <msg << "done"

( ) VALID -- consumed within [b] block's own children
[b]
   [-] -LogEvent
      ($) $Log
      (-) <msg << "done"
   [-] -CheckStatus
      (-) <logId << $Log>id
```

## Compile Rules

| Code | Rule |
|------|------|
| PGE02012 | Duplicate operation label in same scope |
| PGE02013 | Write to label accessor |
| PGE02014 | Access label before operation completes |
| PGE02015 | Unused operation label on `[b]` |
| PGE03012 | Access parallel label output outside collection scope |
| PGE10007 | Chain step label count exceeds chain step count |
