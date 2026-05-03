# Aljam3 Compiler Rules Tracker

This tracker documents the implementation status of all compiler rules using the new granular hex-based format.

| Rule ID | Old ID | Rule Name | Algorithm | Status |
|---|---|---|---|---|
| `JM3Ex00120001` | PGE01003 | One Package Declaration Per File | - | - | ✅ |

| `JM3Ex001F0001` | PGE01001 | Pipeline Section Misordering | - | - | ✅ |

| `JM3Ex001F0002` | PGE01002 | IO Before Trigger | - | - | ✅ |

| `JM3Ex00110001` | PGE01005 | Missing Pipeline Trigger | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Ex001F0003` | PGE01010 | Pipeline IO Name Mismatch | - | - | ✅ |

| `JM3Ex001F0004` | PGE01011 | Duplicate IO Parameter Name | - | - | ✅ |

| `JM3Ex00220001` | PGE01015 | Duplicate Metadata Field | - | - | ✅ |

| `JM3Ex00110002` | PGE01016 | Unmarked Execution Line | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Ex00110003` | PGE01017 | Wrong Block Element Marker | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Ex00620001` | PGE01018 | Tautological or Contradictory Trigger Condition | - | - | ✅ |

| `JM3Ex001F0005` | PGE01020 | Effectless Execution Expression | - | - | ✅ |

| `JM3Ex00110004` | PGE01021 | Empty Data Definition | - | - | ✅ |

| `JM3Ex00610001` | PGE01022 | Empty Error Namespace | - | - | ✅ |

| `JM3Ex001F0006` | PGE01023 | "Retired — Parameterless Macro" | - | - | ✅ |

| `JM3Ex00110005` | PGE01024 | Incompatible Operation Marker | - | - | ✅ |

| `JM3Ex00710001` | PGE01026 | Orphan Continuation Line | - | - | ✅ |

| `JM3Ex00A10001` | PGE01027 | Empty Foreign Code Block | - | - | ✅ |

| `JM3Ex001F0007` | PGE01028 | Native/Derived Block Mutual Exclusion | - | - | ✅ |

| `JM3Ex00810001` | PGE01029 | Invalid Marker for Definition Type | - | - | ✅ |

| `JM3Ex001F0008` | PGE01031 | Forbidden Element in Definition | - | - | ✅ |

| `JM3Ex00110006` | PGE01032 | Missing Trigger Boolean Output | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Ex00810002` | PGE01033 | Unbound Script Variable | - | - | ✅ |

| `JM3Ex00110007` | PGE01034 | Unbound Script Output | - | - | ✅ |

| `JM3Ex00110008` | PGE01035 | Unbound Function Argument | - | - | ✅ |

| `JM3Ex00110009` | PGE01036 | Unbound Function Kwarg | - | - | ✅ |

| `JM3Ex00820001` | PGE01037 | Bind Schema Mismatch | - | - | ✅ |

| `JM3Ex001F0009` | PGE01038 | Code Source Conflict | - | - | ✅ |

| `JM3Ex001F000A` | PGE01039 | CLI Non-String Argument | - | - | ✅ |

| `JM3Ex00710002` | PGE01040 | Orphan Parallel Marker | - | - | ✅ |

| `JM3Ex001F000B` | PGE01041 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F000C` | PGE01042 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F000D` | PGE01043 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F000E` | PGE01044 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F000F` | PGE01045 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0010` | PGE01046 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0011` | PGE01047 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0012` | PGE01049 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0013` | PGE01050 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0014` | PGE01051 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0015` | PGE01052 | Unknown Rule | - | - | ⛔ |

| `JM3Ex001F0016` | PGE01053 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0017` | PGE01054 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0018` | PGE01055 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0019` | PGE01056 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F001A` | PGE01060 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F001B` | PGE01061 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F001C` | PGE01062 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F001D` | PGE01063 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F001E` | PGE01064 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F001F` | PGE01065 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0020` | PGE01066 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0021` | PGE01067 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0022` | PGE01068 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0023` | PGE01069 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0024` | PGE01070 | Unknown Rule | - | - | ✅ |

