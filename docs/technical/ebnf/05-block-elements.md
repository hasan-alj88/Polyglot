---
audience: developer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 5. Block Elements

Block elements are square-bracket markers that begin each line within a block.

```ebnf
block_element       ::= registry_elem
                      | data_flow_elem
                      | execution_elem
                      | control_flow_elem
                      | scope_elem
                      | data_access_elem
                      | logical_elem
                      | continuation_elem
                      | foreign_code_elem
                      | metadata_elem
                      | comment_elem ;

(* Registry *)
registry_elem       ::= "[@]" ;

(* Data Flow *)
data_flow_elem      ::= "[=]" | "[~]" | "[*]" | "[>]" | "[<]" ;

(* Execution *)
execution_elem      ::= "[r]" | "[p]" | "[b]" | "[#]" ;

(* Control Flow *)
control_flow_elem   ::= "[?]" | "[!]" | "[t]" | "[Q]" | "[W]" ;

(* Scope *)
scope_elem          ::= "[\]" | "[/]" | "[{]" | "[}]" ;

(* Data Access *)
data_access_elem    ::= "[.]" | "[:]" ;

(* Logical *)
logical_elem        ::= "[&]" | "[|]" | "[-]" | "[^]" ;

(* Line Continuation *)
continuation_elem   ::= "[+]" ;

(* Foreign Code *)
foreign_code_elem   ::= "[c]" ;

(* Metadata *)
metadata_elem       ::= "[%]" ;

(* Comment *)
comment_elem        ::= "[ ]" ;
```

**Rule:** `[>]` (output fallback) and `[<]` (input fallback) are scoped under `[=]` IO lines — they use the `<!` fallback operator to provide error-recovery values (see §10.2). `[<]` also appears nested under `[#] <param` in `{#}` definitions as a type parameter constraint block (see §4.3).

---
