# Rise RTS

A Rise of Nations-inspired real-time strategy game built with Rust and Bevy 0.17.

## Features

- **Deterministic Simulation**: Fixed-tick simulation with seeded RNG for replay and lockstep multiplayer support
- **Economy System**: Resource gathering (food, wood, gold, stone), villagers, and production queues
- **Tech Tree**: Data-driven technology system with ages and upgrades
- **Cross-Platform**: Runs on Windows, macOS, and Linux

## Project Structure

```
rise/
├── shared/     # Shared types (IDs, commands, resources, snapshots)
├── sim/        # Deterministic simulation core
├── client/     # Bevy client (rendering, input, UI)
├── tools/      # Headless sim runner, replay tools
└── assets/     # Game data files (RON format)
```

## Quick Start

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- For Linux: `sudo apt-get install libasound2-dev libudev-dev libxkbcommon-dev`

### Running the Client

```bash
cargo run -p client
```

### Controls

- **WASD / Arrow Keys**: Pan camera
- **Mouse Scroll**: Zoom in/out
- **Left Click**: Select units
- **Shift + Left Click**: Add to selection
- **Left Drag**: Box select
- **Right Click**: Move selected units
- **Right Click on Resource**: Gather from resource node
- **S**: Stop current action

### Running Headless Simulation

```bash
cargo run -p tools --bin headless_sim -- <ticks> <seed> [commands.ron]
```

### Running Tests

```bash
cargo test --workspace
```

## Architecture

### Determinism & Command Model

The simulation is designed for deterministic replay and lockstep multiplayer:

1. **Fixed Tick Rate**: All game logic runs at a fixed rate (default 20 Hz)
2. **Stamped Commands**: Player inputs are converted to commands with tick + player_id stamps
3. **Deterministic Ordering**: Commands are processed in order: tick → player_id → sequence
4. **Seeded RNG**: All random operations use a deterministic Xoshiro128++ RNG

```
┌─────────┐     ┌──────────────┐     ┌─────────┐     ┌──────────┐
│ Client  │────▶│ CommandBuffer │────▶│ SimTick │────▶│ Snapshot │
│ Input   │     │ (stamped)     │     │ Systems │     │ Output   │
└─────────┘     └──────────────┘     └─────────┘     └──────────┘
```

### Snapshots

Each tick produces a `WorldSnapshot` containing:
- Entity states (position, health, type, owner)
- Player states (resources, population, age)
- Gatherer states, production queues, etc.

Snapshots can be hashed for replay validation.

## Adding New Content

### New Units

1. Add unit type to `shared/src/commands.rs` (`UnitType` enum)
2. Add unit definition to `assets/data/units.ron`
3. Add component setup in `sim/src/systems/production.rs`
4. Add visual rendering in `client/src/render/units.rs`

### New Buildings

1. Add building type to `shared/src/commands.rs` (`BuildingType` enum)
2. Add building component in `sim/src/components/building.rs`
3. Add spawn logic and visual rendering

### New Technologies

1. Edit `assets/data/techs.ron` to add new tech definitions
2. Tech effects are automatically applied through the modifier system
3. Available effects:
   - `GatherRateBonus { resource, percent }`
   - `CarryCapacityBonus { amount }`
   - `BuildSpeedBonus { percent }`
   - `ProductionSpeedBonus { percent }`
   - `UnitHpBonus { percent }`
   - `UnitAttackBonus { amount }`

## TODO / Future Features

- [ ] **A\* Pathfinding**: Replace naive steering with proper pathfinding
- [ ] **Formations**: Group movement with formation shapes
- [ ] **Fog of War**: Vision system and unexplored areas
- [ ] **Projectiles**: Ranged unit attack animations
- [ ] **Lockstep Netcode**: Multiplayer synchronization
- [ ] **Replay System**: Record and playback games
- [ ] **Diff-based Snapshots**: Bandwidth optimization for netcode
- [ ] **Building Construction**: Builder units and construction progress
- [ ] **More Unit Types**: Archers, cavalry, siege weapons
- [ ] **Map Editor**: Create custom maps

## Data Files

Game data is stored in RON format for easy editing:

- `assets/data/techs.ron` - Technology tree definitions
- `assets/data/units.ron` - Unit type definitions

Hot reloading is enabled - changes to these files are picked up automatically.

## Building for Release

See [docs/packaging.md](docs/packaging.md) for platform-specific build instructions.

```bash
# Release build
cargo build --release -p client

# The binary will be at target/release/client
```

## License

MIT License - see LICENSE file for details.

