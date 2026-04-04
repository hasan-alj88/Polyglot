---
audience: pg-coder
type: specification
updated: 2026-03-30
---

# Type Conversions

<!-- @syntax/types/INDEX -->

## Type Identity

Polyglot uses **structural matching** — two types are the same if they have the same fields and field types, regardless of name.

```polyglot
{#} #UserProfile
   [.] .name#string
   [.] .email#string

{#} #ContactInfo
   [.] .name#string
   [.] .email#string

[ ] These are the SAME type — identical schema
```

There is **no implicit coercion** between types. `int` does not auto-promote to `float`, `string` does not coerce to `path`. Explicit conversion (via a pipeline call) is always required.

For the formal rules and compile-error references, see [[type-identity|technical/spec/type-identity]].

## Type Conversions

### Struct → Serial

Always allowed. A struct's fixed (`.`) fields are converted to flexible (`:`) fields in the serial. The struct is always a valid subset of serial's openness.

```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[r] $user#UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30

[ ] struct → serial is always safe
[r] $data#serial << $user
[ ] $data now has :name and :age as flexible fields
```

### Serial → Struct

Allowed only if the serial's fields satisfy the struct's schema. Extra fields in the serial are ignored; missing fields are an error.

The compiler performs best-effort static analysis:
- **Provably matches** — no handling needed
- **Provably mismatches** — PGE04002 (schema mismatch)
- **Cannot prove match** — user must handle with `[!]` + `*Continue >FallBack`. If absent → PGE04009

```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] handled — *Continue provides fallback if serial doesn't match
[r] $defaultUser#UserRecord
   [r] $defaultUser.name << "Unknown"
   [r] $defaultUser.age << 0

[r] $user#UserRecord << $dynamicSerial
   [!] !SchemaMismatch
      [*] *Continue >FallBack << $defaultUser
```

See [TYPE-IDENTITY](../../technical/compile-rules/TYPE-IDENTITY.md) rules 5 and 6, [PGE04009](../../technical/compile-rules/PGE/PGE04009-unhandled-serial-struct-conversion.md).

## See Also

- [[syntax/types/structs|Struct Types]] — struct definitions and field rules
- [[concepts/collections/serial|#Serial]] — schema-free tree type
- [[concepts/pipelines/error-handling|Error Handling]] — `[!]` blocks for handling conversion failures
