---
audience: automation-builder
type: specification
updated: 2026-05-04
status: draft
---

# `=*Combine` — Tree Assembly Operator

<!-- @c:reassemblers -->

The `=*Combine` operator atomically groups disjoint DataTrees into a single parent DataTree without requiring a manual `*Collect` loop. It acts as a structural constructor, nesting the provided inputs exactly one level deep under their respective branch keys.

## Syntax

```aljam3
[-] =*Combine
   (=) <Branch.{Name} << $tree1
   (=) <Branch.{OtherName} << $tree2
   (*) >> $combined
```

### Inputs `(=)`

`=*Combine` accepts a dynamic set of inputs using the `<Branch.` prefix. 

The parameter name immediately following the dot (`.`) is interpreted by the compiler as the **constant branch name** for that input in the resulting tree.

- `(=) <Branch.tree1 << $tree1` means `$tree1` will be inserted into the final combined tree at the Level 1 path `:tree1`.
- You can provide as many `<Branch.{Name}` inputs as needed.

### Outputs `(*)`

- `(*) >> $result` — Output pipeline assigning the assembled tree to the target variable. 
- You may also explicitly target a generic container format if needed via `(*) >Data >> $result`.

## Behavior

1. The operator takes all provided input variables.
2. It constructs a new root DataTree.
3. It creates a Level 1 branch for each input, using the explicit string literal specified in the `<Branch.{Name}` input declaration.
4. The provided tree is nested directly under that branch.

### Example

```aljam3
[ ] Given two independent DataTrees
[-] $users << $File"/data/users.json"
[-] $roles << $File"/data/roles.json"

[ ] Assemble them into a single payload
[-] =*Combine
   (=) <Branch.users << $users
   (=) <Branch.roles << $roles
   (*) >> $payload

[ ] $payload now has the structure:
[ ] ├── :users
[ ] │   └── (contents of $users)
[ ] └── :roles
[ ]     └── (contents of $roles)
```

## Related

- [[INDEX|Reassemble Operators (=*)]]
- [[../concepts/collections/reassemble|Reassemble Concepts]]
