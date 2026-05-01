# Aljam3 Compiler Rules Tracker

This tracker documents the implementation status of all compiler rules using the new granular hex-based format.

| Rule ID | Old ID | Rule Name | Status |
|---|---|---|---|
| `JM3Ex00100001` | PGE01003 | One Package Declaration Per File | ✅ |
| `JM3Ex00110001` | PGE01001 | Pipeline Section Misordering | ✅ |
| `JM3Ex00110002` | PGE01002 | IO Before Trigger | ✅ |
| `JM3Ex00110003` | PGE01005 | Missing Pipeline Trigger | ✅ |
| `JM3Ex00110004` | PGE01010 | Pipeline IO Name Mismatch | ✅ |
| `JM3Ex00110005` | PGE01011 | Duplicate IO Parameter Name | ✅ |
| `JM3Ex00110006` | PGE01015 | Duplicate Metadata Field | ✅ |
| `JM3Ex00110007` | PGE01016 | Unmarked Execution Line | ✅ |
| `JM3Ex00110008` | PGE01017 | Wrong Block Element Marker | ✅ |
| `JM3Ex00110009` | PGE01018 | Tautological or Contradictory Trigger Condition | ✅ |
| `JM3Ex0011000A` | PGE01020 | Effectless Execution Expression | ✅ |
| `JM3Ex0011000B` | PGE01021 | Empty Data Definition | ✅ |
| `JM3Ex0011000C` | PGE01022 | Empty Error Namespace | ✅ |
| `JM3Ex0011000D` | PGE01023 | "Retired — Parameterless Macro" | ✅ |
| `JM3Ex0011000E` | PGE01024 | Incompatible Operation Marker | ✅ |
| `JM3Ex0011000F` | PGE01026 | Orphan Continuation Line | ✅ |
| `JM3Ex00110010` | PGE01027 | Empty Foreign Code Block | ✅ |
| `JM3Ex00110011` | PGE01028 | Native/Derived Block Mutual Exclusion | ✅ |
| `JM3Ex00110012` | PGE01029 | Invalid Marker for Definition Type | ✅ |
| `JM3Ex00110013` | PGE01031 | Forbidden Element in Definition | ✅ |
| `JM3Ex00110014` | PGE01032 | Missing Trigger Boolean Output | ✅ |
| `JM3Ex00110015` | PGE01033 | Unbound Script Variable | ✅ |
| `JM3Ex00110016` | PGE01034 | Unbound Script Output | ✅ |
| `JM3Ex00110017` | PGE01035 | Unbound Function Argument | ✅ |
| `JM3Ex00110018` | PGE01036 | Unbound Function Kwarg | ✅ |
| `JM3Ex00110019` | PGE01037 | Bind Schema Mismatch | ✅ |
| `JM3Ex0011001A` | PGE01038 | Code Source Conflict | ✅ |
| `JM3Ex0011001B` | PGE01039 | CLI Non-String Argument | ✅ |
| `JM3Ex0011001C` | PGE01040 | Orphan Parallel Marker | ✅ |
| `JM3Ex0011001D` | PGE01041 | Unknown Rule | ✅ |
| `JM3Ex0011001E` | PGE01042 | Unknown Rule | ✅ |
| `JM3Ex0011001F` | PGE01043 | Unknown Rule | ✅ |
| `JM3Ex00110020` | PGE01044 | Unknown Rule | ✅ |
| `JM3Ex00110021` | PGE01045 | Unknown Rule | ✅ |
| `JM3Ex00110022` | PGE01046 | Unknown Rule | ✅ |
| `JM3Ex00110023` | PGE01047 | Unknown Rule | ✅ |
| `JM3Ex00110024` | PGE01049 | Unknown Rule | ✅ |
| `JM3Ex00110025` | PGE01050 | Unknown Rule | ✅ |
| `JM3Ex00110026` | PGE01051 | Unknown Rule | ✅ |
| `JM3Ex00110027` | PGE01052 | Unknown Rule | ⛔ |
| `JM3Ex00110028` | PGE01053 | Unknown Rule | ✅ |
| `JM3Ex00110029` | PGE01054 | Unknown Rule | ✅ |
| `JM3Ex0011002A` | PGE01055 | Unknown Rule | ✅ |
| `JM3Ex0011002B` | PGE01056 | Unknown Rule | ✅ |
| `JM3Ex0011002C` | PGE01060 | Unknown Rule | ✅ |
| `JM3Ex0011002D` | PGE01061 | Unknown Rule | ✅ |
| `JM3Ex0011002E` | PGE01062 | Unknown Rule | ✅ |
| `JM3Ex0011002F` | PGE01063 | Unknown Rule | ✅ |
| `JM3Ex00110030` | PGE01064 | Unknown Rule | ✅ |
| `JM3Ex00110031` | PGE01065 | Unknown Rule | ✅ |
| `JM3Ex00110032` | PGE01066 | Unknown Rule | ✅ |
| `JM3Ex00110033` | PGE01067 | Unknown Rule | ✅ |
| `JM3Ex00110034` | PGE01068 | Unknown Rule | ✅ |
| `JM3Ex00110035` | PGE01069 | Unknown Rule | ✅ |
| `JM3Ex00110036` | PGE01070 | Unknown Rule | ✅ |
| `JM3Wx00110037` | PGW01001 | Empty Execution Body | ✅ |
| `JM3Wx00110038` | PGW01003 | No Definitions in File | ✅ |
| `JM3Wx00110039` | PGW01004 | Orphaned Foreign Code | ✅ |
| `JM3Ex00120001` | PGE09001 | Undefined Import Alias | ✅ |
| `JM3Ex00120002` | PGE09002 | Circular Package Dependency | ✅ |
| `JM3Ex00120003` | PGE09004 | Unresolved Import Pipeline Reference | ✅ |
| `JM3Ex00120004` | PGE09005 | Multi-File Version Mismatch | ✅ |
| `JM3Ex00120005` | PGE09006 | Multi-File Package Name Mismatch | ✅ |
| `JM3Ex00120006` | PGE09007 | Duplicate Definition | ✅ |
| `JM3Ex00120007` | PGE09008 | Multi-File Reference Not Found | ✅ |
| `JM3Ex00120008` | PGE09009 | Multi-File Self-Reference | ✅ |
| `JM3Ex00120009` | PGE09010 | Asymmetric Multi-File Reference | ✅ |
| `JM3Ex0012000A` | PGE09011 | Duplicate Import Alias | ✅ |
| `JM3Ex0012000B` | PGE09012 | Import Alias Shadows Standard Library | ✅ |
| `JM3Ex0012000C` | PGE09013 | Circular Pipeline Call | ✅ |
| `JM3Wx0012000D` | PGW09001 | Deprecated Pipeline Reference | ✅ |
| `JM3Wx0012000E` | PGW09002 | Unused Import | ✅ |
| `JM3Ex00200001` | PGE05001 | Sibling Separator Homogeneity | ✅ |
| `JM3Ex00200002` | PGE05002 | Sibling Kind Homogeneity | ✅ |
| `JM3Ex00200003` | PGE05003 | Duplicate Data Field Name | ✅ |
| `JM3Ex00200004` | PGE05004 | Recursive Data Definition | ✅ |
| `JM3Ex00300001` | PGE12001 | Undefined Metadata Field Access | ✅ |
| `JM3Ex00300002` | PGE12002 | Duplicate Alias | ✅ |
| `JM3Ex00300003` | PGE12004 | Empty Metadata Alias | ✅ |
| `JM3Ex00310001` | PGE02006 | Live Metadata Fields Are Pull-Only | ✅ |
| `JM3Wx00310002` | PGW02001 | Default Pull Across State Change | ✅ |
| `JM3Ex00400001` | PGE01006 | Missing Pipeline Queue | ✅ |
| `JM3Ex00400002` | PGE01012 | Queue Definition Must Use #Queue: Prefix | ✅ |
| `JM3Ex00400003` | PGE01013 | Queue Control Contradicts Queue Default | ⛔ |
| `JM3Ex00400004` | PGE01014 | Unresolved Queue Reference | ✅ |
| `JM3Ex00500001` | PGE02001 | Lifecycle Stages | ⛔ |
| `JM3Ex00500002` | PGE02002 | Declared State Is Unreadable | ✅ |
| `JM3Ex00500003` | PGE02003 | Final Is Push-Once | ✅ |
| `JM3Ex00500004` | PGE02004 | Unknown Rule | ✅ |
| `JM3Ex00500005` | PGE02005 | Failed Must Resolve | ⛔ |
| `JM3Ex00500006` | PGE02008 | Access After Release | ✅ |
| `JM3Ex00500007` | PGE02009 | Unreachable Code | ✅ |
| `JM3Ex00500008` | PGE02010 | Discard Default Assignment | ✅ |
| `JM3Ex00500009` | PGE02012 | Duplicate Operation Label | ✅ |
| `JM3Ex0050000A` | PGE02013 | Write To Label Accessor | ✅ |
| `JM3Ex0050000B` | PGE02014 | Label Access Before Completion | ✅ |
| `JM3Ex0050000C` | PGE02015 | Unused Background Label | ✅ |
| `JM3Ex0050000D` | PGE02016 | Unknown Rule | ✅ |
| `JM3Ex0050000E` | PGE02017 | Unknown Rule | ✅ |
| `JM3Wx0050000F` | PGW02002 | Unused Variable | ✅ |
| `JM3Wx00500010` | PGW02003 | Unpushed Output Port | ✅ |
| `JM3Wx00500011` | PGW02004 | Pipeline Terminates on Error | ✅ |
| `JM3Wx00500012` | PGW02005 | Unreachable Code | ⛔ |
| `JM3Ex00510001` | PGE02011 | Data Load Schema Mismatch | ✅ |
| `JM3Ex00520001` | PGE14001 | Ambiguous Constructor Overload | ✅ |
| `JM3Ex00520002` | PGE14002 | Duplicate Constructor Keyword | ✅ |
| `JM3Ex00520003` | PGE14003 | Missing Capture Regex | ✅ |
| `JM3Ex00520004` | PGE14004 | Structural Integrity Violation | ✅ |
| `JM3Ex00520005` | PGE14005 | Target Type Mismatch | ✅ |
| `JM3Ex00520006` | PGE14006 | Failable Pipeline In Constructor | ✅ |
| `JM3Ex00520007` | PGE14007 | Incomplete Field Mapping | ✅ |
| `JM3Ex00520008` | PGE14010 | No Constructor Overload Match | ✅ |
| `JM3Ex00520009` | PGE14011 | Non-Literal Interpolation | ✅ |
| `JM3Ex0052000A` | PGE14012 | Undefined Constructor | ✅ |
| `JM3Ex0052000B` | PGE14013 | Interpolation Source Not Final | ✅ |
| `JM3Ex00530001` | PGE12003 | Undefined Inline Template | ✅ |
| `JM3Ex00530002` | PGE12005 | Inline Format Mismatch | ✅ |
| `JM3Ex00530003` | PGE12006 | Unresolved Template Placeholder | ✅ |
| `JM3Ex00530004` | PGE12007 | Required Input Not In Template | ✅ |
| `JM3Ex00530005` | PGE12008 | Duplicate Template Placeholder | ✅ |
| `JM3Ex00530006` | PGE12009 | Template Type Coercion Failure | ✅ |
| `JM3Ex00530007` | PGE12010 | Optional Placeholder Without Default | ✅ |
| `JM3Wx00530008` | PGW12001 | Template With No Placeholders | ✅ |
| `JM3Wx00530009` | PGW12002 | Optional Placeholder Never Provided | ✅ |
| `JM3Ex00610001` | PGE06004 | Numeric Range Overlap | ✅ |
| `JM3Ex00610002` | PGE06005 | Compound Condition Overlap | ✅ |
| `JM3Ex00610003` | PGE06009 | Conditional Missing Comparison Operator | ✅ |
| `JM3Ex00610004` | PGE06010 | Empty Conditional Scope | ✅ |
| `JM3Ex00610005` | PGE06011 | Duplicate Wildcard Catch-All | ✅ |
| `JM3Ex00610006` | PGE06012 | Unreachable Branch After Wildcard | ✅ |
| `JM3Ex00610007` | PGE06013 | Tautological or Contradictory Branch Condition | ✅ |
| `JM3Ex00610008` | PGE06014 | Wildcard-Only Match | ✅ |
| `JM3Ex00610009` | PGE06015 | Unknown Rule | ✅ |
| `JM3Ex0061000A` | PGE06016 | Unknown Rule | ✅ |
| `JM3Ex00620001` | PGE06001 | Conditional Must Be Exhaustive | ⛔ |
| `JM3Ex00620002` | PGE06002 | Enum Exhaustiveness | ✅ |
| `JM3Ex00620003` | PGE06003 | Numeric Range Not Exhaustive | ✅ |
| `JM3Ex00620004` | PGE06006 | String Exhaustiveness | ✅ |
| `JM3Ex00620005` | PGE06007 | Flexible Field Exhaustiveness | ✅ |
| `JM3Ex00620006` | PGE06008 | Compound Condition Exhaustiveness | ✅ |
| `JM3Ex00630001` | PGE07001 | Error Block Scoping | ✅ |
| `JM3Ex00630002` | PGE07002 | Chain Error Scoping | ✅ |
| `JM3Ex00630003` | PGE07003 | Duplicate Fallback Assignment | ✅ |
| `JM3Ex00630004` | PGE07004 | Duplicate Error Handler | ✅ |
| `JM3Ex00630005` | PGE07005 | Undeclared Error Raise | ✅ |
| `JM3Ex00630006` | PGE07006 | Unused Error Declaration | ✅ |
| `JM3Ex00630007` | PGE07007 | Error Handling Must Be Exhaustive | ✅ |
| `JM3Ex00630008` | PGE07008 | Fallback on Non-Failable Source | ✅ |
| `JM3Ex00630009` | PGE07009 | Unterminated Fallback Chain | ✅ |
| `JM3Wx0063000A` | PGW07001 | Error Handler on Non-Failable Call | ✅ |
| `JM3Wx0063000B` | PGW07002 | Caller Overrides Pipeline Fallback | ✅ |
| `JM3Wx0063000C` | PGW07003 | Missing Fallback Message | ✅ |
| `JM3Wx0063000D` | PGW07004 | Fallback on Non-Failable IO | ✅ |
| `JM3Wx0063000E` | PGW07010 | Suppress on Consumed Output | ✅ |
| `JM3Ex00700001` | PGE03001 | No Push Across Parallel Boundaries | ✅ |
| `JM3Ex00700002` | PGE03004 | Section-Boundary Pairing | ✅ |
| `JM3Ex00700003` | PGE03012 | Parallel Label Isolation | ✅ |
| `JM3Ex00700004` | PGE03018 | Missing Incoming DataFrame | ✅ |
| `JM3Ex00700005` | PGE03019 | Arrival Index Out of Bounds | ✅ |
| `JM3Ex00700006` | PGE03020 | Statements Outside Trigger Blocks | ✅ |
| `JM3Ex00700007` | PGE03023 | Overflow Strategy Missing Storage Error | ✅ |
| `JM3Ex00700008` | PGE03024 | Release With No Remaining Claims | ✅ |
| `JM3Ex00700009` | PGE03025 | Not All Jobs Released | ✅ |
| `JM3Wx0070000A` | PGW03001 | "[b] Called Pipeline Has Discarded Outputs" | ✅ |
| `JM3Wx0070000B` | PGW03002 | Error Handler on Fire-and-Forget | ✅ |
| `JM3Ex00710001` | PGE03007 | Expand Operator Input Mismatch | ✅ |
| `JM3Ex00710002` | PGE03011 | Orphaned Expand IO Marker | ✅ |
| `JM3Ex00720001` | PGE03002 | Parallel Output Must Be Collected | ✅ |
| `JM3Ex00720002` | PGE03003 | Variable Isolation Until Collection | ✅ |
| `JM3Ex00720003` | PGE03005 | "[b] Has No Collectible Output" | ✅ |
| `JM3Ex00720004` | PGE03006 | Race Collector Type Homogeneity | ✅ |
| `JM3Ex00720005` | PGE03008 | Collect Operator IO Mismatch | ✅ |
| `JM3Ex00720006` | PGE03009 | Nested Expand Without Collect | ✅ |
| `JM3Ex00720007` | PGE03010 | Collector Without Expand | ✅ |
| `JM3Ex00720008` | PGE03013 | Collector Metadata Required | ✅ |
| `JM3Ex00720009` | PGE03014 | Expand-Scoped Collector Outside ForEach | ✅ |
| `JM3Ex0072000A` | PGE03015 | Parallel-Scoped Collector Inside ForEach | ✅ |
| `JM3Ex0072000B` | PGE03016 | Collector IO Mismatch | ✅ |
| `JM3Ex0072000C` | PGE03017 | Arrival Operations Outside Collector Context | ✅ |
| `JM3Ex0072000D` | PGE03021 | Parallel Execution Inside Collector | ✅ |
| `JM3Ex0072000E` | PGE03022 | External Trigger Source in Collector | ✅ |
| `JM3Ex00800001` | PGE04001 | Type Mismatch | ⛔ |
| `JM3Ex00800002` | PGE04002 | Schema Mismatch | ✅ |
| `JM3Ex00800003` | PGE04003 | Leaf-Only Assignment | ✅ |
| `JM3Ex00800004` | PGE04004 | Fixed-Schema Keys Are Compile-Time Only | ✅ |
| `JM3Ex00800005` | PGE04005 | Undefined Interpolation Variable | ✅ |
| `JM3Ex00800006` | PGE04006 | Undefined Variable Reference | ⛔ |
| `JM3Ex00800007` | PGE04009 | Unhandled Serial→Struct Conversion | ✅ |
| `JM3Ex00800008` | PGE04010 | Invalid Arithmetic Operator | ✅ |
| `JM3Ex00800009` | PGE04012 | Division by Literal Zero | ✅ |
| `JM3Ex0080000A` | PGE04014 | Invalid Range Bounds | ✅ |
| `JM3Ex0080000B` | PGE04015 | Conditional Type-Operator Mismatch | ✅ |
| `JM3Ex0080000C` | PGE04016 | Invalid Pipeline Input Literal | ✅ |
| `JM3Ex0080000D` | PGE04024 | Non-Value Comparison | ✅ |
| `JM3Ex0080000E` | PGE04028 | Invalid Epoch Value | ✅ |
| `JM3Ex0080000F` | PGE04029 | Unknown Rule | ✅ |
| `JM3Ex00800010` | PGE04030 | Unknown Rule | ✅ |
| `JM3Ex00800011` | PGE04031 | Unknown Rule | ✅ |
| `JM3Ex00800012` | PGE04032 | Unknown Rule | ✅ |
| `JM3Ex00800013` | PGE04033 | Unknown Rule | ✅ |
| `JM3Wx00800014` | PGW04002 | Leading Zeros in Literal | ✅ |
| `JM3Ex00810001` | PGE04011 | Negative Array Index Literal | ✅ |
| `JM3Ex00810002` | PGE04013 | Nested Array Type | ✅ |
| `JM3Ex00810003` | PGE04017 | Array Dimension Access Mismatch | ✅ |
| `JM3Ex00810004` | PGE04025 | Untyped Array | ✅ |
| `JM3Ex00820001` | PGE04007 | Invalid Path String | ✅ |
| `JM3Ex00820002` | PGE04008 | Missing Path Platform Subfield | ✅ |
| `JM3Ex00820003` | PGE04026 | Invalid IANA Timezone | ✅ |
| `JM3Ex00820004` | PGE04027 | Missing Required DateTime Subfield | ✅ |
| `JM3Wx00820005` | PGW04001 | Single-Platform Path | ✅ |
| `JM3Ex00900001` | PGE08001 | Auto-Wire Type Mismatch | ✅ |
| `JM3Ex00900002` | PGE08002 | Auto-Wire Ambiguous Type | ✅ |
| `JM3Ex00900003` | PGE08003 | Auto-Wire Port Count Mismatch | ✅ |
| `JM3Ex00900004` | PGE08004 | Ambiguous Step Reference | ✅ |
| `JM3Ex00900005` | PGE08005 | Unresolved Step Reference | ✅ |
| `JM3Ex00900006` | PGE08006 | Non-Pipeline Step in Chain | ✅ |
| `JM3Ex00900007` | PGE08007 | Invalid Assignment Target | ⛔ |
| `JM3Ex00900008` | PGE08008 | Missing Required Input at Call Site | ✅ |
| `JM3Ex00900009` | PGE08009 | Uncaptured Required Output at Call Site | ✅ |
| `JM3Ex0090000A` | PGE08010 | IO Direction Mismatch | ✅ |
| `JM3Ex0090000B` | PGE08013 | Nested Inline Data | ✅ |
| `JM3Wx0090000C` | PGW08001 | Auto-Wire Succeeded | ✅ |
| `JM3Wx0090000D` | PGW08002 | Unaddressed Input With Default | ✅ |
| `JM3Wx0090000E` | PGW08003 | Uncaptured Output With Default/Fallback | ✅ |
| `JM3Ex00910001` | PGE01004 | Wrapper Structural Constraints | ✅ |
| `JM3Ex00910002` | PGE01008 | Wrapper Must Reference Wrapper Definition | ✅ |
| `JM3Ex00910003` | PGE01009 | Wrapper IO Mismatch | ✅ |
| `JM3Ex00910004` | PGE01025 | Discard in Wrapper IO | ✅ |
| `JM3Ex00910005` | PGE01030 | Missing Pipeline Wrapper | ✅ |
| `JM3Ex00920001` | PGE08011 | Self-Assignment | ✅ |
| `JM3Ex00920002` | PGE08012 | Self-Chain Requires Numeric Indexing | ✅ |
| `JM3Ex00A00001` | PGE10003 | Unknown Permission Category | ✅ |
| `JM3Ex00A00002` | PGE10004 | Undeclared Permission | ✅ |
| `JM3Ex00A00003` | PGE10005 | Invalid Permission Block Marker | ✅ |
| `JM3Ex00A00004` | PGE10006 | Duplicate Permission | ✅ |
| `JM3Ex00A00005` | PGE10008 | Parallel Write Permission Exclusion | ✅ |
| `JM3Ex00A00006` | PGE10009 | Unresolved Permission Template | ✅ |
| `JM3Ex00A00007` | PGE10010 | Permission Resource Not Found | ✅ |
| `JM3Ex00A00008` | PGE10011 | Shell Without Capability | ✅ |
| `JM3Wx00A00009` | PGW10001 | Unused Permission | ✅ |
| `JM3Ex00A10001` | PGE10007 | Chain Step Label Overflow | ✅ |
| `JM3Ex00A10002` | PGE10012 | Code File Outside Scope | ✅ |
| `JM3Wx00A10003` | PGW10003 | Bind Mode Opacity | ✅ |
| `JM3Wx00A10004` | PGW10006 | Shell Variable Expansion | ✅ |
| `JM3Ex00A20001` | PGE10013 | Foreign Resource Outside Scope | ✅ |
| `JM3Ex00A20002` | PGE10014 | AST-Invisible Foreign Code | ✅ |
| `JM3Ex00A20003` | PGE10015 | Opaque Binary Without Sandbox Acknowledgment | ✅ |
| `JM3Ex00A20004` | PGE10016 | Missing Mandatory Metadata for Sandbox-Only | ✅ |
| `JM3Wx00A20005` | PGW10002 | Unverifiable Foreign IO | ✅ |
| `JM3Wx00A20006` | PGW10005 | Unrecognized Foreign Call | ✅ |
| `JM3Wx00A20007` | PGW10007 | Sandbox-Only Enforcement Active | ✅ |