//! Bridge between client and simulation
//!
//! Handles command feeding and snapshot receiving.

use bevy::prelude::*;

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LatestSnapshot::default())
            .add_systems(Update, receive_snapshots);
    }
}

/// Stores the latest snapshot for UI/rendering use
#[derive(Resource, Default)]
pub struct LatestSnapshot {
    pub snapshot: Option<shared::WorldSnapshot>,
}

/// Receive snapshots from the simulation
fn receive_snapshots(
    mut snapshot_events: EventReader<sim::SnapshotEvent>,
    mut latest: ResMut<LatestSnapshot>,
) {
    for event in snapshot_events.read() {
        latest.snapshot = Some(event.snapshot.clone());
    }
}

