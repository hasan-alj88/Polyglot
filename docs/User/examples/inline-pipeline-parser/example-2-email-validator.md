---
title: Example 2 - Email Validator
doc_type: tutorial
created: 2025-12-23
last_updated: 2025-12-23
tags:
  - examples
  - tutorial
  - email-validation
  - inline-pipeline
related_documents:
  - overview.md
  - example-1-config-validator.md
---

# Example 2: Email Validator

**Use Case:** Parse and validate email addresses in `"user@domain"` format

## Complete Implementation

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
[+] -"   if '@' not in email:"
[+] -"       return dict(user='', domain='')"
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

// Both user and domain must be non-empty
[f] $user =? ""
   [r] >is_valid << #False
[f] $domain =? ""
   [r] >is_valid << #False
[f] $user !? ""
   [f] $domain !? ""
      [r] >is_valid << #True
{x}
```

## Usage

```polyglot
// Valid email
[r] |ValidateEmail "admin@example.com"
(|) >is_valid >> $valid  // → #True

// Invalid - missing @
[r] |ValidateEmail "invalid.email"
(|) >is_valid >> $invalid  // → #False (user and domain both empty)

// Invalid - no user
[r] |ValidateEmail "@example.com"
(|) >is_valid >> $invalid  // → #False

// Invalid - no domain
[r] |ValidateEmail "admin@"
(|) >is_valid >> $invalid  // → #False
```

---
