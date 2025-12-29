#!/usr/bin/env python3
"""
Sprite Processing Script

Processes AI-generated sprite sheets:
1. Removes background (white, gray, teal, etc.)
2. Auto-crops to content
3. Resizes/pads to target dimensions
4. Saves with transparency

Usage:
    python process_sprites.py <input_dir> <output_dir>
    python process_sprites.py  # Uses defaults

Requirements:
    pip install pillow numpy
"""

import os
import sys
from pathlib import Path

try:
    from PIL import Image
    import numpy as np
except ImportError:
    print("Installing required packages...")
    import subprocess
    subprocess.check_call([sys.executable, "-m", "pip", "install", "pillow", "numpy"])
    from PIL import Image
    import numpy as np


def remove_background(img: Image.Image, tolerance: int = 30) -> Image.Image:
    """Remove background color (detects corners to find bg color)."""
    img = img.convert("RGBA")
    data = np.array(img)
    
    # Sample corners to detect background color
    h, w = data.shape[:2]
    corners = [
        data[0, 0, :3],           # top-left
        data[0, w-1, :3],         # top-right
        data[h-1, 0, :3],         # bottom-left
        data[h-1, w-1, :3],       # bottom-right
    ]
    
    # Use most common corner color as background
    bg_color = np.median(corners, axis=0).astype(np.uint8)
    print(f"  Detected background color: RGB{tuple(bg_color)}")
    
    # Create mask for background pixels
    diff = np.abs(data[:, :, :3].astype(np.int16) - bg_color.astype(np.int16))
    mask = np.all(diff <= tolerance, axis=2)
    
    # Set background to transparent
    data[mask, 3] = 0
    
    return Image.fromarray(data)


def auto_crop(img: Image.Image, padding: int = 2) -> Image.Image:
    """Crop to content bounding box with optional padding."""
    # Get alpha channel
    if img.mode != "RGBA":
        img = img.convert("RGBA")
    
    alpha = np.array(img)[:, :, 3]
    
    # Find non-transparent pixels
    rows = np.any(alpha > 0, axis=1)
    cols = np.any(alpha > 0, axis=0)
    
    if not np.any(rows) or not np.any(cols):
        return img  # All transparent, return as-is
    
    rmin, rmax = np.where(rows)[0][[0, -1]]
    cmin, cmax = np.where(cols)[0][[0, -1]]
    
    # Add padding
    h, w = alpha.shape
    rmin = max(0, rmin - padding)
    rmax = min(h - 1, rmax + padding)
    cmin = max(0, cmin - padding)
    cmax = min(w - 1, cmax + padding)
    
    return img.crop((cmin, rmin, cmax + 1, rmax + 1))


def extract_sprite_strip(img: Image.Image, num_frames: int = 8) -> Image.Image:
    """Extract the sprite strip from an image, assuming horizontal layout."""
    img = img.convert("RGBA")
    w, h = img.size
    
    # If image is much wider than tall, it's likely a horizontal strip
    if w > h * 2:
        # Already a strip, just return cropped version
        return auto_crop(img)
    
    # Otherwise, try to find the strip region (middle section usually)
    # Look for the row with most non-transparent content
    alpha = np.array(img)[:, :, 3]
    row_content = np.sum(alpha > 0, axis=1)
    
    # Find the main content band
    threshold = np.max(row_content) * 0.5
    content_rows = np.where(row_content > threshold)[0]
    
    if len(content_rows) > 0:
        top = content_rows[0]
        bottom = content_rows[-1]
        # Add some padding
        top = max(0, top - 10)
        bottom = min(h, bottom + 10)
        img = img.crop((0, top, w, bottom))
    
    return auto_crop(img)


