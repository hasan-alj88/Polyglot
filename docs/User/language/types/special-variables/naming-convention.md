# Naming Convention

## Pattern

```
%Category.SpecificName
```

**Examples:**
- `%Formatted_string` - Single-level name (historical)
- `%Pipeline.Inline` - Metadata (not a variable)
- Future: `%Execution.StartTime`

## Rules

1. **Prefix:** Always `%`
2. **Case:** PascalCase for segments
3. **Scope:** Limited to specific contexts
4. **Immutability:** Always read-only
5. **Type:** Explicit type binding required

---