| `JM3Wx0011000A` | PGW01001 | Empty Execution Body | - | - | ✅ |

| `JM3Wx00120002` | PGW01003 | No Definitions in File | - | - | ✅ |

| `JM3Wx00A10002` | PGW01004 | Orphaned Foreign Code | - | - | ✅ |

| `JM3Ex0011000B` | PGE09001 | Undefined Import Alias | - | - | ✅ |

| `JM3Ex00120003` | PGE09002 | Circular Package Dependency | - | - | ✅ |

| `JM3Ex0011000C` | PGE09004 | Unresolved Import Pipeline Reference | - | - | ✅ |

| `JM3Ex00120004` | PGE09005 | Multi-File Version Mismatch | - | - | ✅ |

| `JM3Ex00120005` | PGE09006 | Multi-File Package Name Mismatch | - | - | ✅ |

| `JM3Ex001F0025` | PGE09007 | Duplicate Definition | - | - | ✅ |

| `JM3Ex00120006` | PGE09008 | Multi-File Reference Not Found | - | - | ✅ |

| `JM3Ex00120007` | PGE09009 | Multi-File Self-Reference | - | - | ✅ |

| `JM3Ex00120008` | PGE09010 | Asymmetric Multi-File Reference | - | - | ✅ |

| `JM3Ex00120009` | PGE09011 | Duplicate Import Alias | - | - | ✅ |

| `JM3Ex0012000A` | PGE09012 | Import Alias Shadows Standard Library | - | - | ✅ |

| `JM3Ex0012000B` | PGE09013 | Circular Pipeline Call | - | - | ✅ |

| `JM3Wx001F0026` | PGW09001 | Deprecated Pipeline Reference | - | - | ✅ |

| `JM3Wx0012000C` | PGW09002 | Unused Import | - | - | ✅ |

| `JM3Ex00220002` | PGE05001 | Sibling Separator Homogeneity | - | - | ✅ |

| `JM3Ex00220003` | PGE05002 | Sibling Kind Homogeneity | - | - | ✅ |

| `JM3Ex00220004` | PGE05003 | Duplicate Data Field Name | - | - | ✅ |

| `JM3Ex00220005` | PGE05004 | Recursive Data Definition | - | - | ✅ |

| `JM3Ex00210001` | PGE12001 | Undefined Metadata Field Access | - | - | ✅ |

| `JM3Ex00420001` | PGE12002 | Duplicate Alias | - | - | ✅ |

| `JM3Ex00410001` | PGE12004 | Empty Metadata Alias | - | - | ✅ |

| `JM3Ex00220006` | PGE02006 | Live Metadata Fields Are Pull-Only | - | - | ✅ |

| `JM3Wx00520001` | PGW02001 | Default Pull Across State Change | - | - | ✅ |

| `JM3Ex00310001` | PGE01006 | Missing Pipeline Queue | - | - | ✅ |

| `JM3Ex00320001` | PGE01012 | Queue Definition Must Use #Queue: Prefix | - | - | ✅ |

| `JM3Ex00320002` | PGE01013 | Queue Control Contradicts Queue Default | - | - | ⛔ |

| `JM3Ex00310002` | PGE01014 | Unresolved Queue Reference | - | - | ✅ |

| `JM3Ex00520002` | PGE02001 | Lifecycle Stages | - | - | ⛔ |

| `JM3Ex00520003` | PGE02002 | Declared State Is Unreadable | - | - | ✅ |

| `JM3Ex00520004` | PGE02003 | Final Is Push-Once | - | - | ✅ |

| `JM3Ex001F0027` | PGE02004 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0028` | PGE02005 | Failed Must Resolve | - | - | ⛔ |

| `JM3Ex00520005` | PGE02008 | Access After Release | - | - | ✅ |

| `JM3Ex001F0029` | PGE02009 | Unreachable Code | - | - | ✅ |

| `JM3Ex001F002A` | PGE02010 | Discard Default Assignment | - | - | ✅ |

| `JM3Ex001F002B` | PGE02012 | Duplicate Operation Label | - | - | ✅ |

| `JM3Ex001F002C` | PGE02013 | Write To Label Accessor | - | - | ✅ |

| `JM3Ex001F002D` | PGE02014 | Label Access Before Completion | - | - | ✅ |

| `JM3Ex00750001` | PGE02015 | Unused Background Label | - | - | ✅ |

| `JM3Ex001F002E` | PGE02016 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F002F` | PGE02017 | Unknown Rule | - | - | ✅ |

