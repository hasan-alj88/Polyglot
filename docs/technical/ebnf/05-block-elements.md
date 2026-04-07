---
audience: designer
type: spec
updated: 2026-04-06
---

<!-- @ebnf/INDEX -->

## 5. Block Elements

Block elements are square-bracket markers that begin each line within a block.

```ebnf
block_element       ::= registry_elem
                      | permission_elem
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

(* Permission *)
permission_elem     ::= "[_]" ;

(* Data Flow *)
data_flow_elem      ::= "[=]" | "[~]" | "[*]" | "[>]" | "[<]" ;

(* Execution *)
execution_elem      ::= "[r]" | "[p]" | "[b]" | "[#]" ;
                      (* [#] is dual-context: execution element in {=} pipeline bodies,
                         schema/type declaration element in {#} definition bodies — see §4.3, §9.2 *)

(* Control Flow *)
control_flow_elem   ::= "[?]" | "[!]" | "[T]" | "[Q]" | "[W]" ;

(* Scope *)
scope_elem          ::= "[\]" | "[/]" | "[{]" | "[}]" ;

(* Data Access *)
data_access_elem    ::= "[.]" | "[:]" ;

(* Logical *)
logical_elem        ::= "[&]" | "[|]" | "[-]" | "[^]" ;

(* Line Continuation *)
continuation_elem   ::= "[+]" ;

(* Foreign Code *)
foreign_code_elem   ::= "[C]" ;

(* Metadata *)
metadata_elem       ::= "[%]" ;

(* Comment *)
comment_elem        ::= "[ ]" ;
```

**Rule:** `[>]` (output parameter handling) and `[<]` (input parameter handling) are scoped under `[=]` IO lines — they handle IO parameters (e.g., the `<!` fallback operator provides error-recovery values — see §10.2). These are distinct from `[*] <<` / `[*] >>` collector IO lines, which handle wait/collect semantics for parallel synchronization (see §12). `[<]` also appears nested under `[#] <param` in `{#}` definitions as a type parameter constraint block (see §4.3).

**Rule:** `[#]` serves two contexts. In `{=}` pipeline execution bodies, `[#] <param` loads data parameters (execution element). In `{#}` definition bodies, `[#]` introduces schema composition (`[#] << ##Schema`), field type composition (`[#] << ###FieldType`), schema properties (`[#] %##Property`), and inheritance (`[#] <~ #Parent`) — see §4.3 for grammar rules and §9.2 `data_body_line` for how these integrate into data definitions.

---
