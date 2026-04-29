---
audience: developer
type: spec-index
updated: 2026-04-23
status: stable
---

<!-- @compile-rules/PGE/INDEX -->

# Aljam3 Error Codes (PGE)

Compile errors halt compilation. Each rule below defines a condition under which the Aljam3 compiler refuses to produce an executable. Rules are grouped by thousand-range category; see the [[../INDEX|Compile Rules Index]] for the full directory map.

**Code scheme:** `PGE<range><ordinal>` — `<range>` is a two-digit category bucket (thousand-range), `<ordinal>` is the three-digit position within that range. Files are named `PGE<code>-<slug>.md` and link to this index via `[[../INDEX|Compile Rules]]`.

## PGE01 — Pipeline structure (section ordering, block elements, IO binding)

| Code | Name | Link |
|------|------|------|
| PGE01001 | Pipeline Section Misordering | [[PGE01001-pipeline-execution-order]] |
| PGE01002 | IO Before Trigger | [[PGE01002-io-before-trigger]] |
| PGE01003 | One Package Declaration Per File | [[PGE01003-one-package-per-file]] |
| PGE01004 | Wrapper Structural Constraints | [[PGE01004-macro-structural-constraints]] |
| PGE01005 | Missing Pipeline Trigger | [[PGE01005-missing-trigger]] |
| PGE01006 | Missing Pipeline Queue | [[PGE01006-missing-queue]] |
| PGE01007 | Missing Pipeline Setup/Cleanup | [[PGE01007-missing-setup-cleanup]] |
| PGE01008 | Wrapper Must Reference Wrapper Definition | [[PGE01008-wrapper-must-reference-macro]] |
| PGE01009 | Wrapper IO Mismatch | [[PGE01009-wrapper-io-mismatch]] |
| PGE01010 | Pipeline IO Name Mismatch | [[PGE01010-pipeline-io-name-mismatch]] |
| PGE01011 | Duplicate IO Parameter Name | [[PGE01011-duplicate-io-parameter-name]] |
| PGE01012 | Queue Definition Must Use #Queue: Prefix | [[PGE01012-queue-definition-prefix]] |
| PGE01013 | Queue Control Contradicts Queue Default | [[PGE01013-queue-control-contradiction]] |
| PGE01014 | Unresolved Queue Reference | [[PGE01014-unresolved-queue-reference]] |
| PGE01015 | Duplicate Metadata Field | [[PGE01015-duplicate-metadata-field]] |
| PGE01016 | Unmarked Execution Line | [[PGE01016-unmarked-execution-line]] |
| PGE01017 | Wrong Block Element Marker | [[PGE01017-wrong-block-element-marker]] |
| PGE01018 | Tautological or Contradictory Trigger Condition | [[PGE01018-tautological-trigger-condition]] |
| PGE01020 | Effectless Execution Expression | [[PGE01020-effectless-execution-expression]] |
| PGE01021 | Empty Data Definition | [[PGE01021-empty-data-definition]] |
| PGE01022 | Empty Error Namespace | [[PGE01022-empty-error-namespace]] |
| PGE01023 | "Retired — Parameterless Macro" | [[PGE01023-parameterless-macro]] |
| PGE01024 | Incompatible Operation Marker | [[PGE01024-incompatible-operation-marker]] |
| PGE01025 | Discard in Wrapper IO | [[PGE01025-discard-in-wrapper-io]] |
| PGE01026 | Orphan Continuation Line | [[PGE01026-orphan-continuation]] |
| PGE01027 | Empty Foreign Code Block | [[PGE01027-empty-foreign-code]] |
| PGE01028 | Native/Derived Block Mutual Exclusion | [[PGE01028-base-derived-mutual-exclusion]] |
| PGE01029 | Invalid Marker for Definition Type | [[PGE01029-invalid-marker-for-definition-type]] |
| PGE01030 | Missing Pipeline Wrapper | [[PGE01030-missing-pipeline-wrapper]] |
| PGE01031 | Forbidden Element in Definition | [[PGE01031-forbidden-element-in-definition]] |
| PGE01032 | Missing Trigger Boolean Output | [[PGE01032-missing-trigger-boolean-output]] |
| PGE01033 | Unbound Script Variable | [[PGE01033-unbound-script-variable]] |
| PGE01034 | Unbound Script Output | [[PGE01034-unbound-script-output]] |
| PGE01035 | Unbound Function Argument | [[PGE01035-unbound-function-argument]] |
| PGE01036 | Unbound Function Kwarg | [[PGE01036-unbound-function-kwarg]] |
| PGE01037 | Bind Schema Mismatch | [[PGE01037-bind-schema-mismatch]] |
| PGE01038 | Code Source Conflict | [[PGE01038-code-source-conflict]] |
| PGE01039 | CLI Non-String Argument | [[PGE01039-cli-non-string-argument]] |
| PGE01040 | Orphan Parallel Marker | [[PGE01040-orphan-parallel-marker]] |

