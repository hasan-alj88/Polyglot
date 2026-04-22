---
type: audience-rules
audience: design
classification: internal
updated: 2026-04-22
---

# Audience: Design

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Core Polyglot developers focused on language and system design. This audience covers two complementary scopes:

- **Language design:** grammar (EBNF), type system, compile rules, edge cases, syntax evolution, philosophy
- **Architecture design:** service and runtime architecture (Queue Manager, Trigger Monitor, Dispatcher), host management, system scalability, and coordination

They design how Polyglot code reads, what it means, what the compiler enforces, how the system runs, scales, and coordinates work across hosts and languages.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Precise, specification-oriented, engineering-rigorous |
| Voice | Active voice, present tense, declarative for rules, imperative for instructions |
| Format | EBNF rules, compile error/warning definitions, state machine diagrams, sequence diagrams, architecture decision records, annotated .pg examples |
| Structure | Problem -> design -> constraints -> trade-offs -> decision; Syntax -> semantics -> edge cases -> compile rules |
| Vocabulary | Exact [[glossary]] terms. PL theory and distributed systems terminology expected |
| Code | Canonical .pg examples for every construct. EBNF grammar rules. Compile error codes. Architecture diagrams, protocol definitions |

## ASSUME

- Programming language theory basics (grammars, type systems, scoping)
- Familiarity with EBNF notation
- Comfort with formal specification language
- Understanding of compiler error design
- Distributed systems knowledge (queues, scheduling, fault tolerance)
- Runtime and service design experience
- Familiarity with state machines and protocol design
- Understanding of concurrency models and resource management

## NEVER

- Use marketing language in specifications
- Leave behavior vague — specify exactly what happens in every state
- Leave syntax ambiguous — every construct needs a formal definition
- Provide hand-holding walkthroughs (that's [[audiences/automation-builder]] scope)
- Use informal language where formal specification is clearer
- Contradict [[vision#Core Philosophy]] or [[vision#Language Design]] principles
- Omit failure modes — every design must address error states

## ALWAYS

- Include EBNF grammar rules for every syntactic construct
- Define compile error codes (PGE/PGW) for every rule violation
- Show canonical .pg examples — valid and invalid forms
- Cover edge cases exhaustively
- Specify behavior precisely (what happens, not what "should" happen)
- Include state machine definitions for stateful components
- Document failure modes and recovery strategies
- Cross-reference [[vision]] for philosophy justifications
- Show sequence diagrams for multi-component interactions
- Record architecture decisions with rationale and trade-offs
- Cross-reference related features via [[wikilinks]]
