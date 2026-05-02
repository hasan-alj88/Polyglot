from PIL import Image
import subprocess
import os
import xml.etree.ElementTree as ET

ET.register_namespace('', "http://www.w3.org/2000/svg")

im = Image.open("Aljam3 Logo/Icon.jpg").convert("RGB")
w, h = im.size
pixels = im.load()

# 1. Isolate the block (from previous script)
green_mask = [[False]*h for _ in range(w)]
for x in range(w):
    for y in range(h):
        r, g, b = pixels[x, y]
        if g > 50 and g > r + 10 and g > b + 10:
            green_mask[x][y] = True

visited = [[False]*h for _ in range(w)]
components = []
for x in range(w):
    for y in range(h):
        if green_mask[x][y] and not visited[x][y]:
            comp = []
            queue = [(x, y)]
            visited[x][y] = True
            head = 0
            while head < len(queue):
                cx, cy = queue[head]
                head += 1
                comp.append((cx, cy))
                for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                    nx, ny = cx + dx, cy + dy
                    if 0 <= nx < w and 0 <= ny < h:
                        if green_mask[nx][ny] and not visited[nx][ny]:
                            visited[nx][ny] = True
                            queue.append((nx, ny))
            components.append(comp)

largest_comp = max(components, key=len)
block_pixels = set(largest_comp)

# 2. Bucket colors into 4 shades of green
# Let's find the max and min luminance (or just Green channel) in the block
g_vals = []
for (x, y) in block_pixels:
    r, g, b = pixels[x, y]
    g_vals.append(g)

min_g = min(g_vals)
max_g = max(g_vals)
# divide into 3 thresholds (4 buckets)
buckets = [
    {"name": "dark", "color": "#005500", "pixels": []},
    {"name": "mid", "color": "#00aa00", "pixels": []},
    {"name": "light", "color": "#39FF14", "pixels": []},
    {"name": "yellowish", "color": "#aaff00", "pixels": []}
]
step = (max_g - min_g) / 4

for (x, y) in block_pixels:
    r, g, b = pixels[x, y]
    idx = int((g - min_g) / step)
    if idx >= 4: idx = 3
    buckets[idx]["pixels"].append((x, y))

# 3. Generate BMPs and trace them
svg_paths = []
for i, bucket in enumerate(buckets):
    out = Image.new("1", (w, h), 1) # white bg
    out_pixels = out.load()
    # For a layer, we should probably trace all pixels that are AT LEAST this dark
    # to avoid gaps between layers. So layer 'dark' is just the dark pixels.
    # Layer 'mid' is mid + dark. Layer 'light' is light + mid + dark.
    # Actually, stacking from bottom (lightest/base) to top (darkest details).
    # Base layer: all block_pixels (traced as light green)
    # Layer 2: all pixels except yellowish
    # Layer 3: mid + dark
    # Layer 4: dark only
    pass

# Wait, the easiest stacking is:
# Layer 1 (Base): all block_pixels. Color: yellowish
# Layer 2: light + mid + dark. Color: light green
# Layer 3: mid + dark. Color: mid green
# Layer 4: dark. Color: dark green

layers = [
    {"name": "base", "color": "#b5e51d", "condition": lambda g_val: True},
    {"name": "light", "color": "#39ff14", "condition": lambda g_val: g_val < min_g + step * 3},
    {"name": "mid", "color": "#22b14c", "condition": lambda g_val: g_val < min_g + step * 2},
    {"name": "dark", "color": "#005826", "condition": lambda g_val: g_val < min_g + step * 1},
]

potrace_path = "./potrace-1.16.linux-x86_64/potrace"

for layer in layers:
    out = Image.new("1", (w, h), 1)
    out_pixels = out.load()
    for (x, y) in block_pixels:
        r, g, b = pixels[x, y]
        if layer["condition"](g):
            out_pixels[x, y] = 0 # trace this pixel
    
    bmp_path = f"scratch/layer_{layer['name']}.bmp"
    svg_path = f"scratch/layer_{layer['name']}.svg"
    out.save(bmp_path)
    
    subprocess.run([potrace_path, bmp_path, "-s", "-o", svg_path])
    
    # Extract path from potrace SVG
    tree = ET.parse(svg_path)
    ns = {'svg': 'http://www.w3.org/2000/svg'}
    g_elem = tree.getroot().find('.//svg:g', ns)
    if g_elem is not None:
        g_elem.set('fill', layer['color'])
        svg_paths.append(g_elem)

# 4. Combine into final SVG
final_svg = ET.Element('svg', {'viewBox': f'0 0 {w} {h}', 'width': str(w), 'height': str(h)})
for p in svg_paths:
    final_svg.append(p)

with open("Aljam3 Logo/SVG/New-Vector-Green-Block.svg", "wb") as f:
    f.write(ET.tostring(final_svg))

print("Multi-color trace completed.")
