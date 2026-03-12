# Complete Example: Config Validator

This example demonstrates all %Pipeline.Inline patterns:

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

**Usage:**
```polyglot
[r] |ValidateConfig "Database:config.yml"
(|) >is_valid >> $valid
```

The string `"Database:config.yml"` is:
1. Captured by `%Formatted_string`
2. Passed to `|ParseValidateConfig`
3. Parsed into `config_type="Database"` and `value="config.yml"`
4. Wired to main pipeline inputs
5. Validated by main logic

---
