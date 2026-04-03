---
audience: pg-coder
type: specification
updated: 2026-03-14
status: draft
---

# Line Structure

<!-- @blocks -->
<!-- @identifiers -->
Every Polyglot line follows: `[indentation][block-element][single-expression]`

- One expression per line — no multi-statement lines
- Indentation is exactly **3 spaces** per level — no tabs
- Indentation determines scope (no closing markers needed) — see [[blocks#Closing: Indentation-Based]]
- Block elements (`[X]`) mark each line's role — see [[blocks]] for the full registry
- Expressions contain [[identifiers]] with prefix sigils (`@`, `#`, `=`, `$`, `!`)
- Assignment uses directional operators — see [[operators]]
