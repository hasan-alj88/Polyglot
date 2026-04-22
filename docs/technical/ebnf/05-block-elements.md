---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 5. Block Elements and IO Brackets

Block elements are square-bracket markers `[X]` that begin each line within a block. IO brackets are round-bracket markers `(X)` that wire data into or out of operators. The bracket shape encodes the role: `{X}` defines, `[X]` controls, `(X)` binds IO.

### 5.1 Block Elements (Control Instructions)

```ebnf
block_element       ::= registry_elem
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

(* Permission — RETIRED: [_] block element removed; permissions now use IO markers:
   (-) _PermName in {-} pipelines, (#) _PermName in {#} definitions,
   (@) _PermName in {@} packages. See §5.2 IO Brackets. *)

(* Execution *)
execution_elem      ::= "[-]" | "[=]" | "[b]" | "[#]" | "[$]" ;
                      (* [-] sequential, [=] parallel, [b] background, [#] data load,
                         [$] constructor type binding.
                         [#] is dual-context: execution element in {-} pipeline bodies,
                         schema/type declaration element in {#} definition bodies — see §4.3, §9.2.
                         [$] appears only inside {$} constructor definitions — see §9.13 *)

(* Control Flow *)
control_flow_elem   ::= "[?]" | "[!]" | "[T]" | "[Q]" | "[W]" ;

(* Scope *)
scope_elem          ::= "[\]" | "[/]" ;

(* Data Access *)
data_access_elem    ::= "[.]" | "[:]" ;

(* Logical *)
logical_elem        ::= "[&]" | "[+]" | "[^]" ;
                      (* [+] is OR — unified for both triggers and conditionals *)

(* Line Continuation *)
continuation_elem   ::= "[~]" ;

(* Foreign Code *)
foreign_code_elem   ::= "[C]" ;

(* Metadata *)
metadata_elem       ::= "[%]" ;

(* Comment *)
comment_elem        ::= "[ ]" ;
```

### 5.2 IO Brackets (Data Binding)

```ebnf
io_bracket          ::= "(-)" | "(=)" | "(*)" | "(_)" | "(>)" | "(<)"
                      | "($)" | "(.)" | "( )" ;
```

IO brackets use round brackets to distinguish data binding from control instructions. The symbol inside matches the operator prefix on the parent line:

| IO Bracket | Parent Operator | Meaning |
|------------|-----------------|---------|
| `(-)` | Pipeline call (`-Name`) | Pipeline IO — inputs, outputs, errors |
| `(=)` | Expander (`=ForEach.*`) | Expander IO — expand inputs/outputs |
| `(*)` | Collector (`*Into.*`, `*All`) | Collector IO — wait/collect bindings |
| `(_)` | Permission ref (under `(-)`, `(#)`, `(@)`) | Permission IO — inputs for `__` generic permissions |
| `(>)` | Under `(-)` output line | Output parameter handling (fallback) |
| `(<)` | Under `(-)` input line | Input parameter handling (fallback) |
| `($)` | Pipeline/chain call; `{$}` definition | Dual-context: (1) Operation label — names a call for downstream IO access (under `(-) $Label`). (2) Constructor IO — regex capture parameter declaration (under `{$}` — see §9.13) |
| `(.)` | Under `($)` in chains | Chain step label — names individual chain steps by position |
| `( )` | Any IO context | IO comment — inline annotation |

**Rule:** `(>)` (output parameter handling) and `(<)` (input parameter handling) are scoped under `(-)` IO lines — they handle IO parameters (e.g., the `!<` / `!>` fallback operators provide error-recovery values — see §10.2). These are distinct from `(*) <<` / `(*) >>` collector IO lines, which handle wait/collect semantics for parallel synchronization (see §12). `(<)` also appears nested under `(#) <param` in `{#}` definitions as a type parameter constraint block (see §4.3).

**Rule:** `[#]` serves two contexts. In `{-}` pipeline execution bodies, `(#) <param` loads data parameters (execution element). In `{#}` definition bodies, `[#]` introduces schema composition (`[#] ##Schema`), field type composition (`[#] ###FieldType`), schema properties (`[#] %##Property << value`), and inheritance (`(#) <~ #Parent`) — see §4.3 for grammar rules and §9.2 `data_body_line` for how these integrate into data definitions.

---
