#!/usr/bin/env python3
"""Assemble a looping product-tour.gif from screenshot PNGs.

Usage:
  assemble-product-tour-gif.py <out.gif> <frame1.png> [frame2.png ...]
"""
from __future__ import annotations

import sys
from pathlib import Path

from PIL import Image


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    out = Path(sys.argv[1])
    frames_in = [Path(p) for p in sys.argv[2:]]
    target = (960, 540)
    images: list[Image.Image] = []
    for path in frames_in:
        im = Image.open(path).convert("RGB")
        # Cover-crop to 16:9 then resize
        src_w, src_h = im.size
        target_ratio = target[0] / target[1]
        src_ratio = src_w / src_h
        if src_ratio > target_ratio:
            new_w = int(src_h * target_ratio)
            left = (src_w - new_w) // 2
            im = im.crop((left, 0, left + new_w, src_h))
        else:
            new_h = int(src_w / target_ratio)
            top = (src_h - new_h) // 2
            im = im.crop((0, top, src_w, top + new_h))
        im = im.resize(target, Image.Resampling.LANCZOS)
        # GIF palette
        images.append(im.convert("P", palette=Image.Palette.ADAPTIVE, colors=256))

    out.parent.mkdir(parents=True, exist_ok=True)
    images[0].save(
        out,
        save_all=True,
        append_images=images[1:],
        duration=1800,
        loop=0,
        optimize=True,
    )
    print(f"wrote {out} ({out.stat().st_size} bytes, {len(images)} frames)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
