---
title: Inline Pipeline Parser - Pattern Summary
doc_type: reference
created: 2025-12-23
last_updated: 2025-12-23
tags:
  - examples
  - patterns
  - inline-pipeline
  - reference
related_documents:
  - overview.md
---

# Pattern Summary

All examples follow this structure:

## 1. Parser Pipeline
- Trigger: `[t] |T.Call`
- Input: `<formatted_string:pg.string`
- Outputs: Multiple typed fields
- Logic: Usually Python runtime for parsing

## 2. Main Pipeline
- Metadata: `[%] %Pipeline.Inline`
- Parser invocation: `[%] |ParserPipeline`
- Special variable: `<formatted_string:pg.string << %Formatted_string`
- Output wiring: `>parser_output >> <main_input`
- Business logic using parsed inputs

## 3. Inline Invocation
```polyglot
[r] |MainPipeline "formatted:string:here"
(|) >output >> $result
```

---
