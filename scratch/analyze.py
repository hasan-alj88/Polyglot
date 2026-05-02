from PIL import Image

im = Image.open("Aljam3 Logo/Icon.jpg").convert("RGB")
w, h = im.size
pixels = im.load()

# Find the center of the green block
# We scan horizontally at y = h // 2
green_pixels = []
for x in range(w):
    r, g, b = pixels[x, h // 2]
    if g > 50 and r < g and b < g:
        green_pixels.append(x)

if green_pixels:
    print(f"Green block at center Y spans from X={min(green_pixels)} to X={max(green_pixels)}")
    print(f"Width of green block: {max(green_pixels) - min(green_pixels)}")

# Find the overall bounding box of the green block (roughly)
# by ignoring X < 250 and X > 625 (assuming the braces are outside this)
min_x, max_x, min_y, max_y = w, 0, h, 0
for y in range(h):
    for x in range(250, 625):
        r, g, b = pixels[x, y]
        if g > 50 and g > r + 10 and g > b + 10:
            min_x = min(min_x, x)
            max_x = max(max_x, x)
            min_y = min(min_y, y)
            max_y = max(max_y, y)

print(f"Green block BBox: X({min_x} - {max_x}) Y({min_y} - {max_y})")

