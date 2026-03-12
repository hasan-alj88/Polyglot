---
title: Example 1 - Config Validator
doc_type: tutorial
created: 2025-12-23
last_updated: 2025-12-23
tags:
  - examples
  - tutorial
  - config-validation
  - inline-pipeline
related_documents:
  - overview.md
  - pattern-summary.md
---

# Example 1: Config Validator

**Use Case:** Validate configuration entries in `"type:value"` format

## Complete Implementation

```polyglot
{#} #;MyApp;Config;Type
[A] #ConfigType
[.] .Database
[.] .API
[.] .Cache
{x}


{|} |ParseValidateConfig
[t] |T.Call

[<] <formatted_string:pg.string
[>] >config_type :#ConfigType
[>] >value :pg.string

[w] |W.Runtime.Python3.9
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.formatted_string:pg.string << $formatted_string
(|) >config_type :#ConfigType >> >config_type
(|) >value :pg.string >> >value
(|) <code:pg.string << |U.String.Python""
[+] +"def parse(formatted_string:str)->dict:"
[+] -"   config_type, value = formatted_string.split(':')"
[+] -"   return dict(config_type=config_type, value=value)"
{x}


{|} |ValidateConfig
[%] %Pipeline.Inline
   [%] |ParseValidateConfig
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >config_type :#ConfigType >> <config_type
   (|) >value :pg.string >> <value

[<] <config_type :#ConfigType
[<] <value :pg.string
[>] >is_valid :pg.bool <~ #False

[W] |W.Polyglot.Scope
[m] $result << $config_type
   [?] #ConfigType.Database ? "db_valid"
   [?] #ConfigType.API ? "api_valid"
   [?] #ConfigType.Cache ? "cache_valid"
   [?] * ? "unknown"

[f] $result =? "db_valid"
   [f] $value =? ""
      [r] >is_valid << #False
   [f] *?
      [r] >is_valid << #True
{x}
```

## Usage

```polyglot
// Valid database config
[r] |ValidateConfig "Database:config.yml"
(|) >is_valid >> $db_valid  // → #True

// Empty value (invalid)
[r] |ValidateConfig "Database:"
(|) >is_valid >> $db_invalid  // → #False

// API config
[r] |ValidateConfig "API:https://api.example.com"
(|) >is_valid >> $api_valid  // → #True
```

## Execution Flow

1. String `"Database:config.yml"` passed to |ValidateConfig
2. %Formatted_string captures the string
3. |ParseValidateConfig executes:
   - Splits on `':'`
   - Returns `{config_type: "Database", value: "config.yml"}`
4. Parser outputs wire to main inputs:
   - `config_type` → `<config_type` (enum #ConfigType.Database)
   - `value` → `<value` (string "config.yml")
5. Match expression routes to "db_valid"
6. Value checked: not empty → `>is_valid` = #True

---
