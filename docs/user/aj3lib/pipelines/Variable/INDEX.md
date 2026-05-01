---
audience: automation-builder
type: specification
updated: 2026-04-19
status: draft
---

# -Variable — Variable Manipulation

<!-- @c:pipelines -->
<!-- @c:jm3lib/types/Variable -->
<!-- @c:jm3lib/types/NativeType -->

Variable manipulation pipelines operate on `#Variable` instances -- language-tagged values used by the Bridge system for cross-language data exchange.

No `[@]` import needed.

**PRIMITIVE** -- jm3lib runtime pipelines are direct language runtime integrations. They are implemented by the Aljam3 runtime and cannot be reimplemented in user `.jm3` files.

## Pipelines

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/Variable/Convert\|-Variable.Convert]] | Convert a `#Variable` from one language's type system to another |

## Related

- [[jm3lib/types/Variable|#Variable]] -- language-tagged variable type
- [[jm3lib/types/NativeType|#NativeType]] -- primitive type classifier per host language
- [[jm3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]] -- Bridge pipelines that use `#Variable`
- [[technical/algorithms/bridge-conversion|Bridge Conversion Algorithm]] -- conversion algorithm internals
