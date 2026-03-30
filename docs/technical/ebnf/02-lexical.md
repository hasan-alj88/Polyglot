---
audience: developer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 2. Lexical Elements

### 2.1 Indentation & Lines

```ebnf
indent              ::= { "   " } ;                  (* 3 spaces per level *)

line                ::= indent [ block_element ] expression NEWLINE ;

NEWLINE             ::= '\n' ;
```

**Rule:** Exactly one expression per line. No tabs. Indentation is 3 spaces per level. Scope is determined by indentation depth — no closing markers.

### 2.2 Character Classes

```ebnf
letter              ::= 'A'..'Z' | 'a'..'z' ;
digit               ::= '0'..'9' ;
name_char           ::= letter | digit ;
name                ::= letter { name_char } ;
```

### 2.3 Literals

```ebnf
string_literal      ::= '"' { string_content } '"' ;

string_content      ::= any_char - '"' - '{'
                      | interpolation
                      | "{{" | "}}" ;          (* escaped literal braces *)

interpolation       ::= '{' variable_id '}' ;  (* e.g., {$name}, {$user:location} *)
int_literal         ::= [ '-' ] digit { digit } ;
                      (* Runtime RE: ^-?[0-9]+$  — leading zeros allowed *)
float_literal       ::= [ '-' ] digit { digit } '.' digit { digit } ;
                      (* Runtime RE: ^-?[0-9]+\.[0-9]+$  — leading zeros allowed *)
bool_literal        ::= "#Boolean.True" | "#Boolean.False" ;

literal             ::= string_literal
                      | int_literal
                      | float_literal
                      | bool_literal ;
```

---
