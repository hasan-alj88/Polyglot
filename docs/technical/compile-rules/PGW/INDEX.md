---
audience: developer
type: spec-index
updated: 2026-04-23
status: stable
---

<!-- @compile-rules/PGW/INDEX -->

# Polyglot Warning Codes (PGW)

Compile warnings surface concerns without halting compilation. Rules are grouped by thousand-range, paralleling the PGE categories but covering narrower or softer conditions. See the [[../INDEX|Compile Rules Index]] for the full directory map and the [[../PGE/INDEX|PGE Index]] for the error-severity counterparts.

**Code scheme:** `PGW<range><ordinal>` — `<range>` is a two-digit category bucket (thousand-range), `<ordinal>` is the three-digit position within that range. Files are named `PGW<code>-<slug>.md` and link to this index via `[[../INDEX|Compile Rules]]`.

## PGW01 — Pipeline structure warnings

| Code | Name | Link |
|------|------|------|
| PGW01001 | Empty Execution Body | [[PGW01001-empty-execution-body]] |
| PGW01003 | No Definitions in File | [[PGW01003-no-definitions-in-file]] |
| PGW01004 | Orphaned Foreign Code | [[PGW01004-orphaned-foreign-code]] |

## PGW02 — Variable lifecycle warnings

| Code | Name | Link |
|------|------|------|
| PGW02001 | Default Pull Across State Change | [[PGW02001-default-pull-across-state-change]] |
| PGW02002 | Unused Variable | [[PGW02002-unused-variable]] |
| PGW02003 | Unpushed Output Port | [[PGW02003-unpushed-output-port]] |
| PGW02004 | Pipeline Terminates on Error | [[PGW02004-failed-variable-usage]] |
| PGW02005 | Unreachable Code | [[PGW02005-unreachable-code]] |

## PGW03 — Parallel warnings

| Code | Name | Link |
|------|------|------|
| PGW03001 | "[b] Called Pipeline Has Discarded Outputs" | [[PGW03001-b-discarded-outputs]] |
| PGW03002 | Error Handler on Fire-and-Forget | [[PGW03002-error-handler-on-fire-and-forget]] |

## PGW04 — Type-system warnings

| Code | Name | Link |
|------|------|------|
| PGW04001 | Single-Platform Path | [[PGW04001-single-platform-path]] |
| PGW04002 | Leading Zeros in Literal | [[PGW04002-leading-zeros-in-literal]] |

## PGW07 — Error-handling warnings

| Code | Name | Link |
|------|------|------|
| PGW07001 | Error Handler on Non-Failable Call | [[PGW07001-error-handler-on-non-failable-call]] |
| PGW07002 | Caller Overrides Pipeline Fallback | [[PGW07002-caller-overrides-pipeline-fallback]] |
| PGW07003 | Missing Fallback Message | [[PGW07003-missing-fallback-message]] |
| PGW07004 | Fallback on Non-Failable IO | [[PGW07004-fallback-on-non-failable-io]] |
| PGW07010 | Suppress on Consumed Output | [[PGW07010-suppress-on-consumed-output]] |

## PGW08 — Auto-wire warnings

| Code | Name | Link |
|------|------|------|
| PGW08001 | Auto-Wire Succeeded | [[PGW08001-auto-wire-succeeded]] |
| PGW08002 | Unaddressed Input With Default | [[PGW08002-unaddressed-input-with-default]] |
| PGW08003 | Uncaptured Output With Default/Fallback | [[PGW08003-uncaptured-output-with-default]] |

## PGW09 — Import warnings

| Code | Name | Link |
|------|------|------|
| PGW09001 | Deprecated Pipeline Reference | [[PGW09001-deprecated-pipeline-reference]] |
| PGW09002 | Unused Import | [[PGW09002-unused-import]] |

## PGW10 — Permission warnings

| Code | Name | Link |
|------|------|------|
| PGW10001 | Unused Permission | [[PGW10001-unused-permission]] |
| PGW10002 | Unverifiable Foreign IO | [[PGW10002-unverifiable-foreign-io]] |
| PGW10003 | Bind Mode Opacity | [[PGW10003-bind-mode-opacity]] |
| PGW10005 | Unrecognized Foreign Call | [[PGW10005-unrecognized-foreign-call]] |
| PGW10006 | Shell Variable Expansion | [[PGW10006-shell-variable-expansion]] |
| PGW10007 | Sandbox-Only Enforcement Active | [[PGW10007-sandbox-only-active]] |

## PGW12 — Metadata warnings

| Code | Name | Link |
|------|------|------|
| PGW12001 | Template With No Placeholders | [[PGW12001-missing-inline-format-metadata]] |
| PGW12002 | Optional Placeholder Never Provided | [[PGW12002-optional-placeholder-never-provided]] |

---

## Range Categories

| Range | Description |
|-------|-------------|
| **PGW01** | Pipeline structure warnings |
| **PGW02** | Variable lifecycle warnings |
| **PGW03** | Parallel warnings |
| **PGW04** | Type-system warnings |
| **PGW07** | Error-handling warnings |
| **PGW08** | Auto-wire warnings |
| **PGW09** | Import warnings |
| **PGW10** | Permission warnings |
| **PGW12** | Metadata warnings |

---

## Not-Yet-Assigned Ranges

The following thousand-ranges have no warnings currently defined (the corresponding PGE range may still exist):

- **PGW05xxx** — no warnings currently defined (PGE05 covers data-definition errors).
- **PGW06xxx** — no warnings currently defined (PGE06 covers conditional exhaustiveness errors).
- **PGW11xxx** — range not assigned.
- **PGW13xxx** — range not assigned.
- **PGW14xxx** — no warnings currently defined (PGE14 covers constructor-block errors).

Assign a new PGW range only after confirming no existing rule belongs in a neighboring one.
