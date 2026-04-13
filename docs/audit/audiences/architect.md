---
type: audience-rules
audience: architect
classification: internal
updated: 2026-04-13
---

# Audience: Architects

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Core Polyglot developers focused on service and runtime architecture: the Queue Manager, Trigger Monitor, Dispatcher, host management, and the overall system design. They design how Polyglot runs, scales, and coordinates work across hosts and languages.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Precise, specification-oriented, engineering-rigorous |
| Voice | Active voice, present tense, imperative for instructions |
| Format | Formal specs, state machine diagrams, sequence diagrams, architecture decision records |
| Structure | Problem -> design -> constraints -> trade-offs -> decision |
| Vocabulary | Exact [[glossary]] terms. Distributed systems terminology expected |
| Code | Architecture diagrams, state machines, protocol definitions. .pg examples only when illustrating runtime behavior |

## ASSUME

- Distributed systems knowledge (queues, scheduling, fault tolerance)
- Runtime and service design experience
- Familiarity with state machines and protocol design
- Understanding of concurrency models and resource management

## NEVER

- Use marketing language in specifications
- Leave behavior vague — specify exactly what happens in every state
- Contradict [[vision#Core Philosophy]] principles
- Mix language syntax design concerns into architecture docs (that's [[audiences/designer]] scope)
- Omit failure modes — every design must address error states

## ALWAYS

- Specify behavior precisely (what happens, not what "should" happen)
- Include state machine definitions for stateful components
- Document failure modes and recovery strategies
- Cross-reference [[vision]] for philosophy justifications
- Show sequence diagrams for multi-component interactions
- Record architecture decisions with rationale and trade-offs
