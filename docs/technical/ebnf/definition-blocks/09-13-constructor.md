---
audience: design
type: spec
updated: 2026-04-22
---

<!-- @ebnf/INDEX -->

## 9.13 Constructor Definition (`{$}`)

```ebnf
constructor_def     ::= "{$}" constructor_header NEWLINE
                         constructor_body ;

constructor_header  ::= "$" dotted_name '"' pattern_string '"' ;

pattern_string      ::= { pattern_literal | pattern_capture } ;
pattern_literal     ::= (* any character except '{' and '"' *) ;
pattern_capture     ::= "{" IDENTIFIER "}" ;

constructor_body    ::= { constructor_io_line }
                         constructor_action
                         { constructor_field_line }
                       | constructor_native_body ;

constructor_io_line ::= "($)" "<" IDENTIFIER ".re" "<<" '"' regex_pattern '"' ;

constructor_action  ::= "[$]" "#" type_ref ;

constructor_field_line ::= "[.]" "." IDENTIFIER "<<" source_ref ;

source_ref          ::= "<" IDENTIFIER           (* capture parameter *)
                      | "%" metadata_path         (* runtime metadata *)
                      | "$" IDENTIFIER            (* constructor-sourced variable *) ;

constructor_native_body ::= constructor_native_call
                            constructor_action
                            { constructor_field_line } ;

constructor_native_call ::= "[-]" "-" pipeline_ref NEWLINE
                            { "(-)" io_line } ;
```

**Rules:**

- `{$}` defines a constructor — a compile-time-guaranteed Final value producer with no error surface.
- Constructor identifier uses the `$` prefix (e.g., `$DT`, `$Path`). Follows prefix symmetry: `{#}` → `#Type`, `{-}` → `-Pipeline`, `{$}` → `$Constructor`.
- `($)` IO lines declare regex-validated capture parameters. Each `($) <name.re << "regex"` declares one named capture with its validation pattern. The `.re` field is **mandatory** — every capture must have a regex constraint.
- `[$]` type binding — exactly one per overload, must appear after all `($)` IO lines and before `[.]` field assignments. Declares the target type: `[$] #TargetType`.
- `[.]` fixed field assignment maps captured values or metadata to target type fields. Same syntax as in `{#}` definitions.
- **Keyword overloads** have no `($)` lines — the entire `pattern_string` is treated as an exact-match regex (`^pattern$`). The `constructor_body` then has zero `constructor_io_line` entries.
- **Native pipeline overloads** (`constructor_native_body`) use `[-]` calls to infallible native pipelines. **pglib only** — user-defined constructors cannot use `[-]` calls inside `{$}`. The called pipeline must be a `{N}` native definition that the compiler trusts as infallible.
- **Overload resolution:** A constructor name (e.g., `$DT`) can have multiple `{$}` definitions. Each is an overload. The compiler resolves via regex matching: exactly one match required. Zero matches = compile error. Multiple matches = ambiguity error on the *definitions* (detected at definition compile time).
- **Structural integrity check:** At definition compile time, the compiler verifies that no capture slot's `.re` pattern can match the pattern's literal separators. This prevents values from breaking pattern structure.
- **Interpolation:** Constructor arguments may contain `{$var}` interpolation, but only if `$var` was produced by another constructor. Runtime/IO-sourced variables are a compile error.
- `{$}` creates a branch on the `%` metadata tree at `%definition.$`.
- **Cross-package constructors:** Any package can define `{$}` constructors for types from other packages. Visibility scoped by `[@]` imports — callers only see constructors from imported packages. Overlapping overloads across imports are an ambiguity error at the import site.

**Examples:**

String-parsing overload:

```aljam3
{$} $DT"{hours}:{min}:{seconds}"
   ($) <hours.re << "[0-9][0-9]"
   ($) <min.re << "[0-9][0-9]"
   ($) <seconds.re << "[0-9][0-9]"
   [$] #DT.Time
   [.] .hours << <hours
   [.] .minutes << <min
   [.] .seconds << <seconds
```

Keyword overload:

```aljam3
{$} $DT"Today"
   [$] #DT.Date
   [.] .year << %Runtime.Date.Year
   [.] .month << %Runtime.Date.Month
   [.] .day << %Runtime.Date.Day
```

Native pipeline overload (pglib only):

```aljam3
{$} $DT"Now"
   [-] -DT.Current
      (-) >hours >> $hrs
      (-) >minutes >> $min
      (-) >seconds >> $sec
   [$] #DT.Time
   [.] .hours << $hrs
   [.] .minutes << $min
   [.] .seconds << $sec
```

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.13 `{$}` Constructor | [[syntax/constructors\|constructors]] |
