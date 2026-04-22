---
audience: automation-builder
type: specification
status: retired
updated: 2026-04-22
---

# Chain IO Addressing (Retired)

Chain IO addressing (`>N.param`, `<N.param`, `>LeafName.param`) has been retired along with the `->` chain operator.

Use **operation labels** instead: `(-) $Label` names a pipeline call, and `$Label>output` / `$Label<input` addresses its IO. See [[syntax/operation-labels|Operation Labels]] and [[concepts/pipelines/chains|Chain Execution (Retired)]] for the replacement pattern.
