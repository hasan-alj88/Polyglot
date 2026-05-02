import re
import xml.etree.ElementTree as ET

def get_bbox(d_str):
    # simple regex to find all numbers
    numbers = re.findall(r'-?\d+\.?\d*', d_str)
    # The path consists of commands and coords. This is a very rough bbox assuming all numbers are coords (some are bezier control points, which stay close to the bounding box anyway).
    xs = [float(numbers[i]) for i in range(0, len(numbers)-1, 2)]
    ys = [float(numbers[i+1]) for i in range(0, len(numbers)-1, 2)]
    if not xs: return 0,0,0,0
    return min(xs), min(ys), max(xs), max(ys)

ns = {'svg': 'http://www.w3.org/2000/svg'}
tree = ET.parse("Aljam3 Logo/SVG/Old Logo.svg")
root = tree.getroot()

for path in root.findall('.//svg:path', ns):
    if path.attrib.get('id') in ['path27457', 'path27552']:
        d = path.attrib.get('d', '')
        x1, y1, x2, y2 = get_bbox(d)
        print(f"{path.attrib.get('id')}: X({x1:.1f} - {x2:.1f}) Y({y1:.1f} - {y2:.1f})")

