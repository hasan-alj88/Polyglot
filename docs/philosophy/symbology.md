---
audience: [automation-builder, integrator, design]
type: reference
updated: 2026-04-22
---

<!-- @c:vision -->
<!-- @u:syntax/identifiers -->
<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/line-structure -->
<!-- @c:philosophy/data-trees -->
<!-- @c:audit/reference/glossary -->
# Symbol Design Rationale

> This page expands the syntax design choices summarized in [[vision]]. It explains *why* Polyglot's symbols look the way they do — not what they mean (see [[user/syntax/identifiers]] and [[user/syntax/blocks]] for that).

Polyglot's syntax is visual by design. Every prefix character, bracket shape, and assignment arrow was chosen so that code self-documents its intent. A developer reading Polyglot code can identify what each line *is* — a variable, a type, a pipeline call, an error — before reading a single word of the identifier name. This is not decoration; it is a parsing strategy for both humans and the compiler.

## Why Prefixes?

Every [[glossary#Polyglot Code|Polyglot Code]] identifier starts with a prefix character. This serves three purposes:

- **Human parsing** — the eye recognises `$` as "variable" and `#` as "type" without reading the name. In a trigger-driven language where pipelines, queues, wrappers, triggers, permissions, and execution all coexist, prefixes prevent ambiguity at a glance.

- **Compiler parsing** — the compiler can classify any identifier with a single-character lookahead. No keyword reservation, no context-dependent resolution. A regex `^[$#\-!=@_%~]` partitions the entire identifier space.

- **Self-documenting code** — Polyglot code carries its own legend. `$user` is always a variable. `-ProcessData` is always a pipeline. `!Timeout` is always an error. The prefix *is* the documentation.

<!-- @u:syntax/identifiers -->

## Why Do All Lines Start with Markers?

Polyglot is a trigger-driven language. A pipeline is not a sequence of statements — it contains triggers, IO declarations, queue configuration, wrapper setup/cleanup, conditionals, error handlers, and execution calls. Each of these has a fundamentally different role.

Line markers (`[T]`, `[Q]`, `[W]`, `[-]`, `[!]`, `[?]`) declare which section each line belongs to. There is no ambiguity about a line's purpose: the marker tells you before you read the content. This is the syntactic consequence of Polyglot's design philosophy — every construct declares its intent explicitly.

## Prefix Symbols

<!-- @u:syntax/identifiers -->

Each prefix was chosen for visual resonance with its meaning:

| Symbol | Used For | Visual Rationale |
|--------|----------|-----------------|
| `-` | Sequential pipeline | Single horizontal line — one process, one step at a time |
| `=` | Parallel expander | Two horizontal lines — two processes side by side |
| `*` | Collector | Asterisk as sink/drain — multiple streams into one collection point |
| `_` | Permission | Lowest bar — floor of access, implicit deny, minimum privilege |
| `#` | Data type | Hash/number sign — structured data, table grid |
| `$` | Variable | Dollar sign — concrete instance of data (PHP convention) |
| `!` | Error | Exclamation — warning, something went wrong |
| `@` | Package | At sign — addressing, "where this comes from" |
| `%` | Metadata tree | Fraction symbol — the part of the Big Metadata Tree where you find the full address for all Polyglot objects |
| `;` | Environment | Semicolon — symbolic of foreign code line terminators (C++, Java, JS, Rust, Go) |
| `~` | Continuation | Tilde — `[~]` continues the line above, `.~` iterates levels. Wave = "keep going" |

The prefix set is closed — no new prefixes will be added. Every prefix participates in the `%` metadata tree (see [[user/concepts/data-is-trees]] and [[glossary#Data Tree|Data Tree]]).

## The Three-Bracket System

<!-- @u:syntax/blocks -->

Polyglot uses three bracket shapes, each with a single semantic role:

| Bracket | Role | Meaning |
|---------|------|---------|
| `{X}` | **Define** | Creates a new named object in the metadata tree |
| `[X]` | **Control** | Marks a line's role within a definition |
| `(X)` | **IO** | Declares data flow into or out of an operation |

**`{X}` — Define.** Curly brackets open top-level definitions: types (`{#}`), pipelines (`{-}`), permissions (`{_}`), packages (`{@}`), triggers (`{T}`), wrappers (`{W}`), queues (`{Q}`), errors (`{!}`), natives (`{N}`), and collectors (`{*}`). Every `{X}` creates a branch on the `%` metadata tree. The bracket shape says: *this is a new thing in the world*.

**`[X]` — Control.** Square brackets mark lines inside definitions: triggers (`[T]`), queues (`[Q]`), wrappers (`[W]`), execution (`[-]`, `[=]`, `[b]`), conditionals (`[?]`), errors (`[!]`), setup (`[\]`), cleanup (`[/]`), metadata (`[%]`), data load (`[#]`), and collectors (`[*]`). The bracket shape says: *this is what this line does*.

**`(X)` — IO.** Round brackets declare data flow: pipeline IO (`(-)`), expand IO (`(=)`), collect IO (`(*)`), permission IO (`(_)`, `(#)`, `(-)`), output handling (`(>)`), input handling (`(<)`), and operation labels (`(-) $Label`). The bracket shape says: *data moves here*.

The three shapes are visually distinct at any font size and never overlap in meaning. A `{#}` defines a type; a `[#]` loads data into one; a `(#)` declares a permission dependency on a type definition.

## Assignment Direction

<!-- @u:syntax/operators -->

Most programming languages use `=` for assignment, with data flowing right-to-left: `x = value`. The direction is invisible — you must know the convention to read the flow.

Polyglot makes data flow direction explicit:

| Operator | Direction | Meaning |
|----------|-----------|---------|
| `<<` | Right to left | **Final** assignment — immutable once set |
| `>>` | Left to right | **Final** push — immutable once delivered |
| `<~` | Right to left | **Default** assignment — overwritable once by a later push |
| `~>` | Left to right | **Default** push — overwritable once by a later push |

The arrow shape *is* the data flow. Reading `$result << -ProcessData` tells you: data flows from `-ProcessData` into `$result`, and `$result` becomes Final (immutable). Reading `$data >> >output` tells you: data flows from `$data` out through the `>output` port, and that port becomes Final.

This matters for pipeline design. Push and pull are distinct operations with different implications for data ownership and lifecycle. A variable that receives via `<<` is consumed; a variable that sends via `>>` is delivered. The visual direction removes all ambiguity about which side owns the data and when it becomes immutable.

## Key Narratives

### Pipeline Flow: `-` to `=` to `*`

The prefix characters tell a visual story of data moving through a pipeline system:

- **`-` (sequential)** — data enters a single pipeline, processed one step at a time
- **`=` (parallel)** — data fans out to multiple parallel processes (two lines = two threads)
- **`*` (collect)** — parallel results converge back into a single collection point

This `-` → `=` → `*` progression mirrors the expand/collect pattern that is fundamental to Polyglot's concurrency model.

### Permission as Visual Metaphor

The `_` underscore sits at the bottom of the character — the lowest bar, the floor. Permissions in Polyglot default to implicit deny: nothing is allowed unless explicitly granted. The visual metaphor reinforces this: `_` is the minimum, the baseline, the floor you must build up from. The three tiers (`_`, `__`, `___`) mirror the type system's three tiers (`#`, `##`, `###`), creating a consistent visual language for instance, schema, and field across both systems.

### Prefix as Parse and Document

Every identifier self-declares its category through its prefix. This is not just convenient — it is a compiler guarantee. The prefix partitions the identifier namespace so completely that no two categories can collide. `$name` and `#name` and `-name` coexist without ambiguity. The code is its own legend.

### Assignment as Visible Flow

The `<<`/`>>` and `<~`/`~>` operators make data direction visible in every assignment. Combined with the prefix system, a line like `$result << [-] -ProcessData` can be read as a sentence: "the variable `result` receives the Final output of the sequential call to pipeline `ProcessData`." Every symbol contributes to the reading — nothing is implicit.

---

## Related Philosophy

- [[philosophy/core-philosophy]] — Mind-shift, values, and evolution
- [[philosophy/language-design]] — Design principles and safety model
