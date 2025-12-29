# Sprite Asset Guide

This document contains specifications for all game sprites and AI generation prompts for creating isometric HD assets in the style of Age of Empires 2: Definitive Edition.

## Art Style Guide

### Isometric Perspective
- **Angle**: 2:1 isometric (26.57 degrees)
- **Camera**: Top-down at 45-degree angle
- **Lighting**: Consistent top-left light source
- **Shadows**: Soft drop shadows, not too dark

### Visual Style
- Age of Empires 2 DE inspired
- Detailed but readable at small sizes
- Saturated colors with distinct silhouettes
- Clean edges, no excessive noise
- Transparent backgrounds (PNG format)

### Color Palette
- **Player 1 (Blue)**: #3380E6 primary, team-colored elements
- **Player 2 (Red)**: #E65033 primary, team-colored elements
- **Neutral**: Gray tones for non-player elements
- **Resources**: Natural, saturated colors

---

## Sprite Sheet Specifications

### Unit Sprite Sheets

Units require 8 directional sprites for isometric view. Each animation state has 8 rows (one per direction).

#### Layout Convention
```
Row 0: South (facing camera)
Row 1: Southwest
Row 2: West
Row 3: Northwest
Row 4: North (facing away)
Row 5: Northeast
Row 6: East
Row 7: Southeast
```

#### Villager (`villager_sheet.png`)
- **Sheet Size**: 1024x1024 pixels
- **Frame Size**: 128x128 pixels
- **Animations**:
  - Idle: 8 frames × 8 directions
  - Walk: 8 frames × 8 directions
  - Gather: 8 frames × 8 directions
  - Die: 6 frames × 8 directions
- **Total Frames**: ~240 (30 rows × 8 columns)

#### Soldier (`soldier_sheet.png`)
- **Sheet Size**: 1024x1024 pixels
- **Frame Size**: 128x128 pixels
- **Animations**:
  - Idle: 8 frames × 8 directions
  - Walk: 8 frames × 8 directions
  - Attack: 8 frames × 8 directions
  - Die: 6 frames × 8 directions
- **Total Frames**: ~240 (30 rows × 8 columns)

### Building Sprite Sheets

Buildings are single-direction with construction/activity stages.

#### Town Center (`town_center_sheet.png`)
- **Sheet Size**: 512x512 pixels
- **Frame Size**: 256x256 pixels
- **Frames**:
  - Row 0: Construction stages (4 frames: 25%, 50%, 75%, 100%)
  - Row 1: Activity states (idle, producing)

#### Barracks (`barracks_sheet.png`)
- **Sheet Size**: 384x384 pixels
- **Frame Size**: 192x192 pixels
- **Frames**:
  - Row 0: Construction stages (4 frames)
  - Row 1: Activity states (idle, producing)

### Resource Node Sprite Sheets

Resources show depletion stages.

#### Tree (`tree_sheet.png`)
- **Sheet Size**: 512x192 pixels
- **Frame Size**: 128x192 pixels (taller than wide)
- **Frames**: 4 depletion stages (full → 75% → 50% → stump)

#### Gold Mine (`gold_mine_sheet.png`)
- **Sheet Size**: 512x128 pixels
- **Frame Size**: 128x128 pixels
- **Frames**: 4 depletion stages

#### Stone Quarry (`stone_quarry_sheet.png`)
- **Sheet Size**: 512x128 pixels
- **Frame Size**: 128x128 pixels
- **Frames**: 4 depletion stages

#### Berry Bush (`berry_bush_sheet.png`)
- **Sheet Size**: 384x96 pixels
- **Frame Size**: 96x96 pixels
- **Frames**: 4 depletion stages

---

## AI Generation Prompts

### General Tips for AI Image Generation
1. Always specify "transparent background" or "alpha channel"
2. Include "game asset" or "sprite" to get clean results
3. Mention "isometric 2:1" for correct perspective
4. Reference "Age of Empires 2 HD" for style consistency
5. Specify exact pixel dimensions when possible
6. Request "single frame" or "sprite sheet" explicitly

---

### Unit Prompts

#### Villager - Idle (8 directions)
```
Professional game asset sprite sheet, isometric 2:1 perspective medieval peasant villager,
8 directions arranged in single row (S, SW, W, NW, N, NE, E, SE), 128x128 pixels per frame,
idle stance holding wicker basket, Age of Empires 2 HD style, soft cel shading,
transparent PNG background, single character centered per frame,
consistent top-left lighting with soft shadow, earth tone clothing (brown tunic, beige pants),
game-ready asset, clean edges, no anti-aliasing artifacts
```

#### Villager - Walk Animation (single direction, 8 frames)
```
Professional game asset sprite sheet, isometric 2:1 perspective medieval peasant walking,
8 frame walk cycle arranged horizontally, 128x128 pixels per frame, facing south (toward camera),
Age of Empires 2 HD style, transparent background, smooth animation progression,
earth tone clothing, carrying basket, consistent lighting, game-ready sprite sheet
```

#### Villager - Gathering Animation
```
Professional game asset sprite sheet, isometric medieval peasant gathering resources,
8 frame gathering animation, 128x128 pixels per frame, facing south,
bending down motion collecting berries/crops, Age of Empires 2 HD style,
transparent background, smooth animation, earth tones, game asset
```

#### Soldier - Idle (8 directions)
```
Professional game asset sprite sheet, isometric 2:1 perspective medieval infantry soldier,
8 directions in row (S, SW, W, NW, N, NE, E, SE), 128x128 pixels per frame,
standing guard pose with sword and round shield, chainmail armor, metal helmet,
Age of Empires 2 HD style, transparent background, neutral gray tabard (for team color tinting),
consistent top-left lighting, game-ready asset
```

