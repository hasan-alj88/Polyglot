---
audience: automation-builder
type: specification
updated: 2026-04-23
status: draft
---

# Wildcard Auto-Wire

<!-- @u:operators -->
<!-- @c:syntax/io -->
<!-- @c:syntax/io/operation-labels -->
<!-- @u:technical/ebnf/07-io-parameters -->
<!-- @u:technical/ebnf/10-execution -->

`<* << $Label>*` wires **all** outputs of a labeled operation into **all** inputs of the next pipeline call, with the compiler picking the mapping by type. No per-port line needed — the shape on both sides must line up exactly.

> **Symbol note.** `*` already means "collect all" in Polyglot (`*All`, `*First`, `(*) <<`, `(*) >>`). `<*` reads as "all inputs" and `$Label>*` reads as "all outputs of `$Label`" — same collect-everything intuition, applied to IO.

## Form

```polyglot
[-] -TargetPipeline
   (-) <* << $Label>*
```

- `<*` — all inputs of `-TargetPipeline`
- `$Label>*` — all outputs of the operation previously labelled `(-) $Label`
- `<<` — standard Final assignment (same operator as per-port wiring)

## Bijective type-topology matching

The compiler pairs each output with exactly one input by **type-identity**. The pairing must be:

- **1-to-1** — every output maps to exactly one input
- **onto** — every input receives exactly one output
- **unambiguous** — only one valid pairing exists

If any of these fail, the compiler refuses to auto-wire and you must fall back to explicit per-port wiring.

| Condition | Result | Rule |
|-----------|--------|------|
| Output count ≠ input count | Fail — port count mismatch | PGE08003 |
| Output type has no matching input (or vice versa) | Fail — type mismatch | PGE08001 |
| Multiple ports share a type on either side | Fail — ambiguous | PGE08002 |
| All types unique and paired 1-to-1 | Succeeds (with PGW08001 warning) | PGW08001 |

## Completion-wait semantics

`<* << $Label>*` requires **all** of `$Label`'s outputs to be Final before the target triggers. This gives you implicit completion-wait — no `(*)` barrier, no `[/]`, just "go when the previous step is done."

## Examples

### Valid — one output → one input, same type

```polyglot
[-] -File.Text.Read
   (-) $Read
   (-) <path#path << $path
   (-) >content#string

[-] -Text.Transform
   (-) <* << $Read>*                [ ] ⚠ PGW08001 — works, but explicit is clearer
   (-) >formatted#string >> >output
```

### Valid — multiple ports, all types unique

```polyglot
[-] -Fetch.User
   (-) $Fetch
   (-) <id#int << $userId
   (-) >name#string
   (-) >age#int

[-] -Store.User
   (-) <* << $Fetch>*               [ ] ⚠ PGW08001 — #string→<name, #int→<age
   (-) >stored#bool >> >ok
```

### Explicit per-port wiring — always valid, no warning

```polyglot
[-] -Fetch.User
   (-) $Fetch
   (-) <id#int << $userId
   (-) >name#string
   (-) >age#int

[-] -Store.User
   (-) <name#string << $Fetch>name
   (-) <age#int << $Fetch>age
   (-) >stored#bool >> >ok
```

### Invalid — type mismatch (PGE08001)

```polyglot
[-] -Count.Items
   (-) $A
   (-) <list#array:string << $items
   (-) >total#int

[-] -Format.Label
   (-) <* << $A>*                   [ ] ✗ PGE08001 — >total#int has no #int input
   (-) <text#string
   (-) >formatted#string >> >output
```

### Invalid — ambiguous types (PGE08002)

```polyglot
[-] -Fetch.Both
   (-) $A
   (-) <url#string << $url
   (-) >name#string
   (-) >label#string

[-] -Process.Single
   (-) <* << $A>*                   [ ] ✗ PGE08002 — two #string outputs, one #string input
   (-) <text#string
   (-) >result#string >> >output
```

### Invalid — port count mismatch (PGE08003)

```polyglot
[-] -Fetch.Data
   (-) $A
   (-) <url#string << $url
   (-) >content#string
   (-) >count#int
   (-) >status#string

[-] -Transform.Text
   (-) <* << $A>*                   [ ] ✗ PGE08003 — 3 outputs, 2 inputs
   (-) <text#string
   (-) <flag#int
   (-) >result#string >> >output
```

## When to use it

Use wildcard auto-wire for **terse prototypes** where signatures align and the mapping is obvious. For production code, prefer explicit per-port wiring — it survives signature changes and reads clearly without consulting the types.

## See Also

- [[syntax/io/operation-labels|Operation Labels]] — `(-) $Label` and `$Label>output` access
- [[concepts/pipelines/chains|Chain Execution (Retired)]] — historical context; chains retired in #340, auto-wire recovered as this general feature
- [[technical/compile-rules/PGE/PGE08001-auto-wire-type-mismatch|PGE08001]] — type mismatch
- [[technical/compile-rules/PGE/PGE08002-auto-wire-ambiguous-type|PGE08002]] — ambiguous type
- [[technical/compile-rules/PGE/PGE08003-auto-wire-unmatched-parameter|PGE08003]] — port count mismatch
- [[technical/compile-rules/PGW/PGW08001-auto-wire-succeeded|PGW08001]] — auto-wire succeeded warning
