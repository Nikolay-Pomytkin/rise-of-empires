# Rise of Empires - OpenMemory Guide

This document serves as a living index of the project for AI assistants.

## Project Overview

**Rise of Empires** is a web-first RTS game inspired by Rise of Nations, built with:
- **Rust** for deterministic simulation/runtime
- **TypeScript + Pixi** for rendering/UI/input
- Targets: modern browsers (Pixi/WebGL) with WASM sim integration

### Core Concept
- Tile-grid map with faux-3D (birds-eye/tilted) presentation
- Deterministic simulation for replays/lockstep multiplayer
- Empire/Leader selection system with unique bonuses
- Resource gathering, production queues, tech tree, combat

## Architecture

### Crate Responsibilities

| Crate/Folder | Purpose |
|-------|---------|
| `shared` | IDs, resources, commands, empires |
| `sim` | Deterministic game logic (ECS only) |
| `web-client` | Pixi renderer, UI, input |
| `tools` | CLI utilities |

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

### Rendering Pipeline (Web)
- `web-client/src/render/pixiRenderer.ts` - Pixi `Application` setup and canvas mount
- `web-client/src/engine/` - Render loop, sim step loop scaffolding

### UI Systems
- Implemented in Pixi/DOM inside `web-client/` as the client matures.

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
Pixi layering uses container order/zIndex on display objects as the renderer is expanded.

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

### Debug Overlay
Expose debug overlays in the Pixi client (e.g., text overlays) as the UI matures.

### Logging
Use `bevy::log::info!()` for debug output, visible in terminal.

## Known Issues & Solutions

### Issue: Sprites not rendering
**Cause**: Pixi display object not added to the stage or assets not loaded
**Solution**:
1. Ensure the display object is added to the stage/container
2. Verify asset loading before sprite creation

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
| Rendering | `web-client/src/render/` |
| UI modules | `web-client/src/` |
| Sim components | `sim/src/components/` |
| Sim systems | `sim/src/systems/` |
