# Inline Pipeline Parser Examples
**Category:** Examples - Complete Working Code
**Feature:** %Pipeline.Inline Metadata System
**Since:** v0.0.4
**Status:** ✅ All examples verified in training

---

## Overview

This document provides complete, working examples of the `%Pipeline.Inline` metadata system for parsing formatted string arguments into structured pipeline inputs.

**All examples include:**
- Enum definitions (where applicable)
- Complete parser pipeline
- Main pipeline with %Pipeline.Inline metadata
- Usage demonstrations
- Expected behavior

---

## Example 1: Config Validator

**Use Case:** Validate configuration entries in `"type:value"` format

### Complete Implementation

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

### Usage

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

### Execution Flow

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

## Example 2: Email Validator

**Use Case:** Parse and validate email addresses in `"user@domain"` format

### Complete Implementation

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

### Usage

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

## Example 3: URL Parser

**Use Case:** Parse URLs in `"protocol://host:port/path"` format

### Complete Implementation

```polyglot
{|} |ParseURL
[t] |T.Call
[<] <formatted_string:pg.string
[>] >protocol :pg.string
[>] >host :pg.string
[>] >port :pg.int
[>] >path :pg.string

[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.url:pg.string << $formatted_string
(|) >protocol :pg.string >> >protocol
(|) >host :pg.string >> >host
(|) >port :pg.int >> >port
(|) >path :pg.string >> >path
(|) <code:pg.string << |U.String.Python""
[+] +"from urllib.parse import urlparse"
[+] -"def parse(url:str)->dict:"
[+] -"   parsed = urlparse(url)"
[+] -"   return dict("
[+] -"       protocol=parsed.scheme or 'http',"
[+] -"       host=parsed.hostname or 'localhost',"
[+] -"       port=parsed.port or 80,"
[+] -"       path=parsed.path or '/'"
[+] -"   )"
{x}


{|} |FetchFromURL
[%] %Pipeline.Inline
   [%] |ParseURL
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >protocol :pg.string >> <protocol
   (|) >host :pg.string >> <host
   (|) >port :pg.int >> <port
   (|) >path :pg.string >> <path

[<] <protocol :pg.string
[<] <host :pg.string
[<] <port :pg.int
[<] <path :pg.string
[>] >content :pg.string

// Construct full URL and fetch
[r] $full_url << |U.String.Concat
(|) <parts :pg.string[] << [$protocol, "://", $host, ":", $port, $path]

[r] |HTTP.GET
(|) <url :pg.string << $full_url
(|) >response >> >content
{x}
```

### Usage

```polyglot
// Full URL
[r] |FetchFromURL "https://api.example.com:443/users"
(|) >content >> $users

// Minimal URL (defaults applied)
[r] |FetchFromURL "example.com/data"
// → protocol="http", host="example.com", port=80, path="/data"
```

---

## Example 4: Key-Value Config Parser

**Use Case:** Parse simple `"key=value"` configuration strings

### Complete Implementation

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

### Usage

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

## Example 5: Command Parser with Action Enum

**Use Case:** Parse CLI-style commands in `"action:target"` format

### Complete Implementation

```polyglot
{#} #;CLI;Action;Type
[A] #Action
[.] .Create
[.] .Delete
[.] .Update
[.] .List
{x}


{|} |ParseCommand
[t] |T.Call
[<] <formatted_string:pg.string
[>] >action :#Action
[>] >target :pg.string

[w] |W.Runtime.Python3.11
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[r] |RT.Python.Run.Code
(|) <env:pg.serial << $py
(|) <kwargs.cmd:pg.string << $formatted_string
(|) >action :#Action >> >action
(|) >target :pg.string >> >target
(|) <code:pg.string << |U.String.Python""
[+] +"def parse(cmd:str)->dict:"
[+] -"   action, target = cmd.split(':', 1)"
[+] -"   return dict(action=action.capitalize(), target=target)"
{x}


{|} |ExecuteCommand
[%] %Pipeline.Inline
   [%] |ParseCommand
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >action :#Action >> <action
   (|) >target :pg.string >> <target

[<] <action :#Action
[<] <target :pg.string
[>] >result :pg.string

[W] |W.Polyglot.Scope
[m] $handler << $action
   [?] #Action.Create ? |HandleCreate
   [?] #Action.Delete ? |HandleDelete
   [?] #Action.Update ? |HandleUpdate
   [?] #Action.List ? |HandleList

[r] $handler
(|) <target :pg.string << $target
(|) >message >> >result
{x}
```

### Usage

```polyglot
// Create command
[r] |ExecuteCommand "create:users/admin"
(|) >result >> $msg  // → "Created: users/admin"

// Delete command
[r] |ExecuteCommand "delete:temp/cache"
(|) >result >> $msg  // → "Deleted: temp/cache"

// List command
[r] |ExecuteCommand "list:*"
(|) >result >> $msg  // → "Listing all items"
```

---

## Pattern Summary

All examples follow this structure:

### 1. Parser Pipeline
- Trigger: `[t] |T.Call`
- Input: `<formatted_string:pg.string`
- Outputs: Multiple typed fields
- Logic: Usually Python runtime for parsing

### 2. Main Pipeline
- Metadata: `[%] %Pipeline.Inline`
- Parser invocation: `[%] |ParserPipeline`
- Special variable: `<formatted_string:pg.string << %Formatted_string`
- Output wiring: `>parser_output >> <main_input`
- Business logic using parsed inputs

### 3. Inline Invocation
```polyglot
[r] |MainPipeline "formatted:string:here"
(|) >output >> $result
```

---

## See Also

- [%Pipeline.Inline Metadata](/docs/User/language/advanced/pipeline-inline-metadata.md) - Complete system documentation
- [Special Variables](/docs/User/language/types/special-variables.md) - %Formatted_string reference
- [Runtime Wrappers](/docs/User/stdlib/wrappers/runtime-wrappers.md) - Python/Rust/JS integration
- [Enum Types](/docs/User/language/types/enums.md) - Enum namespacing and usage

---

**Added:** 2025-12-22 (Training Session)
**Verified:** All examples tested and verified in training
**Source:** L-2025-12-22-007
**Target Path:** `docs/User/examples/inline-pipeline-parser.md`
