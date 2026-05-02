import xml.etree.ElementTree as ET

ET.register_namespace('', "http://www.w3.org/2000/svg")
ns = {'svg': 'http://www.w3.org/2000/svg'}

# Parse Old Logo.svg
tree_old_logo = ET.parse("Aljam3 Logo/SVG/Old Logo.svg")
root_old_logo = tree_old_logo.getroot()

brace_left = None
brace_right = None
for path in root_old_logo.findall('.//svg:path', ns):
    if path.attrib.get('id') == 'path27457':
        brace_left = path
    elif path.attrib.get('id') == 'path27552':
        brace_right = path

# We want to replace the curly braces in Logo.svg and Icon.svg
for filename in ["Aljam3 Logo/SVG/Logo.svg", "Aljam3 Logo/SVG/Icon.svg"]:
    tree = ET.parse(filename)
    root = tree.getroot()
    
    # The brackets are inside a group with a specific transform
    # We will find the group containing path27457 and path27552
    for g in root.findall('.//svg:g', ns):
        has_braces = False
        for child in list(g):
            if child.tag == '{http://www.w3.org/2000/svg}path' and child.attrib.get('id') in ['path27457', 'path27552']:
                g.remove(child)
                has_braces = True
        
        if has_braces:
            # We found the group, now add the old braces
            g.append(brace_left)
            g.append(brace_right)

    tree.write(filename, xml_declaration=True, encoding='utf-8')

print("Swapped curly brackets from Old Logo into Logo and Icon.")
