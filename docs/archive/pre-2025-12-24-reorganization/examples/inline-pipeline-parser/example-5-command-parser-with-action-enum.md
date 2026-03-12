# Example 5: Command Parser with Action Enum

**Use Case:** Parse CLI-style commands in `"action:target"` format

## Complete Implementation

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

## Usage

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
