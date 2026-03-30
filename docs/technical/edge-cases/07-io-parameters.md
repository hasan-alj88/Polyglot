---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 7. IO Parameters (S7)

### EC-7.1: IO with field separators

<!-- @io -->
**EBNF:** `input_param ::= '<' name { field_separator name }` — IO params can have sub-fields.

**What it tests:** Dot-navigated IO parameters. See [[io]].

```polyglot
[=] <config.timeout#int << 30
[=] >result.status#string >> $status
```
