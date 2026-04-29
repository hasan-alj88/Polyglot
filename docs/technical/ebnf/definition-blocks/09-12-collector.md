---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.12 Collector Definition (`{*}`)

```ebnf
collector_def       ::= "{*}" collector_id NEWLINE
                         collector_metadata
                         collector_io
                         collector_body ;

collector_id        ::= '*' dotted_name ;
```

**Rules:**
- `{*}` defines a collector — a first-class definition block for user-definable collector logic.
- Collector identifier uses the `*` prefix (e.g., `*First`, `*Agg.Sum`, `*Into.Text.Append`).
- `[%]` metadata must include `.category` (`#CollectorCategory`), `.scope` (`#CollectorScope`), `.overflow` (`#OverflowStrategy`) — missing triggers PGE03013.
- `(*)` IO must include `<Incoming#IncomingDataFrame` — missing triggers PGE03018.
- Body contains only `[-]` variable declarations and `[T]` arrival triggers — no `[=]`, `[Q]`, `[W]`, or external trigger sources.
- `[T]` inside `{*}` takes arrival conditions (`*Arrive`, `*Job.Arrive`), not external trigger references.
- `(T)` IO declares arrival data as `$` variables inside trigger blocks.
- `[*]` inside `{*}` is a release command (`*Job.Release`, `*Arrive.Job.Release`) — every code path must release all jobs (PGE03025).
- `{*}` creates a branch on the `%` metadata tree at `%*`.
- See [[16-collector-definitions]] for the complete grammar.

**Example:**

```aljam3
{*} *First
   [%] .category << #CollectorCategory.Race
   [%] .scope << #CollectorScope.Parallel
   [%] .overflow << #OverflowStrategy.InMemoryOnly
   (*) <Incoming#IncomingDataFrame
   (*) >winner
   (*) !Collector.NoResult

   [T] *Arrive"0"
      (T) >var
      (T) >job
      >> >winner << $var.value
      [*] *Job.Release"[0,N]"
```

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.12 `{*}` Collector | [[technical/spec/collector-definitions\|collector-definitions]] |