## PGE02 — Variable lifecycle (states, push-once, unreachable code)

| Code | Name | Link |
|------|------|------|
| PGE02001 | Lifecycle Stages | [[PGE02001-lifecycle-stages]] |
| PGE02002 | Declared State Is Unreadable | [[PGE02002-declared-state-unreadable]] |
| PGE02003 | Final Is Push-Once | [[PGE02003-final-is-push-once]] |
| PGE02005 | Failed Must Resolve | [[PGE02005-failed-is-terminal]] |
| PGE02006 | Live Metadata Fields Are Pull-Only | [[PGE02006-metadata-pull-only]] |
| PGE02008 | Access After Release | [[PGE02008-access-after-release]] |
| PGE02009 | Unreachable Code | [[PGE02009-unreachable-code]] |
| PGE02010 | Discard Default Assignment | [[PGE02010-discard-default-assignment]] |
| PGE02011 | Data Load Schema Mismatch | [[PGE02011-data-load-schema-mismatch]] |
| PGE02012 | Duplicate Operation Label | [[PGE02012-duplicate-operation-label]] |
| PGE02013 | Write To Label Accessor | [[PGE02013-write-to-label-accessor]] |
| PGE02014 | Label Access Before Completion | [[PGE02014-label-access-before-completion]] |
| PGE02015 | Unused Background Label | [[PGE02015-unused-background-label]] |

## PGE03 — Parallelism and collection (parallel writes, expand/collect)

| Code | Name | Link |
|------|------|------|
| PGE03001 | No Push Across Parallel Boundaries | [[PGE03001-no-push-across-parallel]] |
| PGE03002 | Parallel Output Must Be Collected | [[PGE03002-parallel-output-must-be-collected]] |
| PGE03003 | Variable Isolation Until Collection | [[PGE03003-variable-isolation-until-collection]] |
| PGE03004 | Section-Boundary Pairing | [[PGE03004-section-boundary-pairing]] |
| PGE03005 | "[b] Has No Collectible Output" | [[PGE03005-b-no-collectible-output]] |
| PGE03006 | Race Collector Type Homogeneity | [[PGE03006-race-collector-type-homogeneity]] |
| PGE03007 | Expand Operator Input Mismatch | [[PGE03007-expand-operator-input-mismatch]] |
| PGE03008 | Collect Operator IO Mismatch | [[PGE03008-collect-operator-io-mismatch]] |
| PGE03009 | Nested Expand Without Collect | [[PGE03009-nested-expand-without-collect]] |
| PGE03010 | Collector Without Expand | [[PGE03010-collector-without-expand]] |
| PGE03011 | Orphaned Expand IO Marker | [[PGE03011-orphaned-expand-io-marker]] |
| PGE03012 | Parallel Label Isolation | [[PGE03012-parallel-label-isolation]] |
| PGE03013 | Collector Metadata Required | [[PGE03013-collector-metadata-required]] |
| PGE03014 | Expand-Scoped Collector Outside ForEach | [[PGE03014-expand-scope-outside-foreach]] |
| PGE03015 | Parallel-Scoped Collector Inside ForEach | [[PGE03015-parallel-scope-inside-foreach]] |
| PGE03016 | Collector IO Mismatch | [[PGE03016-collector-io-mismatch]] |
| PGE03017 | Arrival Operations Outside Collector Context | [[PGE03017-arrival-ops-outside-collector]] |
| PGE03018 | Missing Incoming DataFrame | [[PGE03018-missing-incoming-dataframe]] |
| PGE03019 | Arrival Index Out of Bounds | [[PGE03019-arrival-index-out-of-bounds]] |
| PGE03020 | Statements Outside Trigger Blocks | [[PGE03020-statements-outside-trigger-blocks]] |
| PGE03021 | Parallel Execution Inside Collector | [[PGE03021-parallel-inside-collector]] |
| PGE03022 | External Trigger Source in Collector | [[PGE03022-external-trigger-in-collector]] |
| PGE03023 | Overflow Strategy Missing Storage Error | [[PGE03023-overflow-missing-storage-error]] |
| PGE03024 | Release With No Remaining Claims | [[PGE03024-release-no-remaining-claims]] |
| PGE03025 | Not All Jobs Released | [[PGE03025-not-all-jobs-released]] |

