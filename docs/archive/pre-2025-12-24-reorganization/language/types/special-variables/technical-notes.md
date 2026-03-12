# Technical Notes

## Compile-Time vs Runtime

- **Scope checking:** Compile-time
- **Type checking:** Compile-time
- **Value binding:** Runtime

## Performance

- No performance overhead
- Direct value passing (no copying)
- Same efficiency as regular parameter binding

## Error Messages

Common errors when using special variables:

```
Error: %Formatted_string not available in this context
  → Use %Formatted_string only within %Pipeline.Inline metadata

Error: Type mismatch for %Formatted_string
  → Expected: pg.string, Got: pg.int

Error: %Unknown_variable not defined
  → Check special variable name and scope
```

---
