import re

with open("docs/audit/compiler-rules-tracker.md", "r") as f:
    lines = f.readlines()

new_lines = []
counters = {}

for line in lines:
    m = re.search(r"\|\s*`(JM3[EW]x)[0-9A-F]{8}`\s*\|\s*(PGE[0-9]{5}|PGW[0-9]{5})\s*\|\s*(.*?)\s*\|\s*(.*?)\s*\|", line)
    if m:
        prefix = m.group(1)
        old_id = m.group(2)
        name = m.group(3).strip()
        status = m.group(4)
        
        name_lower = name.lower()
        cat = "001F"
        
        if "package" in name_lower or "file" in name_lower: cat = "0012"
        elif "import" in name_lower or "circular" in name_lower: cat = "0012"
        elif "data field" in name_lower or "sibling" in name_lower or "recursive data" in name_lower: cat = "0022"
        elif "schema" in name_lower and "data" in name_lower: cat = "0023"
        elif "queue" in name_lower: cat = "0032"
        elif "metadata" in name_lower or "alias" in name_lower: cat = "0042"
        elif "lifecycle" in name_lower or "state" in name_lower or "push-once" in name_lower or "release" in name_lower: cat = "0052"
        elif "constructor" in name_lower or "overload" in name_lower: cat = "0053"
        elif "template" in name_lower or "format" in name_lower or "placeholder" in name_lower: cat = "0054"
        elif "operator" in name_lower or "range" in name_lower or "condition" in name_lower or "arithmetic" in name_lower: cat = "0062"
        elif "exhaust" in name_lower: cat = "0063"
        elif "error" in name_lower or "fallback" in name_lower or "chain" in name_lower: cat = "0064"
        elif "parallel" in name_lower or "section-boundary" in name_lower or "label isolation" in name_lower: cat = "0072"
        elif "expand" in name_lower: cat = "0073"
        elif "collect" in name_lower or "orphan continuation" in name_lower: cat = "0074"
        elif "background" in name_lower: cat = "0075"
        elif "type" in name_lower or "schema" in name_lower or "leaf" in name_lower or "variable" in name_lower: cat = "0082"
        elif "array" in name_lower: cat = "0083"
        elif "path" in name_lower or "timezone" in name_lower or "epoch" in name_lower: cat = "0084"
        elif "auto-wire" in name_lower or "bind" in name_lower: cat = "0092"
        elif "wrapper" in name_lower: cat = "0093"
        elif "self" in name_lower: cat = "0094"
        elif "permission" in name_lower: cat = "00A2"
        elif "scope" in name_lower: cat = "00A3"
        elif "foreign" in name_lower or "binary" in name_lower or "sandbox" in name_lower: cat = "00A4"
        
        # Override to Syntax/Structure if keywords match
        if any(w in name_lower for w in ["marker", "missing", "invalid", "empty", "orphan", "unmarked", "unbound", "undefined", "unresolved"]):
            cat = cat[:3] + "1"
            
        if cat not in counters: counters[cat] = 1
        else: counters[cat] += 1
            
        new_hex = f"{prefix}{cat}{counters[cat]:04X}"
        new_lines.append(f"| `{new_hex}` | {old_id} | {name} | {status} |")
    else:
        new_lines.append(line.strip())

with open("docs/audit/compiler-rules-tracker.md", "w") as f:
    f.write("\n".join(new_lines))
