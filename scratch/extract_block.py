from PIL import Image

im = Image.open("Aljam3 Logo/Icon.jpg").convert("RGB")
w, h = im.size
pixels = im.load()

# 1. Threshold green pixels
green_mask = [[False]*h for _ in range(w)]
for x in range(w):
    for y in range(h):
        r, g, b = pixels[x, y]
        # It's green if G is dominant
        if g > 50 and g > r + 10 and g > b + 10:
            green_mask[x][y] = True

# 2. Find connected components
visited = [[False]*h for _ in range(w)]
components = []

for x in range(w):
    for y in range(h):
        if green_mask[x][y] and not visited[x][y]:
            # BFS
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

print(f"Found {len(components)} green components.")
largest_comp = max(components, key=len)
print(f"Largest component size: {len(largest_comp)}")

# 3. Create a clean image with ONLY the largest component (the block)
out = Image.new("1", (w, h), 1) # White background
out_pixels = out.load()
for (x, y) in largest_comp:
    out_pixels[x, y] = 0 # Black (for potrace)

# Wait! The minus sign is a hole inside the largest component. 
# We need the minus sign to be a hole (white) in the black block.
# Since we only colored the green pixels black, the black minus sign is ALREADY white in our output!
# This is perfect.

out.save("scratch/block_clean.bmp")
print("Saved clean block.")

