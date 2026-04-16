---
audience: designer
type: spec
updated: 2026-04-05
---

<!-- @ebnf/INDEX -->

## 3. Identifiers

All identifiers require a prefix sigil. Field separators navigate within identifiers.

### 3.1 Prefixed Identifiers

```ebnf
package_id          ::= '@' package_address ;
data_id             ::= '#' dotted_name ;
schema_id           ::= "##" name ;               (* e.g., ##Scalar, ##Flat, ##Contiguous *)
field_type_id       ::= "###" name ;               (* e.g., ###Value, ###Enum, ###ScalarValue, ###ScalarEnum, ###None *)
pipeline_id         ::= '-' dotted_name ;
variable_id         ::= '$' field_path ;
error_id            ::= '!' dotted_name ;
permission_id       ::= '_' name ;                 (* e.g., _DataCeiling, _ReportReader *)
perm_descriptor_id  ::= "__" name ;                (* e.g., __Permission, __PermissionTarget *)
perm_constraint_id  ::= "___" name ;               (* e.g., ___Unix, ___Sandboxed, ___ReadOnly *)

identifier          ::= package_id
                      | data_id
                      | schema_id
                      | field_type_id
                      | pipeline_id
                      | variable_id
                      | error_id
                      | permission_id
                      | perm_descriptor_id
                      | perm_constraint_id ;
```

### 3.2 Field Separators

```ebnf
(* Fixed fields — predefined schema keys *)
fixed_sep           ::= '.' ;

(* Flexible fields — user-defined keys *)
flex_sep            ::= ':' ;

(* Metadata fields — read-only, Polyglot-managed *)
meta_sep            ::= '%' ;

dotted_name         ::= name { fixed_sep name } ;

flex_path           ::= name { flex_sep name } ;

(* Metadata access — query metadata on any named object *)
meta_access         ::= identifier meta_sep name ;

(* A field_path may use fixed OR flexible separators, but NOT both at the same sibling level *)
field_path          ::= name { field_separator name } ;

field_separator     ::= fixed_sep | flex_sep ;
```

**Rule (sibling homogeneity):** All siblings at the same depth level must use the same separator type. Mixing `.` and `:` among siblings is invalid.

### 3.3 Package Addresses

```ebnf
package_address     ::= registry_type flex_sep registry_id
                         fixed_sep package_name
                         { fixed_sep sub_package }
                         [ flex_sep version ] ;

registry_type       ::= "Local" | "Community" | "Registry" ;

(* Registry ID format depends on registry_type:
   Local      — port number (unused port, e.g., :999, :042)
   Community  — username.ProjectName (e.g., :devops.NotificationHub)
   Registry   — registered company name (e.g., :Acme) *)
registry_id         ::= name | digit { digit } ;
package_name        ::= name ;
sub_package         ::= name ;
version             ::= 'v' digit { digit } '.' digit { digit } '.' digit { digit } [ '.' digit { digit } ] ;
```

**Example:** `@Local:999.MyPackage.Sub:v1.2.3.2`

### 3.4 Cross-Package References

```ebnf
cross_pkg_data      ::= '@' name data_id ;            (* @alias#DataName *)
cross_pkg_pipeline  ::= '@' name pipeline_id ;         (* @alias-PipelineName *)
cross_pkg_enum      ::= '@' name '#' dotted_name ;     (* @alias#DataName.EnumField *)
```

---
