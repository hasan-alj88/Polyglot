import xml.etree.ElementTree as ET

ET.register_namespace('', "http://www.w3.org/2000/svg")
ns = {'svg': 'http://www.w3.org/2000/svg'}

for filename in ["Aljam3 Logo/SVG/Logo.svg", "Aljam3 Logo/SVG/Icon.svg"]:
    tree = ET.parse(filename)
    root = tree.getroot()
    
    # Find the braces and change their color to white
    for path in root.findall('.//svg:path', ns):
        if path.attrib.get('id') in ['path27457', 'path27552']:
            # They are currently black or whatever they were in the old logo.
            # Change to white
            path.set('fill', '#ffffff')
            # If there's a style attribute dictating color, override it
            if 'style' in path.attrib:
                style = path.attrib['style']
                # Replace fill:#000000 with fill:#ffffff
                import re
                new_style = re.sub(r'fill:#[0-9a-fA-F]+', 'fill:#ffffff', style)
                if 'fill:' not in new_style:
                    new_style += ';fill:#ffffff'
                path.set('style', new_style)

    tree.write(filename, xml_declaration=True, encoding='utf-8')

print("Updated curly brackets to white in Logo and Icon.")
