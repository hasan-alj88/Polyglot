# Variable Lifecycle - v0.0.5

## State Machine

Variables in Polyglot follow a strict immutability model with controlled state transitions.

### States

1. **Declared** - Variable is declared with type annotation
2. **Pending** - Initial state, no value assigned
3. **Default** - Has default/tentative value (one more push allowed)
4. **Final** - Has final immutable value
5. **Faulted** - Operation failed, variable in error state
6. **Released** - Variable exited scope, memory freed

### State Transitions

```
Path 1: Direct Final
Declared → Pending → [final push << or >>] → Final → [Exit scope] → Released

Path 2: Default Then Final
Declared → Pending → [default push <~ or ~>] → Default → [final push << or >>] → Final → [Exit scope] → Released

Path 3: Default Push Failed
Declared → Pending → [default push <~ or ~> (failed)] → Faulted → [Exit scope] → Released

Path 4: Final Push Failed
Declared → Pending → [final push << or >> (failed)] → Faulted → [Exit scope] → Released

Path 5: Default Then Final Failed
Declared → Pending → [default push <~ or ~>] → Default → [final push << or >> (failed)] → Faulted → [Exit scope] → Released
```

## Push Operators

### Final Push: `<<` (pull) and `>>` (push)

Creates immutable binding. Variable cannot be changed after final push.

```polyglot
[r] $count:int << 42            %% Pull final value
[r] $name:string >> >output     %% Push final value to output
```

### Default Push: `<~` (pull) and `~>` (push)

Creates tentative binding. Variable can receive ONE more final push to override.

```polyglot
[r] $status:string <~ #Status.Unknown  %% Default value
[f] $condition ?= -True
   [r] $status << #Status.Success      %% Final push overrides default
```

## Rules

### Maximum Two Pushes

Each variable can receive at most TWO pushes:
1. **Optional**: One default push (`<~` or `~>`)
2. **Required**: One final push (`<<` or `>>`)

### No Reassignment

❌ **Invalid - multiple final pushes:**
```polyglot
[r] $value:int << 10
[r] $value << 20  %% ERROR: already Final!
```

✅ **Valid - default then final:**
```polyglot
[r] $value:int <~ 10   %% Default
[f] $condition
   [r] $value << 20    %% Final (overrides)
```

### Default Allows Override

Variables in Default state can receive one final push:

```polyglot
[r] $result:string <~ "default"

[f] $hasCustomValue ?= -True
   [r] $result << $customValue  %% Overrides default

%% If fork not taken, $result stays "default"
```

### Pack Operations

Pack operators `[*]` accumulate values across iterations. Output variable receives final value when iteration completes:

```polyglot
[p] ~ForEach.Array
 ~  <array << $items
 ~  >item >> $item
   [*] *Aggregate.Sum
    *  <inc << 1
    *  >sum >> $count  %% Accumulates across iterations

%% $count is Final after iteration completes
```

## Common Patterns

### Pattern 1: Direct Assignment
```polyglot
[r] $config:serial << #AppConfig
[r] $timestamp:datetime << |DT.Now""
```

### Pattern 2: Conditional Override
```polyglot
[r] $level:string <~ "INFO"

[f] $isDebugMode ?= -True
   [r] $level << "DEBUG"

[f] $isError ?= -True
   [r] $level << "ERROR"
```

### Pattern 3: Pipeline Output
```polyglot
[r] |DB.Query
 |  <query << $sql
 |  >results:array.serial >> $data  %% Final push from pipeline output
```

### Pattern 4: Aggregation
```polyglot
[p] ~ForEach.Array
 ~  <array << $customers
 ~  >item >> $customer
   [f] $customer.active ?= -True
      [*] *Aggregate.Sum
       *  <inc << 1
       *  >sum >> $activeCount
```

## Error Handling

### Faulted State

If any push operation fails, variable enters Faulted state:

```polyglot
[r] |FailingPipeline""  %% If fails, variable is Faulted
 |  >result >> $data

%% $data is Faulted if pipeline fails
```

### Faulted Variables

- Cannot be used in expressions
- Propagate errors through pipeline
- Released when scope exits

## Scope and Release

Variables are Released when:
- Pipeline exits
- Fork body exits
- Unpack iteration completes
- Wrapper context closes

```polyglot
[f] $condition
   [r] $temp:string << "value"
   %% $temp Released here when fork exits

[p] ~ForEach.Array
 ~  <array << $items
 ~  >item >> $item
   [r] $processed << |Process"{$item}"
   %% $processed Released each iteration

%% All variables Released when pipeline exits
```

## Comparison with Mutable Variables

### Traditional (Mutable)
```javascript
let status = "unknown";
if (condition) {
  status = "success";  // Reassignment
}
status = "done";  // Another reassignment
```

### Polyglot (Immutable)
```polyglot
[r] $status:string <~ "unknown"  %% Default

[f] $condition
   [r] $status << "success"  %% Final (overrides)

%% Cannot do: [r] $status << "done"  - already Final!
```

## Benefits

1. **Thread Safety** - Immutable values are inherently thread-safe
2. **Predictability** - Value cannot change unexpectedly
3. **Optimization** - Compiler can optimize knowing value won't change
4. **Debugging** - Easier to reason about variable state
5. **Error Tracking** - Faulted state makes errors explicit

---

**Version:** 0.0.5
**Last Updated:** 2026-01-01
**Status:** Core language specification
