---
audience: automation-builder
type: specification
updated: 2026-04-26
---

# ? — Compile-Time Predicates

<!-- @c:syntax/identifiers -->
<!-- @c:pglib/INDEX -->

Boolean Predicates (identified by the `?` sigil) represent **compile-time assertions**. Unlike `-` pipelines which evaluate at runtime, `?` predicates are evaluated strictly during compilation. They enforce structural and topological safety in your Polyglot codebase.

Predicates always return a Boolean value (`#Boolean.True` or `#Boolean.False`) and are typically used in conjunction with the `[?]` conditional switch to halt compilation if a safety check fails.

## `?Queue.*` Namespace

Used to validate Queue topologies, often to prevent circular dependencies or invalid configurations.

| Predicate | Description | Example Usage |
|-----------|-------------|---------------|
| `?Queue.Host.IsEqual"{#QueueName}"` | Checks if the current queue evaluates to the same host as the target queue. | Preventing routing loops in Failover rules. |
| `?Queue.Strategy.IsEqual"{#QueueStrategy}"` | Checks if the queue utilizes a specific scheduling strategy (e.g., `#FIFO`). | Asserting priority structures. |
| `?Queue.IsLocal` | Returns true if the queue resides on `localhost`. | - |

## `?Host.*` Namespace

Used to validate host capabilities before scheduling jobs that require specific hardware or operating systems.

| Predicate | Description | Example Usage |
|-----------|-------------|---------------|
| `?Host.HasCapability"{#GPU}"` | Checks if the target host provides the specified capability. | Asserting GPU presence for ML queues. |
| `?Host.IsOS"{#OS.Linux}"` | Validates the target host Operating System. | Preventing Windows binaries on Linux nodes. |

## `?Math.*` Namespace

Used for numeric comparisons known strictly at compile-time (e.g., comparing two static `#int` values defined in a `#JobRules` block).

| Predicate | Description | Example Usage |
|-----------|-------------|---------------|
| `?Math.IsGreater"{ValueA}", "{ValueB}"` | Checks if ValueA is strictly greater than ValueB. | Validating memory margins (`{$margin.GB} > {$value.GB}`). |

## Example: Preventing Queue Failover Loops

A prime example of compile-time predicates is ensuring that a queue does not attempt to failover to another queue on the same host (which would cause an infinite loop during an outage).

```polyglot
{Q} #QueueRules:Failover
   (#) <FailoverQueue#Queue <~ {#BackupHostQueue}
   [ ] Compile-time safety check: if they are the same host, compilation fails.
   [?] ?Queue.Host.IsEqual"{#FailoverQueue}" =? #Boolean.True
      [!] >> !Queue.InvalidFailoverTarget
```

## See Also
- [[syntax/identifiers|Identifiers]] — Core syntax for Polyglot sigils.
- [[pglib/INDEX|pglib Registry]] — The full standard library catalog.
