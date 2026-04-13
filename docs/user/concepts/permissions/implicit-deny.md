---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# Implicit Deny

<!-- @c:identifiers -->
<!-- @c:blocks -->

Polyglot uses an implicit-deny permission system. Every pipeline starts with zero IO capabilities. To perform any IO — read a file, make a web request, access a database — the package or pipeline must reference a named `{_}` permission object. The `{_}` definition block and `[_]` block element are registered in [[blocks#Permissions]].

This follows the Cisco ACL model: if you don't explicitly allow it, it's denied.

A pipeline with no `[_]` references is **pure computation** — it can transform data, run conditionals, and call other pipelines, but cannot touch the outside world. Any IO call without a matching permission is a compile error.

```polyglot
{-} PureComputation
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] no [_] lines — this pipeline cannot do IO
   [-] $result#int << -Math.Add $a $b
```
