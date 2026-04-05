---
audience: developer
rule: "2.8"
code: PGE02008
name: Access After Release
severity: error
---

### Rule 2.8 — Access After Release
`PGE02008`

**Statement:** A variable in the Released state must not be read, pushed, or referenced. Released is a distinct lifecycle state indicating that a variable previously held a value but its scope has closed. Any reference to a Released variable is a compile error.
**Rationale:** Released variables are no longer accessible — their scope has ended. Unlike Declared variables (never had a value), Released variables had a value but it is no longer valid to access. Use-after-release is always a structural error indicating code placed outside its proper scope.
**Detection:** The compiler tracks variable scope boundaries. When a scope-closing event occurs (collector boundary, `[/]` cleanup exit, expand scope close), all variables local to that scope transition to Released. Any subsequent reference to those variables triggers PGE02008.

**See also:** PGE02001 (lifecycle stages — defines Released state), PGE02002 (declared state unreadable — analogous rule for uninitialized variables)

**VALID:**
```polyglot
[ ] ✓ access within expand scope — before collector closes scope
[p] ~ForEach.Array
   [~] <Array << $files
   [~] >item >> $file

   [r] =File.Text.Read
      [=] <path << $file
      [=] >content >> $text

   [ ] ✓ $text is still in scope
   [r] =Log
      [=] <msg << $text

   [r] *Into.Array
      [*] <item << $text
      [*] >Array >> $results
```

**INVALID:**
```polyglot
[ ] ✗ PGE02008 — access after collector boundary
[p] ~ForEach.Array
   [~] <Array << $files
   [~] >item >> $file

   [r] =File.Text.Read
      [=] <path << $file
      [=] >content >> $text

   [r] *Into.Array
      [*] <item << $text
      [*] >Array >> $results

[ ] $text is Released — collector closed its scope
[r] =Log
   [=] <msg << $text                        [ ] ✗ PGE02008 — $text is Released
```

```polyglot
[ ] ✗ PGE02008 — access after cleanup exits
[\] setup
   [r] $conn#Connection << =DB.Connect
      [=] <url << $dbUrl

[ ] execution body uses $conn...

[/] cleanup
   [r] =DB.Disconnect
      [=] <conn << $conn

[ ] $conn is Released after [/] cleanup
[r] =Query
   [=] <conn << $conn                       [ ] ✗ PGE02008 — $conn is Released
```

**Open point:** None.

### See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — defines Released state and references PGE02008
