---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# Custom Errors

<!-- @c:errors -->
<!-- @c:blocks -->

## Defining Custom Errors (`{!}`)

All user-defined errors live under the `!Error` namespace. `{!} !Name` implicitly creates `!Error:Name.*` in the metadata tree. Use `[:]` for extensible branches and `[.]` for terminal leaves (typed `#Error`):

```aljam3
{!} !Error
   [:] :Validation
      [.] .Empty#Error
         (-) .MessageTemplate << "Field {field} is required"
         (-) .Info
            [:] :field#string
      [.] .TooLong#Error
         (-) .MessageTemplate << "{field} exceeds {maxLength} characters"
         (-) .Info
            [:] :field#string
            [:] :maxLength#int
      [.] .InvalidEmail#Error
         (-) .MessageTemplate << "Invalid email format: {email}"
         (-) .Info
            [:] :email#string
```

This creates `!Error:Validation.Empty`, `!Error:Validation.TooLong`, `!Error:Validation.InvalidEmail` — all carrying the `#Error` struct with their `.MessageTemplate` and `.Info` schema defined at the definition site. The raise site fills `.Info` values only. Note: the pglib `!Validation` namespace (shown in [[pglib/errors/builtin/validation|!Validation]]) is separate — it has fixed leaves defined by the runtime, not user code.

`{!}` creates entries at `%!.Error:Name.*` in the metadata tree. See [[data-is-trees#How Concepts Connect]].

## `!Error` — User-Extensible Namespace

`!Error` is the only namespace with user-extensible children. All other namespaces (`!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Field`, `!Alias`, `!Permission`, `!RT`, `!Env`, `!Storage`, `!Text`, `!CSV`) have Aljam3-defined fixed leaves.

Users extend `!Error` via `{!}` blocks using `[:]` for extensible branches and `[.]` for terminal leaves. Siblings at the same level must all use the same separator (sibling homogeneity rule).

```aljam3
{!} !Error
   [:] :MyApp
      [:] :Auth
         [.] .Expired#Error
            (-) .MessageTemplate << "Token for {userId} expired at {expiredAt}"
            (-) .Info
               [:] :userId#string
               [:] :expiredAt#string
         [.] .Invalid#Error
            (-) .MessageTemplate << "Invalid token format"
      [:] :Data
         [.] .Corrupt#Error
            (-) .MessageTemplate << "Data corrupted in {source}: {reason}"
            (-) .Info
               [:] :source#string
               [:] :reason#string
         [.] .Missing#Error
            (-) .MessageTemplate << "Required data not found: {key}"
            (-) .Info
               [:] :key#string
      [:] :GeneralFailure#Error
         (-) .MessageTemplate << "Application error: {reason}"
         (-) .Info
            [:] :reason#string
```

This creates `!Error:MyApp:Auth.Expired`, `!Error:MyApp:Auth.Invalid`, `!Error:MyApp:Data.Corrupt`, `!Error:MyApp:Data.Missing`, and `!Error:MyApp:GeneralFailure`. Each terminal carries its `.MessageTemplate` and `.Info` schema.

Tree path: `%!.Error:MyApp:Auth.Expired` — `.Error` is Aljam3-defined (fixed), `:MyApp:Auth` are user-extensible (flexible), `.Expired` is a terminal leaf (fixed).
