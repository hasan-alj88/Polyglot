# Additional Examples

## Email Parser

```polyglot
{|} |ParseEmail
[t] |T.Call
[<] <formatted_string:pg.string
[>] >user :pg.string
[>] >domain :pg.string

[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.email:pg.string << $formatted_string
(|) >user :pg.string >> >user
(|) >domain :pg.string >> >domain
(|) <code:pg.string << |U.String.Python""
[+] +"def parse(email:str)->dict:"
[+] -"   user, domain = email.split('@')"
[+] -"   return dict(user=user, domain=domain)"
{x}


{|} |ValidateEmail
[%] %Pipeline.Inline
   [%] |ParseEmail
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >user :pg.string >> <user
   (|) >domain :pg.string >> <domain

[<] <user :pg.string
[<] <domain :pg.string
[>] >is_valid :pg.bool <~ #False

// Email validation logic
{x}
```

**Usage:**
```polyglot
[r] |ValidateEmail "admin@example.com"
(|) >is_valid >> $valid
```

---
