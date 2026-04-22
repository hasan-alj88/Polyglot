---
type: audience-rules
audience: product
classification: internal
updated: 2026-04-22
---

# Audience: Product

<!-- @c:vision:Core Philosophy -->
Authority: [[vision#Core Philosophy]]

## Who They Are

Product managers and owners who define what Polyglot should do and why. They write PRDs, user stories, acceptance criteria, and roadmap items. They translate user needs into actionable requirements and prioritize work across the project.

## Writing Rules

| Rule | Directive |
|------|-----------|
| Tone | Outcome-oriented, clear, stakeholder-friendly |
| Voice | Active voice, present tense, user-centric framing |
| Format | User stories, acceptance criteria (Given/When/Then), prioritized backlogs, decision records |
| Structure | Problem -> user need -> solution -> acceptance criteria -> priority |
| Vocabulary | Use [[glossary]] terms for Polyglot concepts. Product management terminology expected |
| Code | No code examples required. Reference feature names and user-facing behaviors |

## ASSUME

- Agile/Scrum familiarity
- Product management background
- Understanding of user personas and value delivery
- Awareness of Polyglot's vision and positioning (from [[vision]])
- No deep knowledge of syntax, EBNF, or compiler internals

## NEVER

- Include implementation details (code, EBNF, compile rules — that's [[audiences/design]] and [[audiences/developer]] scope)
- Use Polyglot-internal jargon without defining it in user terms
- Leave acceptance criteria unmeasurable or untestable
- Mix technical specification with product requirements
- Contradict [[vision#Core Philosophy]] principles

## ALWAYS

- Frame features in terms of user value and outcomes
- Include measurable acceptance criteria for every requirement
- Specify priority and dependencies between requirements
- Reference user personas (automation-builder, integrator) when defining needs
- Cross-reference [[vision]] for strategic alignment
- Document trade-offs and decisions with rationale
