# Rise of Empires - OpenMemory Guide

This document serves as a living index of the project for AI assistants.

## Project Overview

**Rise of Empires** is a native cross-platform RTS game inspired by Rise of Nations, built with:
- **Rust** programming language
- **Bevy 0.17.0** game engine (ECS architecture)
- **bevy_egui 0.37.0** for UI
- Targets: macOS, Linux, Windows (native) + WASM (web)

### Core Concept
- Tile-grid map with faux-3D (birds-eye/tilted) presentation
- Deterministic simulation for replays/lockstep multiplayer
- Empire/Leader selection system with unique bonuses
- Resource gathering, production queues, tech tree, combat

## Architecture

### Crate Responsibilities

| Crate | Purpose | Bevy Dependency |
|-------|---------|-----------------|
| `shared` | IDs, resources, commands, empires | None |
| `sim` | Deterministic game logic | Minimal (ECS only) |
| `client` | Rendering, UI, input, saves | Full |
| `tools` | CLI utilities | Minimal |

### Key Resources
- `SimWorld` - Entity registry, player data
- `EmpireData` - Loaded empire/leader definitions
- `PlayerModifiers` - Per-player bonuses
- `GameSetupData` - Selected empire/leader during setup
- `GridConfig` - Map size and tile configuration

### Important Components
- `SimEntity` - Links Bevy entity to sim ID
- `SimPosition` - World position (x, z coordinates)
- `Owner` - Player ownership
- `Unit`, `Building`, `ResourceNode` - Entity types
- `HasVisual` - Marker for entities with sprites

## User Defined Namespaces

- [Leave blank - user populates]

## Components & Systems

### Rendering Pipeline
- `setup_grid` - Creates ground plane and grid lines on `OnEnter(InGame)`
- `cleanup_grid` - Despawns grid and game entities on `OnExit(InGame)`
- `update_unit_visuals` - Spawns sprites for units
- `update_building_visuals_sprite` - Spawns sprites for buildings
- `update_resource_node_visuals` - Spawns sprites for resources
- `sync_transforms` - Updates Transform from SimPosition changes

**Note**: Grid is NOT visible in menus - only spawned when entering InGame state.

### UI Systems
- `ui_main_menu` - Title screen
- `ui_play_menu` - New game / Load game selection
- `ui_empire_select` - Empire selection grid
- `ui_leader_select` - Leader selection for chosen empire
- `ui_resources_panel` - Top-left resource display
- `ui_building_panel` - Bottom build menu
- `ui_debug_overlay` - Debug info panel (camera, entities)

### Game Setup Flow
1. MainMenu → PlayMenu → GameSetup (EmpireSelect → LeaderSelect)
2. `setup_game` runs on `OnEnter(GameState::InGame)`
3. Spawns Town Center, villagers, resource nodes
4. Applies empire/leader bonuses to PlayerModifiers

## Patterns & Conventions

### State Machine Pattern
```rust
// Define states
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    // ...
}

// Register
app.init_state::<GameState>();

// Transition
next_state.set(GameState::InGame);

// Conditional systems
.run_if(in_state(GameState::InGame))
```

### 2D Rendering Z-Order
Z-ordering uses positive values (higher Z = rendered on top):
```rust
// Defined in client/src/render/grid.rs
pub mod layers {
    pub const GROUND: f32 = 0.0;
    pub const GRID_LINES: f32 = 1.0;
    pub const RESOURCES: f32 = 2.0;
    pub const BUILDINGS: f32 = 3.0;
    pub const UNITS_BASE: f32 = 4.0;
    pub const UNITS_MAX: f32 = 10.0;
    pub const SELECTION: f32 = 100.0;
    pub const PLACEMENT_GHOST: f32 = 101.0;
}
```
- Camera at (0, 0, 0) with OrthographicProjection (near=-1000, far=1000)
- UI is handled by egui separately (not affected by sprite Z)
- Ground plane color: `Color::srgb(0.2, 0.45, 0.15)` (grass green)
- Background clear color: `Color::srgb(0.08, 0.08, 0.12)` (dark blue-gray)

### Data-Driven Definitions
Empire data in RON format with newtype syntax:
```ron
(
    id: EmpireId("romans"),
    name: "Roman Empire",
    leaders: [
        (
            id: LeaderId("julius_caesar"),
            name: "Julius Caesar",
            // ...
        ),
    ],
)
```

## Debug Tools

### Debug Overlay (`ui_debug_overlay`)
Shows in top-right during gameplay:
- Camera position, zoom, scale, near/far
- Window size, cursor position
- Entity counts (Total, SimEntity, HasVisual, Sprites)
- Game object counts (Units, Buildings, Resources)

### Logging
Use `bevy::log::info!()` for debug output, visible in terminal.

## Known Issues & Solutions

### Issue: Sprites not rendering
**Cause**: Z-ordering incorrect or missing Visibility component
**Solution**: 
1. Use layer constants from `grid.rs::layers` module
2. Camera at Z=0, ground at Z=0, entities at Z=2-10
3. Ensure `Visibility::Visible` is added when spawning sprites manually

### Issue: Empire selection flickers
**Cause**: State transition race condition
**Solution**: Use `egui::Window` instead of `CentralPanel`, add internal state checks

### Issue: RON parsing fails for newtypes
**Cause**: Missing struct syntax
**Solution**: Use `EmpireId("value")` not `"value"`

## File Locations

| What | Where |
|------|-------|
| Empire data | `assets/data/empires/*.roe` |
| Game states | `client/src/game_state.rs` |
| Rendering | `client/src/render/` |
| UI modules | `client/src/ui/` |
| Sim components | `sim/src/components/` |
| Sim systems | `sim/src/systems/` |