## PGE04 — Type system (type and schema mismatches)

| Code | Name | Link |
|------|------|------|
| PGE04001 | Type Mismatch | [[PGE04001-type-mismatch]] |
| PGE04002 | Schema Mismatch | [[PGE04002-schema-mismatch]] |
| PGE04003 | Leaf-Only Assignment | [[PGE04003-leaf-only-assignment]] |
| PGE04004 | Fixed-Schema Keys Are Compile-Time Only | [[PGE04004-fixed-schema-keys-compile-time]] |
| PGE04005 | Undefined Interpolation Variable | [[PGE04005-undefined-interpolation-variable]] |
| PGE04006 | Undefined Variable Reference | [[PGE04006-undefined-variable-reference]] |
| PGE04007 | Invalid Path String | [[PGE04007-invalid-path-string]] |
| PGE04008 | Missing Path Platform Subfield | [[PGE04008-missing-path-platform]] |
| PGE04009 | Unhandled Serial→Struct Conversion | [[PGE04009-unhandled-serial-struct-conversion]] |
| PGE04010 | Invalid Arithmetic Operator | [[PGE04010-invalid-arithmetic-operator]] |
| PGE04011 | Negative Array Index Literal | [[PGE04011-negative-array-index-literal]] |
| PGE04012 | Division by Literal Zero | [[PGE04012-division-by-literal-zero]] |
| PGE04013 | Nested Array Type | [[PGE04013-nested-array-type]] |
| PGE04014 | Invalid Range Bounds | [[PGE04014-invalid-range-bounds]] |
| PGE04015 | Conditional Type-Operator Mismatch | [[PGE04015-conditional-type-operator-mismatch]] |
| PGE04016 | Invalid Pipeline Input Literal | [[PGE04016-invalid-pipeline-input-literal]] |
| PGE04017 | Array Dimension Access Mismatch | [[PGE04017-array-dimension-access-mismatch]] |
| PGE04024 | Non-Value Comparison | [[PGE04024-non-value-comparison]] |
| PGE04025 | Untyped Array | [[PGE04025-untyped-array]] |
| PGE04026 | Invalid IANA Timezone | [[PGE04026-invalid-iana-timezone]] |
| PGE04027 | Missing Required DateTime Subfield | [[PGE04027-missing-required-datetime-subfield]] |
| PGE04028 | Invalid Epoch Value | [[PGE04028-invalid-epoch-value]] |

## PGE05 — Data definitions (separator/kind homogeneity, recursion)

| Code | Name | Link |
|------|------|------|
| PGE05001 | Sibling Separator Homogeneity | [[PGE05001-sibling-separator-homogeneity]] |
| PGE05002 | Sibling Kind Homogeneity | [[PGE05002-sibling-kind-homogeneity]] |
| PGE05003 | Duplicate Data Field Name | [[PGE05003-duplicate-data-field-name]] |
| PGE05004 | Recursive Data Definition | [[PGE05004-recursive-data-definition]] |

## PGE06 — Conditionals and exhaustiveness

| Code | Name | Link |
|------|------|------|
| PGE06001 | Conditional Must Be Exhaustive | [[PGE06001-conditional-must-be-exhaustive]] |
| PGE06002 | Enum Exhaustiveness | [[PGE06002-enum-exhaustiveness]] |
| PGE06003 | Numeric Range Not Exhaustive | [[PGE06003-numeric-range-not-exhaustive]] |
| PGE06004 | Numeric Range Overlap | [[PGE06004-numeric-range-overlap]] |
| PGE06005 | Compound Condition Overlap | [[PGE06005-compound-condition-overlap]] |
| PGE06006 | String Exhaustiveness | [[PGE06006-string-exhaustiveness]] |
| PGE06007 | Flexible Field Exhaustiveness | [[PGE06007-flexible-field-exhaustiveness]] |
| PGE06008 | Compound Condition Exhaustiveness | [[PGE06008-compound-condition-exhaustiveness]] |
| PGE06009 | Conditional Missing Comparison Operator | [[PGE06009-conditional-missing-comparison-operator]] |
| PGE06010 | Empty Conditional Scope | [[PGE06010-empty-conditional-scope]] |
| PGE06011 | Duplicate Wildcard Catch-All | [[PGE06011-duplicate-wildcard-catch-all]] |
| PGE06012 | Unreachable Branch After Wildcard | [[PGE06012-unreachable-branch-after-wildcard]] |
| PGE06013 | Tautological or Contradictory Branch Condition | [[PGE06013-tautological-branch-condition]] |
| PGE06014 | Wildcard-Only Match | [[PGE06014-wildcard-only-match]] |