| `JM3Wx00820002` | PGW02002 | Unused Variable | - | - | ✅ |

| `JM3Wx001F0030` | PGW02003 | Unpushed Output Port | - | - | ✅ |

| `JM3Wx00640001` | PGW02004 | Pipeline Terminates on Error | - | - | ✅ |

| `JM3Wx001F0031` | PGW02005 | Unreachable Code | - | - | ⛔ |

| `JM3Ex00230001` | PGE02011 | Data Load Schema Mismatch | - | - | ✅ |

| `JM3Ex00530001` | PGE14001 | Ambiguous Constructor Overload | - | - | ✅ |

| `JM3Ex00530002` | PGE14002 | Duplicate Constructor Keyword | - | - | ✅ |

| `JM3Ex0011000D` | PGE14003 | Missing Capture Regex | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Ex001F0032` | PGE14004 | Structural Integrity Violation | - | - | ✅ |

| `JM3Ex00820003` | PGE14005 | Target Type Mismatch | - | - | ✅ |

| `JM3Ex00530003` | PGE14006 | Failable Pipeline In Constructor | - | - | ✅ |

| `JM3Ex001F0033` | PGE14007 | Incomplete Field Mapping | - | - | ✅ |

| `JM3Ex00530004` | PGE14010 | No Constructor Overload Match | - | - | ✅ |

| `JM3Ex001F0034` | PGE14011 | Non-Literal Interpolation | - | - | ✅ |

| `JM3Ex00510001` | PGE14012 | Undefined Constructor | - | - | ✅ |

| `JM3Ex001F0035` | PGE14013 | Interpolation Source Not Final | - | - | ✅ |

| `JM3Ex00510002` | PGE12003 | Undefined Inline Template | - | - | ✅ |

| `JM3Ex00540001` | PGE12005 | Inline Format Mismatch | - | - | ✅ |

| `JM3Ex00510003` | PGE12006 | Unresolved Template Placeholder | - | - | ✅ |

| `JM3Ex00540002` | PGE12007 | Required Input Not In Template | - | - | ✅ |

| `JM3Ex00540003` | PGE12008 | Duplicate Template Placeholder | - | - | ✅ |

| `JM3Ex00540004` | PGE12009 | Template Type Coercion Failure | - | - | ✅ |

| `JM3Ex00540005` | PGE12010 | Optional Placeholder Without Default | - | - | ✅ |

| `JM3Wx00540006` | PGW12001 | Template With No Placeholders | - | - | ✅ |

| `JM3Wx00540007` | PGW12002 | Optional Placeholder Never Provided | - | - | ✅ |

| `JM3Ex00620002` | PGE06004 | Numeric Range Overlap | - | - | ✅ |

| `JM3Ex00620003` | PGE06005 | Compound Condition Overlap | - | - | ✅ |

| `JM3Ex00610002` | PGE06009 | Conditional Missing Comparison Operator | - | - | ✅ |

| `JM3Ex00610003` | PGE06010 | Empty Conditional Scope | - | - | ✅ |

| `JM3Ex001F0036` | PGE06011 | Duplicate Wildcard Catch-All | - | - | ✅ |

| `JM3Ex001F0037` | PGE06012 | Unreachable Branch After Wildcard | - | - | ✅ |

| `JM3Ex00620004` | PGE06013 | Tautological or Contradictory Branch Condition | - | - | ✅ |

| `JM3Ex001F0038` | PGE06014 | Wildcard-Only Match | - | - | ✅ |

| `JM3Ex001F0039` | PGE06015 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F003A` | PGE06016 | Unknown Rule | - | - | ✅ |

| `JM3Ex00620005` | PGE06001 | Conditional Must Be Exhaustive | - | - | ⛔ |

