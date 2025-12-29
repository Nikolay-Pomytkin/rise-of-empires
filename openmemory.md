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
- `setup_grid` - Creates ground plane and grid lines at startup
- `update_unit_visuals` - Spawns sprites for units
- `update_building_visuals_sprite` - Spawns sprites for buildings
- `update_resource_node_visuals` - Spawns sprites for resources
- `sync_transforms` - Updates Transform from SimPosition changes

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
- Ground plane: Z = -100 (furthest back)
- Grid lines: Z = -99
- Game entities: Z = 0-100 (positive = in front)
- UI is handled by egui separately

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
**Cause**: Z-ordering incorrect (entities behind ground)
**Solution**: Ground at negative Z, entities at positive Z

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
