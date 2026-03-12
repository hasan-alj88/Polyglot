# v0.0.5 - Type System

**Status:** 💡 Concept Phase - Future Improvements
**Target Release:** Q4 2026

---

## 📋 Overview

Version 0.0.5 will introduce a comprehensive type system with constrained types, cross-language mappings, and type composition. This represents a major enhancement to Polyglot's type capabilities while maintaining its pipeline-orchestration-first paradigm.

**Focus:** Type definitions, constraints, validation, and cross-language interoperability

**Philosophy:** Types as contracts and constraints, not just labels

---

## 📁 Specification Files

### [v0.0.5 Improvement Proposals](v0.0.5-improvement-proposals.md)
**Purpose:** Future improvements and type system enhancements
**Status:** 💡 Early concept

**Contents:**
- Type system enhancements overview
- Cross-language type mapping concepts
- Type composition ideas
- Future feature proposals

### [Type Definitions System](type-definitions-system.md)
**Purpose:** Type definition blocks with `{:}` syntax
**Status:** 💡 Conceptual design

**Contents:**
- `{:}` type definition block syntax
- Constrained types (min/max, patterns, ranges)
- Violation handlers (clip, raise, transform, default)
- Type inheritance and composition

---

## 🎯 Planned Features

### 1. Type Definition Blocks: `{:}`

Define custom constrained types:
```polyglot
{:} Age
[i] .min: pg\int << 0
[i] .max: pg\int << 120
[i] .onViolation: pg\string << "clip"
{x}

// Usage
[r] $userAge: :data.Age << 25    // Valid
[r] $invalid: :data.Age << 150   // Clipped to 120
```

### 2. Cross-Language Types

Map Polyglot types to language-specific types:
```polyglot
{:} UserId
[i] .pgType: pg\string
[i] .pyType: py\str
[i] .rustType: rust\String
[i] .jsType: js\string
{x}
```

**Benefits:**
- Type safety across language boundaries
- Clear expectations for runtime wrappers
- Compiler validation of type mappings

### 3. Constrained Collections

Collections with element constraints:
```polyglot
{:} ValidatedArray
[i] .elementType: :data.Age
[i] .minLength: pg\int << 1
[i] .maxLength: pg\int << 100
{x}

[r] $ages: :data.ValidatedArray << {25, 30, 35}
```

### 4. Optional Types

Explicit optional handling:
```polyglot
{:} OptionalEmail
[i] .baseType: pg\string
[i] .optional: #;Boolean;True
[i] .onNone: pg\string << "default@example.com"
{x}
```

### 5. Type Conversions

Define explicit conversion rules:
```polyglot
{:} Temperature
[i] .baseType: pg\float
[i] .unit: pg\string         // "celsius", "fahrenheit", "kelvin"
[i] .convertTo: :data.Temperature
[i] .conversionLogic: |ConvertTemperature
{x}
```

### 6. Violation Handlers

Four violation handling strategies:
- **clip** - Clamp to valid range
- **raise** - Throw error
- **transform** - Apply transformation function
- **default** - Use default value

```polyglot
{:} Score
[i] .min: pg\int << 0
[i] .max: pg\int << 100
[i] .onViolation: pg\string << "clip"
{x}

{:} Email
[i] .pattern: pg\string << "^[\\w.-]+@[\\w.-]+\\.[a-z]{2,}$"
[i] .onViolation: pg\string << "raise"
[i] .errorType: !;Validation;Email
{x}
```

### 7. Metadata-Driven Type Features

Leverage metadata system from v0.0.4:
```polyglot
{:} UserId
%Constraint "Must be UUID v4 format"
%Backend "postgres"
%Native "uuid"
[i] .baseType: pg\string
[i] .pattern: pg\string << "^[0-9a-f]{8}-..."
{x}
```

---

## 🔑 Design Principles

### 1. Types as Contracts
Types define not just structure, but valid ranges, patterns, and behaviors

### 2. Explicit Violation Handling
No silent failures - violations must be handled explicitly

### 3. Cross-Language Clarity
Clear mappings between Polyglot and target language types

### 4. Composable Types
Build complex types from simpler ones

### 5. Metadata Integration
Leverage v0.0.4 metadata system for type annotations

### 6. Backward Compatible
Existing types continue to work, constrained types are additive

---

## 💡 Conceptual Examples

### Constrained Numeric Type

```polyglot
{:} Percentage
[i] .baseType: pg\float
[i] .min: pg\float << 0.0
[i] .max: pg\float << 100.0
[i] .onViolation: pg\string << "clip"
[i] .precision: pg\int << 2
{x}

[r] $discount: :data.Percentage << 15.5    // Valid
[r] $invalid: :data.Percentage << 150.0    // Clipped to 100.0
```

### Validated String Type

```polyglot
{:} PhoneNumber
[i] .baseType: pg\string
[i] .pattern: pg\string << "^\\+?[1-9]\\d{1,14}$"
[i] .onViolation: pg\string << "raise"
[i] .errorType: !;Validation;PhoneNumber
[i] .formatPipeline: |FormatPhoneNumber
{x}

[r] $phone: :data.PhoneNumber << "+14155552671"
```

### Enum-Based Type

```polyglot
{:} Priority
[i] .baseType: #;Priority;Level
[i] .allowedValues: pg\array{#Priority.Level} << {
     #;Priority;Level;Low,
     #;Priority;Level;Medium,
     #;Priority;Level;High
}
[i] .default: #;Priority;Level;Medium
{x}
```

