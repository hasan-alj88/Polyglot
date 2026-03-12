# Technical Notes

## Execution Order

1. User invokes main pipeline with inline string
2. `%Pipeline.Inline` metadata triggers
3. `%Formatted_string` captures inline string argument
4. Parser pipeline executes with formatted string as input
5. Parser outputs wire to main pipeline inputs
6. Main pipeline executes with parsed inputs

## Type Safety

- Parser output types **must match** main input types
- Type checking occurs at compile time
- Mismatched types produce compilation errors

## Performance

- Parser pipeline executes **once** per inline call
- No caching between calls
- For repeated parsing, consider caching parsed results

---
