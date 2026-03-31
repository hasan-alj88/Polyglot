---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 5. Block Elements (S5)

### EC-5.1: All block element categories used

<!-- @blocks -->
**What it tests:** A file exercising every block element category. See [[blocks]].

| Category | Elements | Where tested |
|----------|----------|-------------|
| Registry | `[@]` | Package imports |
| Data Flow | `[=]` `[~]` `[*]` | IO lines, expand IO, collect invocation |
| Execution | `[r]` `[p]` `[b]` `[s]` | Run, parallel, background, serial load |
| Control Flow | `[?]` `[!]` `[T]` `[Q]` `[W]` | Conditionals, errors, trigger, queue, wrapper |
| Data Access | `[.]` `[:]` | Data definitions |
| Logical | `[&]` `[\|]` `[-]` `[^]` | Conditional compound logic |
| Comment | `[ ]` | Inline comments |

### EC-5.2: Background execution

<!-- @blocks:Execution -->
**EBNF:** `background_line ::= "[b]" exec_expr`

**What it tests:** Fire-and-forget execution. See [[blocks#Execution]].

```polyglot
[b] =Logging.SendMetric
   [=] <event << "user_created"
```
