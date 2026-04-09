---
audience: pg-coder
type: specification
updated: 2026-04-08
status: retired
---

# ##Homogeneous — RETIRED

`##Homogeneous` has been retired. Child type uniformity is now expressed through `%###Type`:

- `%###Type << #SomeType` — all leaves must be that type (replaces ##Homogeneous)
- `%###Type` absent — each leaf declares its own type per-field (replaces ##Heterogeneous)

See [[syntax/types/schema-properties|Schema Properties]] for the full property reference.
