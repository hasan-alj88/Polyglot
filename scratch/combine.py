import xml.etree.ElementTree as ET
import os

ET.register_namespace('', "http://www.w3.org/2000/svg")

# 1. Parse old Icon.svg to get the old braces
tree_old = ET.parse("Aljam3 Logo/SVG/Icon.svg")
root_old = tree_old.getroot()
ns = {'svg': 'http://www.w3.org/2000/svg'}
brace_left = None
brace_right = None
for path in root_old.findall('.//svg:path', ns):
    if path.attrib.get('id') == 'path27457':
        brace_left = path
    elif path.attrib.get('id') == 'path27552':
        brace_right = path

brace_left.set('fill', '#39FF14')
brace_right.set('fill', '#39FF14')

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

# Now let's create the final Vector-Icon.svg
# The block is 875 x 961. Let's make the canvas 875 x 961.
# The old SVG is 412.43 x 412.43.
scale = 875 / 412.43
scale_y = 961 / 412.43

# Wait, if we scale the braces by scale, they might look stretched. Let's use uniform scale and center them vertically.
uniform_scale = 875 / 412.43
y_offset = (961 - (412.43 * uniform_scale)) / 2

icon_svg = ET.Element('svg', {'xmlns': 'http://www.w3.org/2000/svg', 'viewBox': '0 0 875 961', 'width': '875', 'height': '961'})
# Background
ET.SubElement(icon_svg, 'rect', {'width': '875', 'height': '961', 'fill': '#0d1117'})

# Add the green block
icon_svg.append(g_block)

# Add the old braces
g_braces = ET.SubElement(icon_svg, 'g', {'transform': f'translate(0, {y_offset}) scale({uniform_scale})'})
g_braces.append(brace_left)
g_braces.append(brace_right)

with open('scratch/Vector-Icon.svg', 'wb') as f:
    f.write(ET.tostring(icon_svg))

# Now create the final Vector-Logo.svg
# It consists of Icon on top, Text on bottom.
# Logo canvas size: 875 x (961 + 571) = 875 x 1532
logo_svg = ET.Element('svg', {'xmlns': 'http://www.w3.org/2000/svg', 'viewBox': '0 0 875 1532', 'width': '875', 'height': '1532'})
ET.SubElement(logo_svg, 'rect', {'width': '875', 'height': '1532', 'fill': '#0d1117'})

# Add the icon part
logo_svg.append(g_block)

# Add the braces
g_braces_logo = ET.SubElement(logo_svg, 'g', {'transform': f'translate(0, {y_offset}) scale({uniform_scale})'})
g_braces_logo.append(brace_left)
g_braces_logo.append(brace_right)

# Add the text part, translated down by 961
g_text_wrapper = ET.SubElement(logo_svg, 'g', {'transform': 'translate(0, 961)'})
g_text_wrapper.append(g_text)

with open('scratch/Vector-Logo.svg', 'wb') as f:
    f.write(ET.tostring(logo_svg))

print("Combined SVGs created.")