def resize_to_grid(img: Image.Image, frame_width: int, frame_height: int, num_frames: int) -> Image.Image:
    """Resize sprite strip to exact frame dimensions."""
    img = img.convert("RGBA")
    w, h = img.size
    
    # Calculate current frame width (assuming equal spacing)
    current_frame_w = w // num_frames
    
    # Calculate scale factor to match target frame height
    scale = frame_height / h
    new_w = int(w * scale)
    new_h = frame_height
    
    # Resize with high quality
    img = img.resize((new_w, new_h), Image.Resampling.LANCZOS)
    
    # Now adjust to exact frame widths
    target_w = frame_width * num_frames
    
    if new_w != target_w:
        # Create new image with exact dimensions
        result = Image.new("RGBA", (target_w, frame_height), (0, 0, 0, 0))
        # Center the resized image
        offset = (target_w - new_w) // 2
        result.paste(img, (offset, 0), img)
        return result
    
    return img


def process_sprite_sheet(
    input_path: Path,
    output_path: Path,
    frame_size: tuple[int, int] = (128, 128),
    num_frames: int = 8,
    bg_tolerance: int = 35
):
    """Process a single sprite sheet image."""
    print(f"Processing: {input_path.name}")
    
    # Load image
    img = Image.open(input_path)
    print(f"  Original size: {img.size}")
    
    # Remove background
    img = remove_background(img, tolerance=bg_tolerance)
    
    # Extract/crop sprite strip
    img = extract_sprite_strip(img, num_frames)
    print(f"  After crop: {img.size}")
    
    # Resize to target dimensions
    # img = resize_to_grid(img, frame_size[0], frame_size[1], num_frames)
    # print(f"  Final size: {img.size}")
    
    # Save
    output_path.parent.mkdir(parents=True, exist_ok=True)
    img.save(output_path, "PNG")
    print(f"  Saved: {output_path}")


def create_thumbnail(img: Image.Image, max_size: int = 512) -> Image.Image:
    """Create a smaller preview version of an image."""
    img = img.convert("RGBA")
    w, h = img.size
    
    # Calculate scale to fit within max_size
    scale = min(max_size / w, max_size / h, 1.0)  # Don't upscale
    
    if scale < 1.0:
        new_w = int(w * scale)
        new_h = int(h * scale)
        return img.resize((new_w, new_h), Image.Resampling.LANCZOS)
    
    return img


def create_game_ready_sprite(img: Image.Image, target_height: int = 128) -> Image.Image:
    """Scale sprite to game-ready size while maintaining aspect ratio."""
    img = img.convert("RGBA")
    w, h = img.size
    
    # Scale to target height
    scale = target_height / h
    new_w = int(w * scale)
    new_h = target_height
    
    return img.resize((new_w, new_h), Image.Resampling.LANCZOS)


