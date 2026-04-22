---
audience: ai-finder
type: audit-reference
scope: decision-record
category: syntax
issue: "#340"
date: 2026-04-22
updated: 2026-04-22
---

# Retire -> Chain Operator

## Summary

The `->` chain operator was removed from Polyglot. Labeled `[-]` calls with operation labels (`(-) $Label`) and output addressing (`$Label>output`) replace all chain functionality. Wildcard auto-wire (`<* << $A>*`) recovers chain-style auto-wiring as a general feature (issue #345).

## Before

Chains used `->` to compose sequential pipeline calls with automatic IO wiring:

- `->` operator connected pipeline steps
- `($)` and `(.)` step labels identified chain positions
- `.N!ErrorName` syntax addressed errors from specific chain steps
- EBNF had `chain_call`, `step_ref`, `chain_io_param`, `chain_error_block` productions
- Compile rules PGE07002, PGE08006, PGE10007 governed chain behavior

## After

Sequential composition uses existing language primitives:

- `[-]` marker declares sequential pipeline calls
- `(-) $Label` names each call as an operation label
- `$Label>output` and `$Label<input` address step IO
- `<* << $A>*` wildcard auto-wire (issue #345) passes all outputs from one step as inputs to the next
- Chain-specific EBNF productions removed
- Chain-specific compile rules (PGE07002, PGE08006, PGE10007) retired

## Impact

- 11 files modified across docs/
- EBNF: 4 productions removed (`chain_call`, `step_ref`, `chain_io_param`, `chain_error_block`)
- Compile rules: 3 retired (PGE07002, PGE08006, PGE10007)
- `chains.md` rewritten to document labeled `[-]` call patterns
- No new syntax introduced — all replacement features already existed

## Rationale

Three mechanisms already handled everything chains did:

1. **`[-]` marker** — declares sequential intent
2. **`(-) $Label` + `$Label>output`** — step naming and IO addressing
3. **Trigger model** — pipelines launch when inputs reach Final

Chains were a redundant syntax overlay that added grammar complexity without enabling new capabilities. Removing them simplified the EBNF, reduced the compile rule surface, and made sequential composition consistent with the rest of the language.

The wildcard auto-wire feature (`<* << $A>*`, issue #345) was extracted as a general-purpose mechanism available everywhere, not just in chains.

## Related

- [[decisions/2026-04-22-constructor-blocks|Constructor blocks]] — another syntax simplification from the same period
- GitHub: #340 (chain retirement), #345 (wildcard auto-wire)
- Brainstorm: `_bmad-output/brainstorming/brainstorming-session-2026-04-21-002.md`
