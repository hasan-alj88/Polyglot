import xml.etree.ElementTree as ET
tree = ET.parse("Aljam3 Logo/SVG/Old Logo.svg")
root = tree.getroot()
ns = {'svg': 'http://www.w3.org/2000/svg'}
for path in root.findall('.//svg:path', ns):
    print("Path ID:", path.attrib.get('id'), "Length of d:", len(path.attrib.get('d', '')))
