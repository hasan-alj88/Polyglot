---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# Hierarchical Scoping

Permissions operate at two levels: **package ceiling** and **pipeline grant**.

## Package Ceiling

Permission IO in `{@}` references `{_}` ceiling objects, setting the maximum permissions any definition in the package may request. The package ceiling **allows but does not grant** — no definition inherits permissions automatically. See [[packages#Permissions]] for the full ceiling syntax and compile rules (PGE10001, PGE10002).

```aljam3
{_} _LogAnalyzerCeiling
   [.] .intent << #Ceiling
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/*"

{@} LogAnalyzer
   (@) _LogAnalyzerCeiling
```

## Pipeline Grant

Each `{-}` pipeline must explicitly declare `{_}` grant objects via `(-)` IO for the permissions it needs. Grants can only **narrow** what the package ceiling allows — never widen. See [[concepts/pipelines/permissions#Permissions]] for placement within pipeline definitions.

```aljam3
{_} _LogFileGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/app/*.log"
   [.] .path "/var/log/app/current.log"

{_} _AlertGrant
   [.] .intent << #Grant
   [.] .category #Web
   [.] .capability #Request
   [.] .scope "https://alerts.internal/notify"
   [.] .host "alerts.internal"
   [.] .endpoint "/notify"

{-} -ProcessLogs
   (-) _LogFileGrant
   [ ] narrower than ceiling — granted
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   ...

{-} -ComputeStats
   [ ] no permission IO — pure computation, zero IO
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   ...

{-} -SneakyPipeline
   (-) _AlertGrant
   [ ] grant references Web.Request — valid only if ceiling includes it
   ...
```

## No Inheritance

> **Clarification:** "Hierarchical scoping" and "no inheritance" are complementary, not contradictory. The hierarchy exists for **constraint validation** — the compiler checks that every grant falls within its package ceiling. "No inheritance" means grants are never automatic — each pipeline must explicitly declare what it needs. This is a security feature: the hierarchy provides bounds, explicit declaration prevents silent privilege escalation.

Permissions are never inherited. Every definition must declare the `{_}` grant objects it requires via IO, even if the package ceiling allows them. This makes each definition's IO footprint explicit and auditable.
