---
code: PGE01049
name: Invalid Definition Target
---
# Rule 1.49 — Invalid Definition Target

**Statement:** A definition marker (`{-}`, `{#}`, `{@}`, etc.) must be immediately followed by its corresponding specific identifier type. 

**Rationale:** The EBNF sequence for a definition is strict. A Pipeline Definition `{-}` must be followed by a Pipeline Identifier (`-PipelineName`). If it is followed by a Variable (`$var`), a Data type (`#Type`), or garbage, it fundamentally breaks the definition contract.

**Detection:** The Compiler Validation phase scans the immediate next token following any definition marker. If it does not match the valid target mapping for that specific marker, it emits PGE01049.

**Valid Mappings:**
- `{@}` -> `Registry` or `PackageName` or `Package`
- `{#}` -> `Data`
- `{-}` -> `Pipeline`
- `{T}` -> `Trigger` or `InlineInstruction`
- `{W}` -> `Wrapper`
- `{Q}` -> `QueueConfig`
- `{!}` -> `Error`
- `{_}` -> `Data`
- `{*}` -> `Collector`
- `{$}` -> `Constructor`
- `{//}` -> `CommentText`
