---
audience: automation-builder
type: operator
updated: 2026-05-02
status: stable
---

# =ForEach.Column

Iterates over a strictly 2D `##Dataframe` column-by-column. 

This is a `jm3lib` syntactic wrapper that atomically transposes the matrix (swapping the Depth 0 and Depth 1 enum keys) and then iterates it at Depth 1.

## IO Parameters

- `(=) <Data` (required) — The `##Dataframe` to iterate
- `(=) >item` (required) — Outputs the entire structural Record for each column (the vertical slice)
- `(=) >key` (optional) — Outputs the flat Enum representation of the column header (e.g., `.name`, `.age`)

## Example

```aljam3
[ ] Given a dataframe
[-] $users##Dataframe <<
   ($)        | .name#string | .age#int
   ($) .user1 | "Hasan"      | 35
   ($) .user2 | "Paul"       | 28

[ ] Iterate column-by-column
[=] =ForEach.Column
   (=) <Data << $users
   (=) >item >> $colRecord
   (=) >key >> $colKey
   [ ]
   [ ] First iteration: 
   [ ] $colKey = .name
   [ ] $colRecord = { .user1: "Hasan", .user2: "Paul" }
```

## Internal AST Expansion

The compiler natively unrolls `=ForEach.Column` by applying a tree permutation followed by a depth limit:
```aljam3
[ ] Unrolled representation
[-] $tempTransposed##Dataframe << =*PermuteLevels
   (=) <Data << $users
   (*) <Permute << [1, 0]

[=] =ForEach
   (=) <Data << $tempTransposed
   (=) <Depth << 1
```
