---
audience: designer
type: spec
updated: 2026-04-09
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
                         enum_type_param must resolve to a ###ScalarEnum type (PGE04022 if not). *)
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

### 4.3 Generic Types, Wrappers, and {#} Schema Rules

```ebnf
(* --- {#} schema and generic type rules --- *)

schema_composition  ::= "[#]" "<<" schema_id ;
                      (* e.g., [#] << ##Flat — compose a schema into this type *)
                      (* Multiple [#] << lines accumulate: one line, one schema.
                         Two schemas setting the same % property to the same value → agree (no error).
                         Two schemas setting the same % property to different values → PGE11001. *)

field_type_composition ::= "[#]" "<<" field_type_id ;
                      (* e.g., [#] << ###ScalarEnum — declare explicit field type *)

schema_property     ::= "[#]" "%##" dotted_name assignment_op expression ;
                      (* e.g., [#] %##Flexible << #FlexKind.Fixed *)
                      (* e.g., [#] %##Alias << "int" *)
                      (* e.g., [#] %##Depth.Max << 0 *)
                      (* e.g., [#] %##Count << #Bound.Inf *)

field_type_property ::= "[#]" "%###" dotted_name assignment_op expression ;
                      (* e.g., [#] %###Kind << #FieldKind.Enum — field-level metadata *)
                      (* e.g., [#] %###Type << #string — all leaves share this type *)

generic_param       ::= "[#]" '<#' name NEWLINE
                         { indent param_constraint NEWLINE } ;
                      (* Type parameter input — e.g., [#] <#KeyType, [#] <#ValueType *)
                      (* The # definition itself is passed as data (all definitions are trees) *)

value_param         ::= "[#]" '<' name schema_id [ default_push_left value_expr ] NEWLINE
                         { indent param_constraint NEWLINE } ;
                      (* Value input parameter — e.g., [#] <Dim##Dimension <~ "1D" *)
                      (* e.g., [#] <regex#RawString *)

param_constraint    ::= "[<]" "<<" schema_id ;
                      (* Nested under [#] <param — constrains what types may bind *)
                      (* e.g., [<] << ##Scalar — param must satisfy ##Scalar *)

(* --- Schema parameterization inside [#] << --- *)

schema_param_bind   ::= "[#]" "<<" schema_id NEWLINE
                         { indent "[#]" '<#' name "<<" type_ref NEWLINE }
                         { indent "[#]" '<' name "<<" value_expr NEWLINE } ;
                      (* e.g., [#] << ##Array
                                   [#] <#ValueType << <#ValueType
                                   [#] <Dim << <Dim
                         Positional binding: : separator in type annotations resolves params left-to-right.
                         e.g., #array:float:2D → ValueType=Float, Dim=2D *)

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
```

**Rule:** Parameterized types use generic `{#}` definitions with `[#] <#param` type inputs and `[#] <param` value inputs. The `:` separator in type annotations binds positionally to declared parameters (e.g., `#array:float:2D` → ValueType=Float, Dim=2D). Default values use `<~`; missing required params = compile error. `{W}` defines wrappers; `[W]` invokes them. Schema composition (`[#] << ##Name`) accumulates — each line adds one schema's properties. Parameterized schemas accept their own `[#] <#param` / `[#] <param` bindings nested under the `[#] <<` line. Schema properties (`[#] %##`) declare branch-level compile-time metadata. Field type properties (`[#] %###`) declare leaf-level metadata. Param constraints (`[<]`) restrict what types may bind to a parameter. Schema references (`##`) are only valid inside `{#}` definitions (PGE05006).

**Named `##` schemas:** ##Leaf, ##Scalar, ##Flat, ##Deep (depth); ##Inf (value); ##Contiguous, ##Sparse, ##Rectangular, ##Sorted (structure); ##Enum (classification); ##Fields, ##Nullable, ##Result, ##String, ##Map, ##Array, ##Set, ##Dataframe (parameterized).

**`%##` properties (branch-level):** %##Flexible (#FlexKind), %##Key (type ref), %##Range (range expr), %##Schema (list of ##), %##Active (#ActiveKind), %##Ordered (#Boolean), %##Sorted (#Boolean), %##Gap (#Boolean), %##Regular (#Boolean), %##Count (#Bound), %##Count.Min (#uint), %##Propagate (#Boolean), %##Level.N (scope), %##Depth.Max (#Bound), %##Alias (#NestedKeyString).

**`%###` properties (leaf-level):** %###Kind (#FieldKind), %###Type (type ref), %###Unique (#Boolean).

### 4.4 Tree Child Accessor

```ebnf
child_access        ::= variable_id '<' name { '<' name } ;
                      (* e.g., $myMap<name, $myArray<0, $matrix<0<1 *)
                      (* Chained: $cube<2<3<0 — branch 2, branch 3, leaf 0 *)
```

**Rule:** The `<` character after a `$variable` is a tree child accessor. It navigates into flexible children declared with `[:]` in `{#}` definitions. Fixed fields still use `.` (`$user.name`). The parser distinguishes `<` by context — after `[#]` in a `{#}` definition it is a parameter declaration; after a `$variable` it is child access.

---
