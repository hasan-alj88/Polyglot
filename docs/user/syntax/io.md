---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# IO Parameters

<!-- @u:operators -->
<!-- @c:pipelines -->
<!-- @c:identifiers -->
Input and output parameters bind data into and out of operators. IO labels are [[identifiers#Serialized Identifiers]]. Assignment uses [[operators]] (`<<`, `>>`, `<~`, `~>`). For how IO assignment mode controls pipeline triggering, see [[concepts/pipelines/io-triggers#IO as Implicit Triggers]]. IO ports live as nested typed sections in the metadata tree at `%-:{name}:{instance}.<` (inputs) and `.>` (outputs) — see [[data-is-trees#IO Ports — Nested Typed Sections]].

## Sections

| Doc | Content |
|-----|---------|
| [[syntax/io/io-labels\|IO Labels & Line Pattern]] | `<`/`>` prefixes, `(-)` `(=)` `(*)` `(_)` markers, scoping rules |
| [[syntax/io/io-variables\|IO Inputs as Variables]] | `(-)` inputs become `$`-prefixed variables |
| [[syntax/io/error-declaration\|Error Declaration]] | `(-) !ErrorName` syntax |
| [[syntax/io/environment-declaration\|Environment Declaration]] | `(-) ;EnvName` syntax, multiple environments |
| [[syntax/io/pipeline-call\|Pipeline Call IO]] | `[-]` execution with `(-)` IO lines |
| [[syntax/io/chain-io\|Chain IO Addressing]] | Step references, pipeline-perspective `<`/`>`, auto-wire |
| [[syntax/io/operation-labels\|Operation Labels]] | `($)` labels, `$Label>output` access, chain step labels |
| [[syntax/io/collection-operators\|Collection Operators]] | `=ForEach` expand, `*` collect, wait/collect IO |
| [[syntax/io/io-parameter-handling\|IO Parameter Handling]] | `(>)`/`(<)` fallback, error-specific fallback, chain exception |
