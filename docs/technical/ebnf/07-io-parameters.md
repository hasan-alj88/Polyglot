---
audience: designer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 7. IO Parameters

```ebnf
io_param            ::= input_param | output_param ;

input_param         ::= '<' name { field_separator name } ;
output_param        ::= '>' name { field_separator name } ;
```

---