| `JM3Ex00630001` | PGE06002 | Enum Exhaustiveness | - | - | ✅ |

| `JM3Ex00620006` | PGE06003 | Numeric Range Not Exhaustive | - | - | ✅ |

| `JM3Ex00630002` | PGE06006 | String Exhaustiveness | - | - | ✅ |

| `JM3Ex00630003` | PGE06007 | Flexible Field Exhaustiveness | - | - | ✅ |

| `JM3Ex00620007` | PGE06008 | Compound Condition Exhaustiveness | - | - | ✅ |

| `JM3Ex00640002` | PGE07001 | Error Block Scoping | - | - | ✅ |

| `JM3Ex00640003` | PGE07002 | Chain Error Scoping | - | - | ✅ |

| `JM3Ex00640004` | PGE07003 | Duplicate Fallback Assignment | - | - | ✅ |

| `JM3Ex00640005` | PGE07004 | Duplicate Error Handler | - | - | ✅ |

| `JM3Ex00640006` | PGE07005 | Undeclared Error Raise | - | - | ✅ |

| `JM3Ex00640007` | PGE07006 | Unused Error Declaration | - | - | ✅ |

| `JM3Ex00630004` | PGE07007 | Error Handling Must Be Exhaustive | - | - | ✅ |

| `JM3Ex00640008` | PGE07008 | Fallback on Non-Failable Source | - | - | ✅ |

| `JM3Ex00640009` | PGE07009 | Unterminated Fallback Chain | - | - | ✅ |

| `JM3Wx0064000A` | PGW07001 | Error Handler on Non-Failable Call | - | - | ✅ |

| `JM3Wx0064000B` | PGW07002 | Caller Overrides Pipeline Fallback | - | - | ✅ |

| `JM3Wx00610004` | PGW07003 | Missing Fallback Message | - | - | ✅ |

| `JM3Wx0064000C` | PGW07004 | Fallback on Non-Failable IO | - | - | ✅ |

| `JM3Wx001F003B` | PGW07010 | Suppress on Consumed Output | - | - | ✅ |

| `JM3Ex00720001` | PGE03001 | No Push Across Parallel Boundaries | - | - | ✅ |

| `JM3Ex00720002` | PGE03004 | Section-Boundary Pairing | - | - | ✅ |

| `JM3Ex00720003` | PGE03012 | Parallel Label Isolation | - | - | ✅ |

