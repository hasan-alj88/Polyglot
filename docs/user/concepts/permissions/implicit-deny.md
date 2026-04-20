---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# Implicit Deny

<!-- @c:identifiers -->
<!-- @c:blocks -->
<!-- @u:philosophy/cybersecurity -->
<!-- @u:philosophy/core-philosophy#Implicit Deny Permission -->

Polyglot uses an implicit-deny permission system. Every pipeline starts with zero IO capabilities. To perform any IO — read a file, make a web request, access a database — the block must declare a named `{_}` permission object via its IO markers (`(-)` for pipelines, `(#)` for data definitions).

This follows the Cisco ACL model: if you don't explicitly allow it, it's denied.

A pipeline with no `_` IO declarations is **pure computation** — it can transform data, run conditionals, and call other pipelines, but cannot touch the outside world. Any IO call without a matching permission is a compile error (PGE10004).

```polyglot
{-} PureComputation
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] no permission IO — this pipeline cannot do IO
   [-] $result#int << -Math.Add $a $b
```
