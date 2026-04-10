---
type: audience-rules
audience: designer
updated: 2026-04-03
---

# Audience: Designers

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Core Polyglot developers focused on language syntax and semantics: the grammar (EBNF), type system, compile rules, edge cases, and language evolution. They design how Polyglot code reads, what it means, and what the compiler enforces.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Formal, grammar-oriented, specification-precise |
| Voice | Active voice, present tense, declarative for rules |
| Format | EBNF rules, compile error/warning definitions, annotated .pg examples |
| Structure | Syntax -> semantics -> edge cases -> compile rules |
| Vocabulary | Exact [[glossary]] terms. PL theory terminology expected |
| Code | Canonical .pg examples for every construct. EBNF grammar rules. Compile error codes |

## ASSUME

- Programming language theory basics (grammars, type systems, scoping)
- Familiarity with EBNF notation
- Comfort with formal specification language
- Understanding of compiler error design

## NEVER

- Include runtime implementation details (Queue Manager, Dispatcher — that's [[audiences/architect]] scope)
- Leave syntax ambiguous — every construct needs a formal definition
- Provide hand-holding walkthroughs (that's [[audiences/pg-coder]] scope)
- Use informal language where formal specification is clearer
- Contradict [[vision#Language Design]] principles

## ALWAYS

- Include EBNF grammar rules for every syntactic construct
- Define compile error codes (PGE/PGW) for every rule violation
- Show canonical .pg examples — valid and invalid forms
- Cover edge cases exhaustively
- Cross-reference related features via [[wikilinks]]
- Specify behavior at the language level, not the runtime level