| `JM3Ex0011000E` | PGE03018 | Missing Incoming DataFrame | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Ex001F003C` | PGE03019 | Arrival Index Out of Bounds | - | - | ✅ |

| `JM3Ex00520006` | PGE03020 | Statements Outside Trigger Blocks | - | - | ✅ |

| `JM3Ex00610005` | PGE03023 | Overflow Strategy Missing Storage Error | - | - | ✅ |

| `JM3Ex00520007` | PGE03024 | Release With No Remaining Claims | - | - | ✅ |

| `JM3Ex00520008` | PGE03025 | Not All Jobs Released | - | - | ✅ |

| `JM3Wx001F003D` | PGW03001 | "[b] Called Pipeline Has Discarded Outputs" | - | - | ✅ |

| `JM3Wx0064000D` | PGW03002 | Error Handler on Fire-and-Forget | - | - | ✅ |

| `JM3Ex00620008` | PGE03007 | Expand Operator Input Mismatch | - | - | ✅ |

| `JM3Ex00710003` | PGE03011 | Orphaned Expand IO Marker | - | - | ✅ |

| `JM3Ex00720004` | PGE03002 | Parallel Output Must Be Collected | - | - | ✅ |

| `JM3Ex00740001` | PGE03003 | Variable Isolation Until Collection | - | - | ✅ |

| `JM3Ex00740002` | PGE03005 | "[b] Has No Collectible Output" | - | - | ✅ |

| `JM3Ex00740003` | PGE03006 | Race Collector Type Homogeneity | - | - | ✅ |

| `JM3Ex00620009` | PGE03008 | Collect Operator IO Mismatch | - | - | ✅ |

| `JM3Ex00730001` | PGE03009 | Nested Expand Without Collect | - | - | ✅ |

| `JM3Ex00730002` | PGE03010 | Collector Without Expand | - | - | ✅ |

| `JM3Ex00420002` | PGE03013 | Collector Metadata Required | - | - | ✅ |

| `JM3Ex00730003` | PGE03014 | Expand-Scoped Collector Outside ForEach | - | - | ✅ |

| `JM3Ex00720005` | PGE03015 | Parallel-Scoped Collector Inside ForEach | - | - | ✅ |

| `JM3Ex00740004` | PGE03016 | Collector IO Mismatch | - | - | ✅ |

| `JM3Ex00740005` | PGE03017 | Arrival Operations Outside Collector Context | - | - | ✅ |

| `JM3Ex00720006` | PGE03021 | Parallel Execution Inside Collector | - | - | ✅ |

| `JM3Ex00740006` | PGE03022 | External Trigger Source in Collector | - | - | ✅ |

| `JM3Ex00820004` | PGE04001 | Type Mismatch | - | - | ⛔ |

| `JM3Ex00820005` | PGE04002 | Schema Mismatch | - | - | ✅ |

| `JM3Ex00820006` | PGE04003 | Leaf-Only Assignment | - | - | ✅ |

| `JM3Ex00820007` | PGE04004 | Fixed-Schema Keys Are Compile-Time Only | - | - | ✅ |

| `JM3Ex00810003` | PGE04005 | Undefined Interpolation Variable | - | - | ✅ |

| `JM3Ex00810004` | PGE04006 | Undefined Variable Reference | - | - | ⛔ |

| `JM3Ex001F003E` | PGE04009 | Unhandled Serial→Struct Conversion | - | - | ✅ |

| `JM3Ex00610006` | PGE04010 | Invalid Arithmetic Operator | - | - | ✅ |

| `JM3Ex001F003F` | PGE04012 | Division by Literal Zero | - | - | ✅ |

| `JM3Ex00610007` | PGE04014 | Invalid Range Bounds | - | - | ✅ |

| `JM3Ex0062000A` | PGE04015 | Conditional Type-Operator Mismatch | - | - | ✅ |

| `JM3Ex0011000F` | PGE04016 | Invalid Pipeline Input Literal | - | - | ✅ |

| `JM3Ex001F0040` | PGE04024 | Non-Value Comparison | - | - | ✅ |

| `JM3Ex00810005` | PGE04028 | Invalid Epoch Value | - | - | ✅ |

| `JM3Ex001F0041` | PGE04029 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0042` | PGE04030 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0043` | PGE04031 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0044` | PGE04032 | Unknown Rule | - | - | ✅ |

| `JM3Ex001F0045` | PGE04033 | Unknown Rule | - | - | ✅ |

| `JM3Wx001F0046` | PGW04002 | Leading Zeros in Literal | - | - | ✅ |

| `JM3Ex00830001` | PGE04011 | Negative Array Index Literal | - | - | ✅ |

| `JM3Ex00820008` | PGE04013 | Nested Array Type | - | - | ✅ |

| `JM3Ex00830002` | PGE04017 | Array Dimension Access Mismatch | - | - | ✅ |

| `JM3Ex00820009` | PGE04025 | Untyped Array | - | - | ✅ |

| `JM3Ex00810006` | PGE04007 | Invalid Path String | - | - | ✅ |

| `JM3Ex00810007` | PGE04008 | Missing Path Platform Subfield | - | - | ✅ |

| `JM3Ex00810008` | PGE04026 | Invalid IANA Timezone | - | - | ✅ |

