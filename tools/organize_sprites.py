#!/usr/bin/env python3
"""
Organize processed sprites into game asset folders.

This script renames and copies sprites to the appropriate game directories.
"""

import shutil
from pathlib import Path

# Define source and destination
PROJECT_ROOT = Path(__file__).parent.parent
PROCESSED_DIR = PROJECT_ROOT / "client" / "assets" / "sprites" / "processed" / "game_ready"
SPRITES_DIR = PROJECT_ROOT / "client" / "assets" / "sprites"

# Create target directories
UNITS_DIR = SPRITES_DIR / "units"
BUILDINGS_DIR = SPRITES_DIR / "buildings"
RESOURCES_DIR = SPRITES_DIR / "resources"

for d in [UNITS_DIR, BUILDINGS_DIR, RESOURCES_DIR]:
    d.mkdir(parents=True, exist_ok=True)

# Mapping: source filename -> (destination folder, new name)
SPRITE_MAPPING = {
    # Units
    "gemini_5z9via5z9via5z9v.png": (UNITS_DIR, "soldier_idle.png"),
    "gemini_bsa57vbsa57vbsa5.png": (UNITS_DIR, "soldier_directions.png"),
    "gemini_4qp0y44qp0y44qp0.png": (UNITS_DIR, "villager_directions.png"),
    "gemini_948l3r948l3r948l.png": (UNITS_DIR, "villager_walk.png"),
    "gemini_ns2my5ns2my5ns2m.png": (UNITS_DIR, "villager_gather.png"),
    "mai-image-1_a_Single_horizontal_sp.png": (UNITS_DIR, "villager.png"),  # Main villager sprite
    "imagen-4.0-ultra-generate-001_a_Professional_2D_game.png": (UNITS_DIR, "villager_walk_alt.png"),
    
    # Buildings
    "gemini_9vxzk59vxzk59vxz.png": (BUILDINGS_DIR, "town_center.png"),
    "gemini_douokdouokdouokd.png": (BUILDINGS_DIR, "barracks.png"),
    
    # The mixed sheet needs special handling - skip for now
    # "gemini_3zy73p3zy73p3zy7.png": needs to be split
}

def main():
    print("Organizing sprites...")
    print(f"Source: {PROCESSED_DIR}")
    print()
    
    for src_name, (dest_dir, dest_name) in SPRITE_MAPPING.items():
        src_path = PROCESSED_DIR / src_name
        dest_path = dest_dir / dest_name
        
        if src_path.exists():
            shutil.copy2(src_path, dest_path)
            print(f"✓ {src_name}")
            print(f"  -> {dest_path.relative_to(PROJECT_ROOT)}")
        else:
            print(f"✗ {src_name} - NOT FOUND")
    
    print()
    print("Done! Sprites organized into:")
    print(f"  - {UNITS_DIR.relative_to(PROJECT_ROOT)}/")
    print(f"  - {BUILDINGS_DIR.relative_to(PROJECT_ROOT)}/")
    print(f"  - {RESOURCES_DIR.relative_to(PROJECT_ROOT)}/")
    print()
    print("Note: gemini_3zy73p3zy73p3zy7.png is a mixed sheet and needs manual splitting.")
    print("You can use an image editor to extract:")
    print("  - Top row: villager idle/walk")
    print("  - Middle row: soldier walk")
    print("  - Bottom row: soldier attack")

if __name__ == "__main__":
    main()
