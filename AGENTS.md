# AGENTS

## Project overview
Rise RTS is a real-time strategy game built with Rust and Bevy 0.17. The repository contains deterministic simulation code, shared types, a legacy Bevy client, a newer TypeScript web client skeleton, and tooling for headless simulation and replay.

## Repository layout
- `shared/`: Shared types (IDs, commands, resources, snapshots).
- `sim/`: Deterministic simulation core.
- `client/`: Bevy client (rendering, input, UI).
- `web-client/`: TypeScript web client skeleton (Pixi).
- `tools/`: Headless sim runner and replay tools.
- `assets/`: Game data files (`.roe` RON format).

## Build & run
- Native client: `cargo run -p client` (or `cargo run -p client --features dev` for faster iteration).
- Web client (WASM): `cd client && trunk serve` (ensure `wasm32-unknown-unknown` target and `trunk` installed).
- Headless sim: `cargo run -p tools --bin headless_sim -- <ticks> <seed> [commands.ron]`.

## Testing
- Workspace tests: `cargo test --workspace`.

## Best practices
- **Determinism first**: Keep simulation code deterministic. Avoid non-deterministic sources (wall clock time, random without seeded RNG, unordered iteration without stable ordering).
- **Data-driven content**: Prefer updating `.roe` data files for units/techs over hard-coding. Keep enums in `shared` in sync with data.
- **Performance**: Simulation is tick-based; keep per-tick systems efficient and avoid unnecessary allocations.
- **Separation of concerns**: Shared types live in `shared/`, simulation logic in `sim/`, rendering/UI in `client/` or `web-client/`.
- **Web client**: Follow `docs/web-client-migration.md` for migration strategy and runtime bridge API.
- **Assets**: Use existing schemas in `assets/data/` and verify hot-reload compatibility.

## Common updates
- **New units**: Update `shared/src/commands.rs` (`UnitType`), `assets/data/units.roe`, `sim/src/systems/production.rs`, and `client/src/render/units.rs`.
- **New buildings**: Update `shared/src/commands.rs` (`BuildingType`), simulation components and spawn logic, and rendering.
- **New techs**: Update `assets/data/techs.roe` (effects handled by modifier system).
