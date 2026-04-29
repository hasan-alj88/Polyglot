---
audience: design
type: spec
updated: 2026-04-03
---

# Metadata Tree Specification

<!-- @c:user/concepts/data-is-trees -->
<!-- @c:user/concepts/metadata -->
<!-- @u:EBNF -->
<!-- @u:philosophy/data-trees -->
<!-- @u:syntax/blocks#Metadata -->

This document formally specifies the `%` metadata tree — the unified structure that organizes all Aljam3 objects. For the user-friendly introduction, see [[data-is-trees|user/concepts/data-is-trees]]. For field listings and access patterns, see [[metadata|user/concepts/metadata]].

## Sections

- [[FULL-TREE|Full Tree]] — complete `%` tree in ASCII art with all branches, definitions, instances, and aliases
- [[path-grammar|Path Grammar]] — path grammar, shorthand accessors, and resolution rules
- [[object-types|Object Type Branches]] — fixed branches for each object type prefix
- [[instance-lifecycle|Instance Lifecycle]] — creation, numbering, and release of instances
- [[string-subtypes|String Subtype Nesting]] — string subtypes under `%#:String` and alias resolution
- [[enum-rules|Enum Instance Rules]] — active-field-only invariant and architecture safeguards
- [[io-ports|IO Port Nesting]] — input/output port structure within instances
- [[branches|Branch Specifications]] — wrapper, queue, trigger, and permission branch details
- [[definition-templates|Definition Templates]] — compile-time structural templates and schema properties
- [[field-expansion|Field Expansion]] — string field expansion and related references
