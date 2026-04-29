---
audience: design
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 1. File Structure (S1)

### EC-1.1: Multiple definitions in one file

<!-- @u:EBNF:file -->
**EBNF:** `file ::= package_block { definition }` — the `{ }` repetition allows zero or more definitions after the package block.

**What it tests:** A single `.aj3` file containing `{@}`, `{#}`, `{-}`, and `{ }` definitions together.

**Cross-refs:** [[packages]] (package block), [[blocks]] (definition elements)

```aljam3
{@} @Local:001.Multi:v1.0.0

{#} #Status
   [.] .Active

{-} -First
   [T] -T.Call
   [W] -W.Aljam3

{-} -Second
   [T] -T.Call
   [W] -W.Aljam3
```

### EC-1.2: File with only package block (no definitions)

**EBNF:** `file ::= package_block { definition }` — zero definitions is valid.

**What it tests:** Minimal valid `.aj3` file.

```aljam3
{@} @Local:001.Empty:v1.0.0
```