### Composite Type

```polyglot
{:} ValidatedUser
[i] .fields: pg\serial << {
     "email": :data.Email,
     "age": :data.Age,
     "phone": :data.PhoneNumber,
     "priority": :data.Priority
}
[i] .requiredFields: pg\array{pg\string} << {"email", "age"}
{x}
```

---

## 🔄 Integration with Existing Features

### With v0.0.4 Metadata System
```polyglot
{:} ApiKey
%Doc "API key for external service authentication"
%Deprecated "Use OAuth2 tokens instead"
%Security "sensitive-data"
[i] .baseType: pg\string
[i] .pattern: pg\string << "^[A-Za-z0-9]{32}$"
{x}
```

### With v0.0.4 Loop System
```polyglot
[r] $ages: pg\array{:data.Age} << {25, 30, 150}  // 150 handled by constraint

[r] ~ForEach
[~] <array << $ages
[~] >item >> $age                                 // Each $age is :data.Age

   [r] $category << "adult"                       // Process constrained age

   [v] *Collect.Into.Array
   [*] <item << $category
   [*] >array >> $categories
```

### With Pipeline I/O
```polyglot
{|} ValidateUserData
[|] <input :data.ValidatedUser                   // Constrained input
[t] |T.Call
[W] |W.Polyglot.Scope
[|] >output :#;Boolean;True << #;Boolean;True   // Validation result
{x}
```

---

## 📖 Reading Order

### Understanding v0.0.5 Concepts:
1. [v0.0.5 Improvement Proposals](v0.0.5-improvement-proposals.md) - Overview
2. [Type Definitions System](type-definitions-system.md) - `{:}` syntax details
3. This README - Conceptual examples and integration

### Understanding Type System Evolution:
1. Current types (v0.0.3): [/docs/user/language/types.md](../../user/language/types.md)
2. v0.0.4 enhancements: [../v0.0.4/](../v0.0.4/)
3. v0.0.5 concepts: This folder

---

## ⚠️ Concept Phase Status

**Current Status:** 💡 Early conceptual design

**What's Complete:**
- ✅ Core concept (type definition blocks with `{:}`)
- ✅ Violation handler strategies
- ✅ Cross-language type mapping vision

**What's In Progress:**
- 🔄 Detailed syntax specification
- 🔄 Constraint expression language
- 🔄 Type composition rules

**What's Pending:**
- ⏳ Complete feature specification
- ⏳ Integration patterns with v0.0.4
- ⏳ Standard type library design
- ⏳ Migration strategy from v0.0.4
- ⏳ Performance considerations

---

## 🔗 Related Documentation

**Version Roadmap:** [../version-roadmap.md](../version-roadmap.md) - Version timeline
**v0.0.4 Specifications:** [../v0.0.4/](../v0.0.4/) - Current design work
**Current Type System:** [/docs/user/language/types.md](../../user/language/types.md)
**Technical Architecture:** [/docs/Tech/implementation/technical/](../../Tech/implementation/technical/)

---

## 💭 Open Questions

### Type System Design

1. **Constraint Expression Language:**
   - Inline expressions vs pipeline references?
   - How complex should constraints be?
   - Performance implications?

2. **Type Composition:**
   - Inheritance model?
   - Mixin patterns?
   - Interface-like contracts?

3. **Cross-Language Types:**
   - How to handle type mismatches?
   - Fallback strategies?
   - Runtime type checking?

4. **Violation Handling:**
   - Should "transform" support custom pipelines?
   - Can violations cascade?
   - Error recovery strategies?

5. **Type Inference:**
   - How much inference with constrained types?
   - Explicit vs implicit constraint propagation?
   - Performance trade-offs?

### Integration Concerns

1. **Backward Compatibility:**
   - Can v0.0.4 code use v0.0.5 types?
   - Migration strategy for existing types?
   - Deprecation timeline?

2. **Standard Library:**
   - Which types should be built-in?
   - Extension mechanisms?
   - Third-party type libraries?

3. **Performance:**
   - Runtime overhead of constraint checking?
   - Compile-time optimization opportunities?
   - Caching strategies?

---

## 📊 Implementation Timeline (Tentative)

### Phase 1: Design Finalization (Q3 2026)
- Complete syntax specification
- Finalize constraint expression language
- Define type composition rules
- Create comprehensive examples

### Phase 2: Core Implementation (Q4 2026)
- `{:}` block parser
- Basic constraint validation
- Violation handler implementations
- Type definition registry

### Phase 3: Advanced Features (Q1 2027)
- Cross-language type mappings
- Type composition
- Metadata integration
- Standard type library

### Phase 4: Optimization & Refinement (Q1 2027)
- Performance optimization
- Compiler type checking
- Runtime efficiency
- Migration tools

---

## 🎯 Success Criteria

**Specification Complete When:**
- ✅ All syntax finalized
- ✅ Constraint expression language defined
- ✅ Type composition rules clear
- ✅ Integration patterns documented
- ✅ Migration strategy established

**Implementation Complete When:**
- ✅ Parser supports `{:}` blocks
- ✅ All violation handlers work
- ✅ Cross-language mappings functional
- ✅ Standard type library available
- ✅ Performance acceptable
- ✅ Documentation complete

---

**Last Updated:** 2025-12-12
**Specification Status:** 💡 Concept Phase (20% complete)
**Implementation Target:** Q4 2026 - Q1 2027
