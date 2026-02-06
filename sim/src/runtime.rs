use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use shared::{GameCommand, PlayerId, StampedCommand, WorldSnapshot};

use crate::{CommandBuffer, SimPlugin, SimWorld, SnapshotEvent, TickScheduler};

/// Thin runtime wrapper intended for web clients.
///
/// API surface maps to the proposed WASM bridge:
/// - `init`
/// - `enqueue_commands`
/// - `step`
/// - `get_snapshot`
pub struct SimRuntime {
    app: App,
    latest_snapshot: Option<WorldSnapshot>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{GameCommand, PlayerId, StampedCommand};

    #[test]
    fn step_emits_snapshot() {
        let mut runtime = SimRuntime::init(20, 12345);
        runtime.add_player(PlayerId::PLAYER_1);

        runtime.step();

        let snapshot = runtime
            .get_snapshot()
            .expect("a snapshot should be emitted every step");
        assert_eq!(snapshot.tick, 1);
    }

    #[test]
    fn queued_commands_are_accepted_and_step_advances() {
        let mut runtime = SimRuntime::init(20, 12345);
        runtime.add_player(PlayerId::PLAYER_1);

        runtime.enqueue_commands(vec![StampedCommand::new(
            1,
            PlayerId::PLAYER_1,
            0,
            GameCommand::Stop {
                entities: Vec::new(),
            },
        )]);

        runtime.step();

        let snapshot = runtime
            .get_snapshot()
            .expect("a snapshot should still be emitted after processing commands");
        assert_eq!(snapshot.tick, 1);
    }
}

impl SimRuntime {
    /// Initialize the deterministic sim runtime.
    pub fn init(tick_rate: u32, rng_seed: u64) -> Self {
        let mut app = App::new();
        app.add_plugins(SimPlugin {
            tick_rate,
            rng_seed,
        });
        app.world_mut().insert_resource(Time::<Fixed>::default());

        Self {
            app,
            latest_snapshot: None,
        }
    }

    /// Add a player to sim world state.
    pub fn add_player(&mut self, player_id: PlayerId) {
        self.app
            .world_mut()
            .resource_mut::<SimWorld>()
            .add_player(player_id);
    }

    /// Queue commands for future execution.
    pub fn enqueue_commands(&mut self, commands: Vec<StampedCommand>) {
        let mut buffer = self.app.world_mut().resource_mut::<CommandBuffer>();
        for command in commands {
            buffer.push(command);
        }
    }

    /// Advance the simulation by one fixed tick.
    pub fn step(&mut self) {
        self.app.world_mut().run_schedule(FixedUpdate);

        let mut events = self
            .app
            .world_mut()
            .resource_mut::<Messages<SnapshotEvent>>();

        for event in events.drain() {
            self.latest_snapshot = Some(event.snapshot.clone());
        }
    }

    /// Read the latest generated snapshot.
    pub fn get_snapshot(&self) -> Option<&WorldSnapshot> {
        self.latest_snapshot.as_ref()
    }

    /// Convenience helper to queue commands for current tick + input delay.
    pub fn enqueue_for_next_tick(
        &mut self,
        player_id: PlayerId,
        input_delay_ticks: u64,
        commands: Vec<GameCommand>,
    ) {
        let current_tick = self.app.world().resource::<TickScheduler>().tick();
        let target_tick = current_tick + input_delay_ticks + 1;

        let stamped: Vec<StampedCommand> = commands
            .into_iter()
            .enumerate()
            .map(|(idx, command)| StampedCommand::new(target_tick, player_id, idx as u64, command))
            .collect();

        self.enqueue_commands(stamped);
    }
}