| `JM3Ex00110010` | PGE04027 | Missing Required DateTime Subfield | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Wx00840001` | PGW04001 | Single-Platform Path | - | - | ✅ |

| `JM3Ex0082000A` | PGE08001 | Auto-Wire Type Mismatch | - | - | ✅ |

| `JM3Ex0082000B` | PGE08002 | Auto-Wire Ambiguous Type | - | - | ✅ |

| `JM3Ex00920001` | PGE08003 | Auto-Wire Port Count Mismatch | - | - | ✅ |

| `JM3Ex001F0047` | PGE08004 | Ambiguous Step Reference | - | - | ✅ |

| `JM3Ex00110011` | PGE08005 | Unresolved Step Reference | - | - | ✅ |

| `JM3Ex0064000E` | PGE08006 | Non-Pipeline Step in Chain | - | - | ✅ |

| `JM3Ex00110012` | PGE08007 | Invalid Assignment Target | - | - | ⛔ |

| `JM3Ex00110013` | PGE08008 | Missing Required Input at Call Site | `missing_token_detector` | `missing_token_detector` | ✅ |

| `JM3Ex001F0048` | PGE08009 | Uncaptured Required Output at Call Site | - | - | ✅ |

| `JM3Ex001F0049` | PGE08010 | IO Direction Mismatch | - | - | ✅ |

| `JM3Ex001F004A` | PGE08013 | Nested Inline Data | - | - | ✅ |

| `JM3Wx00920002` | PGW08001 | Auto-Wire Succeeded | - | - | ✅ |

| `JM3Wx001F004B` | PGW08002 | Unaddressed Input With Default | - | - | ✅ |

| `JM3Wx0064000F` | PGW08003 | Uncaptured Output With Default/Fallback | - | - | ✅ |

| `JM3Ex00930001` | PGE01004 | Wrapper Structural Constraints | - | - | ✅ |

| `JM3Ex00930002` | PGE01008 | Wrapper Must Reference Wrapper Definition | - | - | ✅ |

| `JM3Ex00930003` | PGE01009 | Wrapper IO Mismatch | - | - | ✅ |

| `JM3Ex00930004` | PGE01025 | Discard in Wrapper IO | - | - | ✅ |

| `JM3Ex00910001` | PGE01030 | Missing Pipeline Wrapper | - | - | ✅ |

| `JM3Ex00940001` | PGE08011 | Self-Assignment | - | - | ✅ |

| `JM3Ex00640010` | PGE08012 | Self-Chain Requires Numeric Indexing | - | - | ✅ |

| `JM3Ex00A20001` | PGE10003 | Unknown Permission Category | - | - | ✅ |

| `JM3Ex00A20002` | PGE10004 | Undeclared Permission | - | - | ✅ |

| `JM3Ex00A10003` | PGE10005 | Invalid Permission Block Marker | - | - | ✅ |

| `JM3Ex00A20003` | PGE10006 | Duplicate Permission | - | - | ✅ |

| `JM3Ex00720007` | PGE10008 | Parallel Write Permission Exclusion | - | - | ✅ |

| `JM3Ex00510004` | PGE10009 | Unresolved Permission Template | - | - | ✅ |

| `JM3Ex00A20004` | PGE10010 | Permission Resource Not Found | - | - | ✅ |

| `JM3Ex001F004C` | PGE10011 | Shell Without Capability | - | - | ✅ |

| `JM3Wx00A20005` | PGW10001 | Unused Permission | - | - | ✅ |

| `JM3Ex00640011` | PGE10007 | Chain Step Label Overflow | - | - | ✅ |

| `JM3Ex0012000D` | PGE10012 | Code File Outside Scope | - | - | ✅ |

| `JM3Wx00920003` | PGW10003 | Bind Mode Opacity | - | - | ✅ |

| `JM3Wx0082000C` | PGW10006 | Shell Variable Expansion | - | - | ✅ |

| `JM3Ex00A30001` | PGE10013 | Foreign Resource Outside Scope | - | - | ✅ |

| `JM3Ex00A40001` | PGE10014 | AST-Invisible Foreign Code | - | - | ✅ |

| `JM3Ex00A40002` | PGE10015 | Opaque Binary Without Sandbox Acknowledgment | - | - | ✅ |

| `JM3Ex00410002` | PGE10016 | Missing Mandatory Metadata for Sandbox-Only | - | - | ✅ |

| `JM3Wx00A40003` | PGW10002 | Unverifiable Foreign IO | - | - | ✅ |

| `JM3Wx00A40004` | PGW10005 | Unrecognized Foreign Call | - | - | ✅ |

| `JM3Wx00A40005` | PGW10007 | Sandbox-Only Enforcement Active | - | - | ✅ |
