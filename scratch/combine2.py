import xml.etree.ElementTree as ET
import re

ET.register_namespace('', "http://www.w3.org/2000/svg")
ns = {'svg': 'http://www.w3.org/2000/svg'}

# 1. Parse old Icon.svg to get the old braces
tree_old = ET.parse("Aljam3 Logo/SVG/Old Logo.svg")
root_old = tree_old.getroot()

brace_left = None
brace_right = None
for path in root_old.findall('.//svg:path', ns):
    if path.attrib.get('id') == 'path27457':
        brace_right = path
    elif path.attrib.get('id') == 'path27552':
        brace_left = path

# Change color to white
brace_left.set('fill', '#ffffff')
brace_right.set('fill', '#ffffff')
if 'style' in brace_left.attrib: del brace_left.attrib['style']
if 'style' in brace_right.attrib: del brace_right.attrib['style']

# 2. Parse potrace block.svg
tree_block = ET.parse("scratch/block.svg")
root_block = tree_block.getroot()
g_block = root_block.find('.//svg:g', ns)
g_block.set('fill', '#39FF14')

# 3. Parse potrace text.svg
tree_text = ET.parse("scratch/text.svg")
root_text = tree_text.getroot()
g_text = root_text.find('.//svg:g', ns)
g_text.set('fill', '#FFD700')

# Now let's calculate the transformation for the braces
# We want to center them at X=437.5, Y=498.0
# The old braces are centered roughly at X=206, Y=170 in the old viewBox
# To make them look proportionate, we use a scale of 4.5
scale = 4.5
old_cx = 206
old_cy = 170

target_cx = 437.5
target_cy = 498.0

tx = target_cx - (old_cx * scale)
ty = target_cy - (old_cy * scale)

icon_svg = ET.Element('svg', {'viewBox': '0 0 875 961', 'width': '875', 'height': '961'})
ET.SubElement(icon_svg, 'rect', {'width': '875', 'height': '961', 'fill': '#0d1117'})
icon_svg.append(g_block)

g_braces = ET.SubElement(icon_svg, 'g', {'transform': f'translate({tx}, {ty}) scale({scale})'})
g_braces.append(brace_left)
g_braces.append(brace_right)

with open('scratch/Vector-Icon.svg', 'wb') as f:
    f.write(ET.tostring(icon_svg))

# Logo SVG
logo_svg = ET.Element('svg', {'viewBox': '0 0 875 1532', 'width': '875', 'height': '1532'})
ET.SubElement(logo_svg, 'rect', {'width': '875', 'height': '1532', 'fill': '#0d1117'})
logo_svg.append(g_block)

g_braces_logo = ET.SubElement(logo_svg, 'g', {'transform': f'translate({tx}, {ty}) scale({scale})'})
g_braces_logo.append(brace_left)
g_braces_logo.append(brace_right)

g_text_wrapper = ET.SubElement(logo_svg, 'g', {'transform': 'translate(0, 961)'})
g_text_wrapper.append(g_text)

with open('scratch/Vector-Logo.svg', 'wb') as f:
    f.write(ET.tostring(logo_svg))

print("Combined SVGs created with perfect positioning and clean block!")
