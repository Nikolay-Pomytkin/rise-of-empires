//! WASM bindings for browser clients.

use std::cell::RefCell;

use shared::{PlayerId, StampedCommand, WorldSnapshot};
use wasm_bindgen::prelude::*;

use crate::SimRuntime;

#[wasm_bindgen]
pub struct WasmSim {
    inner: RefCell<SimRuntime>,
}

#[wasm_bindgen]
impl WasmSim {
    #[wasm_bindgen(constructor)]
    pub fn init(tick_rate: u32, rng_seed: u64) -> Self {
        Self {
            inner: RefCell::new(SimRuntime::init(tick_rate, rng_seed)),
        }
    }

    pub fn add_player(&self, player_id: u8) {
        self.inner.borrow_mut().add_player(PlayerId(player_id));
    }

    /// Accepts a RON array of `StampedCommand`.
    pub fn enqueue_commands(&self, commands_ron: &str) -> Result<(), JsValue> {
        let commands: Vec<StampedCommand> = ron::from_str(commands_ron)
            .map_err(|e| JsValue::from_str(&format!("invalid command payload: {e}")))?;
        self.inner.borrow_mut().enqueue_commands(commands);
        Ok(())
    }

    pub fn step(&self) {
        self.inner.borrow_mut().step();
    }

    /// Returns the latest snapshot as RON.
    pub fn get_snapshot(&self) -> Result<String, JsValue> {
        let snapshot: Option<WorldSnapshot> = self.inner.borrow().get_snapshot().cloned();
        ron::ser::to_string(&snapshot)
            .map_err(|e| JsValue::from_str(&format!("failed to serialize snapshot: {e}")))
    }
}
