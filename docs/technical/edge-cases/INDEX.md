---
audience: developer
type: reference
updated: 2026-03-30
---

# EBNF Edge Cases

<!-- @EBNF -->
Structured edge case catalog for validating the Polyglot Code grammar ([[technical/ebnf/INDEX|EBNF]]). Each case maps to an EBNF production rule and tests a boundary condition, ambiguity, or interaction between rules.

## How to Use

Review in batches by section. Each edge case includes:
- **EBNF ref** — production rule being tested
- **What it tests** — the boundary or ambiguity
- **Cross-refs** — related spec files that govern the behavior
- **Example** — minimal Polyglot Code snippet

---

## Section Files

| File | Section | Edge Cases |
|------|---------|------------|
| [01-file-structure.md](01-file-structure.md) | S1. File Structure | EC-1.1, EC-1.2 |
| [02-lexical.md](02-lexical.md) | S2. Lexical Elements | EC-2.1--2.4 |
| [03-identifiers.md](03-identifiers.md) | S3. Identifiers | EC-3.1--3.7 |
| [04-type-system.md](04-type-system.md) | S4. Type System | EC-4.1--4.18 |
| [05-block-elements.md](05-block-elements.md) | S5. Block Elements | EC-5.1--5.2 |
| [06-operators.md](06-operators.md) | S6. Operators | EC-6.1--6.4 |
| [07-io-parameters.md](07-io-parameters.md) | S7. IO Parameters | EC-7.1 |
| [08-expressions.md](08-expressions.md) | S8. Expressions | EC-8.1--8.6 |
| [09-definition-blocks.md](09-definition-blocks.md) | S9. Definition Blocks | EC-9.1--9.18 |
| [10-execution.md](10-execution.md) | S10. Execution Statements | EC-10.1--10.13 |
| [11-control-flow.md](11-control-flow.md) | S11. Control Flow | EC-11.1--11.6 |
| [12-collections.md](12-collections.md) | S12. Collection Operations | EC-12.1--12.13 |
| [13-comments.md](13-comments.md) | S13. Comments | EC-13.1--13.3 |
| [14-lifecycle.md](14-lifecycle.md) | S14. Variable Lifecycle | EC-14.1--14.4 |
| [15-metadata-blocks.md](15-metadata-blocks.md) | S15. Metadata Blocks | EC-15.1--15.4 |
| [16-trigger-io-wiring.md](16-trigger-io-wiring.md) | S16. Trigger IO Wiring | EC-16.1--16.3 |
| [17-negation-operators.md](17-negation-operators.md) | S17. Negation Operators | EC-17.1--17.2 |
| [18-macro-structure.md](18-macro-structure.md) | S18. Macro Structure | EC-18.1--18.4 |
| [19-collections-gaps.md](19-collections-gaps.md) | S19. Collections -- Gaps | EC-19.1--19.4 |
| [20-lifecycle-gaps.md](20-lifecycle-gaps.md) | S20. Variable Lifecycle -- Gaps | EC-20.1--20.3 |
| [21-registry-type.md](21-registry-type.md) | S21. Third Registry Type | EC-21.1 |
| [22-control-flow-gaps.md](22-control-flow-gaps.md) | S22. Control Flow -- Gaps | EC-22.1--22.4 |
| [23-stress-tests.md](23-stress-tests.md) | S23. Stress Tests | ST-1--ST-6 |
| [24-datatype-defs.md](24-datatype-defs.md) | S24. Datatype Definitions | EC-24.1--24.20 |

---

## Coverage Matrix

| EBNF Section | Edge Cases | Covered Productions |
|-------------|-----------|-------------------|
| S1 File Structure | EC-1.1, EC-1.2 | `file`, `definition` |
| S2 Lexical | EC-2.1--2.4 | `indent`, `bool_literal`, `int_literal`, `float_literal`, `string_literal` |
| S3 Identifiers | EC-3.1--3.7 | `package_address`, `cross_pkg_enum`, `cross_pkg_pipeline`, `field_path`, sibling homogeneity |
| S4 Types | EC-4.1--4.18 | `array_type`, `element_type`, `serial_type`, `user_type`, `inline_pipeline_call`, path types, multidimensional arrays |
| S5 Blocks | EC-5.1--5.2 | All block element categories, `[b]` background |
| S6 Operators | EC-6.1--6.4 | All assignment ops, all comparison ops, range ops, arithmetic |
| S7 IO | EC-7.1 | `input_param` with field separators |
| S8 Expressions | EC-8.1--8.6 | `inline_data`, empty `{}`, chained arithmetic, discard default restriction, self-assignment detection |
| S9 Definitions | EC-9.1--9.18 | Package imports, enum/value fields, pipeline structure, triggers, IO modes, macro parallel fork, empty definitions, trigger anomalies |
| S10 Execution | EC-10.1--10.13 | Pipeline call + error, stdlib call, chain execution, chain IO, chain auto-wire, chain errors, serial load, parallel, effectless exec_expr restriction |
| S11 Control Flow | EC-11.1--11.3 | Conditional chains, error scoping, logical operators |
| S12 Collections | EC-12.1--12.13 | All expand variants, all collect variants, direct output, multiple collectors, sync/race collectors, multi-wave, [*] <</>>/semantics |
| S13 Comments | EC-13.1--13.3 | Square, curly, multiline |
| S14 Lifecycle | EC-14.1--14.4 | Default->Final, Final immutability, leaf-only, all-or-none |
| S15 Metadata Blocks | EC-15.1--15.4 | `[%]` user fields, alias, `.info#serial`, `%` live accessor |
| S16 Trigger IO Wiring | EC-16.1--16.3 | Trigger outputs, multi-output wiring, mixed fill modes |
| S17 Negation Operators | EC-17.1--17.2 | `<!?`, `>!?`, `<=!?`, `>=!?`, negation in compound logic |
| S18 Macro Structure | EC-18.1--18.4 | `{M}` full structure, `[W]` usage wiring, no-output macro, zero-param macro |
| S19 Collections -- Gaps | EC-19.1--19.4 | `*Into.Serial`, `*Into.Level`, `*Second`, `[b]` collector |
| S20 Lifecycle -- Gaps | EC-20.1--20.3 | Declared state, Released state, `~>` on output params |
| S21 Third Registry Type | EC-21.1 | `Registry` address format |
| S22 Control Flow -- Gaps | EC-22.1--22.4 | `*?` exhaustiveness, nested conditionals, `%status` switch, `[^]` XOR |
| S23 Stress Tests | ST-1--ST-6 | Full onboarding, complex conditionals, race+chain, multi-wave+expand, deep nesting, macro+timer |
| S24 Datatype Definitions | EC-24.1--24.18 | Scalar regex boundaries, `<~` inheritance, ##/### composition, collection parameterized inheritance, %## property completeness |

**Total: 51 original + 33 new + 18 datatype + 2 exec_expr + 3 assignment + 5 empty-def + 3 trigger = 115 edge cases across 24 sections.**
