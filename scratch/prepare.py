from PIL import Image

# Process Icon
im = Image.open("Aljam3 Logo/Icon.jpg").convert("RGB")
w, h = im.size
pixels = im.load()

# We want to isolate the green block and the black minus.
# The background is black. The new brackets are bright green on the edges.
# The minus is black in the center of the green block.
# Let's paint the left 20% and right 20% black to erase the brackets.
erase_w = int(w * 0.22)
for y in range(h):
    for x in range(erase_w):
        pixels[x, y] = (0, 0, 0)
    for x in range(w - erase_w, w):
        pixels[x, y] = (0, 0, 0)

# Now convert everything that is NOT black into BLACK (for potrace) and black to WHITE.
out = Image.new("1", (w, h))
out_pixels = out.load()
for y in range(h):
    for x in range(w):
        r, g, b = pixels[x, y]
        # If it's dark (background or minus), make it white so it doesn't get traced as a shape,
        # OR wait, potrace traces black. We want the green block to be black.
        # So if it's green (e.g. g > 50), make it black. Else white.
        if g > 30 or r > 30 or b > 30:
            out_pixels[x, y] = 0 # Black (traced)
        else:
            out_pixels[x, y] = 1 # White (ignored)
out.save("scratch/block.bmp")

# Process Text from Logo
im2 = Image.open("Aljam3 Logo/Logo.jpg").convert("RGB")
w2, h2 = im2.size
# Text is in the bottom part (y > 961)
text_im = im2.crop((0, 961, w2, h2))
tw, th = text_im.size
text_pixels = text_im.load()

out2 = Image.new("1", (tw, th))
out2_pixels = out2.load()
for y in range(th):
    for x in range(tw):
        r, g, b = text_pixels[x, y]
        if r > 50 or g > 50: # Yellow is r+g
            out2_pixels[x, y] = 0
        else:
            out2_pixels[x, y] = 1
out2.save("scratch/text.bmp")

print("Prepared BMP files.")