def batch_process(input_dir: Path, output_dir: Path):
    """Process all images in input directory."""
    
    # Create subdirectories
    processed_dir = output_dir / "full"      # Full resolution, bg removed
    preview_dir = output_dir / "preview"     # Small previews for viewing
    game_dir = output_dir / "game_ready"     # Sized for game use
    
    processed_dir.mkdir(parents=True, exist_ok=True)
    preview_dir.mkdir(parents=True, exist_ok=True)
    game_dir.mkdir(parents=True, exist_ok=True)
    
    # Define processing rules based on filename patterns
    rules = {
        "villager": {"frame_size": (128, 128), "num_frames": 8},
        "soldier": {"frame_size": (128, 128), "num_frames": 8},
        "town_center": {"frame_size": (256, 256), "num_frames": 1},
        "barracks": {"frame_size": (192, 192), "num_frames": 1},
        "tree": {"frame_size": (128, 192), "num_frames": 4},
        "gold": {"frame_size": (128, 128), "num_frames": 4},
        "stone": {"frame_size": (128, 128), "num_frames": 4},
        "berry": {"frame_size": (96, 96), "num_frames": 4},
    }
    
    # Default rule
    default_rule = {"frame_size": (128, 128), "num_frames": 8}
    
    # Track all processed files for summary
    processed_files = []
    
    # Find all PNG images
    for subdir in ["gemini", "lmarena", ""]:
        search_dir = input_dir / subdir if subdir else input_dir
        if not search_dir.exists():
            continue
            
        for img_path in search_dir.glob("*.png"):
            # Determine output name and rule
            name_lower = img_path.stem.lower()
            
            # Find matching rule
            rule = default_rule
            output_name = img_path.stem
            for key, r in rules.items():
                if key in name_lower:
                    rule = r
                    output_name = key
                    break
            
            # Use a cleaner name
            clean_name = img_path.stem.replace("Gemini_Generated_Image_", "gemini_")
            
            # Avoid overwriting - add suffix if exists
            counter = 1
            base_name = clean_name
            while (processed_dir / f"{clean_name}.png").exists():
                clean_name = f"{base_name}_{counter}"
                counter += 1
            
            try:
                print(f"Processing: {img_path.name}")
                
                # Load and process
                img = Image.open(img_path)
                original_size = img.size
                print(f"  Original size: {original_size}")
                
                # Remove background
                img_processed = remove_background(img, tolerance=35)
                
                # Extract/crop sprite strip
                img_processed = extract_sprite_strip(img_processed, rule["num_frames"])
                processed_size = img_processed.size
                print(f"  After crop: {processed_size}")
                
                # Save full resolution (bg removed, cropped)
                full_path = processed_dir / f"{clean_name}.png"
                img_processed.save(full_path, "PNG")
                print(f"  Saved full: {full_path}")
                
                # Save preview (smaller for viewing)
                preview = create_thumbnail(img_processed, max_size=512)
                preview_path = preview_dir / f"{clean_name}.png"
                preview.save(preview_path, "PNG", optimize=True)
                print(f"  Saved preview: {preview_path} ({preview.size})")
                
                # Save game-ready (scaled for actual game use)
                game_sprite = create_game_ready_sprite(img_processed, target_height=128)
                game_path = game_dir / f"{clean_name}.png"
                game_sprite.save(game_path, "PNG", optimize=True)
                print(f"  Saved game-ready: {game_path} ({game_sprite.size})")
                
                processed_files.append({
                    "original": img_path.name,
                    "clean_name": clean_name,
                    "original_size": original_size,
                    "processed_size": processed_size,
                    "preview_size": preview.size,
                    "game_size": game_sprite.size,
                })
                
            except Exception as e:
                print(f"  ERROR: {e}")
                import traceback
                traceback.print_exc()
    
    # Print summary
    print("\n" + "="*60)
    print("PROCESSING SUMMARY")
    print("="*60)
    for f in processed_files:
        print(f"\n{f['clean_name']}:")
        print(f"  Original: {f['original']} {f['original_size']}")
        print(f"  Processed: {f['processed_size']} -> Preview: {f['preview_size']} -> Game: {f['game_size']}")


def main():
    # Default paths
    project_root = Path(__file__).parent.parent
    input_dir = project_root / "generated_assets"
    output_dir = project_root / "client" / "assets" / "sprites" / "processed"
    
    # Override from command line
    if len(sys.argv) >= 2:
        input_dir = Path(sys.argv[1])
    if len(sys.argv) >= 3:
        output_dir = Path(sys.argv[2])
    
    print(f"Input directory: {input_dir}")
    print(f"Output directory: {output_dir}")
    print()
    
    if not input_dir.exists():
        print(f"ERROR: Input directory does not exist: {input_dir}")
        sys.exit(1)
    
    batch_process(input_dir, output_dir)
    
    print()
    print("Done! Processed sprites saved to:", output_dir)
    print()
    print("Output structure:")
    print(f"  {output_dir}/full/     - Full resolution, background removed")
    print(f"  {output_dir}/preview/  - Small previews for viewing (<512px)")
    print(f"  {output_dir}/game_ready/ - Scaled for game use (128px height)")
    print()
    print("Next steps:")
    print("1. Review the preview/ folder to identify each sprite")
    print("2. Rename appropriately (villager.png, soldier.png, etc.)")
    print("3. Copy from game_ready/ to client/assets/sprites/units/, buildings/, or resources/")


if __name__ == "__main__":
    main()
