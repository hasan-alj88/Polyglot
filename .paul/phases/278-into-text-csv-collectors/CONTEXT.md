# Issue #278: Text & CSV Collection Operators - Context

**Gathered:** 2026-04-11
**Status:** Ready for planning

<domain>
## Issue Boundary

Add text and CSV collection operators to pglib, completing the expand/collect pattern for text data. Includes:
- Text collectors (*Into.Text.Append, *Into.Text.Merge)
- CSV collectors (*Into.CSV.Rows, *Into.CSV.Merge)
- Text/CSV expanders (=ForEach.Text.Lines, =ForEach.CSV.Rows)
- Comparator pipeline (=Text.Diff) with #TextDiffs data type
- PPTD overflow infrastructure (RAM → disk → batch collection)
- *Agg.Concatenate separator parameter update

</domain>

<decisions>
## Implementation Decisions

### Operator Set

Two text collectors (separate operators, user chooses):
- `*Into.Text.Append` — simple concatenation with separator, order by expand index or arrival
- `*Into.Text.Merge` — k-way merge using #TextDiffs against a base text

Two CSV collectors:
- `*Into.CSV.Rows` — collect rows into CSV text
- `*Into.CSV.Merge` — k-way merge on CSV (row-aware, preserves header)

Two expanders (matched set):
- `=ForEach.Text.Lines` — expand text into lines with index
- `=ForEach.CSV.Rows` — expand CSV into rows with index

One comparator pipeline:
- `=Text.Diff` — compares original vs modified, outputs #TextDiffs

One update:
- `*Agg.Concatenate` — add optional `<separator` parameter (backwards-compatible)

### Data Types

- `#TextDiff` — single entry: .line (#int), .op (#DiffOp: .Add/.Delete/.Replace), .content (#String)
- `#TextDiffs` — #Array.TextDiff
- `#MergeConflict` — .line (#int), .sources (#Array.TextDiff)
- `#MergeResult` — .text (#String), .conflicts (#Array.MergeConflict)
- `#DiffStats` — summary counts from =Text.Diff
- `#MergeStrategy` — enum: .FirstWins, .LastWins, .KeepBoth, .SkipConflicts, .DiffOnly
- `#CollectOrder` — enum: .ExpandIndex (default), .Arrival

### Conflict Handling

User-configurable via `<conflict` parameter (#MergeStrategy enum):
- .FirstWins — first job's change wins
- .LastWins — last job's change wins
- .KeepBoth — both changes kept, marked in output
- .SkipConflicts — apply non-conflicts, output conflicts separately in >result.conflicts
- .DiffOnly — produce merged diff record without modifying anything

User controls everything: can apply non-conflicts only and keep a conflict list for later processing, or just keep a record without updating files.

### PPTD Overflow Infrastructure

PPTD (Parallel Processing Temporary Directory) is NOT the default — only activates when RAM threshold exceeded.

Overflow chain:
```
RAM (default) → overflow → PPTD (distributable across multiple) → overflow → batch collection (pause, collect, free space, resume)
```

If RAM handles it, none of the overflow machinery activates.

Designed here for text/CSV collectors but the concept is reusable by any collector.

Queue configuration via `-Q.Overflow.*`:
- `-Q.Overflow.RamLimit` — max RAM (bytes) before spilling to PPTD
- `-Q.Overflow.PptdPaths` — directory paths for PPTD allocation
- `-Q.Overflow.PptdMaxSize` — max disk per PPTD before batch collection
- `-Q.Overflow.BatchThreshold` — PPTD usage ratio triggering batch collection (default: 0.85)
- `-Q.Overflow.MaxPptdCount` — max PPTDs to distribute across (default: 3)

PPTD is system-managed: auto-created and auto-deleted.

### Error Types

| Error | Operators | When |
|-------|-----------|------|
| `!Storage.Space` | All collectors | All overflow options exhausted (RAM + PPTD + batch) |
| `!Text.Diff.EmptyInput` | =Text.Diff | Either input is empty |
| `!Text.Lines.Empty` | =ForEach.Text.Lines | Input text is empty |
| `!Text.Append.EmptyResult` | *Into.Text.Append | All fragments were empty |
| `!Text.Merge.InvalidLineNumber` | *Into.Text.Merge, *Into.CSV.Merge | Diff references line outside base range |
| `!Text.Merge.EmptyBase` | *Into.Text.Merge, *Into.CSV.Merge | Base text is empty |
| `!CSV.Parse.MalformedRow` | =ForEach.CSV.Rows | Row has wrong field count |
| `!CSV.Parse.Empty` | =ForEach.CSV.Rows | CSV content is empty |
| `!CSV.Parse.InvalidDelimiter` | =ForEach.CSV.Rows | Delimiter is empty or multi-char |
| `!CSV.Collect.SchemaMismatch` | *Into.CSV.Rows | Row has different fields than headers |
| `!CSV.Collect.EmptyResult` | *Into.CSV.Rows | Zero rows collected |
| `!CSV.Merge.HeaderConflict` | *Into.CSV.Merge | Diffs modify the header row |

### Permissions

All collectors require `_File.TempWrite` permission for PPTD paths — but only when overflow actually spills to disk. Expanders and comparator are pure computation (no permissions).

### *Agg.Concatenate Update

Add optional `<separator` (#String, default "") — backwards-compatible, existing usage unchanged.

</decisions>

<specifics>
## Specific Ideas

- The k-way merge collector takes #TextDiffs from a separate =Text.Diff comparator pipeline — the comparator is its own pglib pipeline, not baked into the collector
- Conflict handling parameters allow partial application: apply non-conflicts and keep conflict list for later processing
- CSV merge is row-aware and preserves header rows
- PPTD concept parallels how databases handle memory pressure — spill to disk transparently

</specifics>

<deferred>
## Deferred Ideas

- `*Into.Serial.Merge` for structured data (JSON/YAML/TOML deep merge) — separate issue
- `*Into.Text.Anchor` (section anchor insertion) — revisit if use cases emerge
- `*Into.Text.Patch` (unified diff format) — may be subsumed by *Into.Text.Merge with .DiffOnly strategy
- PPTD generalization to existing collectors (Array, Map, etc.) — separate issue when needed
- Column-based CSV expansion (`=ForEach.CSV.Columns`) — evaluate demand first

</deferred>

---

*Issue: 278-into-text-csv-collectors*
*Context gathered: 2026-04-11*
