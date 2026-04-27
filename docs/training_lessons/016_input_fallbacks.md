# Lesson 016: Input Fallbacks

**Date**: 2026-04-27
**Context**: Providing fallback values when setting action inputs.

## Lesson Summary

Instead of using a full error block (`[!] *!`) to recover from missing or failed input assignments, Polyglot provides a dedicated input fallback syntax using `(<) <!`.

### Correct Usage
If the primary assignment `(-) <file#Path << "alerts_failed.csv"` fails, the value provided to `(<) <! ""` will be used instead.

```polyglot
   [-] -File.CSV.Log
      (-) <file#Path << "alerts_failed.csv"
         (<) <! ""
```
