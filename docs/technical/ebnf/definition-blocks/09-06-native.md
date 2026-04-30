---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.6 Native Definition (`{N}`)

```ebnf
native_def          ::= "{N}" native_pipeline_id NEWLINE
                         native_def_body ;

native_pipeline_id  ::= '-' dotted_name ;

native_def_body     ::= { ( native_metadata_line | comment_line ) NEWLINE }
                         { indent ( io_decl_line | error_decl_line ) NEWLINE } ;
                      (* Native definitions contain ONLY metadata and IO declarations.
                         No [T], [Q], [W], or execution body. *)

native_metadata_line ::= "[%]" '.' native_field assignment_op value_expr ;

native_field        ::= "Kind"                   (* #NativeKind enum *)
                      | language_name             (* e.g., "Rust", "Cpp" *)
                      | "description" ;           (* human-readable description *)

language_name       ::= upper_letter { letter } ; (* Rust, Cpp, etc. *)
```

**Rules:**
- `{N}` defines a compiler-native pipeline — implemented in the host language, not Aljam3.
- `[%]` metadata under `{N}` implicitly scopes to `%Native.*` — all fixed `.` fields.
- `.Kind` is mandatory — must be one of `#NativeKind.Trigger`, `.Queue`, `.Wrapper`, `.Execution`, `.Intrinsic`.
- At least one `.<Language>` binding is required — must match the configured host language.
- No execution body (`[-]`, `[=]`, `[b]`, `[?]`), no `[T]`, no `[Q]`, no `[W]`.
- `(-)` IO declarations define the public interface (inputs, outputs, errors) — same as any pipeline.
- `{N}` definitions can only appear in aj3lib `.aj3` files — user `.aj3` files cannot define native pipelines.

**Example:**

```aljam3
{N} -File.Text.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextRead"
   [%] .description << "Read text file contents"
   (-) <path#path
   (-) >content#string
   (-) !File.NotFound
   (-) !File.PermissionDenied
```

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.6 `{N}` Native | [[concepts/pipelines/INDEX#Native vs Derived\|Native vs Derived]] |
