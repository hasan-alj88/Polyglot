# Lesson 006: Error Handling Blocks

**Date**: 2026-04-27
**Context**: Handling errors within action blocks.

## Lesson Summary

You can catch and handle errors directly within an action block by using an explicit error handling block defined by `[!] *!`.

### Syntax Example
```polyglot
   [-] -File.CSV.Write
      (-) <file << $file
      (-) <data << $worldTemp
      (-) <alertLevel << $threshold
      [!] *!
         [-] >file >> ""
```

If `-File.CSV.Write` throws an error, execution drops into the `[!] *!` block where fallback logic or cleanup can be executed (e.g., returning an empty string).
