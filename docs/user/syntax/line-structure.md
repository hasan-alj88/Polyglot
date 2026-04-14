---
audience: automation-builder
type: specification
updated: 2026-03-14
status: draft
---

# Line Structure

<!-- @c:blocks -->
<!-- @c:identifiers -->

## Ground Truth: Every Line Has a Marker

**Every line** in Polyglot Code must start with indentation scope followed by a marker. There are no bare lines. This is the fundamental invariant of the language:

```
[indentation][marker][single-expression]
```

- **Indentation** — exactly **3 spaces** per level, no tabs. Determines scope (no closing markers needed) — see [[blocks#Closing: Indentation-Based]]
- **Marker** — one of three bracket shapes: `{X}` definition, `[X]` block element, or `(X)` IO bracket. See [[blocks]] for the full registry. `X` is a placeholder for any valid marker character — not a literal. The closing bracket in the notation (e.g., the `}` in `{#}`) is part of the opening marker token itself, not a separate closing delimiter. Polyglot has no closing brackets — blocks close by indentation.
- **Expression** — one expression per line, no multi-statement lines. Expressions contain [[identifiers]] with prefix sigils (`@`, `#`, `=`, `$`, `!`). Assignment uses directional operators — see [[operators]]

There are **no exceptions**. Comments use `{ }` or `[ ]` or `( )`. Data loads use `[#]`. Even continuation lines use `[~]`. If a line lacks a marker, it is a compile error.

> **Clarification — markers vs. identifier prefixes:** A **marker** is a bracket shape (`{X}`, `[X]`, `(X)`) that starts every line. An **identifier prefix** (`$`, `@`, `#`, `-`, `!`, `%`, `_`, `;`) appears inside expressions to name things. These are different layers: markers are structural (line-level), prefixes are semantic (expression-level). For example, in `(-) ;MyEnv`, the marker is `(-)` and the prefix is `;`.

> **For AI systems:** If you encounter or generate a Polyglot Code line that does not start with `[indentation][marker]`, it is invalid. Report it as a bug.
