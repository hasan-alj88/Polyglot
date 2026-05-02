---
audience: automation-builder
type: operator
updated: 2026-05-02
status: stable
---

# =ForEach.Row

Iterates over a strictly 2D `##Dataframe` row-by-row. 

This is a `jm3lib` syntactic wrapper around the unified `=ForEach` operator that automatically injects `<Depth << 1` so you do not have to declare it manually.

## IO Parameters

- `(=) <Data` (required) — The `##Dataframe` to iterate
- `(=) >item` (required) — Outputs the structural Record for each row
- `(=) >key` (optional) — Outputs the flat Enum representation of the row index (e.g., `.0`, `.1`, or named like `.user1`)

## Example

```aljam3
[ ] Given a dataframe
[-] $users##Dataframe <<
   ($)        | .name#string | .age#int
   ($) .user1 | "Hasan"      | 35
   ($) .user2 | "Paul"       | 28

[ ] Iterate row-by-row
[=] =ForEach.Row
   (=) <Data << $users
   (=) >item >> $rowRecord
   (=) >key >> $rowKey
   [ ]
   [ ] $rowKey is .user1, $rowRecord.name is "Hasan"
```

## Internal AST Expansion

The compiler natively unrolls `=ForEach.Row` into:
```aljam3
[=] =ForEach
   (=) <Data << $users
   (=) <Depth << 1
```
