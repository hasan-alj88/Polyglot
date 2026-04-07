---
audience: designer
type: spec
updated: 2026-04-04
---

<!-- @ebnf/INDEX -->

## 4. Type System

### 4.1 Type Annotations

```ebnf
type_annotation     ::= '#' type_expr ;

type_expr           ::= basic_type
                      | collection_type
                      | wildcard_type
                      | user_type
                      | live_type ;

live_type           ::= "live" type_expr ;    (* Polyglot-managed, read-only *)

basic_type          ::= "RawString" | "string" | "int" | "uint" | "float"
                      | "sci" | "eng" | "dim" | "bool" | "path" ;
                      (* RawString: compiler intrinsic, literal raw chars, no interpolation.
                         string (#String): struct with .string#RawString + .regex#RawString.
                         int, uint, float, sci, eng, dim: #String subtypes with pre-set .regex patterns.
                         bool (#Boolean): ##Enum type, not a #String subtype.
                         path (#path): struct with .Unix#string + .Windows#string. *)

collection_type     ::= array_type | dict_type | dataframe_type | serial_type ;

array_type          ::= "array" [ flex_sep type_param ] [ flex_sep dimension ] ;
                      (* e.g., #array:int, #array:float:2D, #array:Person *)
dict_type           ::= "dict" flex_sep type_param flex_sep type_param ;
                      (* e.g., #dict:string:int — key type : value type *)
dataframe_type      ::= "dataframe" flex_sep enum_type_param flex_sep type_param ;
                      (* e.g., #dataframe:SalesColumns:string — column enum : cell value type.
                         enum_type_param must resolve to a ###ScalarEnum type (##EnumLeafs). *)
serial_type         ::= "serial" ;

type_param          ::= basic_type | dimension | user_type | wildcard_type ;
                      (* Nested type refs drop the # prefix within type context *)
enum_type_param     ::= user_type ;
                      (* Must resolve to a ###ScalarEnum type at compile time — PGE04022 if not *)
dimension           ::= digit { digit } "D" ;
                      (* e.g., :2D, :3D — omitted defaults to 1D *)

wildcard_type       ::= "*" ;                 (* #* — any type; used in generic constraints *)

user_type           ::= dotted_name ;         (* e.g., Person — no # prefix in type annotations *)
```

**Rule:** `#` starts a type context. Within that context, nested type references separated by `:` **drop the `#` prefix** — the compiler resolves them. Examples: `$score#int`, `$users#array:Person`, `$map#dict:string:int`, `$matrix#array:float:2D`.

### 4.2 Typed Variable

```ebnf
typed_variable      ::= variable_id [ type_annotation ] ;
typed_field         ::= field_ref [ type_annotation ] ;
typed_io_param      ::= io_param [ type_annotation ] ;
```

### 4.3 Type Macros, Wrappers, and {#} Schema Rules