## PGE07 — Error handling and fallbacks

| Code | Name | Link |
|------|------|------|
| PGE07001 | Error Block Scoping | [[PGE07001-error-block-scoping]] |
| PGE07002 | Chain Error Scoping | [[PGE07002-chain-error-scoping]] |
| PGE07003 | Duplicate Fallback Assignment | [[PGE07003-duplicate-fallback-assignment]] |
| PGE07004 | Duplicate Error Handler | [[PGE07004-duplicate-error-handler]] |
| PGE07005 | Undeclared Error Raise | [[PGE07005-undeclared-error-raise]] |
| PGE07006 | Unused Error Declaration | [[PGE07006-unused-error-declaration]] |
| PGE07007 | Error Handling Must Be Exhaustive | [[PGE07007-error-handling-must-be-exhaustive]] |
| PGE07008 | Fallback on Non-Failable Source | [[PGE07008-fallback-on-non-failable-source]] |
| PGE07009 | Unterminated Fallback Chain | [[PGE07009-unterminated-fallback-chain]] |

## PGE08 — Auto-wire and IO assignment

| Code | Name | Link |
|------|------|------|
| PGE08001 | Auto-Wire Type Mismatch | [[PGE08001-auto-wire-type-mismatch]] |
| PGE08002 | Auto-Wire Ambiguous Type | [[PGE08002-auto-wire-ambiguous-type]] |
| PGE08003 | Auto-Wire Port Count Mismatch | [[PGE08003-auto-wire-unmatched-parameter]] |
| PGE08004 | Ambiguous Step Reference | [[PGE08004-ambiguous-step-reference]] |
| PGE08005 | Unresolved Step Reference | [[PGE08005-unresolved-step-reference]] |
| PGE08006 | Non-Pipeline Step in Chain | [[PGE08006-non-pipeline-step-in-chain]] |
| PGE08007 | Invalid Assignment Target | [[PGE08007-invalid-assignment-target]] |
| PGE08008 | Missing Required Input at Call Site | [[PGE08008-missing-required-input]] |
| PGE08009 | Uncaptured Required Output at Call Site | [[PGE08009-uncaptured-required-output]] |
| PGE08010 | IO Direction Mismatch | [[PGE08010-io-direction-mismatch]] |
| PGE08011 | Self-Assignment | [[PGE08011-self-assignment]] |
| PGE08012 | Self-Chain Requires Numeric Indexing | [[PGE08012-self-chain-requires-indexing]] |
| PGE08013 | Nested Inline Data | [[PGE08013-nested-inline-data]] |

## PGE09 — Packages and imports

| Code | Name | Link |
|------|------|------|
| PGE09001 | Undefined Import Alias | [[PGE09001-undefined-import-alias]] |
| PGE09002 | Circular Package Dependency | [[PGE09002-circular-package-dependency]] |
| PGE09003 | Unresolved Pipeline Reference | [[PGE09003-unresolved-pipeline-reference]] |
| PGE09004 | Unresolved Import Pipeline Reference | [[PGE09004-unresolved-import-pipeline-reference]] |
| PGE09005 | Multi-File Version Mismatch | [[PGE09005-multi-file-version-mismatch]] |
| PGE09006 | Multi-File Package Name Mismatch | [[PGE09006-multi-file-package-name-mismatch]] |
| PGE09007 | Duplicate Definition | [[PGE09007-duplicate-pipeline-definition]] |
| PGE09008 | Multi-File Reference Not Found | [[PGE09008-multi-file-reference-not-found]] |
| PGE09009 | Multi-File Self-Reference | [[PGE09009-multi-file-self-reference]] |
| PGE09010 | Asymmetric Multi-File Reference | [[PGE09010-asymmetric-multi-file-reference]] |
| PGE09011 | Duplicate Import Alias | [[PGE09011-duplicate-import-alias]] |
| PGE09012 | Import Alias Shadows Standard Library | [[PGE09012-import-alias-shadows-pglib]] |
| PGE09013 | Circular Pipeline Call | [[PGE09013-circular-pipeline-call]] |

## PGE10 — Permissions and sandbox

