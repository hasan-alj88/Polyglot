---
audience: pg-coder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:IncomingDataFrame"
metadata_instance: "%#:IncomingDataFrame:N"
---

# #IncomingDataFrame Struct

<!-- @c:types -->

System-provided input type for `{*}` collector definitions. Every `{*}` block must declare `(*) <Incoming#IncomingDataFrame` (PGE03018).

`#IncomingDataFrame` is a **subtype of `#Dataframe`** — a 2-level data tree where the 1st level is enumeration (arrival index) and the 2nd level is a `#Record` with the arrived variable info.

---

## Definition

```polyglot
{#} #IncomingDataFrame
   [%] .description << "System input for collector definitions — arrival-ordered variable data"
   [#] ##Dataframe
   [.] .arrival#int
   [.] .variable#RawString
   [.] .value#Serial
   [.] .status#JobStatus
   [.] .jobUid#RawString
```

---

## Fields

| Field | Type | Description |
|-------|------|-------------|
| `.arrival` | `#int` | Arrival sequence number (0-indexed) |
| `.variable` | `#RawString` | Canonical parameter name from `{*}` definition |
| `.value` | `#Serial` | The variable's current value |
| `.status` | `#JobStatus` | Running, Completed, Failed, Cancelled |
| `.jobUid` | `#RawString` | UID of the producing job |

---

## Usage

`(*) <Incoming` maps to the nameless parameters at the collector invocation site — the `(*) $Var` declarations. These arrive in **arrival order**, NOT declaration order.

Processable via standard `=ForEach.Dataframe` expanders.

---

## Related

- [[JobStatus]] — job lifecycle state enum
- [[Dataframe]] — parent type
- [[technical/spec/collector-definitions\|Collector Definitions]] — `{*}` block specification