#### Soldier - Attack Animation
```
Professional game asset sprite sheet, isometric medieval soldier sword attack animation,
8 frame attack swing cycle, 128x128 pixels per frame, facing south,
overhead sword slash motion, chainmail armor, round shield on back,
Age of Empires 2 HD style, transparent background, dynamic action poses, game asset
```

#### Soldier - Walk Animation
```
Professional game asset sprite sheet, isometric medieval soldier marching animation,
8 frame walk cycle, 128x128 pixels per frame, facing south toward camera,
sword at side, shield on arm, chainmail armor, determined stride,
Age of Empires 2 HD style, transparent background, game-ready
```

---

### Building Prompts

#### Town Center
```
Professional game asset, isometric 2:1 perspective medieval town center building,
256x256 pixels, large central structure with stone foundation and wooden upper floors,
thatched roof with smoke from chimney, open market stalls on ground level,
Age of Empires 2 HD style, transparent background, detailed but readable at 50% scale,
warm wood browns and gray stone colors, top-left lighting with soft shadows
```

#### Town Center - Construction Stages (sprite sheet)
```
Professional game asset sprite sheet, isometric medieval town center construction stages,
4 frames horizontal: foundation only, walls half-built, roof framing, completed building,
256x256 pixels per frame, Age of Empires 2 HD style, transparent background,
scaffolding visible in construction phases, consistent perspective and lighting
```

#### Barracks
```
Professional game asset, isometric 2:1 perspective medieval barracks military building,
192x192 pixels, wooden longhouse structure with weapon racks outside,
training dummy visible, military banners, fortified wooden walls,
Age of Empires 2 HD style, transparent background, functional military aesthetic,
darker wood tones, top-left lighting
```

#### Barracks - Construction Stages
```
Professional game asset sprite sheet, isometric medieval barracks construction,
4 frames: foundation, frame structure, walls, completed with details,
192x192 pixels per frame, Age of Empires 2 HD style, transparent background
```

---

### Resource Node Prompts

#### Tree - Depletion Stages
```
Professional game asset sprite sheet, isometric 2:1 perspective oak tree resource,
4 depletion stages horizontal: full lush tree, partially harvested (75%), 
heavily harvested (50%), tree stump only,
128x192 pixels per frame (taller format), rich green foliage fading to bare branches,
Age of Empires 2 HD style, transparent background, game resource node
```

#### Gold Mine
```
Professional game asset, isometric 2:1 perspective gold mine entrance,
128x128 pixels, rocky cave opening carved into hillside, visible gold veins in rock,
wooden support beams at entrance, mining cart tracks, warm golden glow from within,
Age of Empires 2 HD style, transparent background, resource node for RTS game
```

#### Gold Mine - Depletion Stages
```
Professional game asset sprite sheet, isometric gold mine depletion stages,
4 frames: rich gold veins visible, moderate deposits, sparse gold, depleted gray rock,
128x128 pixels per frame, Age of Empires 2 HD style, transparent background
```

#### Stone Quarry
```
Professional game asset, isometric 2:1 perspective stone quarry,
128x128 pixels, cut stone blocks stacked, quarry pit with exposed rock face,
wooden crane/pulley system, gray and tan stone colors,
Age of Empires 2 HD style, transparent background, resource node
```

#### Berry Bush
```
Professional game asset, isometric 2:1 perspective berry bush food resource,
96x96 pixels, lush green shrub with bright red/purple berries,
natural wild appearance, Age of Empires 2 HD style, transparent background,
game resource node, vibrant colors
```

#### Berry Bush - Depletion Stages
```
Professional game asset sprite sheet, isometric berry bush depletion,
4 frames: full berries, 75% berries, sparse berries, empty bush,
96x96 pixels per frame, green foliage throughout, only berries decrease,
Age of Empires 2 HD style, transparent background
```

---

## Alternative: Quick Placeholder Assets

If you need quick placeholders before generating custom assets:

### Free Asset Sources
1. **Kenney.nl** - Free CC0 game assets
   - Search for "RTS" or "Medieval" packs
   - High quality, consistent style

2. **OpenGameArt.org** - Various licenses
   - Search "isometric medieval"
   - Check license compatibility

3. **itch.io Asset Packs** - Mix of free and paid
   - Many RTS-specific packs available

### Procedural Placeholders
The game currently uses colored rectangles as fallbacks:
- Units: Player-colored rectangles
- Buildings: Tan/brown squares
- Resources: Color-coded by type (green=food, brown=wood, gold=gold, gray=stone)

---

## File Organization

```
client/assets/sprites/
├── buildings/
│   ├── town_center.png          # Single static sprite
│   ├── town_center_sheet.png    # Animated sprite sheet
│   ├── barracks.png
│   └── barracks_sheet.png
├── units/
│   ├── villager.png             # Single static sprite
│   ├── villager_sheet.png       # Full animation sheet
│   ├── soldier.png
│   └── soldier_sheet.png
├── resources/
│   ├── tree.png
│   ├── tree_sheet.png
│   ├── gold_mine.png
│   ├── gold_mine_sheet.png
│   ├── stone_quarry.png
│   ├── stone_quarry_sheet.png
│   ├── berry_bush.png
│   └── berry_bush_sheet.png
└── README.md
```

---

## Integration Notes

### For Static Sprites (current implementation)
Place single PNG files in the appropriate folders. The game will load them automatically on native builds.

### For Animated Sprite Sheets (future)
1. Create sprite sheet following specifications above
2. Update `assets/data/animations.roe` with correct frame counts
3. The animation system will handle frame progression

### Team Colors
For units and buildings that need team coloring:
1. Create sprites with neutral gray in team-colored areas
2. Use shader or runtime tinting to apply player colors
3. Alternatively, create separate sprites per team color
