from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "icons" / "tray_blob.png"
DESTINATION = ROOT / "icons" / "icon.ico"
SIZES = (16, 24, 32, 48, 64, 128, 256)


source = Image.open(SOURCE).convert("RGBA")
frames = []

for size in SIZES:
    canvas = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    blob = source.copy()
    blob.thumbnail((round(size * 0.88), round(size * 0.78)), Image.Resampling.LANCZOS)
    canvas.alpha_composite(blob, ((size - blob.width) // 2, (size - blob.height) // 2))
    frames.append(canvas)

frames[-1].save(DESTINATION, format="ICO", append_images=frames[:-1])
