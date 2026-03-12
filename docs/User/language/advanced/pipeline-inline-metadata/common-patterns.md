# Common Patterns

## Pattern 1: Simple Key-Value Parser

```polyglot
{|} |ParseKeyValue
[t] |T.Call
[<] <formatted_string:pg.string
[>] >key :pg.string
[>] >value :pg.string

// Parse "key:value" format
{x}
```

## Pattern 2: Multi-Field Parser

```polyglot
{|} |ParseURL
[t] |T.Call
[<] <formatted_string:pg.string
[>] >protocol :pg.string
[>] >host :pg.string
[>] >port :pg.int
[>] >path :pg.string

// Parse "protocol://host:port/path" format
{x}
```

## Pattern 3: Enum-Based Parser

```polyglot
{|} |ParseCommand
[t] |T.Call
[<] <formatted_string:pg.string
[>] >action :#ActionType
[>] >target :pg.string

// Parse "action:target" into enum + string
{x}
```

---
