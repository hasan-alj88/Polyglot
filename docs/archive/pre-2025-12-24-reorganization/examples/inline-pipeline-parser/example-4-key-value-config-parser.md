# Example 4: Key-Value Config Parser

**Use Case:** Parse simple `"key=value"` configuration strings

## Complete Implementation

```polyglot
{|} |ParseKeyValue
[t] |T.Call
[<] <formatted_string:pg.string
[>] >key :pg.string
[>] >value :pg.string

[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.kv:pg.string << $formatted_string
(|) >key :pg.string >> >key
(|) >value :pg.string >> >value
(|) <code:pg.string << |U.String.Python""
[+] +"def parse(kv:str)->dict:"
[+] -"   if '=' not in kv:"
[+] -"       return dict(key=kv, value='')"
[+] -"   key, value = kv.split('=', 1)  # Only split on first ="
[+] -"   return dict(key=key.strip(), value=value.strip())"
{x}


{|} |SetConfig
[%] %Pipeline.Inline
   [%] |ParseKeyValue
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >key :pg.string >> <key
   (|) >value :pg.string >> <value

[<] <key :pg.string
[<] <value :pg.string
[>] >success :pg.bool <~ #False

// Store in config registry
[r] |Config.Set
(|) <key :pg.string << $key
(|) <value :pg.string << $value
(|) >result :pg.bool >> >success
{x}
```

## Usage

```polyglot
// Standard key=value
[r] |SetConfig "database_url=postgresql://localhost/mydb"
(|) >success >> $set

// Key with spaces
[r] |SetConfig "api key = abc123def456"
// → key="api key", value="abc123def456" (trimmed)

// Value with = character
[r] |SetConfig "connection_string=user=admin;pass=test123"
// → key="connection_string", value="user=admin;pass=test123"
```

---
