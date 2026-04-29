---
name: Push/Pull Terminology
description: Aljam3 uses push/pull not read/write for data flow operations
type: feedback
---

Use "push" and "pull" instead of "write" and "read" when describing Aljam3 data flow operations.

**Why:** Aljam3's assignment model is directional — `<<`/`>>` push data, pulling is consuming data. "Read/write" is generic programming terminology that doesn't capture the directional semantics.

**How to apply:** In all compile rules, spec docs, and Aljam3 discussions — say "push into" not "write to", "pull from" not "read from". Rule names should follow (e.g., "Push-Once" not "Write-Once").