```ebnf
(* --- {#} schema rules (no generic type parameters) --- *)

schema_inheritance  ::= "[#]" "<~" data_id ;
                      (* e.g., [#] <~ #String — inherit schema, can specialize *)
                      (* e.g., [#] <~ #Map:#UnsignedInt:$ValueType — parameterized inheritance inside {M} *)

schema_composition  ::= "[#]" "<<" schema_id ;
                      (* e.g., [#] << ##Flat — compose a schema into this type *)
                      (* Multiple [#] << lines accumulate: one line, one schema.
                         Two schemas setting the same % property to the same value → agree (no error).
                         Two schemas setting the same % property to different values → PGE11001. *)

field_type_composition ::= "[#]" "<<" field_type_id ;
                      (* e.g., [#] << ###ScalarEnum — declare explicit field type *)

schema_property     ::= "[#]" "%##" dotted_name assignment_op expression ;
                      (* e.g., [#] %##Children.Type << #UnsignedInt *)
                      (* e.g., [#] %##Alias << "int" *)
                      (* e.g., [#] %##Depth.Max << 0 *)

field_type_property ::= "[#]" "%###" dotted_name assignment_op expression ;
                      (* e.g., [#] %###Value — field-level metadata *)

type_constraint     ::= "[<]" "<<" schema_id ;
                      (* Nested under [#] <Param in {M} — constrains a macro parameter *)
                      (* e.g., [<] << ##Scalar — param must be scalar type *)

(* --- {M} type macro definitions --- *)

macro_def           ::= "{M}" '#' dotted_name NEWLINE
                         indent ( macro_param | macro_type_param ) NEWLINE
                         { indent macro_type_body_line NEWLINE } ;
                      (* At least one parameter required — parameterless {M} is PGE01023 *)
                      (* e.g., {M} #Array, {M} #String.Subtype *)

macro_type_body_line ::= macro_param
                       | macro_type_param
                       | nested_data_def
                       | exec_line
                       | comment_line ;

macro_param         ::= "[#]" '<' name schema_id [ default_push_left value_expr ] NEWLINE
                         { indent type_constraint NEWLINE } ;
                      (* Value input — e.g., [#] <Name#RawString, [#] <Dim##Dimension <~ "1D" *)

macro_type_param    ::= "[#]" "<#" name NEWLINE
                         { indent type_constraint NEWLINE } ;
                      (* Type-as-data-tree input — e.g., [#] <#ValueType *)
                      (* The # definition itself is passed as data (all definitions are trees) *)

nested_data_def     ::= "{#}" data_id NEWLINE
                         { indent data_body_line NEWLINE } ;
                      (* Macro body generates {#} definitions — e.g., {#} #{$ArrayName} *)

(* --- [M] macro invocation inside {#} --- *)

macro_invoke        ::= "[M]" '#' dotted_name NEWLINE
                         { indent macro_arg NEWLINE } ;
                      (* e.g., [M] #String.Subtype *)

macro_arg           ::= "[#]" '<' name "<<" value_expr NEWLINE
                         { indent macro_arg_fallback NEWLINE } ;
                      (* e.g., [#] <Name << "Int" *)

macro_arg_fallback  ::= "[<]" error_id "<<" value_expr ;
                      (* e.g., [<] !Alias.Clash << "integer" — fallback on error *)

(* --- {W} wrapper definitions --- *)

wrapper_def         ::= "{W}" '=' dotted_name NEWLINE
                         { indent wrapper_body_line NEWLINE } ;
                      (* e.g., {W} =W.Polyglot, {W} =W.DB.Connection *)

wrapper_body_line   ::= scope_setup
                       | scope_cleanup
                       | from_outer
                       | to_outer
                       | exec_line
                       | comment_line ;
                      (* Wrappers contain [\]/[/] scope and [{]/[}] IO — never {#} definitions *)

(* --- Macro dispatch rule --- *)
(* Macros overload by signature — ordered list of parameter count and kind:
   <#ParamName  = type input  (a # definition as data tree)
   <ParamName   = value input (a typed value)
   Dispatch matches by parameter count AND parameter kind.
   Two overloads with identical signature = compile error (PGE01019). *)
```

**Rule:** Parameterized types use `{M}` type macros to generate `{#}` definitions at compile time. `{M}` defines type macros; `[M]` invokes them inside `{#}` blocks. `{W}` defines wrappers; `[W]` invokes them. `<~` in `{#}` means **only** inheritance — never macro invocation. Schema composition (`[#] << ##Name`) accumulates — each line adds one schema's properties. Schema properties (`[#] %##`) declare tree-level compile-time metadata. Field type properties (`[#] %###`) declare leaf-level metadata. Type constraints (`[<]`) restrict what types may bind to a macro parameter. Schema references (`##`) are only valid inside `{#}` definitions (PGE05006). Two macros with identical signatures (same name, same parameter count and kind) produce compile error PGE01019.

### 4.4 Tree Child Accessor

```ebnf
child_access        ::= variable_id '<' name { '<' name } ;
                      (* e.g., $myMap<name, $myArray<0, $matrix<0<1 *)
                      (* Chained: $cube<2<3<0 — branch 2, branch 3, leaf 0 *)
```

**Rule:** The `<` character after a `$variable` is a tree child accessor. It navigates into flexible children declared with `[:]` in `{#}` definitions. Fixed fields still use `.` (`$user.name`). The parser distinguishes `<` by context — after `[#]` in a `{M}` macro it is a parameter declaration; after a `$variable` it is child access.

---
