#!/usr/bin/env python3
"""Generate the 12 demo thumbnails embedded by smart-core (assets/thumbs/*.jpg).
Deterministic abstract art that matches each demo file's name."""
import math, os, random
from PIL import Image, ImageDraw, ImageFilter

OUT = os.path.join(os.path.dirname(__file__), "..", "smart-files", "assets", "thumbs")
os.makedirs(OUT, exist_ok=True)
W, H = 640, 420


def grad(c0, c1, vert=True):
    im = Image.new("RGB", (W, H))
    px = im.load()
    n = H if vert else W
    for i in range(n):
        t = i / (n - 1)
        c = tuple(int(c0[k] + (c1[k] - c0[k]) * t) for k in range(3))
        if vert:
            for x in range(W): px[x, i] = c
        else:
            for y in range(H): px[i, y] = c
    return im


def blobs(im, items, blur=30):
    ov = Image.new("RGB", (W, H), (0, 0, 0))
    d = ImageDraw.Draw(ov)
    for (cx, cy, rx, ry, col) in items:
        d.ellipse([cx - rx, cy - ry, cx + rx, cy + ry], fill=col)
    ov = ov.filter(ImageFilter.GaussianBlur(blur))
    return Image.blend(im, ov, 0.55)


def bars(im, cols, bh=26):
    d = ImageDraw.Draw(im)
    y = 40
    for c in cols:
        d.rectangle([40, y, W - 40, y + bh], fill=c)
        y += bh + 14
    return im


def ui(dark, accent, lines=6):
    im = grad((24, 28, 38), (14, 17, 26)) if dark else grad((246, 247, 250), (232, 235, 242))
    d = ImageDraw.Draw(im)
    d.rectangle([0, 0, W, 44], fill=(30, 34, 46) if dark else (255, 255, 255))
    d.rectangle([30, 14, 150, 30], fill=accent)
    y = 80
    for i in range(lines):
        w = int(W * (0.7 if i % 3 else 0.5))
        d.rectangle([40, y, 40 + w, y + 16], fill=(52, 58, 74) if dark else (210, 215, 226))
        y += 34
    d.rectangle([W - 170, 70, W - 40, 70 + 120], fill=accent)
    return im


jobs = {}

def frosted_glass():
    im = grad((208, 214, 232), (126, 142, 178))
    d = ImageDraw.Draw(im, "RGBA")
    for (cy, a) in [(120, 150), (210, 120), (300, 90)]:
        d.rounded_rectangle([W//2-70, cy-55, W//2+70, cy+55], 55, fill=(170, 195, 235, a), outline=(255,255,255,120), width=2)
    return im

jobs["frosted-glass"] = frosted_glass
jobs["texture-01"] = lambda: blobs(grad((38, 84, 92), (214, 122, 122)), [(120, 140, 60, 220, (20, 40, 50)), (330, 200, 90, 260, (180, 70, 90)), (520, 120, 70, 200, (240, 150, 120))])
jobs["talk-notes"] = lambda: ui(True, (232, 108, 32))
jobs["design-scan"] = lambda: blobs(grad((8, 12, 10), (16, 26, 18)), [(160, 300, 220, 90, (60, 220, 90)), (480, 120, 160, 70, (120, 255, 140))], 24)
jobs["brand-board"] = lambda: ui(False, (47, 111, 235))
jobs["poster-01"] = lambda: blobs(grad((2, 3, 8), (8, 10, 22)), [(320, 210, 130, 130, (40, 90, 220)), (320, 210, 60, 60, (180, 220, 255))], 18)
jobs["package-a"] = lambda: blobs(grad((18, 18, 22), (30, 28, 30)), [(200, 300, 260, 60, (230, 110, 30)), (470, 140, 120, 90, (70, 74, 86))], 26)
jobs["ui-exploration"] = lambda: bars(grad((20, 22, 30), (26, 28, 38)), [(232, 68, 82), (240, 150, 40), (238, 210, 70), (70, 190, 120), (56, 150, 235), (120, 90, 220), (220, 90, 190)])
jobs["layout-study"] = lambda: ui(False, (120, 130, 150))
jobs["reference-01"] = lambda: blobs(grad((214, 196, 172), (178, 156, 132)), [(150, 130, 110, 80, (120, 90, 60)), (430, 260, 140, 100, (90, 70, 52)), (480, 110, 80, 60, (240, 230, 214))])
jobs["landing-draft"] = lambda: ui(False, (43, 168, 122))
jobs["package-b"] = lambda: blobs(grad((10, 14, 26), (18, 26, 48)), [(300, 220, 180, 110, (47, 111, 235)), (470, 120, 90, 70, (111, 155, 255))], 22)

for name, fn in jobs.items():
    im = fn().convert("RGB")
    im.save(os.path.join(OUT, name + ".jpg"), quality=82)
    print("wrote", name)
print("done", len(jobs))