| Code | Name | Link |
|------|------|------|
| PGE10003 | Unknown Permission Category | [[PGE10003-unknown-permission-category]] |
| PGE10004 | Undeclared Permission | [[PGE10004-undeclared-permission]] |
| PGE10005 | Invalid Permission Block Marker | [[PGE10005-permission-output]] |
| PGE10006 | Duplicate Permission | [[PGE10006-duplicate-permission]] |
| PGE10007 | Chain Step Label Overflow | [[PGE10007-chain-step-label-overflow]] |
| PGE10008 | Parallel Write Permission Exclusion | [[PGE10008-parallel-write-permission-exclusion]] |
| PGE10009 | Unresolved Permission Template | [[PGE10009-unresolved-permission-template]] |
| PGE10010 | Permission Resource Not Found | [[PGE10010-permission-resource-not-found]] |
| PGE10011 | Shell Without Capability | [[PGE10011-shell-without-capability]] |
| PGE10012 | Code File Outside Scope | [[PGE10012-code-file-outside-scope]] |
| PGE10013 | Foreign Resource Outside Scope | [[PGE10013-foreign-resource-outside-scope]] |
| PGE10014 | AST-Invisible Foreign Code | [[PGE10014-ast-invisible-foreign-code]] |
| PGE10015 | Opaque Binary Without Sandbox Acknowledgment | [[PGE10015-opaque-binary-without-sandbox-only]] |
| PGE10016 | Missing Mandatory Metadata for Sandbox-Only | [[PGE10016-missing-unsafe-metadata]] |

## PGE12 — Metadata and inline templates

| Code | Name | Link |
|------|------|------|
| PGE12001 | Undefined Metadata Field Access | [[PGE12001-undefined-metadata-field-access]] |
| PGE12002 | Duplicate Alias | [[PGE12002-duplicate-alias]] |
| PGE12003 | Undefined Inline Template | [[PGE12003-invalid-inline-pipeline-argument]] |
| PGE12004 | Empty Metadata Alias | [[PGE12004-empty-metadata-alias]] |
| PGE12005 | Inline Format Mismatch | [[PGE12005-inline-format-mismatch]] |
| PGE12006 | Unresolved Template Placeholder | [[PGE12006-unresolved-template-placeholder]] |
| PGE12007 | Required Input Not In Template | [[PGE12007-required-input-not-in-template]] |
| PGE12008 | Duplicate Template Placeholder | [[PGE12008-duplicate-template-placeholder]] |
| PGE12009 | Template Type Coercion Failure | [[PGE12009-template-type-coercion-failure]] |
| PGE12010 | Optional Placeholder Without Default | [[PGE12010-optional-placeholder-without-default]] |

## PGE14 — Constructor blocks ({$})

| Code | Name | Link |
|------|------|------|
| PGE14001 | Ambiguous Constructor Overload | [[PGE14001-ambiguous-constructor-overload]] |
| PGE14002 | Duplicate Constructor Keyword | [[PGE14002-duplicate-constructor-keyword]] |
| PGE14003 | Missing Capture Regex | [[PGE14003-missing-capture-regex]] |
| PGE14004 | Structural Integrity Violation | [[PGE14004-structural-integrity-violation]] |
| PGE14005 | Target Type Mismatch | [[PGE14005-target-type-mismatch]] |
| PGE14006 | Failable Pipeline In Constructor | [[PGE14006-failable-pipeline-in-constructor]] |
| PGE14007 | Incomplete Field Mapping | [[PGE14007-incomplete-field-mapping]] |
| PGE14010 | No Constructor Overload Match | [[PGE14010-no-constructor-overload-match]] |
| PGE14011 | Non-Literal Interpolation | [[PGE14011-non-literal-interpolation]] |
| PGE14012 | Undefined Constructor | [[PGE14012-undefined-constructor]] |
| PGE14013 | Interpolation Source Not Final | [[PGE14013-interpolation-source-not-final]] |

---

## Range Categories

| Range | Description |
|-------|-------------|
| **PGE01** | Pipeline structure (section ordering, block elements, IO binding) |
| **PGE02** | Variable lifecycle (states, push-once, unreachable code) |
| **PGE03** | Parallelism and collection (parallel writes, expand/collect) |
| **PGE04** | Type system (type and schema mismatches) |
| **PGE05** | Data definitions (separator/kind homogeneity, recursion) |
| **PGE06** | Conditionals and exhaustiveness |
| **PGE07** | Error handling and fallbacks |
| **PGE08** | Auto-wire and IO assignment |
| **PGE09** | Packages and imports |
| **PGE10** | Permissions and sandbox |
| **PGE12** | Metadata and inline templates |
| **PGE14** | Constructor blocks ({$}) |

---

## Reserved / Retired Ranges

- **PGE11xxx** — currently unused. Do not reuse this range for new rules; it is reserved for future categorization (or retired from a prior scheme). Check the [[../INDEX|Compile Rules Index]] before assigning.
- **PGE13xxx** — currently unused, same policy as PGE11xxx.
