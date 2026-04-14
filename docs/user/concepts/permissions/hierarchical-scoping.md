---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# Hierarchical Scoping

Permissions operate at two levels: **package ceiling** and **pipeline grant**.

## Package Ceiling

`[_]` lines in `{@}` reference `{_}` ceiling objects, setting the maximum permissions any definition in the package may request. The package ceiling **allows but does not grant** — no definition inherits permissions automatically. See [[packages#Permissions]] for the full ceiling syntax and compile rules (PGE10001, PGE10002).

```polyglot
{_} _LogAnalyzerCeiling
   [.] .intent << #Ceiling
   [.] .File.Read "/var/log/*"
   [.] .File.Write "/tmp/reports/*"
   [.] .Web.Request "https://alerts.internal/*"
   [.] .System.Env "LOG_LEVEL"

{@} LogAnalyzer
   [_] _LogAnalyzerCeiling
```

## Pipeline Grant

Each `{-}` pipeline must explicitly reference `{_}` grant objects for the permissions it needs. Grants can only **narrow** what the package ceiling allows — never widen. See [[concepts/pipelines/permissions#Permissions]] for placement within pipeline definitions.

```polyglot
{_} _LogFileGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*.log"

{_} _AlertGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*.log"
   [.] .Web.Request "https://alerts.internal/notify"

{-} -ProcessLogs
   [_] _LogFileGrant
   [ ] narrower than ceiling — granted
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   ...

{-} -ComputeStats
   [ ] no [_] lines — pure computation, zero IO
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   ...

{-} -SneakyPipeline
   [_] _AlertGrant
   [ ] grant references Web.Request — valid only if ceiling includes it
   ...
```

## No Inheritance

> **Clarification:** "Hierarchical scoping" and "no inheritance" are complementary, not contradictory. The hierarchy exists for **constraint validation** — the compiler checks that every grant falls within its package ceiling. "No inheritance" means grants are never automatic — each pipeline must explicitly declare what it needs. This is a security feature: the hierarchy provides bounds, explicit declaration prevents silent privilege escalation.

Permissions are never inherited. Every definition must reference the `{_}` grant objects it requires, even if the package ceiling allows them. This makes each definition's IO footprint explicit and auditable.
