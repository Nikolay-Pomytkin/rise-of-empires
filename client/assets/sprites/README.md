# Sprite Assets

This folder contains all game sprites for Rise RTS.

## Quick Start

See **[ASSET_GUIDE.md](ASSET_GUIDE.md)** for:
- Complete sprite sheet specifications
- AI generation prompts (Midjourney/DALL-E/Stable Diffusion)
- Art style guidelines
- File organization

## Current Assets

### Units (`units/`)
- `villager.png` - 128x128, transparent background
- `soldier.png` - 128x128, transparent background

### Buildings (`buildings/`)
- `town_center.png` - 256x256, transparent background
- `barracks.png` - 192x192, transparent background

### Resources (`resources/`)
- `tree.png` - 128x192 (taller), transparent background
- `gold_mine.png` - 128x128, transparent background
- `stone_quarry.png` - 128x128, transparent background
- `berry_bush.png` - 96x96, transparent background

## Fallback Rendering

If sprite files are missing or fail to load, the game uses colored rectangles:
- **Units**: Player-colored (blue/red) rectangles
- **Buildings**: Tan/brown squares
- **Resources**: Color-coded (green=food, brown=wood, gold=gold, gray=stone)

## Adding New Assets

1. Create PNG with transparent background
2. Follow size specifications in ASSET_GUIDE.md
3. Place in appropriate subfolder
4. Assets are hot-reloaded on native builds (no restart needed)
