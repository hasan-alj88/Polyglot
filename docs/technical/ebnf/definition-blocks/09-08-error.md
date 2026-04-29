---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.8 Error Definition

```ebnf
error_def           ::= "{!}" error_namespace_id NEWLINE
                         indent error_body_line NEWLINE
                         { indent error_body_line NEWLINE } ;
                      (* At least one leaf required — empty {!} is PGE01022 *)

error_namespace_id  ::= '!' dotted_name ;

error_body_line     ::= "[.]" fixed_field "#Error"       (* terminal leaf *)
                      | "[:]" flexible_field              (* user-extensible branch — !Error only *)
                      | metadata_line
                      | comment_line ;
                      (* Siblings at same level must use same separator — PGE05001 *)
```

`{!}` defines an error tree. Each terminal leaf is typed `#Error`. The namespace uses the `!` prefix. pglib error namespaces (`!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Field`, `!Alias`, `!Permission`, `!RT`) are built-in and use `[.]` fixed leaves only.

User-defined `{!} !Name` implicitly nests under `!Error` in the metadata tree, creating `!Error:Name.*`. Only `{!} !Error` allows `[:]` flexible children for user-extensible branches. All other `{!}` namespaces use `[.]` fixed leaves only.

**pglib example** (runtime-defined, fixed leaves):
```aljam3
{!} !Validation
   [.] .Schema#Error
   [.] .Type#Error
   [.] .Regex#Error
```

**User example** (extensible branches under `!Error`):
```aljam3
{!} !Error
   [:] :MyApp
      [:] :Auth
         [.] .Expired#Error
         [.] .Invalid#Error
      [:] :Data
         [.] .Corrupt#Error
```

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.8 `{!}` Error | [[concepts/errors\|errors]] |
