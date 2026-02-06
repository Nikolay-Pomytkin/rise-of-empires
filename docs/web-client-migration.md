# Web Client Migration Plan

This branch starts the client reboot path with deterministic Rust sim retained as authoritative logic.

## Direction

- Keep `sim/` as source of truth for gameplay + determinism.
- Expose a compact runtime API for WASM clients:
  - `init`
  - `enqueue_commands`
  - `step`
  - `get_snapshot`
- Rebuild rendering/UI/input in a web-first TypeScript client (`web-client/`) using Pixi.

## What is included

1. `sim::SimRuntime` wrapper for browser-oriented control flow.
2. Optional `sim::wasm_api` (`wasm_api` feature) exposing the runtime through `wasm-bindgen`.
3. A new `web-client/` TypeScript + Vite + Pixi skeleton that runs a render loop and sim step loop.
4. Bun-first workflow for the web client (`bun install`, `bun run dev`) with standard npm-compatible package metadata.

## Next steps

- Replace `MockSimBridge` with real WASM bindings from the `sim` crate.
- Map browser input into `GameCommand` creation and command stamping.
- Render snapshot entities with sprites, selection outlines, and UI overlays.
- Once parity is reached, remove Bevy rendering client.


## Bun/network troubleshooting

If `bun install` fails, run:

```bash
cd web-client
bun run doctor:registry
```

A 403 status from the registry check indicates proxy/firewall policy is blocking npm registry access, not a client code issue.
