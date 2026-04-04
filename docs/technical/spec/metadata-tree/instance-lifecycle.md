---
audience: [architect, designer]
type: spec
updated: 2026-04-03
---

# Instance Lifecycle

<!-- @source:metadata-tree/INDEX -->

## Creation

An instance is created when:
- A pipeline is triggered (`%=:Name:N` where N is the next sequential number)
- A wrapper is invoked via `[W]` (`%W:Name:N`)
- A queue dispatches a pipeline (`%Q:Name:N`)
- A variable is declared (`%$:name:N`)
- An expand/collect operator begins execution (`%~:Name:N`, `%*:Name:N`)

## Numbering

Instances use sequential zero-based numbering: `:0`, `:1`, `:2`, etc. Numbers are scoped to the definition — each pipeline, variable, or operator tracks its own counter.

## Release

Instances are released when:
- A pipeline completes or fails (all stages resolved)
- A wrapper completes cleanup or fails
- A queue is destroyed (all assigned pipelines released)
- A variable leaves scope ([[variable-lifecycle#Released]])
- An operator finishes collection

Released instances are no longer addressable. Their `live` metadata is discarded.

See also: [[object-types|Object Type Branches]], [[branches|Branch Specifications]]
