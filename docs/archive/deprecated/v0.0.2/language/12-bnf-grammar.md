---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: docs/technical/ebnf/INDEX.md
---

# BNF Grammar Specification

**Version:** 0.0.2
**Last Updated:** 2025-11-16
**Status:** Complete

---

## Overview

This document provides **two formal BNF grammars** for the Polyglot language v0.0.2, corresponding to the two compilation phases:

1. **Surface Syntax Grammar (Phase 1)** - Compact form users write
2. **Canonical Syntax Grammar (Phase 2)** - Full render form after macro expansion

### Compilation Pipeline

```
User Source Code
      ↓
[Phase 1: Parse]
   Uses: Surface Syntax Grammar
   Output: Surface AST
      ↓
[Phase 2: Transform]
   - Macro expansion
   - Default insertions
   - Element reordering
   Output: Canonical AST
      ↓
[Phase 3: Validate]
   Uses: Canonical Syntax Grammar
   Output: Validated IR
      ↓
[Phase 4: Code Generation]
```

**Important:** This grammar is based on the v0.0.2 audit documents and language specification, resolving all inconsistencies from v0.0.1.

---

## Table of Contents

1. [Notation Conventions](#notation-conventions)
2. [Phase 1: Surface Syntax Grammar](#phase-1-surface-syntax-grammar)
3. [Phase 2: Canonical Syntax Grammar](#phase-2-canonical-syntax-grammar)
4. [Transformation Rules](#transformation-rules)
5. [Validation Constraints](#validation-constraints)
6. [Lexical Elements](#lexical-elements)
7. [Type System](#type-system)
8. [Operators](#operators)
9. [Literals](#literals)
10. [Complete Grammar Reference](#complete-grammar-reference)

---

## Notation Conventions

### BNF Metasyntax

This grammar uses Extended Backus-Naur Form (EBNF) notation:

| Symbol | Meaning |
|--------|---------|
| `::=` | Definition |
| `|` | Alternation (or) |
| `[ ]` | Optional (zero or one) |
| `{ }` | Repetition (zero or more) |
| `( )` | Grouping |
| `" "` | Literal terminal |
| `< >` | Non-terminal symbol |
| `/* */` | Grammar comment |

### Terminology

- **Terminal**: Literal token that appears in source code
- **Non-terminal**: Abstract syntax element defined by production rules
- **Production**: Grammar rule defining how non-terminals expand
- **Surface Syntax**: Compact form users write (pre-transformation)
- **Canonical Syntax**: Full render form after transformations (post-validation)

---

# Phase 1: Surface Syntax Grammar

**Purpose:** Defines what users can write in source code. This is the **compact form** where:
- Elements can appear in any order
- Some required elements may be omitted (provided by macros)
- Flexible and permissive

**Used By:** Parser (lexer → parser → surface AST)

---

## Program Structure (Surface)

```bnf
<program> ::= <package-declaration-block>
              { <top-level-element> }

/* CRITICAL: Every .pg file MUST start with exactly ONE package declaration block
   This declares which package this file belongs to and its dependencies */

<package-declaration-block> ::= "[@]" <package-spec>
                                [ <alias-declaration> ]
                                { <import-declaration> }
                                "[X]"

<package-spec> ::= <registry-path> "::" <version>

<registry-path> ::= <registry-tier> "@" <package-path>

<package-path> ::= <identifier> { "." <identifier> }

<version> ::= <major> "." <minor> "." <patch>

<major> ::= <digit> { <digit> }
<minor> ::= <digit> { <digit> }
<patch> ::= <digit> { <digit> }

<alias-declaration> ::= "[A]" <identifier>

<import-declaration> ::= "[<]" "@" <import-alias> "<<" <package-spec>

<import-alias> ::= <identifier>

<top-level-element> ::= <pipeline-definition>
                      | <enumeration-definition>
                      | <error-definition>
                      | <comment>

<comment> ::= <single-line-comment>
            | <multi-line-comment>

<single-line-comment> ::= "//" <text-until-eol>

<multi-line-comment> ::= "/*" <text-until-close> "*/"
```

---

## Pipeline Definition (Surface)

```bnf
<pipeline-definition> ::= "[|]" <pipeline-name>
                          { <pipeline-element> }
                          "[X]"

<pipeline-name> ::= <identifier>

<pipeline-element> ::= <input-declaration>
                     | <output-declaration>
                     | <trigger-declaration>
                     | <queue-control>
                     | <wrapper-context>
                     | <operation>
                     | <parallel-block>
                     | <join-block>
                     | <error-handler>
                     | <switch-case>
                     | <comment>

/* Note: No ordering constraints in surface syntax.
   Elements can appear in any order user writes them.
   Validation happens after transformation to canonical form. */
```

---

## Input/Output Declarations (Surface)

```bnf
<input-declaration> ::= "[i]" <input-spec>
                      | <fixed-declaration>

<input-spec> ::= <required-input>
               | <default-input>

<required-input> ::= <field-name> ":" <type>

<default-input> ::= <field-name> ":" <type> "<<" <expression>

<fixed-declaration> ::= "[=]" <field-name> "<<" <expression>

<output-declaration> ::= "[o]" <output-spec>

<output-spec> ::= <field-name> ":" <type>
                | "#None"

<macro-export> ::= "[m]" <field-name>

/* Note: Keywords removed:
   - "Fixed" → use [=] block marker
   - "Default" → use inline << in [i] declaration
   - "Exposed" → use [m] block marker for macro exports
*/
```

---

## Operations (Surface)

```bnf
<operation> ::= <sequential-operation>
              | <parallel-operation>
              | <nested-operation>

<sequential-operation> ::= "[r]" <operation-body>

<parallel-operation> ::= "[p]" <operation-body>

<nested-operation> ::= "[~]" <operation>

<operation-body> ::= <pipeline-call>
                   | <variable-assignment>
                   | <unpack-operation>

<pipeline-call> ::= "|" <pipeline-reference>
                    { <input-assignment> }
                    { <output-extraction> }

<pipeline-reference> ::= <package-pipeline>
                       | <local-pipeline>
                       | <standard-lib-pipeline>

<package-pipeline> ::= "@" <package-name> "|" <identifier>

<local-pipeline> ::= <identifier>

<standard-lib-pipeline> ::= <namespace> "." <identifier>
                          | <namespace> "." <identifier> { "." <identifier> }

<namespace> ::= "W" | "Q" | "Y" | "U" | "T"

<variable-assignment> ::= <field-name> ":" <type> "<<" <expression>

<unpack-operation> ::= "~" <unpack-target>

<unpack-target> ::= <variable-name>
                  | <array-literal>
                  | <enumeration-reference>
                  | <unpack-pipeline>

<unpack-pipeline> ::= <identifier> { "." <identifier> }
```

---

## Input/Output Passing (Surface)

```bnf
<input-assignment> ::= "[<]" <field-name> ":" <type> "<<" <expression>

<output-extraction> ::= "[>]" <field-name> [ ":" <type> ] ">>" <variable-name>
```

---

## Parallel and Join (Surface)

```bnf
<parallel-block> ::= { <parallel-operation> }
                     <join-block>

<join-block> ::= "[Y]" <join-operation>
                 { <output-extraction> }

<join-operation> ::= "|Y.Join"
                   | "|Y.JoinAny"
                   | "|Y.JoinTimeout"
```

---

## Triggers (Surface)

```bnf
<trigger-declaration> ::= <simple-trigger>
                        | <logical-trigger>

<simple-trigger> ::= "[t]" <trigger-spec>

<logical-trigger> ::= <or-trigger>
                    | <and-trigger>
                    | <xor-trigger>
                    | <grouped-trigger>

<or-trigger> ::= "[+]" <trigger-spec>    /* OR - implicit grouping with above */

<and-trigger> ::= "[&]" <trigger-spec>   /* AND - explicit (or implicit at first level) */

<xor-trigger> ::= "[^]" <trigger-spec>   /* XOR - exclusive OR, implicit grouping */

<grouped-trigger> ::= <group-start> { <group-member> }
                    | <nested-group>

<group-start> ::= "[.]"  /* empty line - group marker */

<group-member> ::= "[~]" <trigger-declaration>

<nested-group> ::= <logical-operator> "[.]" { "[~][~]" <trigger-declaration> }

<logical-operator> ::= "[+]" | "[&]" | "[^]"

<trigger-spec> ::= "|T.Call"
                 | "|T.Cli"
                 | <scheduled-trigger>
                 | <file-trigger>
                 | <interval-trigger>

<scheduled-trigger> ::= "|T.Daily" { <input-assignment> }
                      | "|T.Weekly" { <input-assignment> }
                      | "|T.Monthly" { <input-assignment> }

<file-trigger> ::= "|T.File.Modified" { <input-assignment> }
                 | "|T.File.Created" { <input-assignment> }
                 | "|T.File.Deleted" { <input-assignment> }

<interval-trigger> ::= "|T.Every.Seconds" { <input-assignment> }
                     | "|T.Every.Minute"
                     | "|T.Every.Hour"

/* Note: Logical operators provide implicit grouping
   - [+] OR: groups with condition above
   - [&] AND: explicit or implicit at first trigger level
   - [^] XOR: groups with condition above
   - [.] + [~]: explicit grouping (empty line + prefix)
   - [+][.], [&][.], [^][.]: nested groups with double [~][~]
*/
```

---

## Queue Control (Surface)

```bnf
<queue-control> ::= "[Q]" <queue-operation>

<queue-operation> ::= "|Q.Pause"
                    | "|Q.Resume"
                    | "|Q.Kill"
                    | "|Q.PriorityBump"
                    | "|Q.Queue.Assign" { <input-assignment> }
                    | "|Q.Status"
                    | <conditional-queue-op>

<conditional-queue-op> ::= "|Q.PauseIf." <condition-spec>
                         | "|Q.DispatchIf." <condition-spec>

<condition-spec> ::= "RAM." <comparison-suffix>
                   | "CPU." <comparison-suffix>
                   | "String." <string-condition>
                   | "Number." <comparison-suffix>

<comparison-suffix> ::= "MoreThan"
                      | "LessThan"
                      | "GreaterThan"
                      | "Equals"

<string-condition> ::= "IsEmpty"
                     | "IsNotEmpty"
                     | "Contains"
```

---

## Wrapper Context (Surface)

```bnf
<wrapper-context> ::= "[w]" <wrapper-spec>

<wrapper-spec> ::= "|W.Python" [ <version-suffix> ]
                 | "|W.Node" [ <version-suffix> ]
                 | "|W.Rust"
                 | "|W.Go"
                 | "|W.Ruby" [ <version-suffix> ]
                 | "|W.Deno"
                 | "|W.NoSetup.NoCleanup"

<version-suffix> ::= <digit> { <digit> } [ "." <digit> { <digit> } ]
```

---

## Error Handling (Surface)

```bnf
<error-handler> ::= "[!]" <error-catch>
                    { <nested-operation> }

<error-catch> ::= "!" <error-type>
                  { <output-extraction> }

<error-type> ::= <package-error>
               | <local-error>
               | <builtin-error>

<package-error> ::= "@" <package-name> "!" <identifier> { "." <identifier> }

<local-error> ::= <identifier> { "." <identifier> }

<builtin-error> ::= "pg." <error-namespace> "." <identifier>

<error-namespace> ::= "FileSystem"
                    | "Network"
                    | "Validation"
                    | "Runtime"
```

---

## Switch/Conditional (Surface)

```bnf
<switch-case> ::= "[?]" <comparison-expression>
                  { <nested-operation> }

<comparison-expression> ::= <field-name> <comparison-operator> <value>
                          | <field-name> <negation-operator> <value>
                          | <field-name> <range-comparison>
                          | <field-name> <pattern-match>

<comparison-operator> ::= "=?" | ">?" | "<?" | ">=?" | "<=?"

<negation-operator> ::= "=!?" | ">!?" | "<!?" | ">=!?" | "<=!?"

<range-comparison> ::= <range-inclusive>
                     | <range-exclusive>
                     | <range-mixed>
                     | <range-negated>

<range-inclusive> ::= "?[" <value> "," <value> "]"   /* both inclusive */

<range-exclusive> ::= "?(" <value> "," <value> ")"   /* both exclusive */

<range-mixed> ::= "?[" <value> "," <value> ")"       /* left incl, right excl */
                | "?(" <value> "," <value> "]"       /* left excl, right incl */

<range-negated> ::= "<!?<" <value> "," <value>       /* NOT between */

<pattern-match> ::= "?" <pattern-type> <string-literal>
                  | "=!?" <pattern-type> <string-literal>  /* negated */

<pattern-type> ::= "*"   /* wildcard */
                 | "re"  /* regex */

<value> ::= <literal>
          | <enumeration-value>
          | <boolean-literal>

/* Note: Old syntax removed:
   - "?>" match operator → use comparison operators (=?, >?, <?, etc.)
   - ".." range operator → use bracket/paren syntax (?[a, b], etc.)
   - "True"/"False" keywords → use #True/#False enumerations
*/
```

---

## Enumeration Definition (Surface)

```bnf
<enumeration-definition> ::= "[#]" <enumeration-name>
                             { <enumeration-element> }
                             "[X]"

<enumeration-name> ::= <identifier> { "." <identifier> }

<enumeration-element> ::= <field-definition>
                        | <alias-definition>
                        | <comment>

<field-definition> ::= "[<]" <field-name> ":" <type> "<<" <expression>

<alias-definition> ::= "[A]" <identifier>

<enumeration-reference> ::= "#" <enumeration-name> [ "." <identifier> ]

<reserved-enum> ::= "#Path.Identifiers." <identifier> { "." <identifier> }
                  | "#Queues." <identifier>
                  | "#Status." <identifier>
                  | "#None"
```

---

## Error Definition (Surface)

```bnf
<error-definition> ::= "[!]" "!" <error-name>
                       <message-field>
                       <code-field>
                       <trace-field>
                       { <custom-field> }
                       "[X]"

<error-name> ::= <identifier> { "." <identifier> }

<message-field> ::= "[<]" ".message" ":" "pg\string" "<<" <string-literal>

<code-field> ::= "[<]" ".code" ":" "pg\int" "<<" <integer-literal>

<trace-field> ::= "[<]" ".trace" ":" "pg\string" "<<" <string-literal>

<custom-field> ::= "[<]" <field-name> ":" <type> "<<" <expression>
```

---

# Phase 2: Canonical Syntax Grammar

**Purpose:** Defines the **full render form** after all transformations. This is the strict form where:
- All required elements must be present
- Elements are reordered to canonical positions
- All macros are expanded
- All defaults are inserted

**Used By:** Validator, Type Checker, Code Generator (canonical AST → IR → bytecode)

---

## Program Structure (Canonical)

```bnf
<canonical-program> ::= <package-declaration-block>
                        { <canonical-definition> }

/* Package declaration block is preserved from surface syntax
   Validation ensures exactly one package declaration block exists
   All imports are resolved and validated */

<package-declaration-block> ::= "[@]" <package-spec>
                                [ <alias-declaration> ]
                                { <import-declaration> }
                                "[X]"

<package-spec> ::= <registry-path> "::" <version>

<registry-path> ::= <registry-tier> "@" <package-path>

<package-path> ::= <identifier> { "." <identifier> }

<version> ::= <major> "." <minor> "." <patch>

<alias-declaration> ::= "[A]" <identifier>

<import-declaration> ::= "[<]" "@" <import-alias> "<<" <package-spec>

<canonical-definition> ::= <canonical-pipeline>
                         | <canonical-enumeration>
                         | <canonical-error>
```

---

## Pipeline Definition (Canonical)

```bnf
<canonical-pipeline> ::= "[|]" <pipeline-name>
                         <input-section>
                         <trigger-section>
                         <wrapper-section>
                         <control-section>
                         <operation-section>
                         <output-section>
                         "[X]"

<pipeline-name> ::= <identifier>

/* REQUIRED SECTIONS */

<input-section> ::= { <input-declaration> }
/* Zero or more inputs - order preserved from surface syntax */

<trigger-section> ::= <trigger-declaration>
/* EXACTLY ONE trigger - REQUIRED in canonical form
   Compiler error if missing after macro expansion */

<wrapper-section> ::= <wrapper-context>
/* EXACTLY ONE wrapper - REQUIRED in canonical form
   Default |W.NoSetup.NoCleanup inserted if missing */

<control-section> ::= { <queue-control> }
/* Zero or more queue control operations */

<operation-section> ::= { <canonical-operation> | <canonical-error-handler> }
/* Zero or more operations in execution order */

<output-section> ::= [ <output-declaration> ]
/* Zero or one output declaration
   If missing and no return value, compiler may insert [o] #None */
```

---

## Canonical Operations

```bnf
<canonical-operation> ::= <sequential-operation>
                        | <canonical-parallel-block>
                        | <canonical-switch-block>

<canonical-parallel-block> ::= { <parallel-operation> }
                               <join-block>
/* Parallel operations MUST be followed by join in canonical form */

<canonical-switch-block> ::= { <switch-case> }
/* All switch branches validated for consistent outputs */

<canonical-error-handler> ::= "[!]" <error-catch>
                              { <nested-operation> }
/* Error handlers validated after operations they catch */
```

---

## Canonical Enumeration

```bnf
<canonical-enumeration> ::= "[#]" <enumeration-name>
                            { <field-definition> }
                            [ <alias-definition> ]
                            "[X]"

/* Validation in canonical form:
   - At least one field required
   - Reserved enumeration schemas enforced
   - Path.Identifiers MUST have .unix and .windows fields
   - All field types validated */
```

---

## Canonical Error Definition

```bnf
<canonical-error> ::= "[!]" "!" <error-name>
                      <message-field>
                      <code-field>
                      <trace-field>
                      { <custom-field> }
                      "[X]"

/* Validation in canonical form:
   - .message field REQUIRED
   - .code field REQUIRED
   - .trace field REQUIRED
   - All three must appear before custom fields */
```

---

# Transformation Rules

These rules define how **Surface Syntax** transforms into **Canonical Syntax**.

---

## Rule 1: Trigger Requirement

```
RULE: trigger-required
INPUT: <pipeline-definition> from surface syntax
CHECK: count(<trigger-declaration>) in pipeline body
IF count = 0
  THEN COMPILER ERROR: "Pipeline '<name>' must have exactly one [t] trigger"
ELSE IF count > 1
  THEN COMPILER ERROR: "Pipeline '<name>' has multiple triggers (found <count>)"
ELSE
  ACCEPT: Move <trigger-declaration> to <trigger-section>
```

**Example:**
```polyglot
// Surface Syntax - ERROR (no trigger)
[|] MyPipeline
[i] .input: pg\string
[r] |DoStuff
[X]

// Compiler Error:
// Pipeline 'MyPipeline' must have exactly one [t] trigger
```

---

## Rule 2: Wrapper Default Insertion

```
RULE: wrapper-default
INPUT: <pipeline-definition> from surface syntax
CHECK: count(<wrapper-context>) in pipeline body
IF count = 0
  THEN INSERT: [w] |W.NoSetup.NoCleanup
       OUTPUT: Move to <wrapper-section>
ELSE IF count = 1
  THEN ACCEPT: Move <wrapper-context> to <wrapper-section>
ELSE IF count > 1
  THEN COMPILER ERROR: "Pipeline '<name>' has multiple wrappers (found <count>)"
```

**Example:**
```polyglot
// Surface Syntax (no wrapper)
[|] MyPipeline
[t] |T.Call
[i] .input: pg\string
[r] |DoStuff
[X]

// Canonical Form (wrapper inserted)
[|] MyPipeline
[i] .input: pg\string
[t] |T.Call
[w] |W.NoSetup.NoCleanup  // ← INSERTED BY COMPILER
[r] |DoStuff
[X]
```

---

## Rule 3: Element Reordering

```
RULE: canonical-order
INPUT: <pipeline-definition> from surface syntax
OUTPUT: Reorder elements to canonical sections:
  1. <input-section>:     All [i] input declarations
  2. <trigger-section>:   Single [t] trigger
  3. <wrapper-section>:   Single [w] wrapper
  4. <control-section>:   All [Q] queue controls
  5. <operation-section>: All [r], [p], [!] operations
  6. <output-section>:    Single [o] output (if present)
```

**Example:**
```polyglot
// Surface Syntax (any order)
[|] MyPipeline
[r] |DoStuff
[t] |T.Call
[i] .input: pg\string
[o] .result: pg\string
[X]

// Canonical Form (reordered)
[|] MyPipeline
[i] .input: pg\string        // 1. Inputs
[t] |T.Call                  // 2. Trigger
[w] |W.NoSetup.NoCleanup     // 3. Wrapper (inserted)
[r] |DoStuff                 // 4. Operations
[o] .result: pg\string       // 5. Output
[X]
```

---

## Rule 4: Parallel Block Validation

```
RULE: parallel-join-required
INPUT: <parallel-block> from surface syntax
CHECK: Is there a <join-block> after all <parallel-operation>?
IF no join found
  THEN COMPILER ERROR: "Parallel block must be followed by [Y] |Y.Join"
ELSE
  ACCEPT: Create <canonical-parallel-block>
```

**Example:**
```polyglot
// Surface Syntax - ERROR (no join)
[p] |TaskA
[>] .result >> result_a
[p] |TaskB
[>] .result >> result_b
// Missing [Y] |Y.Join

// Compiler Error:
// Parallel block must be followed by [Y] |Y.Join
```

---

## Rule 5: Switch Branch Consistency

```
RULE: switch-output-consistency
INPUT: { <switch-case> } from surface syntax
CHECK: All switch branches declare same output fields
FOR each branch:
  COLLECT output field names
IF output field sets differ across branches
  THEN COMPILER ERROR: "All switch branches must output same fields"
ELSE
  ACCEPT: Create <canonical-switch-block>
```

**Example:**
```polyglot
// Surface Syntax - ERROR (inconsistent outputs)
[?] .status =? #Status.Success
[~][o] .result: pg\string

[?] .status =? #Status.Failed
[~][o] .error: !            // Different field!

// Compiler Error:
// All switch branches must output same fields
// Branch 1 outputs: .result
// Branch 2 outputs: .error
```

---

## Rule 6: Macro Expansion

```
RULE: macro-expansion
INPUT: Macro call in surface syntax
PROCESS:
  1. Resolve macro definition
  2. Substitute macro parameters
  3. Inline macro body into pipeline
  4. Re-parse inlined code as surface syntax
  5. Apply all transformation rules recursively
OUTPUT: Expanded canonical form
```

**Example:**
```polyglot
// Surface Syntax (with macro)
[|] MyPipeline
@MyMacros.DailyAt9AM      // ← Macro call
[i] .input: pg\string
[r] |DoStuff
[X]

// After Macro Expansion
[|] MyPipeline
[t] |T.Daily              // ← Expanded from macro
[<] .time: pg\dt << DT"09:00:"
[i] .input: pg\string
[r] |DoStuff
[X]

// Canonical Form (after all rules applied)
[|] MyPipeline
[i] .input: pg\string
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"
[w] |W.NoSetup.NoCleanup  // ← Inserted
[r] |DoStuff
[X]
```

---

## Rule 7: Output Default (Optional)

```
RULE: output-default-none (OPTIONAL - implementation-defined)
INPUT: <canonical-pipeline>
CHECK: Does pipeline have <output-section>?
IF no output section AND pipeline has no return value
  THEN OPTIONALLY INSERT: [o] #None
ELSE
  ACCEPT: Output section as-is
```

**Example:**
```polyglot
// Surface Syntax (no output)
[|] LogMessage
[i] .msg: pg\string
[t] |T.Call
[r] |U.Log.Info
[<] .msg: pg\string << .msg
[X]

// Canonical Form (may insert [o] #None)
[|] LogMessage
[i] .msg: pg\string
[t] |T.Call
[w] |W.NoSetup.NoCleanup
[r] |U.Log.Info
[<] .msg: pg\string << .msg
[o] #None                    // ← OPTIONALLY INSERTED
[X]
```

---

# Validation Constraints

These constraints are enforced on the **Canonical Syntax** after all transformations.

---

## Structural Constraints

### 0. Package Declaration (File-Level)
```
CONSTRAINT: package-declaration-required
FOR each .pg file:
  REQUIRE: exactly 1 <package-declaration> at file start
  ERROR if: No package declaration found
  ERROR if: Multiple package declarations found
  ERROR if: Package declaration not first non-comment element
```

### 1. Pipeline Structure
```
CONSTRAINT: pipeline-structure
FOR each <canonical-pipeline>:
  REQUIRE: exactly 1 <trigger-section>
  REQUIRE: exactly 1 <wrapper-section>
  REQUIRE: 0 or more <input-declaration>
  REQUIRE: 0 or more <queue-control>
  REQUIRE: 0 or more <operation>
  REQUIRE: 0 or 1 <output-declaration>
```

### 2. Block Pairing
```
CONSTRAINT: block-pairing
FOR each definition:
  [|] MUST be paired with [X]
  [#] MUST be paired with [X]
  [!] (definition) MUST be paired with [X]
```

### 3. Parallel Synchronization
```
CONSTRAINT: parallel-join
FOR each <canonical-parallel-block>:
  REQUIRE: <join-block> immediately after all <parallel-operation>
  ERROR if: Operations follow parallel block without join
```

---

## Type Constraints

### 4. Type Completeness
```
CONSTRAINT: type-completeness
FOR each variable declaration:
  REQUIRE: explicit type annotation
  ERROR if: Type is missing or ambiguous
```

### 5. Error Type Requirements
```
CONSTRAINT: error-fields
FOR each <canonical-error>:
  REQUIRE: .message: pg\string field
  REQUIRE: .code: pg\int field
  REQUIRE: .trace: pg\string field
  ERROR if: Any required field is missing
```

### 6. Path Identifier Requirements
```
CONSTRAINT: path-identifiers
FOR each #Path.Identifiers.* enumeration:
  REQUIRE: .unix: pg\path field
  REQUIRE: .windows: pg\path field
  ERROR if: Either field is missing
```

---

## Semantic Constraints

### 7. Switch Branch Consistency
```
CONSTRAINT: switch-outputs
FOR each <canonical-switch-block>:
  COLLECT: output fields from all branches
  REQUIRE: all branches output identical field sets
  ERROR if: field sets differ across branches
```

### 8. Error Propagation
```
CONSTRAINT: error-handling
FOR each operation that may raise error:
  IF: error is caught with [!] block
    THEN: error is handled in current scope
  ELSE: error propagates to caller
  REQUIRE: caller handles OR propagates further
```

---

# Lexical Elements

These definitions apply to **both** Surface and Canonical syntax.

---

## Identifiers and Names

```bnf
<identifier> ::= <letter> { <letter> | <digit> | "_" }

<letter> ::= "a" | "b" | "c" | ... | "z"
           | "A" | "B" | "C" | ... | "Z"

<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

<alphanumeric> ::= <letter> | <digit>

<field-name> ::= "." <identifier>

<variable-name> ::= <identifier>

<package-name> ::= <registry-tier> "." <package-identifier>

<registry-tier> ::= "Local"
                  | "Community." <author>
                  | "Company." <organization>

<author> ::= <identifier> "@" <identifier>

<organization> ::= <identifier> "@" <identifier>

<package-identifier> ::= <identifier>
```

---

## Reserved Keywords

**REMOVED:** Polyglot has ZERO keywords. All syntax uses symbolic operators and block markers.

Previously had 5 keywords (True, False, Fixed, Default, Exposed) - all replaced with:
- `True`/`False` → `#Boolean.True`/`#Boolean.False` (reserved enumeration with `#True`/`#False` aliases)
- `Fixed` → `[=]` block marker for immutable assignment
- `Default` → inline `<<` assignment in `[i]` declaration
- `Exposed` → `[m]` block marker for macro-exported variables

---

## Whitespace and Comments

```bnf
<whitespace> ::= <space> | <tab> | <newline> | <carriage-return>

<space> ::= " "

<tab> ::= "\t"

<newline> ::= "\n"

<carriage-return> ::= "\r"

/* Whitespace is significant for line structure but not within expressions */
```

---

# Type System

Type definitions apply to **both** Surface and Canonical syntax.

---

## Type Definitions

```bnf
<type> ::= <primitive-type>
         | <collection-type>
         | <special-type>
         | <mutable-type>
         | <enumeration-type>
         | <error-type-ref>

<primitive-type> ::= "pg\int"
                   | "pg\uint"
                   | "pg\float"
                   | "pg\string"
                   | "pg\bool"
                   | "pg\path"
                   | "pg\dt"

<collection-type> ::= "pg\array{" <type> "}"
                    | "pg\set{" <type> "}"

<special-type> ::= "pg\serial"

<mutable-type> ::= "pg.mutable\" <mutable-base>

<mutable-base> ::= "int"
                 | "uint"
                 | "float"
                 | "string"
                 | "bool"
                 | "path"
                 | "array{" <type> "}"
                 | "set{" <type> "}"

<enumeration-type> ::= "#" <enumeration-name>

<error-type-ref> ::= "!"
```

---

# Operators

Operator definitions apply to **both** Surface and Canonical syntax.

---

## Operator Definitions

```bnf
<operator> ::= <pipeline-operator>
             | <unpack-operator>
             | <package-operator>
             | <enumeration-operator>
             | <error-operator>
             | <assignment-operator>
             | <comparison-operator>
             | <negation-operator>
             | <range-operator>
             | <pattern-operator>
             | <type-separator>

<pipeline-operator> ::= "|"

<unpack-operator> ::= "~"

<package-operator> ::= "@"

<enumeration-operator> ::= "#"

<error-operator> ::= "!"

<assignment-operator> ::= "<<" | ">>"

<comparison-operator> ::= "=?" | ">?" | "<?" | ">=?" | "<=?"

<negation-operator> ::= "=!?" | ">!?" | "<!?" | ">=!?" | "<=!?"

<range-operator> ::= "?[" | "?(" | "]" | ")"
                   | "<!?<"  /* negated range */

<pattern-operator> ::= "?" <pattern-prefix> <string-literal>

<pattern-prefix> ::= "*"   /* wildcard */
                   | "re"  /* regex */

<type-separator> ::= "\"

/* Note: Old operators removed:
   - "?>" (match operator) → replaced by comparison operators
   - ".." (range operator) → replaced by bracket/paren syntax
*/
```

---

## Operator Semantics

### Comparison Operators

**Type-aware comparison operators** that behave differently based on data type:

```polyglot
// Equality
[?] .age =? 18              // equals (numeric, string, enum, etc.)
[?] .status =? #Active      // enum equality
[?] .name =? "John"         // string equality

// Ordering (numeric, datetime, string)
[?] .age >? 18              // greater than
[?] .age <? 65              // less than
[?] .age >=? 18             // greater or equal
[?] .age <=? 65             // less or equal
```

### Range Operators

**Bracket/paren notation** indicates inclusivity (mathematical interval notation):
- `[` = inclusive boundary
- `(` = exclusive boundary

```polyglot
// Range comparisons
[?] .age ?[18, 65]          // 18 <= age <= 65 (both inclusive)
[?] .age ?(18, 65)          // 18 < age < 65 (both exclusive)
[?] .age ?[18, 65)          // 18 <= age < 65 (left incl, right excl)
[?] .age ?(18, 65]          // 18 < age <= 65 (left excl, right incl)
```

### Negation Operators

**Negated comparisons** using `!` prefix before `?`:

```polyglot
// Negated equality
[?] .status =!? #Inactive   // not equals

// Negated ordering (less useful, included for completeness)
[?] .age >!? 65             // NOT greater than (same as <=? 65)
[?] .age <!? 18             // NOT less than (same as >=? 18)

// Negated range
[?] .age <!?< 18, 65        // NOT between 18 and 65
```

### Pattern Matching

**Wildcard and regex** pattern matching:

```polyglot
// Wildcard patterns (shell-style glob)
[?] .filename ? *"*.csv"              // any file ending with .csv
[?] .filename ? *"data_*.txt"         // files matching pattern

// Regex patterns
[?] .email ? re"^[a-z]+@.*"          // regex match
[?] .code ? re"[A-Z]{3}-\d{4}"       // pattern: ABC-1234

// Negated patterns
[?] .filename =!? *"*.tmp"            // NOT matching wildcard
[?] .email =!? re".*@spam\..*"       // NOT matching regex
```

### Logical Operators (Triggers)

**Implicit and explicit grouping** for trigger conditions:

```polyglot
// Implicit grouping with [+] OR
[t] |T.IsMorning
[+] |T.IsEvening             // (IsMorning OR IsEvening)

// Implicit grouping with [^] XOR
[t] |T.OnWeekday
[^] |T.OnWeekend             // (OnWeekday XOR OnWeekend)

// Explicit grouping with [.]
[.]                          // Empty line - group marker
[~][t] |T.Condition1
[~][+] |T.Condition2         // Explicitly grouped

// Nested grouping
[t] |T.A
[+][.]                       // OR with nested group
[~][~][t] |T.B               // Double [~] for nested member
[~][~][t] |T.C
// Meaning: A OR (B AND C)
```

### Block Markers

**New block markers** replacing keywords:

```polyglot
// [=] Fixed/immutable assignment (replaces "Fixed" keyword)
[=] .config: pg\string << "production"

// [m] Macro-exported variable (replaces "Exposed" keyword)
[m] .time: pg\dt

// [*] Line continuation
[<] .url: pg\string << "postgresql://"
[*] >"admin:pass@"           // [*] >" for string concatenation
[*] >"localhost:5432/"

// [+] OR logical operator (triggers)
// [&] AND logical operator (triggers)
// [^] XOR logical operator (triggers)
// [.] Group marker (on empty line)
// [~] Group member prefix
```

**Important Notes:**
- All operators are **context-sensitive** and **type-aware**
- Comparison operators work differently for strings, numbers, datetimes, enums
- Pattern operators only work with string types
- Range operators work with numeric types and datetimes
- Negation operators provide logical NOT for any comparison
- Logical operators only apply to trigger conditions
- Block markers must appear at the start of a line

---

# Literals

Literal definitions apply to **both** Surface and Canonical syntax.

---

## Literal Values

```bnf
<literal> ::= <boolean-literal>
            | <integer-literal>
            | <float-literal>
            | <string-literal>
            | <datetime-literal>
            | <path-literal>
            | <array-literal>
            | <set-literal>
            | <serial-literal>

<boolean-literal> ::= <boolean-enum>

<boolean-enum> ::= "#Boolean.True" | "#Boolean.False"
                 | "#True" | "#False"  /* aliases */

/* Note: "True" and "False" keywords removed.
   Use #Boolean.True / #Boolean.False (or aliases #True / #False) */

<integer-literal> ::= [ "-" ] <digit> { <digit> }

<float-literal> ::= [ "-" ] <digit> { <digit> } "." <digit> { <digit> }
                  | [ "-" ] <digit> { <digit> } [ "." <digit> { <digit> } ] "e" [ "-" | "+" ] <digit> { <digit> }

<string-literal> ::= '"' { <string-char> | <interpolation> } '"'
                   | <multiline-string>

<string-char> ::= <any-char-except-quote-or-backslash>
                | <escape-sequence>

<escape-sequence> ::= "\\" ( "n" | "t" | "r" | "\\" | '"' | "{" | "}" )

<interpolation> ::= "{" <field-name> "}"

<multiline-string> ::= <string-literal>
                       { "[*]" '>"' <string-literal> }

/* Note: Line continuation changed from [^] to [*]
   [^] is now used for XOR logical operator */
```

---

## DateTime Literals

```bnf
<datetime-literal> ::= <dt-prefix> '"' <datetime-value> '"'

<dt-prefix> ::= "DT"
              | "DT.Gregorian"
              | "DT.Hijri"
              | "DT.Chinese"
              | "DT.Hebrew"
              | "DT.Persian"

<datetime-value> ::= <time-only>
                   | <date-only>
                   | <date-and-time>

<time-only> ::= <hh> ":" <mm> ":"
              | <hh> ":" <mm> ":" <ss>

<date-only> ::= <yyyy> "-" <mm> "-" <dd>

<date-and-time> ::= <date-only> " " <time-only>

<hh> ::= <digit> <digit>
<mm> ::= <digit> <digit>
<ss> ::= <digit> <digit>
<yyyy> ::= <digit> <digit> <digit> <digit>
<dd> ::= <digit> <digit>
```

---

## Path Literals

```bnf
<path-literal> ::= "\\\\" <path-identifier> "\\\\" { <path-segment> }

<path-identifier> ::= <identifier>
                    | "UnixRoot"
                    | "WindowsRoot"
                    | "NoPath"
                    | <letter> /* Windows drive letter */

<path-segment> ::= <identifier> "\\"
                 | <identifier> "." <extension> "\\"
                 | <identifier> "." <extension>

<extension> ::= <alphanumeric> { <alphanumeric> }
```

---

## Collection Literals

```bnf
<array-literal> ::= "array{" [ <array-elements> ] "}"

<array-elements> ::= <expression> { "," <expression> }
                   | <expression> { "[^]" <expression> }

<set-literal> ::= "set{" [ <set-elements> ] "}"

<set-elements> ::= <expression> { "," <expression> }

<serial-literal> ::= "serial{" [ <serial-fields> ] "}"

<serial-fields> ::= <serial-field> { "," <serial-field> }
                  | <serial-field> { "[^]" <serial-field> }

<serial-field> ::= '"' <identifier> '"' ":" <serial-value>

<serial-value> ::= <literal>
                 | <serial-literal>
```

---

## Expressions

```bnf
<expression> ::= <literal>
               | <variable-reference>
               | <enumeration-reference>
               | <field-access>
               | <pipeline-call>
               | <unpack-operation>

<variable-reference> ::= <field-name>
                       | <identifier>

<field-access> ::= <expression> "." <identifier>
```

---

# Complete Grammar Reference

## Surface Syntax Summary

```bnf
/* ===== SURFACE SYNTAX (PHASE 1: PARSE) ===== */

<program> ::= <package-declaration-block>
              { <top-level-element> }

<package-declaration-block> ::= "[@]" <package-spec>
                                [ <alias-declaration> ]
                                { <import-declaration> }
                                "[X]"

<package-spec> ::= <registry-path> "::" <version>

<top-level-element> ::= <pipeline-definition>
                      | <enumeration-definition>
                      | <error-definition>
                      | <comment>

<pipeline-definition> ::= "[|]" <pipeline-name>
                          { <pipeline-element> }
                          "[X]"

<pipeline-element> ::= <input-declaration>
                     | <output-declaration>
                     | <trigger-declaration>
                     | <queue-control>
                     | <wrapper-context>
                     | <operation>
                     | <join-block>
                     | <error-handler>
                     | <switch-case>
                     | <comment>

<operation> ::= "[r]" <operation-body>
              | "[p]" <operation-body>
              | "[~]" <operation>

<operation-body> ::= <pipeline-call>
                   | <variable-assignment>
                   | <unpack-operation>

/* See full sections above for complete definitions */
```

---

## Canonical Syntax Summary

```bnf
/* ===== CANONICAL SYNTAX (PHASE 2: VALIDATE) ===== */

<canonical-program> ::= <package-declaration-block>
                        { <canonical-definition> }

<package-declaration-block> ::= "[@]" <package-spec>
                                [ <alias-declaration> ]
                                { <import-declaration> }
                                "[X]"

<package-spec> ::= <registry-path> "::" <version>

<canonical-definition> ::= <canonical-pipeline>
                         | <canonical-enumeration>
                         | <canonical-error>

<canonical-pipeline> ::= "[|]" <pipeline-name>
                         <input-section>
                         <trigger-section>        /* REQUIRED */
                         <wrapper-section>        /* REQUIRED */
                         <control-section>
                         <operation-section>
                         <output-section>
                         "[X]"

<input-section> ::= { <input-declaration> }

<trigger-section> ::= <trigger-declaration>      /* exactly 1 */

<wrapper-section> ::= <wrapper-context>          /* exactly 1 */

<control-section> ::= { <queue-control> }

<operation-section> ::= { <canonical-operation> | <canonical-error-handler> }

<output-section> ::= [ <output-declaration> ]    /* 0 or 1 */

/* See full sections above for complete definitions */
```

---

# Grammar Notes

## Parsing Considerations

1. **Line-Oriented Structure**
   - Every valid line starts with a block marker `[x]`
   - Block markers determine parsing context
   - Continuation marker `[^]` for multi-line literals

2. **Two-Phase Parsing**
   - Phase 1: Parse surface syntax → surface AST
   - Phase 2: Transform to canonical → canonical AST

3. **Whitespace Handling**
   - Whitespace within expressions is insignificant
   - Indentation is cosmetic (not significant like Python)
   - Newlines separate statements

4. **Operator Precedence**
   - Polyglot has NO arithmetic operators (`+`, `-`, `*`, `/`)
   - HAS comparison operators: `=?`, `>?`, `<?`, `>=?`, `<=?` (type-aware)
   - HAS negation operators: `=!?`, `>!?`, `<!?`, `>=!?`, `<=!?`, `<!?<`
   - HAS range operators: `?[`, `?(`, `]`, `)` (bracket/paren inclusivity)
   - HAS pattern operators: `?` with `*` (wildcard) or `re` (regex)
   - All operators are context-sensitive and type-aware

5. **Type Separator**
   - ALWAYS backslash `\` for types (`pg\int`)
   - NEVER forward slash `/`

6. **Comments**
   - Forward slash for comments (`//`, `/* */`)
   - Backslash for paths and types

7. **Case Sensitivity**
   - Block markers are case-sensitive
   - Identifiers are case-sensitive
   - Reserved enumerations are case-sensitive

---

# Implementation Notes

## Parser Requirements

### Phase 1: Surface Syntax Parser

1. **Lexical Analysis**
   - Tokenize based on block markers first
   - Recognize operators as single tokens
   - Handle multi-character operators: `<<`, `>>`, `=?`, `>?`, `<?`, `>=?`, `<=?`, `=!?`, `>!?`, `<!?`, `>=!?`, `<=!?`, `<!?<`, `?[`, `?(`, `[*]`, `[+]`, `[&]`, `[^]`, `[.]`, `[~]`, `[=]`, `[m]`

2. **Syntax Analysis**
   - Context-sensitive parsing based on block markers
   - Track nesting depth via `[~]` prefix
   - Build surface AST (flexible structure)

### Phase 2: Transformer

1. **Macro Expansion**
   - Resolve all macro definitions
   - Inline macro bodies
   - Re-parse expanded code

2. **Default Insertion**
   - Check for missing wrappers
   - Insert `[w] |W.NoSetup.NoCleanup` if needed

3. **Element Reordering**
   - Group elements into canonical sections
   - Preserve semantic meaning while enforcing structure

4. **Build Canonical AST**
   - Strict structure matching canonical grammar

### Phase 3: Validator

1. **Structural Validation**
   - Verify all pipelines have triggers
   - Check block pairing (`[|]`/`[X]`, etc.)
   - Validate parallel/join pairing

2. **Type Validation**
   - Check type completeness
   - Verify error type required fields
   - Validate path identifier schemas

3. **Semantic Validation**
   - Check switch branch consistency
   - Validate error propagation
   - Verify queue operations

4. **Error Recovery**
   - Report clear error messages
   - Provide context and suggestions

---

# Examples

## Example 1: Simple Pipeline

**Surface Syntax (what user writes):**
```polyglot
[|] Greet
[i] .name: pg\string
[r] |U.Log.Info
[<] .msg: pg\string << "Hello, {.name}!"
[t] |T.Call
[X]
```

**Canonical Syntax (after transformation):**
```polyglot
[|] Greet
[i] .name: pg\string
[t] |T.Call
[w] |W.NoSetup.NoCleanup         // ← Inserted by compiler
[r] |U.Log.Info
[<] .msg: pg\string << "Hello, {.name}!"
[o] #None                        // ← Optionally inserted
[X]
```

---

## Example 2: Macro Expansion

**Surface Syntax (with macro):**
```polyglot
[|] DailyReport
@Macros.ScheduleDaily(DT"09:00:")   // ← Macro call
[i] .recipient: pg\string
[r] |GenerateReport
[>] .report: pg\string >> content
[r] |SendEmail
[<] .body: pg\string << content
[X]
```

**After Macro Expansion:**
```polyglot
[|] DailyReport
[t] |T.Daily                        // ← From macro
[<] .time: pg\dt << DT"09:00:"     // ← From macro
[i] .recipient: pg\string
[r] |GenerateReport
[>] .report: pg\string >> content
[r] |SendEmail
[<] .body: pg\string << content
[X]
```

**Canonical Syntax:**
```polyglot
[|] DailyReport
[i] .recipient: pg\string
[t] |T.Daily
[<] .time: pg\dt << DT"09:00:"
[w] |W.NoSetup.NoCleanup            // ← Inserted
[r] |GenerateReport
[>] .report: pg\string >> content
[r] |SendEmail
[<] .body: pg\string << content
[o] #None                           // ← Optionally inserted
[X]
```

---

## Example 3: Error Handling

**Surface Syntax:**
```polyglot
[|] SafeRead
[i] .path: pg\path
[t] |T.Call
[r] |U.File.Read
[<] .path: pg\path << .path
[>] .content: pg\string >> data
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err
[r] .data: pg\string << "default"
[o] .data: pg\string
[X]
```

**Canonical Syntax (no changes needed, already valid):**
```polyglot
[|] SafeRead
[i] .path: pg\path
[t] |T.Call
[w] |W.NoSetup.NoCleanup            // ← Inserted
[r] |U.File.Read
[<] .path: pg\path << .path
[>] .content: pg\string >> data
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err
[r] .data: pg\string << "default"
[o] .data: pg\string
[X]
```

---

## See Also

### Related Documentation

- [Complete Syntax Reference](01-syntax-complete.md) - High-level syntax guide
- [Type System](02-type-system.md) - Type semantics and rules
- [Operators](05-operators.md) - Operator semantics
- [Block Markers](06-block-markers.md) - Block marker reference
- [Quick Language Reference](../audit/quick-language-reference.md) - Fast reference

### Implementation Guides

- [Decision Log](../audit/decision-log.md) - Syntax design decisions
- [Documentation Plan](../documentation-plan.md) - Documentation strategy

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.0.2 | 2025-11-16 | **Keyword Elimination Update**: Removed all 5 keywords (True, False, Fixed, Default, Exposed) and replaced with symbolic syntax. Added comparison operators (`=?`, `>?`, etc.), negation operators (`=!?`, `<!?<`, etc.), range operators (`?[`, `?(`, etc.), pattern matching (`?*`, `?re`), and logical operators (`[+]`, `[&]`, `[^]`, `[.]`, `[~]`). Updated all examples to use new syntax. |
| 0.0.2 | 2025-11-13 | Two-phase BNF grammar (Surface + Canonical) with transformation rules |

---

**End of BNF Grammar Specification**